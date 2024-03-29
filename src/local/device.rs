use {
	crate::{local::SpaDevice, prelude::*, pw::Properties, Core},
	libspa_sys::spa_device,
};

impl SpaDevice {
	#[doc(alias = "wp_spa_device_new_wrap")]
	pub fn new_wrap(core: &Core, spa_device_handle: NonNull<spa_device>, properties: Option<&Properties>) -> SpaDevice {
		unsafe {
			let properties = properties.map(|p| p.to_glib_none().0).unwrap_or(ptr::null_mut());
			from_glib_full(ffi::wp_spa_device_new_wrap(
				core.to_glib_none().0,
				spa_device_handle.as_ptr() as *mut _,
				properties,
			))
		}
	}

	#[doc(alias = "spa-device-handle")]
	pub fn spa_device_handle(&self) -> Option<NonNull<spa_device>> {
		unsafe {
			let mut value = Value::from_type(Type::POINTER);
			glib::gobject_ffi::g_object_get_property(
				self.upcast_ref::<GObject>().to_glib_none().0,
				b"spa-device-handle\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value
				.get::<Option<NonNull<Pointee>>>()
				.expect("spa-device-handle property")
				.map(|p| p.cast())
		}
	}
}
