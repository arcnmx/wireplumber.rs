//! WirePlumber module example
//!
//! An example showing how to write a simple WirePlumber plugin module.
//! Following along with the [source code](../src/static_link_module/static-link.rs.html) is
//! recommended. Additional explanation and documentation is located in the [plugin module
//! documentation](wireplumber::plugin).

use {
	futures::{channel::mpsc, future, FutureExt, StreamExt},
	glib::{prelude::*, Error, SourceId, Variant},
	once_cell::unsync::OnceCell,
	serde::{Deserialize, Serialize},
	std::{future::Future, iter, pin::Pin},
	wireplumber::{
		core::{Object, ObjectFeatures},
		error,
		log::{info, warning},
		lua::from_variant,
		plugin::{self, AsyncPluginImpl, SimplePlugin, SimplePluginObject, SourceHandlesCell},
		prelude::*,
		pw::{self, Link, Node, Port, Properties, ProxyFeatures},
		registry::{Constraint, ConstraintType, Interest, ObjectManager},
	},
};

/// [GLib logging domain](glib::g_log) that doubles as the
/// [plugin's name](glib::subclass::types::ObjectSubclass::NAME)
const LOG_DOMAIN: &'static str = "static-link";

/// A list of user-specified [Constraints](Constraint)
/// used to find each end of the port to be linked.
#[derive(Debug, Clone, Deserialize, Serialize, Variant)]
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

/// serde boolean default
#[doc(hidden)]
fn true_() -> bool {
	true
}

/// User configuration for the [StaticLink] plugin
#[derive(Debug, Clone, Deserialize, Serialize, Variant)]
pub struct StaticLinkArgs {
	/// The source node to link to `input`
	output: Vec<Constraint>,
	/// The sink node to link to `output`
	input: Vec<Constraint>,
	/// Describes how to link the ports of the `input` node to the `output`
	#[serde(default, rename = "mappings")]
	port_mappings: Vec<PortMapping>,
	/// Whether to mark any created links as `link.passive`
	///
	/// Defaults to `true`
	#[serde(default = "true_")]
	passive: bool,
	/// Whether to mark any created links as `object.linger`
	///
	/// A lingering link will remain in place even after this module's parent process has exited.
	///
	/// Defaults to `true`
	#[serde(default = "true_")]
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

/// Link all ports between `output` and `input` matching [`mappings`][PortMapping]
fn link_ports<'a>(
	mappings: &'a [PortMapping],
	core: &'a Core,
	output: &'a Node,
	input: &'a Node,
	link_props: &'a Properties,
) -> impl Iterator<Item = Result<Link, Error>> + 'a {
	mappings.iter().flat_map(move |mapping| {
		let port_input_interest: Interest<Port> = mapping
			.input
			.iter()
			.chain(iter::once(&Constraint::compare(
				ConstraintType::default(),
				pw::PW_KEY_PORT_DIRECTION,
				"in",
				true,
			)))
			.collect();
		let port_inputs = port_input_interest.filter(input).into_iter();

		let port_output_interest: Interest<Port> = mapping
			.output
			.iter()
			.chain(iter::once(&Constraint::compare(
				ConstraintType::default(),
				pw::PW_KEY_PORT_DIRECTION,
				"out",
				true,
			)))
			.collect();
		let port_outputs = move || port_output_interest.clone().filter(output).into_iter();

		port_inputs.flat_map(move |i| port_outputs().map(move |o| Link::new(&core, &o, &i, link_props)))
	})
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
	input_interest: Interest<Node>,
	output_interest: Interest<Node>,
	mut rx: mpsc::Receiver<()>,
) {
	let link_props = Properties::new();
	link_props.insert(pw::PW_KEY_LINK_PASSIVE, arg.passive);
	link_props.insert(pw::PW_KEY_OBJECT_LINGER, arg.linger);
	while let Some(()) = rx.next().await {
		let inputs = input_interest.clone().filter(&om).into_iter();
		let outputs = || output_interest.clone().filter(&om).into_iter();
		let pairs = inputs.flat_map(|i| outputs().map(move |o| (i.clone(), o)));

		let mut links = Vec::new();
		for (input, output) in pairs {
			info!(domain: LOG_DOMAIN, "linking {input} to {output}");
			if arg.port_mappings.is_empty() {
				links.push(Link::new(&core, &output, &input, &link_props));
			} else {
				links.extend(link_ports(&arg.port_mappings, &core, &output, &input, &link_props));
			}
		}
		let links = links.into_iter().filter_map(|l| match l {
			Ok(link) => Some(link.activate_future(ProxyFeatures::MINIMAL).map(|res| match res {
				Err(e) if Link::error_is_exists(&e) => info!(domain: LOG_DOMAIN, "{:?}", e),
				Err(e) => warning!(domain: LOG_DOMAIN, "Failed to activate link: {:?}", e),
				Ok(_) => (),
			})),
			Err(e) => {
				warning!(domain: LOG_DOMAIN, "Failed to create link: {:?}", e);
				None
			},
		});
		future::join_all(links).await;
	}
}

/// The main entry point of the plugin
pub async fn main_async(
	plugin: &SimplePluginObject<StaticLink>,
	core: Core,
	arg: StaticLinkArgs,
) -> Result<impl IntoIterator<Item = impl Future<Output = ()>>, Error> {
	let om = ObjectManager::new();

	let output_interest: Interest<Node> = arg.output.iter().collect();
	om.add_interest(output_interest.clone());

	let input_interest: Interest<Node> = arg.input.iter().collect();
	om.add_interest(input_interest.clone());

	let (link_nodes_signal, rx) = mpsc::channel(1);

	let port_signals = {
		let mut object_added = om.signal_stream(ObjectManager::SIGNAL_OBJECT_ADDED);
		let link_nodes_signal = link_nodes_signal.clone();
		let plugin = plugin.downgrade();
		async move {
			while let Some((obj,)) = object_added.next().await {
				let node: Node = obj.dynamic_cast().unwrap();
				let plugin = match plugin.upgrade() {
					Some(plugin) => plugin,
					None => break,
				};
				plugin.spawn_local(
					node
						.signal_stream(Node::SIGNAL_PORTS_CHANGED)
						.map(|_| Ok(()))
						.forward(link_nodes_signal.clone())
						.map(drop),
				);
			}
		}
	};
	let object_signals = om
		.signal_stream(ObjectManager::SIGNAL_OBJECTS_CHANGED)
		.map(|_| Ok(()))
		.forward(link_nodes_signal)
		.map(drop);

	om.request_object_features(Node::static_type(), ObjectFeatures::ALL);
	core.install_object_manager(&om);

	// NOTE: waiting for `installed` really isn't necessary since the loop waits for a signal anyway...
	om.installed_future().await?;

	let main_loop = main_loop(om, core, arg, input_interest, output_interest, rx);

	Ok([
		port_signals.boxed_local(),
		object_signals.boxed_local(),
		main_loop.boxed_local(),
	])
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
	handles: SourceHandlesCell,
}

/// This makes [StaticLink] an async plugin that can be used with [plugin::plugin_export] below.
impl AsyncPluginImpl for StaticLink {
	type EnableFuture = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

	fn register_source(&self, source: SourceId) {
		self.handles.push(source);
	}

	/// The real entry point of the plugin
	///
	/// Spawns as asynchronous [main_async] for each [StaticLinkArgs] supplied by the user.
	fn enable(&self, this: Self::Type) -> Self::EnableFuture {
		let core = this.plugin_core();
		let context = this.plugin_context();
		let res = self
			.handles
			.try_init(context.clone())
			.map_err(|_| error::invariant(format_args!("{LOG_DOMAIN} plugin has already been enabled")));
		async move {
			res?;
			let loops = this
				.args
				.get()
				.unwrap()
				.iter()
				.map(|arg| main_async(&this, core.clone(), arg.clone()));
			for spawn in future::try_join_all(loops).await?.into_iter().flat_map(|l| l) {
				this.spawn_local(spawn);
			}
			Ok(())
		}
		.boxed_local()
	}

	/// Plugin deinitializer
	///
	/// Cleans up after itself by cancelling any pending futures
	/// that were previously spawned by `enable`
	fn disable(&self) {
		self.handles.clear();
	}
}

/// This makes [StaticLink] a plugin that can be used with [plugin::simple_plugin_subclass] below.
impl SimplePlugin for StaticLink {
	type Args = Vec<StaticLinkArgs>;

	fn init_args(&self, args: Self::Args) {
		self.args.set(args).unwrap();
	}

	fn decode_args(args: Option<Variant>) -> Result<Self::Args, Error> {
		args
			.map(|args| from_variant(&args))
			.unwrap_or(Ok(Default::default()))
			.map_err(error::invalid_argument)
	}
}

// macros take care of entry point boilerplate by impl'ing a bunch of traits for us

plugin::simple_plugin_subclass! {
	impl ObjectSubclass for LOG_DOMAIN as StaticLink { }
}

plugin::plugin_export!(StaticLink);
