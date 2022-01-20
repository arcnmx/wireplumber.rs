use std::future::Future;
use std::ffi::{CStr, CString};
use std::fmt;
use std::pin::Pin;

use glib::{Error, IsA, translate::{ToGlibPtr, FromGlib, IntoGlib}, ffi::gconstpointer};
use pipewire_sys::pw_proxy;
use crate::{ValueIterator, SpaPod};
use crate::{Proxy, PipewireObject, pw::{self, FromPipewirePropertyString}, LibraryErrorEnum};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ObjectFeatures(pub u32); // TODO: consider keeping this as u32, and just keep the inherent impls (requires no changes to `auto`)

impl ObjectFeatures {
	pub const NONE: Self = Self(0);
	pub const ALL: Self = Self(ffi::WP_OBJECT_FEATURES_ALL);

	pub fn with_bits(bits: u32) -> Self {
		Self(bits)
	}

	pub fn bits(&self) -> u32 {
		self.0
	}
}

impl crate::ProxyFeatures {
	pub const ALL: Self = unsafe { Self::from_bits_unchecked(ffi::WP_PIPEWIRE_OBJECT_FEATURES_ALL as u32) };
	pub const MINIMAL: Self = unsafe { Self::from_bits_unchecked(ffi::WP_PIPEWIRE_OBJECT_FEATURES_MINIMAL as u32) };
}

impl FromGlib<u32> for ObjectFeatures {
	unsafe fn from_glib(features: u32) -> Self {
		Self::with_bits(features)
	}
}

impl IntoGlib for ObjectFeatures {
	type GlibType = u32;

	fn into_glib(self) -> Self::GlibType {
		self.bits()
	}
}

impl Into<u32> for ObjectFeatures {
	fn into(self) -> u32 {
		self.bits()
	}
}

impl From<u32> for ObjectFeatures {
	fn from(features: u32) -> Self {
		Self::with_bits(features)
	}
}

macro_rules! impl_object_features {
	($($id:ident:$ty:ident,)*) => {
		$(
			impl From<crate::$id> for ObjectFeatures {
				fn from(features: crate::$id) -> Self {
					Self::with_bits(features.bits())
				}
			}

			impl From<ObjectFeatures> for crate::$id {
				fn from(features: ObjectFeatures) -> crate::$id {
					crate::$id::from_bits_truncate(features.bits())
				}
			}

			impl $crate::$ty {
				#[doc(alias = "wp_object_activate")]
				pub fn activate<P, Q>(&self, features: $crate::$id, cancellable: Option<&P>, callback: Q) where
					P: IsA<::gio::Cancellable>,
					Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
				{
					crate::traits::ObjectExt::activate(self, features.into(), cancellable, callback)
				}

				#[doc(alias = "wp_object_activate_closure")]
				pub fn activate_closure<P>(&self, features: $crate::$id, cancellable: Option<&P>, closure: &glib::Closure) where
					P: IsA<gio::Cancellable>,
				{
					crate::traits::ObjectExt::activate_closure(self, features.into(), cancellable, closure)
				}

				#[doc(alias = "wp_object_activate")]
				pub fn activate_future(&self, features: $crate::$id) -> impl std::future::Future<Output=Result<(), glib::Error>> + Unpin {
					crate::traits::ObjectExt::activate_future(self, features.into())
				}

				#[doc(alias = "wp_object_deactivate")]
				pub fn deactivate(&self, features: $crate::$id) {
					crate::traits::ObjectExt::deactivate(self, features.into())
				}

				#[doc(alias = "wp_object_get_active_features")]
				pub fn active_features(&self) -> $crate::$id {
					crate::traits::ObjectExt::active_features(self)
						.into()
				}

				#[doc(alias = "wp_object_get_supported_features")]
				pub fn supported_features(&self) -> $crate::$id {
					crate::traits::ObjectExt::supported_features(self)
						.into()
				}

				#[doc(alias = "wp_object_update_features")]
				pub fn update_features(&self, activated: $crate::$id, deactivated: $crate::$id) {
					crate::traits::ObjectExt::update_features(self, activated.into(), deactivated.into())
				}
			}
		)*
	};
}

impl_object_features! {
	MetadataFeatures:Metadata, NodeFeatures:Node, PluginFeatures:Plugin, ProxyFeatures:Proxy, SessionItemFeatures:SessionItem, SpaDeviceFeatures:SpaDevice,
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

impl fmt::Display for PipewireObject {
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
