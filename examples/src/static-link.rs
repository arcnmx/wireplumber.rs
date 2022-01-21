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

const LOG_DOMAIN: &'static str = "static-link";

use wireplumber::{
	Properties, Core, Node, pw, Link, Port,
	object::{Object, ObjectFeatures},
	plugin::{self, AsyncPluginImpl, SimplePlugin},
	registry::{ConstraintType, Constraint, Interest, ObjectManager},
	info, warning,
};

#[derive(Debug, Clone, Variant)]
struct PortMapping {
	output: Vec<Constraint>,
	input: Vec<Constraint>,
}

fn true_() -> bool { true }

#[derive(Debug, Clone, Variant)]
pub struct StaticLinkArgs {
	output: Vec<Constraint>,
	input: Vec<Constraint>,
	port_mappings: Vec<PortMapping>,
	passive: bool,
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

async fn main_loop(
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
			// TODO: why does this not return a normal string?
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

async fn main_async(core: Core, arg: StaticLinkArgs) -> Result<impl IntoIterator<Item=impl Future<Output=()>>, Error> {
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

#[derive(Default)]
pub struct StaticLink {
	args: OnceCell<Vec<StaticLinkArgs>>,
	handles: RefCell<Vec<SourceId>>,
}

impl AsyncPluginImpl for StaticLink {
	type EnableFuture = Pin<Box<dyn Future<Output=Result<(), Error>>>>;
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

impl SimplePlugin for StaticLink {
	type Args = Vec<StaticLinkArgs>;

	fn init_args(&self, args: Self::Args) {
		self.args.set(args).unwrap();
	}
}

plugin::simple_plugin_subclass! {
	impl ObjectSubclass for LOG_DOMAIN as StaticLink { }
}

plugin::plugin_export!(StaticLink);
