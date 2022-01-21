use crate::{Link, Node, Port, Core, Object, Properties, Direction, pw, ProxyFeatures, LibraryErrorEnum};
use crate::prelude::*;
use glib::Error;
use std::future::Future;

impl Link {
	pub fn new<O: LinkTarget + std::fmt::Debug, I: LinkTarget + std::fmt::Debug>(core: &Core, output: &O, input: &I, props: &Properties) -> crate::Result<Self> {
		let props = Properties::new_clone(props);
		output.write_props(&props, Direction::Output)?;
		input.write_props(&props, Direction::Input)?;

		Self::from_factory(core, "link-factory", Some(&props))
			.ok_or_else(|| Error::new(LibraryErrorEnum::OperationFailed, "factory did not produce a link???"))
	}

	pub fn activate_future(&self) -> impl Future<Output=crate::Result<()>> {
		AsRef::<Object>::as_ref(self).activate_future(ProxyFeatures::MINIMAL.into())
	}

	pub fn error_is_exists(e: &Error) -> bool {
		e.message().ends_with(": File exists") // TODO
	}
}

pub trait LinkTarget {
	fn write_props(&self, props: &Properties, dir: Direction) -> crate::Result<()>;
}

impl LinkTarget for Node {
	fn write_props(&self, props: &Properties, dir: Direction) -> crate::Result<()> {
		match dir {
			Direction::Output => props.insert(pw::PW_KEY_LINK_OUTPUT_NODE, self.bound_id()),
			Direction::Input => props.insert(pw::PW_KEY_LINK_INPUT_NODE, self.bound_id()),
			_ => unreachable!(),
		}
		Ok(())
	}
}

impl LinkTarget for Port {
	fn write_props(&self, props: &Properties, dir: Direction) -> crate::Result<()> {
		let node_id = self.node_id()?;
		match dir {
			Direction::Output => {
				props.insert(pw::PW_KEY_LINK_OUTPUT_PORT, self.port_index()?);
				props.insert(pw::PW_KEY_LINK_OUTPUT_NODE, node_id);
			},
			Direction::Input => {
				props.insert(pw::PW_KEY_LINK_INPUT_PORT, self.port_index()?);
				props.insert(pw::PW_KEY_LINK_INPUT_NODE, node_id);
			},
			_ => unreachable!(),
		}
		Ok(())
	}
}
