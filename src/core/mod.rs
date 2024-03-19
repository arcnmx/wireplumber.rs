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
//!   Core::run(None, None, |context, mainloop, core| {
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
//! - [Initialization](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/wp_api.html)
//! - [Core](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/core_api.html)
//! - [Object](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/object_api.html)

pub use {
	self::{
		features::ObjectFeatures,
		object::ObjectExt2,
		subclass::{ObjectImpl, ObjectImplExt},
	},
	crate::auto::{
		traits::ObjectExt, BaseDirsFlags, Conf, Core, CoreFeatures, Device, Factory, FeatureActivationTransition,
		InitFlags, Object,
	},
};

mod conf;
mod core;
mod features;
mod object;
mod subclass;
