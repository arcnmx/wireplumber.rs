#[cfg(feature = "v0_4_11")]
use crate::dbus::{Dbus, DbusFeatures};
use crate::{
	core::ObjectExt,
	local::{SpaDevice, SpaDeviceFeatures},
	plugin::{Plugin, PluginFeatures},
	prelude::*,
	pw::{Link, LinkFeatures, Metadata, MetadataFeatures, Node, NodeFeatures, Proxy, ProxyFeatures},
	session::{SessionItem, SessionItemFeatures},
};

// TODO: consider keeping this as u32, and just keep the inherent impls
// (requires no changes to `auto`)
#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ObjectFeatures(pub u32);

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

macro_rules! impl_object_features {
	($($id:ident:$ty:ident($($sub:ident),*),)*) => {
		$(
			impl From<$id> for ObjectFeatures {
				fn from(features: $id) -> Self {
					Self::with_bits(features.bits())
				}
			}

			impl From<ObjectFeatures> for $id {
				fn from(features: ObjectFeatures) -> $id {
					$id::from_bits_truncate(features.bits())
				}
			}

			$(
				impl From<$id> for $sub {
					fn from(features: $id) -> Self {
						Self::from_bits_truncate(features.bits())
					}
				}

				impl From<$sub> for $id {
					fn from(features: $sub) -> $id {
						$id::from_bits_truncate(features.bits())
					}
				}
			)*

			impl $ty {
				#[doc(alias = "wp_object_activate")]
				pub fn activate<P, Q, F: Into<$id>>(&self, features: F, cancellable: Option<&P>, callback: Q) where
					P: IsA<::gio::Cancellable>,
					Q: FnOnce(Result<(), Error>) + Send + 'static,
				{
					ObjectExt::activate(self, features.into().into(), cancellable, callback)
				}

				#[doc(alias = "wp_object_activate_closure")]
				pub fn activate_closure<P, F: Into<$id>>(&self, features: F, cancellable: Option<&P>, closure: glib::Closure) where
					P: IsA<gio::Cancellable>,
				{
					ObjectExt::activate_closure(self, features.into().into(), cancellable, closure)
				}

				#[doc(alias = "wp_object_activate")]
				pub fn activate_future<F: Into<$id>>(&self, features: F) -> impl Future<Output=Result<(), Error>> + Unpin {
					ObjectExt::activate_future(self, features.into().into())
				}

				#[doc(alias = "wp_object_deactivate")]
				pub fn deactivate<F: Into<$id>>(&self, features: F) {
					ObjectExt::deactivate(self, features.into().into())
				}

				#[doc(alias = "wp_object_get_active_features")]
				pub fn active_features(&self) -> $id {
					ObjectExt::active_features(self)
						.into()
				}

				#[doc(alias = "wp_object_get_supported_features")]
				pub fn supported_features(&self) -> $id {
					ObjectExt::supported_features(self)
						.into()
				}

				#[doc(alias = "wp_object_update_features")]
				pub fn update_features<A: Into<$id>, D: Into<$id>>(&self, activated: A, deactivated: D) {
					ObjectExt::update_features(self, activated.into().into(), deactivated.into().into())
				}
			}
		)*
	};
}

impl_object_features! {
	MetadataFeatures:Metadata(ProxyFeatures),
	NodeFeatures:Node(ProxyFeatures),
	LinkFeatures:Link(ProxyFeatures),
	PluginFeatures:Plugin(),
	ProxyFeatures:Proxy(),
	SessionItemFeatures:SessionItem(),
	SpaDeviceFeatures:SpaDevice(ProxyFeatures),
}

#[cfg(feature = "v0_4_11")]
impl_object_features! {
	DbusFeatures:Dbus(),
}
