#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
mod auto;

pub use auto::*;
pub use ffi;

pub type Result<T> = std::result::Result<T, glib::Error>;
pub type SpaType = i32;
pub type SpaIdTable = glib::ffi::gconstpointer;
pub type SpaIdValue = glib::ffi::gconstpointer;

pub mod pw;
pub mod prelude;

mod error;

mod core;
pub use crate::core::*;

mod log;
pub use log::*;

mod proxy;
pub use proxy::*;

mod iterator;
pub use iterator::*;

mod impl_node;
pub use impl_node::*;

mod transition;
pub use transition::*;

mod properties;
pub use properties::*;

mod spa;
pub use spa::*;

mod si;
pub use si::*;

mod interest;
pub use interest::*;

pub mod lua;

pub mod signals;
