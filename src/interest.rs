use crate::ObjectInterest;
use glib::{translate::{ToGlibPtr, from_glib}, IsA, Object};

impl ObjectInterest {
	#[doc(alias = "wp_object_interest_matches")]
	pub fn matches<O: IsA<Object>>(&self, object: O) -> bool {
		unsafe {
			from_glib(ffi::wp_object_interest_matches(self.to_glib_none().0, object.to_glib_none().0 as *mut _))
		}
	}
}
