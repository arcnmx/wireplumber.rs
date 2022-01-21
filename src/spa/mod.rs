pub use crate::auto::{
	SpaType,
	SpaPod,
	SpaPodParser,
	SpaPodBuilder,
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
mod traits;
#[cfg(feature = "libspa")]
mod libspa_pod;

pub use id_table::SpaIdTable;
pub use id_value::SpaIdValue;
#[cfg(feature = "libspa")]
pub use libspa_pod::DebugValue;

pub use traits::{SpaPrimitive, SpaValue, SpaBool};
