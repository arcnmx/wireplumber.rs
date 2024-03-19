//! Error handling and the [Result] alias
//!
//! # See also
//!
//! [C API docs](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/wperror_api.html)

pub use crate::auto::LibraryErrorEnum;
#[doc(no_inline)]
pub use glib::Error;
use {
	crate::prelude::*,
	glib::{error::ErrorDomain, Quark},
	std::fmt::Display,
};

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for LibraryErrorEnum {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match self {
			LibraryErrorEnum::Invariant => "invariant check failed",
			LibraryErrorEnum::InvalidArgument => "invalid argument",
			LibraryErrorEnum::OperationFailed => "operation failed",
			LibraryErrorEnum::ServiceUnavailable => "service unavailable",
			LibraryErrorEnum::__Unknown(error) => return write!(f, "error {error}"),
		};
		f.write_str(name)
	}
}

impl ErrorDomain for LibraryErrorEnum {
	#[doc(alias = "WP_DOMAIN_LIBRARY")]
	#[doc(alias = "wp_domain_library_quark")]
	fn domain() -> Quark {
		unsafe { from_glib(ffi::wp_domain_library_quark()) }
	}

	fn code(self) -> i32 {
		self.into_glib()
	}

	fn from(code: i32) -> Option<Self> {
		match code {
			ffi::WP_LIBRARY_ERROR_INVARIANT
			| ffi::WP_LIBRARY_ERROR_INVALID_ARGUMENT
			| ffi::WP_LIBRARY_ERROR_OPERATION_FAILED => unsafe { Some(from_glib(code)) },
			_ => None,
		}
	}
}

macro_rules! error_constructors {
	($(
		$(#[$meta:meta])*
		$f:ident => $var:ident,
	)*) => {
		$(
			$(#[$meta])*
			pub fn $f<E: Display>(err: E) -> Error {
				Error::new(LibraryErrorEnum::$var, &err.to_string())
			}
		)*
	};
}

error_constructors! {
	/// Wrap a new [Error] under the [LibraryErrorEnum::InvalidArgument] domain
	invalid_argument => InvalidArgument,
	/// Wrap a new [Error] under the [LibraryErrorEnum::OperationFailed] domain
	operation_failed => OperationFailed,
	/// Wrap a new [Error] under the [LibraryErrorEnum::Invariant] domain
	invariant => Invariant,
	/// Wrap a new [Error] under the [LibraryErrorEnum::ServiceUnavailable] domain
	service_unavailable => ServiceUnavailable,
}
