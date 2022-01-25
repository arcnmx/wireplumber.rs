use crate::prelude::*;
use crate::{
	session::{SessionItemFeatures, SessionItem},
	local::{SpaDeviceFeatures, SpaDevice},
	plugin::{PluginFeatures, Plugin},
	core::ObjectExt,
	pw::{
		MetadataFeatures, Metadata,
		NodeFeatures, Node,
		ProxyFeatures, Proxy,
	},
};

#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ObjectFeatures(pub u32); // TODO: consider keeping this as u32, and just keep the inherent impls (requires no changes to `auto`)

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
	($($id:ident:$ty:ident,)*) => {
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

			impl $ty {
				#[doc(alias = "wp_object_activate")]
				pub fn activate<P, Q>(&self, features: $id, cancellable: Option<&P>, callback: Q) where
					P: IsA<::gio::Cancellable>,
					Q: FnOnce(Result<(), Error>) + Send + 'static,
				{
					ObjectExt::activate(self, features.into(), cancellable, callback)
				}

				#[doc(alias = "wp_object_activate_closure")]
				pub fn activate_closure<P>(&self, features: $id, cancellable: Option<&P>, closure: &glib::Closure) where
					P: IsA<gio::Cancellable>,
				{
					ObjectExt::activate_closure(self, features.into(), cancellable, closure)
				}

				#[doc(alias = "wp_object_activate")]
				pub fn activate_future(&self, features: $id) -> impl Future<Output=Result<(), Error>> + Unpin {
					ObjectExt::activate_future(self, features.into())
				}

				#[doc(alias = "wp_object_deactivate")]
				pub fn deactivate(&self, features: $id) {
					ObjectExt::deactivate(self, features.into())
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
				pub fn update_features(&self, activated: $id, deactivated: $id) {
					ObjectExt::update_features(self, activated.into(), deactivated.into())
				}
			}
		)*
	};
}

impl_object_features! {
	MetadataFeatures:Metadata, NodeFeatures:Node, PluginFeatures:Plugin, ProxyFeatures:Proxy, SessionItemFeatures:SessionItem, SpaDeviceFeatures:SpaDevice,
}
