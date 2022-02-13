//! WirePlumber module example
//!
//! An example showing how to write a simple WirePlumber plugin module.
//! Following along with the [source code](../src/static_link_module/static-link.rs.html) is recommended.
//! Additional explanation and documentation is located in the [plugin module documentation](wireplumber::plugin).

use std::pin::Pin;
use std::iter;
use std::future::Future;
use std::cell::RefCell;

use futures::{FutureExt, StreamExt, future};
use futures::channel::mpsc;
use glib::{Variant, Error, SourceId};
use glib::prelude::*;
use glib::once_cell::unsync::OnceCell;

use wireplumber::prelude::*;
use wireplumber::{
	core::{Core, Object, ObjectFeatures},
	plugin::{self, AsyncPluginImpl, SimplePlugin},
	registry::{ConstraintType, Constraint, Interest, ObjectManager},
	pw::{self, Node, Port, Link, Properties},
	info, warning,
};

/// [GLib logging domain](glib::g_log) that doubles as the
/// [plugin's name](glib::subclass::types::ObjectSubclass::NAME)
const LOG_DOMAIN: &'static str = "static-link";

/// A list of user-specified [Constraints](Constraint)
/// used to find each end of the port to be linked.
#[derive(Debug, Clone, Variant)]
pub struct PortMapping {
	/// A description of the output ports to link.
	///
	/// The constraint `port.direction=out` is implied.
	output: Vec<Constraint>,
	/// A description of the input ports to link.
	///
	/// The constraint `port.direction=in` is implied.
	input: Vec<Constraint>,
}

/// User configuration for the [StaticLink] plugin
#[derive(Debug, Clone, Variant)]
pub struct StaticLinkArgs {
	/// The source node to link to `input`
	output: Vec<Constraint>,
	/// The sink node to link to `output`
	input: Vec<Constraint>,
	/// Describes how to link the ports of the `input` node to the `output`
	port_mappings: Vec<PortMapping>,
	/// Whether to mark any created links as `link.passive`
	///
	/// Defaults to `true`
	passive: bool,
	/// Whether to mark any created links as `object.linger`
	///
	/// A lingering link will remain in place even after this module's parent process has exited.
	///
	/// Defaults to `true`
	linger: bool,
}

impl Default for StaticLinkArgs {
	fn default() -> Self {
		Self {
			output: Default::default(),
			input: Default::default(),
			port_mappings: Default::default(),
			passive: true,
			linger: true,
		}
	}
}

/// The main logic of the plugin
///
/// After all [interests](Interest) are [registered](main_async),
/// this waits for events indicating that new nodes match the configured [`arg`](StaticLinkArgs)
/// and decides how to link them together.
pub async fn main_loop(
	om: ObjectManager,
	core: Core,
	arg: StaticLinkArgs,
	input_interest: Interest<Node>, output_interest: Interest<Node>,
	mut rx: mpsc::Receiver<()>
) {
	let link_props = Properties::new_empty();
	link_props.insert(pw::PW_KEY_LINK_PASSIVE, arg.passive);
	link_props.insert(pw::PW_KEY_OBJECT_LINGER, arg.linger);
	while let Some(()) = rx.next().await {
		let inputs = input_interest.filter(&om);
		let outputs = || output_interest.filter(&om);
		let pairs = inputs.flat_map(|i| outputs().map(move |o| (i.clone(), o)));

		let mut links = Vec::new();
		for (input, output) in pairs {
			info!(domain: LOG_DOMAIN, "linking {} to {}", input, output);
			if arg.port_mappings.is_empty() {
				match Link::new(&core, &output, &input, &link_props) {
					Ok(link) => links.push(link),
					Err(e) => warning!(domain: LOG_DOMAIN, "Failed to create link: {:?}", e),
				}
			} else {
				for mapping in &arg.port_mappings {
					let port_input_interest: Interest<Port> = mapping.input.iter().chain(iter::once(
						&Constraint::compare(ConstraintType::default(), pw::PW_KEY_PORT_DIRECTION, "in", true)
					)).collect();
					let port_inputs = port_input_interest.filter(&input);

					let port_output_interest: Interest<Port> = mapping.output.iter().chain(iter::once(
						&Constraint::compare(ConstraintType::default(), pw::PW_KEY_PORT_DIRECTION, "out", true)
					)).collect();
					let port_outputs = || port_output_interest.filter(&output);

					let core = &core;
					let link_props = &link_props;
					links.extend(port_inputs.flat_map(|i| port_outputs().map(move |o|
						Link::new(&core, &o, &i, &link_props)
					)).filter_map(|res| match res {
						Ok(link) => Some(link),
						Err(e) => {
							warning!(domain: LOG_DOMAIN, "Failed to create link: {:?}", e);
							None
						},
					}));
				}
			}
		}
		future::join_all(links.into_iter().map(|l| l.activate_future().map(|res| match res {
			Err(e) if Link::error_is_exists(&e) => info!(domain: LOG_DOMAIN, "{:?}", e),
			Err(e) => warning!(domain: LOG_DOMAIN, "Failed to activate link: {:?}", e),
			Ok(_) => (),
		}))).await;
	}
}

/// The main entry point of the plugin
pub async fn main_async(core: Core, arg: StaticLinkArgs) -> Result<impl IntoIterator<Item=impl Future<Output=()>>, Error> {
	let om = ObjectManager::new();
	let context = core.g_main_context().unwrap();

	let output_interest: Interest<Node> = arg.output.iter().collect();
	om.add_interest_full(&output_interest);

	let input_interest: Interest<Node> = arg.input.iter().collect();
	om.add_interest_full(&input_interest);

	let (link_nodes_signal, rx) = mpsc::channel(1);

	let port_signals = {
		let mut object_added = om.signal_stream(ObjectManager::SIGNAL_OBJECT_ADDED);
		let context = context.clone();
		let link_nodes_signal = link_nodes_signal.clone();
		async move {
			while let Some((obj,)) = object_added.next().await {
				let node: Node = obj.dynamic_cast().unwrap();
				context.spawn_local(node.signal_stream(Node::SIGNAL_PORTS_CHANGED)
					.map(|_| Ok(())).forward(link_nodes_signal.clone()).map(drop)
				);
			}
		}
	};
	let object_signals = om.signal_stream(ObjectManager::SIGNAL_OBJECTS_CHANGED)
		.map(|_| Ok(())).forward(link_nodes_signal).map(drop);

	let signal_installed = om.signal_stream(ObjectManager::SIGNAL_INSTALLED);

	om.request_object_features(Object::static_type(), ObjectFeatures::ALL);
	core.install_object_manager(&om);

	// NOTE: waiting for `installed` really isn't necessary since the loop waits for a signal anyway...
	signal_installed.once().await?;

	let main_loop = main_loop(om, core, arg, input_interest, output_interest, rx);

	Ok([port_signals.boxed_local(), object_signals.boxed_local(), main_loop.boxed_local()])
}

/// The plugin instance
///
/// The instance struct contains any relevant mutable data,
/// and is initialized by [SimplePlugin::init_args].
/// It is also a shared ref-counted [GObject](glib::Object) instance
/// via [`SimplePluginObject<Self>`](plugin::SimplePluginObject),
/// so it must be accessed and manipulated via `&self`.
#[derive(Default)]
pub struct StaticLink {
	/// Arguments specified by the user at plugin initialization
	args: OnceCell<Vec<StaticLinkArgs>>,
	/// Mutable data keeps track of any futures
	/// that the plugin spawns on the [MainLoop](glib::MainLoop).
	handles: RefCell<Vec<SourceId>>,
}

/// This makes [StaticLink] an async plugin that can be used with [plugin::plugin_export] below.
impl AsyncPluginImpl for StaticLink {
	type EnableFuture = Pin<Box<dyn Future<Output=Result<(), Error>>>>;

	/// The real entry point of the plugin
	///
	/// Spawns as asynchronous [main_async] for each [StaticLinkArgs] supplied by the user.
	fn enable(&self, this: Self::Type) -> Self::EnableFuture {
		let core = this.core().unwrap();
		async move {
			let loops = this.args.get().unwrap().iter()
				.map(|arg| main_async(core.clone(), arg.clone()));
			this.handles.borrow_mut().extend(
				future::try_join_all(loops).await?.into_iter().flat_map(|l| l).map(|spawn|
					core.g_main_context().unwrap().spawn_local(spawn)
				)
			);
			Ok(())
		}.boxed_local()
	}

	/// Plugin deinitializer
	///
	/// Cleans up after itself by cancelling any pending futures
	/// that were previously spawned by `enable`
	fn disable(&self) {
		let context = self.instance_ref().core().unwrap().g_main_context().unwrap();
		for source in self.handles.borrow_mut().drain(..) {
			// TODO: how does this differ from source.remove()?
			if let Some(source) = context.find_source_by_id(&source) {
				// TODO: will completed future sources be considered destroyed? or will they never exist on the context to begin with?
				if !source.is_destroyed() {
					source.destroy();
				}
			}
		}
	}
}

/// This makes [StaticLink] a plugin that can be used with [plugin::simple_plugin_subclass] below.
impl SimplePlugin for StaticLink {
	type Args = Vec<StaticLinkArgs>;

	fn init_args(&self, args: Self::Args) {
		self.args.set(args).unwrap();
	}
}

// macros take care of entry point boilerplate by impl'ing a bunch of traits for us

plugin::simple_plugin_subclass! {
	impl ObjectSubclass for LOG_DOMAIN as StaticLink { }
}

plugin::plugin_export!(StaticLink);
