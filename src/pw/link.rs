#[cfg(feature = "v0_4_11")]
use crate::pw::LinkState;
use crate::{
	core::Core,
	prelude::*,
	pw::{self, Direction, Link, Node, Port, Properties},
};

impl Link {
	#[doc(alias("wp_link_new_from_factory"))]
	pub fn new<O: LinkTarget + Debug, I: LinkTarget + Debug>(
		core: &Core,
		output: &O,
		input: &I,
		props: &Properties,
	) -> Result<Self, Error> {
		let props = Properties::new_clone(props);
		output.write_props(&props, Direction::Output)?;
		input.write_props(&props, Direction::Input)?;

		Self::from_factory(core, "link-factory", Some(props))
			.ok_or_else(|| Error::new(LibraryErrorEnum::OperationFailed, "factory did not produce a link???"))
	}

	pub fn error_is_exists(e: &Error) -> bool {
		e.message().ends_with(": File exists") // TODO
	}

	#[cfg(feature = "v0_4_11")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_11")))]
	#[doc(alias = "wp_link_get_state")]
	#[doc(alias = "get_state")]
	pub fn state(&self) -> LinkState {
		unsafe { from_glib(ffi::wp_link_get_state(self.to_glib_none().0, ptr::null_mut())) }
	}

	#[cfg(feature = "v0_4_11")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_11")))]
	#[doc(alias = "wp_link_get_state")]
	#[doc(alias = "get_state")]
	pub fn state_result(&self) -> Result<LinkState, Error> {
		unsafe {
			let mut error = ptr::null();
			match from_glib(ffi::wp_link_get_state(self.to_glib_none().0, &mut error)) {
				LinkState::Error => {
					let msg: Option<&glib::GStr> = from_glib_none(error);
					Err(Error::new(
						LibraryErrorEnum::OperationFailed,
						msg.map(|s| s.as_str()).unwrap_or("unspecified link state error"),
					))
				},
				state => Ok(state),
			}
		}
	}
}

pub trait LinkTarget {
	fn write_props(&self, props: &Properties, dir: Direction) -> Result<(), Error>;
}

impl LinkTarget for Node {
	fn write_props(&self, props: &Properties, dir: Direction) -> Result<(), Error> {
		match dir {
			Direction::Output => props.insert(pw::PW_KEY_LINK_OUTPUT_NODE, self.bound_id()),
			Direction::Input => props.insert(pw::PW_KEY_LINK_INPUT_NODE, self.bound_id()),
			_ => unreachable!(),
		}
		Ok(())
	}
}

impl LinkTarget for Port {
	fn write_props(&self, props: &Properties, dir: Direction) -> Result<(), Error> {
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

#[cfg(not(feature = "v0_4_11"))]
impl StaticType for pw::LinkFeatures {
	fn static_type() -> Type {
		pw::ProxyFeatures::static_type()
	}
}

#[cfg(feature = "v0_4_11")]
impl<E> From<Result<LinkState, E>> for LinkState {
	fn from(res: Result<LinkState, E>) -> Self {
		match res {
			Ok(state) => state,
			Err(_) => LinkState::Error,
		}
	}
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match self {
			Direction::Input => "input",
			Direction::Output => "output",
			Direction::__Unknown(direction) => panic!("unknown WP_DIRECTION {direction}"),
		};
		f.write_str(name)
	}
}
