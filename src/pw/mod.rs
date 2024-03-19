//! PipeWire proxy objects
//!
//! All types derived from [Proxy] describe an [object](PipewireObject) on the remote service, with
//! its own associated key-value [Properties].
//!
//! These types cannot be created directly, instances must instead be obtained using the
//! [registry](crate::registry) subsystem to asynchronously listen for new objects as they become
//! bound and visible on the [remote connection](crate::core).
//!
//! # See also
//!
//! C API docs for:
//! - [Properties](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/properties_api.html)
//! - [Proxy](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/proxy_api.html)
//! - [PipewireObject](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/pipewire_object_api.html)
//! - [GlobalProxy](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/global_proxy_api.html)
//! - [Node](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/node_api.html)
//! - [Port](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/port_api.html)
//! - [Link](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/link_api.html)
//! - [Device](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/device_api.html)
//! - [Client](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/client_api.html)
//! - [Metadata](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/metadata_api.html)

#[cfg(feature = "libspa")]
use libspa::utils::Direction as SpaDirection;
pub use {
	self::{
		keys::*,
		link::LinkTarget,
		proxy::{PipewireObjectExt2, ProxyExt2},
	},
	crate::auto::{
		traits::{GlobalProxyExt, MetadataExt, PipewireObjectExt, ProxyExt},
		Client, Device, Direction, GlobalProxy, Link, LinkState, Metadata, MetadataFeatures, Node, NodeFeatures, NodeState,
		PipewireObject, Port, Properties, PropertiesItem, Proxy, ProxyFeatures,
	},
};

mod client;
mod keys;
mod link;
mod node;
mod port;
mod properties;
mod proxy;

#[cfg(feature = "libspa")]
#[cfg_attr(docsrs, doc(cfg(feature = "libspa")))]
impl From<Direction> for SpaDirection {
	fn from(dir: Direction) -> Self {
		match dir {
			Direction::Input => Self::Input,
			Direction::Output => Self::Output,
			Direction::__Unknown(v) => Self::from_raw(v as libspa_sys::spa_direction),
		}
	}
}

#[cfg(feature = "libspa")]
#[cfg_attr(docsrs, doc(cfg(feature = "libspa")))]
impl From<SpaDirection> for Direction {
	fn from(dir: SpaDirection) -> Self {
		match dir {
			SpaDirection::Input => Self::Input,
			SpaDirection::Output => Self::Output,
			dir => Self::__Unknown(dir.as_raw() as crate::ffi::WpDirection),
		}
	}
}
