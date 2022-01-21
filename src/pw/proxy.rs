use pipewire_sys::pw_proxy;
use crate::spa::SpaPod;
use crate::pw::{self, Proxy, ProxyFeatures, PipewireObject, FromPipewirePropertyString};
use crate::prelude::*;

impl ProxyFeatures {
	pub const ALL: Self = unsafe { Self::from_bits_unchecked(ffi::WP_PIPEWIRE_OBJECT_FEATURES_ALL as u32) };
	pub const MINIMAL: Self = unsafe { Self::from_bits_unchecked(ffi::WP_PIPEWIRE_OBJECT_FEATURES_MINIMAL as u32) };
}

pub trait ProxyExt2: 'static {
	#[doc(alias = "wp_proxy_get_pw_proxy")]
	#[doc(alias = "get_pw_proxy")]
	fn pw_proxy_raw(&self) -> *mut pw_proxy;

	#[doc(alias = "wp_proxy_set_pw_proxy")]
	fn set_pw_proxy_raw(&self, proxy: *mut pw_proxy);

	// TODO: bound_id() -> Option<>
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

	fn object_id(&self) -> Result<u32, Error>;

	#[doc(alias = "wp_pipewire_object_get_property")]
	#[doc(alias = "get_property")]
	fn get_pw_property(&self, key: &str) -> Option<String>;
	fn get_pw_property_cstring(&self, key: &str) -> Option<CString>;

	fn with_pw_property_cstr<R, F: FnOnce(&CStr) -> R>(&self, key: &str, f: F) -> Option<R>;
	fn with_pw_property<R, F: FnOnce(&str) -> R>(&self, key: &str, f: F) -> Option<R>;

	#[doc(alias = "wp_pipewire_object_get_property")]
	#[doc(alias = "get_property")]
	fn pw_property<T: FromPipewirePropertyString>(&self, key: &str) -> Result<T, Error>;
	fn pw_property_optional<T: FromPipewirePropertyString>(&self, key: &str) -> Result<Option<T>, Error>;

	#[doc(alias = "wp_pipewire_object_enum_params")]
	fn params_future(&self, id: Option<&str>, filter: Option<&SpaPod>) -> Pin<Box<dyn Future<Output=Result<ValueIterator<SpaPod>, Error>> + 'static>>;
}

impl<O: IsA<PipewireObject>> PipewireObjectExt2 for O {
	fn native_info(&self) -> gconstpointer {
		unsafe {
			ffi::wp_pipewire_object_get_native_info(self.as_ref().to_glib_none().0)
		}
	}

	fn object_id(&self) -> Result<u32, Error> {
		self.pw_property(pw::PW_KEY_OBJECT_ID)
	}

	fn get_pw_property_cstring(&self, key: &str) -> Option<CString> {
		self.with_pw_property_cstr(key, |cstr| cstr.to_owned())
	}

	fn get_pw_property(&self, key: &str) -> Option<String> {
		self.with_pw_property(key, |str| str.to_owned())
	}

	fn with_pw_property_cstr<R, F: FnOnce(&CStr) -> R>(&self, key: &str, f: F) -> Option<R> {
		let this = self.as_ref();
		unsafe {
			let value = ffi::wp_pipewire_object_get_property(this.to_glib_none().0, key.to_glib_none().0);
			if value.is_null() {
				None
			} else {
				Some(f(CStr::from_ptr(value)))
			}
		}
	}

	fn with_pw_property<R, F: FnOnce(&str) -> R>(&self, key: &str, f: F) -> Option<R> {
		let this = self.as_ref();
		self.with_pw_property_cstr(key, move |cstr| match cstr.to_str() {
			Err(e) => {
				wp_warning!(self: this, "pw_property {} ({:?}) was not valid UTF-8: {:?}", key, cstr, e);
				None
			},
			Ok(str) => Some(f(str)),
		}).flatten()
	}

	fn pw_property<T: FromPipewirePropertyString>(&self, key: &str) -> Result<T, Error> {
		match self.with_pw_property(key, T::from_pipewire_string) {
			None => Err(Error::new(LibraryErrorEnum::InvalidArgument, &format!("pw_property {} not found", key))),
			Some(Err(e)) => Err(Error::new(LibraryErrorEnum::InvalidArgument, &format!("pw_property {} failed to parse: {:?}", key, e))),
			Some(Ok(v)) => Ok(v),
		}
	}

	fn pw_property_optional<T: FromPipewirePropertyString>(&self, key: &str) -> Result<Option<T>, Error> {
		self.with_pw_property(key, T::from_pipewire_string)
			.transpose()
			.map_err(|e| Error::new(LibraryErrorEnum::InvalidArgument, &format!("pw_property {} failed to parse: {:?}", key, e)))
	}

	fn params_future(&self, id: Option<&str>, filter: Option<&SpaPod>) -> Pin<Box<dyn Future<Output=Result<ValueIterator<SpaPod>, Error>> + 'static>> {
		let res = self.enum_params_future(id, filter);
		Box::pin(async move {
			res.await.map(|r| ValueIterator::with_inner(r.unwrap()))
		})
	}
}

impl Display for PipewireObject {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(res) = self.with_pw_property(pw::PW_KEY_OBJECT_PATH, |path| {
			f.write_str(path)
		}) {
			return res
		}

		f.write_str("pw.object")?;

		self.with_pw_property(pw::PW_KEY_OBJECT_ID, |id| {
			write!(f, "({})", id)
		}).unwrap_or(Ok(()))
	}
}
