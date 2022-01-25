//! Miscellaneous types and helpers
//!
//! # See also
//!
//! C API docs for:
//!
//! - [WpIterator](https://pipewire.pages.freedesktop.org/wireplumber/c_api/iterator_api.html)
//! - [Transition](https://pipewire.pages.freedesktop.org/wireplumber/c_api/transitions_api.html)
//! - [State Storage](https://pipewire.pages.freedesktop.org/wireplumber/c_api/state_api.html)

pub use crate::auto::{
	State,
	Iterator as WpIterator,
	Transition, TransitionStep,
	traits::TransitionExt,
};

mod transition;
pub use transition::TransitionExt2;

mod iterator;
pub use iterator::ValueIterator;
