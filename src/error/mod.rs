use glib::{Quark, error::ErrorDomain};
use crate::prelude::*;

pub use crate::auto::LibraryErrorEnum;
pub use glib::Error;

pub type Result<T> = std::result::Result<T, Error>;

impl ErrorDomain for LibraryErrorEnum {
	#[doc(alias = "WP_DOMAIN_LIBRARY")]
	#[doc(alias = "wp_domain_library_quark")]
	fn domain() -> Quark {
		unsafe {
			from_glib(ffi::wp_domain_library_quark())
		}
	}

	fn code(self) -> i32 {
		self.into_glib()
	}

	fn from(code: i32) -> Option<Self> {
		match code {
			ffi::WP_LIBRARY_ERROR_INVARIANT | ffi::WP_LIBRARY_ERROR_INVALID_ARGUMENT | ffi::WP_LIBRARY_ERROR_OPERATION_FAILED => unsafe {
				Some(from_glib(code))
			},
			_ => None,
		}
	}
}
