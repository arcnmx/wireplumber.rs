use glib::translate::ToGlibPtr;
use pipewire_sys::{pw_core, pw_context};
use crate::Core;

impl Core {
	#[doc(alias = "wp_core_get_pw_core")]
	#[doc(alias = "get_pw_core")]
	pub fn pw_core_raw(&self) -> *mut pw_core {
		unsafe {
			ffi::wp_core_get_pw_core(self.to_glib_none().0)
		}
	}

	#[doc(alias = "wp_core_get_pw_context")]
	#[doc(alias = "get_pw_context")]
	pub fn pw_context_raw(&self) -> *mut pw_context {
		unsafe {
			ffi::wp_core_get_pw_context(self.to_glib_none().0)
		}
	}
}
