//! Wrappers around [`pw_impl_node`](pipewire_sys::pw_impl_node) and other local objects
//!
//! # See also
//!
//! C API docs for:
//!
//! - [ImplNode](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/impl_node_api.html)
//! - [ImplModule](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/impl_module_api.html)
//! - [SpaDevice](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/spa_device_api.html)

#[cfg(feature = "v0_4_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_2")))]
pub use crate::auto::ImplModule;
pub use crate::auto::{ImplMetadata, ImplNode, SpaDevice, SpaDeviceFeatures};

mod device;
mod node;
