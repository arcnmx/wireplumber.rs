use crate::SpaPodBuilder;
use glib::translate::ToGlibPtr;
use glib::ffi::gconstpointer;
use std::iter::FromIterator;

impl SpaPodBuilder {
	#[doc(alias = "wp_spa_pod_builder_add_bytes")]
	pub fn add_bytes(&self, value: &[u8]) {
		unsafe {
			ffi::wp_spa_pod_builder_add_bytes(self.to_glib_none().0, value.as_ptr() as *const _, value.len() as _)
		}
	}

	#[doc(alias = "wp_spa_pod_builder_add_pointer")]
	pub fn add_pointer(&self, type_name: &str, value: gconstpointer) {
		unsafe {
			ffi::wp_spa_pod_builder_add_pointer(self.to_glib_none().0, type_name.to_glib_none().0, value)
		}
	}
}
