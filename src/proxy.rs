use glib::{IsA, translate::{ToGlibPtr, FromGlib, IntoGlib}, ffi::gconstpointer};
use pipewire_sys::pw_proxy;
use crate::{Proxy, PipewireObject};

#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ObjectFeatures(pub u32); // TODO: consider keeping this as u32, and just keep the inherent impls (requires no changes to `auto`)

impl ObjectFeatures {
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
