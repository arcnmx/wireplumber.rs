use glib::{translate::ToGlibPtr, ffi::gconstpointer};
use glib::IsA;
use pipewire_sys::pw_proxy;
use crate::{Proxy, PipewireObject};

pub trait ProxyExt2: 'static {
	#[doc(alias = "wp_proxy_get_pw_proxy")]
	#[doc(alias = "get_pw_proxy")]
	fn pw_proxy_raw(&self) -> *mut pw_proxy;

	#[doc(alias = "wp_proxy_set_pw_proxy")]
	fn set_pw_proxy_raw(&self, proxy: *mut pw_proxy);
}

impl<O: IsA<Proxy>> ProxyExt2 for O {
	fn pw_proxy_raw(&self) -> *mut pw_proxy {
		unsafe {
			ffi::wp_proxy_get_pw_proxy(self.as_ref().to_glib_none().0)
		}
	}

	fn set_pw_proxy_raw(&self, proxy: *mut pw_proxy) {
		unsafe {
			ffi::wp_proxy_set_pw_proxy(self.as_ref().to_glib_none().0, proxy)
		}
	}
}

pub trait PipewireObjectExt2: 'static {
	#[doc(alias = "wp_pipewire_object_get_native_info")]
	#[doc(alias = "get_native_info")]
	fn native_info(&self) -> gconstpointer;
}

impl<O: IsA<PipewireObject>> PipewireObjectExt2 for O {
	fn native_info(&self) -> gconstpointer {
		unsafe {
			ffi::wp_pipewire_object_get_native_info(self.as_ref().to_glib_none().0)
		}
	}
}
