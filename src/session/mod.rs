//! Session item factories
//!
//! # See also
//!
//! C API docs for:
//!
//! - [SiFactory](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/si_factory_api.html)
//! - [Session Item](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/session_item_api.html)
//! - [Interfaces](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/si_interfaces_api.html)
#[cfg(feature = "v0_4_10")]
pub use crate::auto::SiAdapterPortsState;
pub use crate::auto::{
	traits::{SessionItemExt, SiAcquisitionExt, SiAdapterExt, SiFactoryExt, SiLinkExt, SiLinkableExt},
	SessionItem, SessionItemFeatures, SiAcquisition, SiAdapter, SiFactory, SiLink, SiLinkable,
};
use crate::{prelude::*, spa::SpaPod};

pub trait SiAdapterExt2: 'static {
	#[doc(alias = "wp_si_adapter_set_ports_format")]
	fn set_ports_format<P: FnOnce(Result<(), Error>) + Send + 'static>(
		&self,
		format: Option<&SpaPod>,
		mode: Option<&str>,
		callback: P,
	);

	fn set_ports_format_future(
		&self,
		format: Option<SpaPod>,
		mode: Option<String>,
	) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>>;
}

impl<O: IsA<SiAdapter>> SiAdapterExt2 for O {
	fn set_ports_format<P: FnOnce(Result<(), Error>) + Send + 'static>(
		&self,
		format: Option<&SpaPod>,
		mode: Option<&str>,
		callback: P,
	) {
		type DynCallback = dyn FnOnce(Result<(), Error>) + Send + 'static;
		let callback = Box::new(callback) as Box<DynCallback>;
		let userdata = Box::into_raw(Box::new(callback));
		unsafe extern "C" fn set_ports_format_trampoline(
			_source_object: *mut glib::gobject_ffi::GObject,
			res: *mut gio::ffi::GAsyncResult,
			user_data: gpointer,
		) {
			let mut error = ptr::null_mut();
			let _ = ffi::wp_si_adapter_set_ports_format_finish(_source_object as *mut _, res, &mut error);
			let result = if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			};
			let callback = Box::from_raw(user_data as *mut Box<DynCallback>);
			callback(result)
		}
		unsafe {
			ffi::wp_si_adapter_set_ports_format(
				self.as_ref().to_glib_none().0,
				format.to_glib_full(),
				mode.to_glib_none().0,
				Some(set_ports_format_trampoline),
				userdata as *mut libc::c_void,
			)
		}
	}

	fn set_ports_format_future(
		&self,
		format: Option<SpaPod>,
		mode: Option<String>,
	) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
		Box::pin(gio::GioFuture::new(self, move |obj, _cancellable, send| {
			obj.set_ports_format(format.as_ref(), mode.as_ref().map(|s| s.as_str()), move |res| {
				send.resolve(res)
			})
		}))
	}
}

pub trait SiAcquisitionExt2: 'static {
	fn acquire<P: IsA<SiLink>, Q: IsA<SiLinkable>, R: FnOnce(Result<(), Error>) + Send + 'static>(
		&self,
		acquisitor: &P,
		item: &Q,
		callback: R,
	);
	fn acquire_future<P: IsA<SiLink> + 'static, P_: AsRef<P>, Q: IsA<SiLinkable> + 'static, Q_: AsRef<Q>>(
		&self,
		acquisitor: P,
		item: Q,
	) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>>;
}

impl<O: IsA<SiAcquisition>> SiAcquisitionExt2 for O {
	fn acquire<P: IsA<SiLink>, Q: IsA<SiLinkable>, R: FnOnce(Result<(), Error>) + Send + 'static>(
		&self,
		acquisitor: &P,
		item: &Q,
		callback: R,
	) {
		type DynCallback = dyn FnOnce(Result<(), Error>) + Send + 'static;
		let callback = Box::new(callback) as Box<DynCallback>;
		let userdata = Box::into_raw(Box::new(callback));
		unsafe extern "C" fn acquire_trampoline(
			_source_object: *mut glib::gobject_ffi::GObject,
			res: *mut gio::ffi::GAsyncResult,
			user_data: gpointer,
		) {
			let mut error = ptr::null_mut();
			let _ = ffi::wp_si_acquisition_acquire_finish(_source_object as *mut _, res, &mut error);
			let result = if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			};
			let callback = Box::from_raw(user_data as *mut Box<DynCallback>);
			callback(result)
		}
		unsafe {
			ffi::wp_si_acquisition_acquire(
				self.as_ref().to_glib_none().0,
				acquisitor.as_ref().to_glib_none().0,
				item.as_ref().to_glib_none().0,
				Some(acquire_trampoline),
				userdata as *mut libc::c_void,
			)
		}
	}

	fn acquire_future<P: IsA<SiLink> + 'static, P_: AsRef<P>, Q: IsA<SiLinkable> + 'static, Q_: AsRef<Q>>(
		&self,
		acquisitor: P,
		item: Q,
	) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
		Box::pin(gio::GioFuture::new(self, move |obj, _cancellable, send| {
			obj.acquire(acquisitor.as_ref(), item.as_ref(), move |res| send.resolve(res))
		}))
	}
}
