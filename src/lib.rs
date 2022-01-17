#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
mod auto;

pub use auto::*;
pub use ffi;

/// Export dependencies for use from macros
#[doc(hidden)]
pub mod lib {
	pub use glib;
	pub use gio;
}

pub type Result<T> = std::result::Result<T, glib::Error>;
pub type SpaType = i32;
pub type SpaIdTable = glib::ffi::gconstpointer;
pub type SpaIdValue = glib::ffi::gconstpointer;

pub mod pw;
pub mod prelude;

mod error;

mod core;
pub use crate::core::*;

#[macro_use]
mod log;
pub use log::*;

mod object;
pub use object::*;

mod object_manager;
pub use object_manager::*;

mod plugin;
pub use plugin::*;

mod proxy;
pub use proxy::*;

mod iterator;
pub use iterator::*;

mod impl_node;
pub use impl_node::*;

mod transition;
pub use transition::*;

mod node;
pub use node::*;

mod link;
pub use link::*;

mod port;
pub use port::*;

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
