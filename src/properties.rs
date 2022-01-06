use glib::translate::{from_glib_full, ToGlibPtr};
use std::ptr::NonNull;
use libspa_sys::spa_dict;
use pipewire_sys::pw_properties;

use crate::Properties;

impl Properties {
	#[doc(alias = "wp_properties_new_copy")]
	pub unsafe fn new_copy(props: &pw_properties) -> Properties {
		from_glib_full(ffi::wp_properties_new_copy(props))
	}

	#[doc(alias = "wp_properties_new_copy_dict")]
	pub unsafe fn new_copy_dict(dict: &spa_dict) -> Properties {
		from_glib_full(ffi::wp_properties_new_copy_dict(dict))
	}

	#[doc(alias = "wp_properties_new_wrap")]
	pub unsafe fn new_wrap(props: *const pw_properties) -> Properties {
		from_glib_full(ffi::wp_properties_new_wrap(props))
	}

	#[doc(alias = "wp_properties_new_take")]
	pub unsafe fn new_wrap_mut(props: *mut pw_properties) -> Properties {
		from_glib_full(ffi::wp_properties_new_take(props))
	}

	#[doc(alias = "wp_properties_new_wrap_dict")]
	pub unsafe fn new_wrap_dict(dict: *const spa_dict) -> Properties {
		from_glib_full(ffi::wp_properties_new_wrap_dict(dict))
	}

	#[doc(alias = "wp_properties_add_from_dict")]
	pub unsafe fn add_from_dict(&self, dict: &spa_dict) -> usize {
		ffi::wp_properties_add_from_dict(self.to_glib_none().0, dict) as usize
	}

	#[doc(alias = "wp_properties_peek_dict")]
	pub fn peek_dict(&self) -> &spa_dict {
		unsafe {
			&*ffi::wp_properties_peek_dict(self.to_glib_none().0)
		}
	}

	#[doc(alias = "wp_properties_to_pw_properties")]
	pub fn to_pw_properties(&self) -> NonNull<pw_properties> {
		unsafe {
			NonNull::new_unchecked(ffi::wp_properties_to_pw_properties(self.to_glib_none().0))
		}
	}

	#[doc(alias = "wp_properties_unref_and_take_pw_properties")]
	pub fn unref_and_take_pw_properties(self) -> NonNull<pw_properties> {
		unsafe {
			NonNull::new_unchecked(ffi::wp_properties_unref_and_take_pw_properties(self.to_glib_full()))
		}
	}

	#[doc(alias = "wp_properties_update_from_dict")]
	pub unsafe fn update_from_dict(&self, dict: &spa_dict) -> usize {
		ffi::wp_properties_update_from_dict(self.to_glib_none().0, dict) as usize
	}
}
