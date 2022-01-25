//! Wrappers around [`pw_impl_node`](pipewire_sys::pw_impl_node) and other local objects
//!
//! # See also
//!
//! C API docs for:
//!
//! - [ImplNode](https://pipewire.pages.freedesktop.org/wireplumber/c_api/impl_node_api.html)
//! - [ImplModule](https://pipewire.pages.freedesktop.org/wireplumber/c_api/impl_module_api.html)
//! - [SpaDevice](https://pipewire.pages.freedesktop.org/wireplumber/c_api/spa_device_api.html)

pub use crate::auto::{
	ImplModule,
	ImplEndpoint,
	ImplMetadata,
	ImplNode,
	SpaDevice, SpaDeviceFeatures,
};

mod node;
mod device;
