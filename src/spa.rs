use glib::ffi::gconstpointer;
use glib::translate::{ToGlibPtrMut, from_glib};
use glib::{Value, Type, Cast, Object};
use glib::translate::from_glib_full;
use glib::translate::ToGlibPtr;
use libspa_sys::{spa_device, spa_pod};
use std::ptr::{self, NonNull};
use std::slice::from_raw_parts;
use crate::{Core, Properties, SpaPod, SpaType, SpaDevice, SpaPodParser, SpaPodBuilder};

impl SpaPod {
	#[doc(alias = "get_spa_type")]
	pub fn spa_type(&self) -> Option<SpaType> {
		unsafe {
			match ffi::wp_spa_pod_get_spa_type(self.to_glib_none().0) {
				ffi::WP_SPA_TYPE_INVALID => None,
				value => Some(value as _),
			}
		}
	}
}

impl SpaDevice {
	#[doc(alias = "wp_spa_device_new_wrap")]
	pub fn new_wrap(core: &Core, spa_device_handle: NonNull<spa_device>, properties: Option<&Properties>) -> SpaDevice {
		unsafe {
			let properties = properties.map(|p| p.to_glib_none().0).unwrap_or(ptr::null_mut());
			from_glib_full(ffi::wp_spa_device_new_wrap(core.to_glib_none().0, spa_device_handle.as_ptr() as *mut _, properties))
		}
	}

	#[doc(alias = "spa-device-handle")]
	pub fn spa_device_handle(&self) -> Option<NonNull<spa_device>> {
		unsafe {
			let mut value = Value::from_type(Type::POINTER);
			glib::gobject_ffi::g_object_get_property(self.upcast_ref::<Object>().to_glib_none().0, b"spa-device-handle\0".as_ptr() as *const _, value.to_glib_none_mut().0);
			NonNull::new(glib::gobject_ffi::g_value_get_pointer(value.to_glib_none().0) as *mut _)
		}
	}
}

impl SpaPodParser {
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

impl SpaPod {
	/// borrows pod for the lifetime of the returned object
	#[doc(alias = "wp_spa_pod_new_wrap")]
	pub unsafe fn new_wrap_raw_mut(pod: *mut spa_pod) -> SpaPod {
		from_glib_full(ffi::wp_spa_pod_new_wrap(pod))
	}

	/// borrows pod for the lifetime of the returned object
	#[doc(alias = "wp_spa_pod_new_wrap_const")]
	pub unsafe fn new_wrap_const(pod: *const spa_pod) -> SpaPod {
		from_glib_full(ffi::wp_spa_pod_new_wrap_const(pod))
	}

	#[doc(alias = "wp_spa_pod_new_bytes")]
	pub fn new_bytes(value: &[u8]) -> SpaPod {
		unsafe {
			from_glib_full(ffi::wp_spa_pod_new_bytes(value.as_ptr() as *const _, value.len() as _))
		}
	}

	#[doc(alias = "wp_spa_pod_new_pointer")]
	pub fn new_pointer(type_name: &str, value: gconstpointer) -> SpaPod {
		unsafe {
			from_glib_full(ffi::wp_spa_pod_new_pointer(type_name.to_glib_none().0, value))
		}
	}

	#[doc(alias = "wp_spa_pod_get_bytes")]
	#[doc(alias = "get_bytes")]
	pub fn bytes(&self) -> Option<&[u8]> {
		let mut value = ptr::null();
		let mut len = 0;
		unsafe {
			if from_glib(ffi::wp_spa_pod_get_bytes(self.to_glib_none().0, &mut value, &mut len)) {
				Some(from_raw_parts(value as *const _, len as usize))
			} else {
				None
			}
		}
	}

	#[doc(alias = "wp_spa_pod_get_choice_type")]
	#[doc(alias = "get_choice_type")]
	pub fn choice_type(&self) -> Option<crate::SpaIdValue> {
		unsafe {
			let res = ffi::wp_spa_pod_get_choice_type(self.to_glib_none().0);
			todo!()
		}
	}

	#[doc(alias = "wp_spa_pod_get_pointer")]
	#[doc(alias = "get_pointer")]
	pub fn pointer(&self) -> Option<gconstpointer> {
		let mut res = ptr::null();
		unsafe {
			if from_glib(ffi::wp_spa_pod_get_pointer(self.to_glib_none().0, &mut res)) {
				Some(res)
			} else {
				None
			}
		}
	}

	#[doc(alias = "wp_spa_pod_set_pointer")]
	pub fn set_pointer(&self, type_name: &str, value: gconstpointer) -> bool {
		unsafe {
			from_glib(ffi::wp_spa_pod_set_pointer(self.to_glib_none().0, type_name.to_glib_none().0, value))
		}
	}

	#[doc(alias = "wp_spa_pod_get_spa_pod")]
	#[doc(alias = "get_spa_pod")]
	pub fn spa_pod_raw(&self) -> &spa_pod {
		unsafe {
			&*ffi::wp_spa_pod_get_spa_pod(self.to_glib_none().0)
		}
	}
}
