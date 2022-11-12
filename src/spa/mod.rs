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

#[cfg(any(feature = "experimental", feature = "dox"))]
pub use self::props::SpaProps;
#[cfg(any(feature = "experimental", feature = "dox"))]
pub use self::route::{SpaRoute, SpaRoutes};
#[cfg(any(feature = "v0_4_8", feature = "dox"))]
pub use crate::auto::{SpaJson, SpaJsonBuilder, SpaJsonParser};
#[cfg(feature = "libspa")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
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
#[cfg(any(feature = "v0_4_8", feature = "dox"))]
mod json;
#[cfg(feature = "libspa")]
mod libspa_pod;
mod parser;
mod pod;
#[cfg(any(feature = "experimental", feature = "dox"))]
mod props;
#[cfg(any(feature = "experimental", feature = "dox"))]
mod route;
mod traits;
#[path = "type.rs"]
mod type_;
