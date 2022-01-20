use glib::translate::{TryFromGlib, IntoGlib, GlibNoneError, UnsafeFrom};
use std::convert::TryFrom;
use std::fmt;
use crate::SpaType;

impl SpaType {
	pub fn from_id(id: ffi::WpSpaType) -> Option<Self> {
		match id {
			ffi::WP_SPA_TYPE_INVALID => None,
			inner => Some(unsafe {
				Self::unsafe_from(inner)
			}),
		}
	}

	pub fn number(&self) -> ffi::WpSpaType {
		self.into_glib()
	}
}

impl TryFromGlib<ffi::WpSpaType> for SpaType {
	type Error = GlibNoneError;

	unsafe fn try_from_glib(val: ffi::WpSpaType) -> Result<Self, Self::Error> {
		Self::try_from(val)
	}
}

impl TryFrom<ffi::WpSpaType> for SpaType {
	type Error = GlibNoneError;

	fn try_from(value: ffi::WpSpaType) -> Result<Self, Self::Error> {
		Self::from_id(value).ok_or(GlibNoneError)
	}
}

impl IntoGlib for SpaType {
	type GlibType = ffi::WpSpaType;
	fn into_glib(self) -> Self::GlibType {
		self.inner
	}
}

impl fmt::Debug for SpaType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut f = f.debug_struct("SpaType");
		f.field("id", &self.into_glib());
		if let Some(name) = self.name() {
			f.field("name", &name);
		}
		f.finish()
	}
}
