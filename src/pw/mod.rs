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
//! - [Properties](https://pipewire.pages.freedesktop.org/wireplumber/c_api/properties_api.html)
//! - [Proxy](https://pipewire.pages.freedesktop.org/wireplumber/c_api/proxy_api.html)
//! - [PipewireObject](https://pipewire.pages.freedesktop.org/wireplumber/c_api/pipewire_object_api.html)
//! - [GlobalProxy](https://pipewire.pages.freedesktop.org/wireplumber/c_api/global_proxy_api.html)
//! - [Node](https://pipewire.pages.freedesktop.org/wireplumber/c_api/node_api.html)
//! - [Port](https://pipewire.pages.freedesktop.org/wireplumber/c_api/port_api.html)
//! - [Link](https://pipewire.pages.freedesktop.org/wireplumber/c_api/link_api.html)
//! - [Device](https://pipewire.pages.freedesktop.org/wireplumber/c_api/device_api.html)
//! - [Client](https://pipewire.pages.freedesktop.org/wireplumber/c_api/client_api.html)
//! - [Metadata](https://pipewire.pages.freedesktop.org/wireplumber/c_api/metadata_api.html)
//! - [Endpoint](https://pipewire.pages.freedesktop.org/wireplumber/c_api/endpoint_api.html)

pub use crate::auto::{
	PipewireObject,
	Proxy, ProxyFeatures,
	GlobalProxy,
	Endpoint,
	Device,
	Node, NodeState, NodeFeatures,
	Port, Direction,
	Link, LinkFeatures,
	Metadata, MetadataFeatures,
	Properties,
	traits::{
		PipewireObjectExt,
		ProxyExt,
		GlobalProxyExt,
		EndpointExt,
		MetadataExt,
	},
};
#[cfg(any(feature = "v0_4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_2")))]
pub use crate::auto::PropertiesItem;
#[cfg(any(feature = "v0_4_11", feature = "dox"))]
pub use crate::auto::LinkState;

mod keys;
pub use keys::*;

mod proxy;
pub use proxy::{PipewireObjectExt2, ProxyExt2};
mod node;
mod port;
mod link;
pub use link::LinkTarget;
mod properties;

#[cfg(feature = "libspa")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl From<Direction> for libspa::Direction {
	fn from(dir: Direction) -> Self {
		match dir {
			Direction::Input => Self::Input,
			Direction::Output => Self::Output,
			Direction::__Unknown(v) => panic!("unsupported WpDirection value: {}", v),
		}
	}
}

#[cfg(feature = "libspa")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl From<libspa::Direction> for Direction {
	fn from(dir: libspa::Direction) -> Self {
		match dir {
			libspa::Direction::Input => Self::Input,
			libspa::Direction::Output => Self::Output,
		}
	}
}
