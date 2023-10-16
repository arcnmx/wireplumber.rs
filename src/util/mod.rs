//! Miscellaneous types and helpers
//!
//! # See also
//!
//! C API docs for:
//!
//! - [WpIterator](https://pipewire.pages.freedesktop.org/wireplumber/c_api/iterator_api.html)
//! - [Transition](https://pipewire.pages.freedesktop.org/wireplumber/c_api/transitions_api.html)
//! - [State Storage](https://pipewire.pages.freedesktop.org/wireplumber/c_api/state_api.html)

pub use {
	self::{
		iterator::{IntoValueIterator, ValueIterator},
		transition::TransitionExt2,
	},
	crate::auto::{traits::TransitionExt, Iterator as WpIterator, State, Transition, TransitionStep},
};

#[cfg(feature = "futures")]
pub(crate) mod futures;
mod iterator;
mod transition;
