#![doc(html_root_url = "https://arcnmx.github.io/wireplumber.rs/v0.1.0/")]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! [WirePlumber](https://pipewire.pages.freedesktop.org/wireplumber/index.html) library bindings
//!
//! This crate provides a high-level interface to [PipeWire](https://pipewire.org/)'s [API](https://docs.pipewire.org/page_api.html)
//! via libwireplumber. Explore the documentation for the various [modules below](#modules) for
//! information on how to start using WirePlumber with Rust.
//!
//! # Initialization
//!
//! ## Service Daemon
//!
//! For creating a new wireplumber instance, you'll need to start with a [`Core`]. Start with the
//! [module documentation](crate::core) for examples.
//!
//! ## Dynamic Modules
//!
//! Exporting a dynamic plugin starts with implementing the [PluginImpl](plugin::PluginImpl) trait.
//! The [plugin documentation](crate::plugin) will get you started on creating one!
//!
//! # PipeWire Connection
//!
//! Once you have a [core connection](crate::core), the [registry](crate::registry) will allow you
//! to inspect and influence the state of the [remote PipeWire service and objects](crate::pw).
//!
//! # Examples
//!
//! Besides those found in this documentation,
#![doc = concat!("[additional examples](https://github.com/arcnmx/wireplumber.rs/tree/", env!("RELEASE_TAG"), "/examples/src)")]
//! can be found alongside the [source code](https://github.com/arcnmx/wireplumber.rs), and can be
//! built and run via Cargo:
//!
//! ```bash
//! $ cargo run -p wp-examples --bin wpexec -- --help
//! ... snip ...
//!
//! ## try out the default lua example:
//! $ cargo run -p wp-examples --bin wpexec
//!
//! ## or load the example plugin module:
//! $ cargo build --workspace --examples &&
//!   cargo run -p wp-examples --bin wpexec -- --type wireplumber
//! ```
//!
//! It's recommended to poke around their source code in a local checkout, but you can also view
//! the generated documentation and source code online:
#![doc = concat!("- [wpexec](https://arcnmx.github.io/wireplumber.rs/", env!("RELEASE_TAG"), "/wpexec/index.html)")]
#![doc = concat!("- [static-link module](https://arcnmx.github.io/wireplumber.rs/", env!("RELEASE_TAG"), "/static_link_module/index.html)")]
//! # Upstream Documentation
//!
//! WirePlumber is a [GObject library](https://gtk-rs.org/), and its [C API documentation](https://pipewire.pages.freedesktop.org/wireplumber/c_api.html)
//! may also be helpful.

#[allow(unused_imports)]
#[rustfmt::skip]
mod auto;

/// Export dependencies for use from macros
#[doc(hidden)]
pub mod lib {
	pub use {gio, glib};
}

pub mod core;
#[cfg(feature = "v0_4_11")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_11")))]
pub mod dbus;
pub mod error;
pub mod local;
pub mod log;
#[cfg(feature = "lua")]
#[cfg_attr(docsrs, doc(cfg(feature = "lua")))]
pub mod lua;
pub mod plugin;
pub mod prelude;
pub mod pw;
pub mod registry;
pub mod session;
#[cfg(feature = "glib-signal")]
#[cfg_attr(docsrs, doc(cfg(feature = "glib-signal")))]
pub mod signals;
pub mod spa;
pub mod util;

#[cfg(feature = "v0_4_2")]
pub(crate) use self::pw::PropertiesItem;
#[cfg(feature = "v0_4_10")]
pub(crate) use self::session::SiAdapterPortsState;
#[cfg(feature = "v0_4_8")]
pub(crate) use self::spa::json::SpaJson;
#[cfg(feature = "v0_4_11")]
pub(crate) use self::{dbus::DBusState, pw::LinkState};
/// gir needs to know where to find these
pub(crate) use crate::{
	core::{Object, ObjectFeatures},
	plugin::Plugin,
	pw::{Direction, Endpoint, GlobalProxy, Metadata, NodeState, PipewireObject, Port, Properties, Proxy},
	registry::{ConstraintType, ConstraintVerb, InterestMatch, InterestMatchFlags, ObjectInterest, ObjectManager},
	session::{SessionItem, SiAcquisition, SiEndpoint, SiLink, SiLinkable},
	spa::SpaPod,
	util::{Transition, WpIterator as Iterator},
};
#[doc(no_inline)]
pub use {
	self::{
		core::{Core, InitFlags},
		error::{Error, Result},
		log::Log,
	},
	ffi,
};
