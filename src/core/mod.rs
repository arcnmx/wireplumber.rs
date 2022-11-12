//! WirePlumber's [entry point](Core) and base [Object] type.
//!
//! # PipeWire Main Loop
//!
//! The [Core] is used to initialize the library and connect to an external PipeWire service. The
//! most basic self-contained WirePlumber daemon can be started like so:
//!
//! ```no_run
//! use wireplumber::Core;
//!
//! fn main() {
//!   Core::init();
//!   Core::run(None, |context, mainloop, core| {
//!     context.spawn_local(async move {
//!       # #[cfg(feature = "futures")]
//!       match core.connect_future().await {
//!         Ok(()) => println!("Connected to PipeWire!"),
//!         Err(e) => println!("Failed to connect: {e:?}"),
//!       }
//!       mainloop.quit(); // return from Core::run() and disconnect
//!     });
//!   });
//! }
//! ```
//!
//! # Subclassing
//!
//! A type can register itself as a [subclass](glib::subclass) of [Object] by
//! implementing the [ObjectImpl] trait.
//!
//! # See also
//!
//! C API docs for:
//!
//! - [Initialization](https://pipewire.pages.freedesktop.org/wireplumber/c_api/wp_api.html)
//! - [Core](https://pipewire.pages.freedesktop.org/wireplumber/c_api/core_api.html)
//! - [Object](https://pipewire.pages.freedesktop.org/wireplumber/c_api/object_api.html)

#[cfg(any(feature = "v0_4_5", feature = "dox"))]
pub use crate::auto::Factory;
pub use {
	self::{
		features::ObjectFeatures,
		subclass::{ObjectImpl, ObjectImplExt},
	},
	crate::auto::{traits::ObjectExt, Core, FeatureActivationTransition, InitFlags, Object},
};

mod core;
mod features;
mod subclass;
