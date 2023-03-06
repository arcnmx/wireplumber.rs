use crate::{prelude::*, util::Transition};

pub trait TransitionExt2: 'static {
	fn return_error(&self, error: Error);
}

impl<O: IsA<Transition>> TransitionExt2 for O {
	fn return_error(&self, error: Error) {
		unsafe {
			ffi::wp_transition_return_error(self.as_ref().to_glib_none().0, error.into_glib_ptr());
		}
	}
}
