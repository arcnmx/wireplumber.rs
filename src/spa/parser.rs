use crate::{SpaPodParser, SpaPod};
use crate::prelude::*;
use glib::translate::{ToGlibPtr, from_glib, from_glib_full};
use glib::ffi::gconstpointer;
use std::slice::from_raw_parts;
use std::ffi::CStr;
use std::ptr;

impl SpaPodParser {
	#[doc(alias = "wp_spa_pod_parser_new_object")]
	pub fn new_object(pod: &SpaPod) -> (Self, Option<&'static str>) {
		unsafe {
			// TODO: this needs a lifetime to attach the parser's lifetime to `pod`
			// TODO: this can return back `wp_spa_id_value_short_name(wp_spa_id_table_find_value (table, id))` via second parameter
			let mut id_name = ptr::null();
			let res = from_glib_full(ffi::wp_spa_pod_parser_new_object(pod.to_glib_none().0, &mut id_name));
			let id_name = match id_name {
				id_name if id_name.is_null() => None,
				id_name => Some(CStr::from_ptr(id_name)),
			};
			let id_name = id_name.and_then(|id_name| match id_name.to_str() {
				Ok(str) => Some(str),
				Err(e) => {
					wp_warning!("failed to parse spa pod ID name as UTF-8: {:?}", id_name);
					None
				},
			});
			(res, id_name)
		}
	}

	#[doc(alias = "wp_spa_pod_parser_get_bytes")]
	#[doc(alias = "get_bytes")]
	pub fn bytes(&self) -> Option<&[u8]> {
		let mut data = ptr::null();
		let mut len = 0;
		unsafe {
			if from_glib(ffi::wp_spa_pod_parser_get_bytes(self.to_glib_none().0, &mut data, &mut len)) {
				Some(from_raw_parts(data as *const u8, len as usize))
			} else {
				None
			}
		}
	}

	#[doc(alias = "wp_spa_pod_parser_get_pointer")]
	#[doc(alias = "get_pointer")]
	pub fn pointer(&self) -> Option<gconstpointer> {
		let mut data = ptr::null();
		unsafe {
			if from_glib(ffi::wp_spa_pod_parser_get_pointer(self.to_glib_none().0, &mut data)) {
				Some(data)
			} else {
				None
			}
		}
	}
}
