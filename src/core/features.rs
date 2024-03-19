use crate::{
	core::{Device, Factory, ObjectExt2},
	local::{ImplMetadata, ImplNode, SpaDevice, SpaDeviceFeatures},
	plugin::{Plugin, PluginFeatures},
	prelude::*,
	pw::{Client, GlobalProxy, Link, Metadata, MetadataFeatures, Node, NodeFeatures, Port, Proxy, ProxyFeatures},
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
	($($id:ident$(:$ty:ident)*($($sub:ident),*),)*) => {
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

			$(
				impl ObjectExt2 for $ty {
					type Features = $id;
				}
			)*
		)*
	};
}

impl_object_features! {
	MetadataFeatures:Metadata:ImplMetadata(ProxyFeatures),
	NodeFeatures:Node(ProxyFeatures),
	PluginFeatures:Plugin(),
	ProxyFeatures:Proxy:Factory:Device:Port:Client:Link:ImplNode:GlobalProxy(),
	SessionItemFeatures:SessionItem(),
	SpaDeviceFeatures:SpaDevice(ProxyFeatures),
}

impl ObjectExt2 for Core {
	type Features = ObjectFeatures;
}
