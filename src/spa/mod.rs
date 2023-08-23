//! [Simple Plugin API](https://docs.pipewire.org/page_spa_plugins.html) [POD encoding](https://docs.pipewire.org/page_spa_pod.html)
//!
//! [SpaPod] wraps a [libspa_sys::spa_pod], providing high-level accessors and enabling mutation
//! of the Plain Old Data serialized inside. [SpaType] and the traits provided by this module
//! describe the format and meaning of the data within.
//!
//! # See also
//!
//! C API docs for:
//! - [SpaPod](https://pipewire.pages.freedesktop.org/wireplumber/c_api/spa_pod_api.html)
//! - [SpaType](https://pipewire.pages.freedesktop.org/wireplumber/c_api/spa_type_api.html)

#[cfg(feature = "experimental")]
pub use self::props::SpaProps;
#[cfg(feature = "experimental")]
pub use self::route::{SpaRoute, SpaRoutes};
#[cfg(feature = "v0_4_8")]
pub use crate::auto::{SpaJson, SpaJsonBuilder, SpaJsonParser};
#[cfg(feature = "libspa")]
#[cfg_attr(docsrs, doc(cfg(feature = "libspa")))]
pub use libspa;
#[cfg(feature = "libspa")]
pub use libspa_pod::DebugValue;
pub use {
	self::{
		id_table::SpaIdTable,
		id_value::SpaIdValue,
		traits::{SpaBool, SpaPrimitive, SpaValue},
		type_::SpaType,
	},
	crate::auto::{SpaPod, SpaPodBuilder, SpaPodParser},
	libspa_sys as ffi,
};

mod builder;
mod id_table;
mod id_value;
#[cfg(feature = "v0_4_8")]
mod json;
#[cfg(feature = "libspa")]
mod libspa_pod;
mod parser;
mod pod;
#[cfg(feature = "experimental")]
mod props;
#[cfg(feature = "experimental")]
mod route;
mod traits;
#[path = "type.rs"]
mod type_;
