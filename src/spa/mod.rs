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

pub use crate::auto::{
	SpaPod,
	SpaPodParser,
	SpaPodBuilder,
};

#[cfg(any(feature = "v0_4_8", feature = "dox"))]
pub use crate::auto::{
	SpaJson,
	SpaJsonParser,
	SpaJsonBuilder,
};

pub use libspa_sys as ffi;
#[cfg(feature = "libspa")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
pub use libspa;

#[path = "type.rs"]
mod type_;
mod pod;
mod parser;
mod builder;
mod id_table;
mod id_value;
#[cfg(any(feature = "v0_4_8", feature = "dox"))]
mod json;
#[cfg(any(feature = "experimental", feature = "dox"))]
mod props;
#[cfg(any(feature = "experimental", feature = "dox"))]
mod route;
mod traits;
#[cfg(feature = "libspa")]
mod libspa_pod;

pub use type_::SpaType;
pub use id_table::SpaIdTable;
pub use id_value::SpaIdValue;
#[cfg(feature = "libspa")]
pub use libspa_pod::DebugValue;

pub use traits::{SpaPrimitive, SpaValue, SpaBool};
#[cfg(any(feature = "experimental", feature = "dox"))]
pub use props::SpaProps;
#[cfg(any(feature = "experimental", feature = "dox"))]
pub use route::{SpaRoute, SpaRoutes};
