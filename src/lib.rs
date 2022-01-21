#![doc(html_root_url = "https://arcnmx.github.io/wireplumber.rs/")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

//! [WirePlumber](https://pipewire.pages.freedesktop.org/wireplumber/index.html) library bindings
//!
//! This crate provides a high-level interface to [PipeWire](https://pipewire.org/)'s [API](https://docs.pipewire.org/page_api.html)
//! via libwireplumber.
//!
//! # Upstream Documentation
//!
//! WirePlumber is a [GObject library](https://gtk-rs.org/), and its [C API documentation](https://pipewire.pages.freedesktop.org/wireplumber/c_api.html)
//! may also be helpful.

#[allow(unused_imports)]
mod auto;

pub use ffi;

/// Export dependencies for use from macros
#[doc(hidden)]
pub mod lib {
	pub use glib;
	pub use gio;
}

pub mod pw;
pub mod prelude;

pub mod log;
pub mod local;
pub mod session;
pub mod registry;
pub mod plugin;
pub mod object;
pub mod spa;
pub mod util;

pub mod error;
pub use error::{Result, Error};

mod core;
pub use crate::core::{Core, InitFlags};

pub use log::Log;

pub mod lua;

pub mod signals;

/// gir needs to know where to find these
pub(crate) use crate::{
	pw::{PipewireObject, Proxy, GlobalProxy, Port, Metadata, Properties, Endpoint, Direction, NodeState},
	object::{Object, ObjectFeatures},
	plugin::Plugin,
	spa::{SpaIdTable, SpaIdValue, SpaPod},
	session::{SessionItem, SiLink, SiLinkable, SiEndpoint, SiAcquisition},
	registry::{ObjectManager, ObjectInterest, InterestMatch, InterestMatchFlags, ConstraintType, ConstraintVerb},
	util::{WpIterator as Iterator, Transition},
};
#[cfg(any(feature = "v0_4_2", feature = "dox"))]
pub(crate) use crate::pw::PropertiesItem;
