use crate::pw::{self, Link, LinkFeatures, Node, Port, Properties, Direction};
use crate::core::Core;
use crate::prelude::*;

impl Link {
	pub fn new<O: LinkTarget + Debug, I: LinkTarget + Debug>(core: &Core, output: &O, input: &I, props: &Properties) -> Result<Self, Error> {
		let props = Properties::new_clone(props);
		output.write_props(&props, Direction::Output)?;
		input.write_props(&props, Direction::Input)?;

		Self::from_factory(core, "link-factory", Some(&props))
			.ok_or_else(|| Error::new(LibraryErrorEnum::OperationFailed, "factory did not produce a link???"))
	}

	pub fn error_is_exists(e: &Error) -> bool {
		e.message().ends_with(": File exists") // TODO
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

#[cfg(all(feature = "pipewire", feature = "v0_4_11"))]
mod pipewire_types {
	use crate::pw::LinkState;
	use pipewire::link::LinkState as PwLinkState;

	impl LinkState {
		#[cfg_attr(feature = "dox", doc(cfg(all(feature = "pipewire", feature = "v0_4_11"))))]
		pub fn from_pw(pw: PwLinkState) -> Result<LinkState, &str> {
			Ok(match pw {
				PwLinkState::Error(e) => return Err(e),
				PwLinkState::Unlinked => LinkState::Unlinked,
				PwLinkState::Init => LinkState::Init,
				PwLinkState::Negotiating => LinkState::Negotiating,
				PwLinkState::Allocating => LinkState::Allocating,
				PwLinkState::Paused => LinkState::Paused,
				PwLinkState::Active => LinkState::Active,
			})
		}

		#[cfg_attr(feature = "dox", doc(cfg(all(feature = "pipewire", feature = "v0_4_11"))))]
		pub fn to_pw(&self) -> Result<PwLinkState<'static>, i32> {
			Ok(match self {
				&LinkState::__Unknown(v) => return Err(v),
				LinkState::Error => PwLinkState::Error(Default::default()),
				LinkState::Unlinked => PwLinkState::Unlinked,
				LinkState::Init => PwLinkState::Init,
				LinkState::Negotiating => PwLinkState::Negotiating,
				LinkState::Allocating => PwLinkState::Allocating,
				LinkState::Paused => PwLinkState::Paused,
				LinkState::Active => PwLinkState::Active,
			})
		}
	}

	#[cfg_attr(feature = "dox", doc(cfg(all(feature = "pipewire", feature = "v0_4_11"))))]
	impl From<PwLinkState<'_>> for LinkState {
		fn from(pw: PwLinkState) -> Self {
			Self::from_pw(pw).unwrap_or(LinkState::Error)
		}
	}

	#[cfg_attr(feature = "dox", doc(cfg(all(feature = "pipewire", feature = "v0_4_11"))))]
	impl From<LinkState> for PwLinkState<'_> {
		fn from(state: LinkState) -> Self {
			state.to_pw().expect("Unsupported WpNodeState value")
		}
	}
}

#[cfg(feature = "v0_4_11")]
impl StaticType for LinkFeatures {
	fn static_type() -> Type {
		unsafe { from_glib(ffi::wp_link_features_get_type()) }
	}
}

#[cfg(not(feature = "v0_4_11"))]
impl StaticType for LinkFeatures {
	fn static_type() -> Type {
		pw::ProxyFeatures::static_type()
	}
}
