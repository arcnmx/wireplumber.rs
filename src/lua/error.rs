use {
	crate::{error, prelude::*},
	glib::variant::VariantTypeMismatchError,
	std::{
		convert::Infallible,
		error::Error as StdError,
		fmt,
		num::{ParseIntError, TryFromIntError},
		str::Utf8Error,
	},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuaError {
	Custom(String),
	Glib(Error),
	Conversion(TryFromIntError),
	Parse(ParseIntError),
	Utf8(Utf8Error),
	TypeMismatch(VariantTypeMismatchError),
	UnsupportedType(Cow<'static, VariantTy>),
	LengthMismatch { actual: usize, expected: usize },
}

impl From<TryFromIntError> for LuaError {
	fn from(e: TryFromIntError) -> Self {
		LuaError::Conversion(e)
	}
}

impl From<ParseIntError> for LuaError {
	fn from(e: ParseIntError) -> Self {
		LuaError::Parse(e)
	}
}

impl From<Utf8Error> for LuaError {
	fn from(e: Utf8Error) -> Self {
		LuaError::Utf8(e)
	}
}

impl From<VariantTypeMismatchError> for LuaError {
	fn from(e: VariantTypeMismatchError) -> Self {
		LuaError::TypeMismatch(e)
	}
}

impl From<Error> for LuaError {
	fn from(e: Error) -> Self {
		LuaError::Glib(e)
	}
}

impl From<Infallible> for LuaError {
	fn from(v: Infallible) -> Self {
		match v {}
	}
}

impl From<LuaError> for Error {
	fn from(e: LuaError) -> Error {
		match e {
			LuaError::Glib(e) => e,
			LuaError::Custom(e) => error::operation_failed(e),
			e => error::invalid_argument(e),
		}
	}
}

impl fmt::Display for LuaError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			LuaError::Custom(e) => fmt::Display::fmt(e, f),
			LuaError::Glib(e) => fmt::Display::fmt(e, f),
			LuaError::Conversion(e) => fmt::Display::fmt(e, f),
			LuaError::Parse(e) => fmt::Display::fmt(e, f),
			LuaError::Utf8(e) => fmt::Display::fmt(e, f),
			LuaError::TypeMismatch(e) => fmt::Display::fmt(e, f),
			LuaError::UnsupportedType(t) => write!(f, "type {t} is not supported by lua"),
			LuaError::LengthMismatch { actual, expected } => write!(f, "invalid length {actual}, expected {expected}"),
		}
	}
}

impl StdError for LuaError {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			LuaError::Glib(e) => Some(e),
			LuaError::Conversion(e) => Some(e),
			LuaError::Parse(e) => Some(e),
			LuaError::Utf8(e) => Some(e),
			LuaError::TypeMismatch(e) => Some(e),
			LuaError::Custom(..) | LuaError::UnsupportedType(..) | LuaError::LengthMismatch { .. } => None,
		}
	}
}

#[cfg(feature = "serde")]
impl serde::de::Error for LuaError {
	fn invalid_length(actual: usize, expected: &dyn serde::de::Expected) -> Self {
		let expected = expected.to_string();
		match expected.parse() {
			Ok(expected) => LuaError::LengthMismatch { actual, expected },
			_ => Self::custom(format_args!("invalid length {actual}, expected {expected}")),
		}
	}

	fn custom<T: Display>(msg: T) -> Self {
		LuaError::Custom(msg.to_string())
	}
}

#[cfg(feature = "serde")]
impl serde::ser::Error for LuaError {
	fn custom<T: Display>(msg: T) -> Self {
		LuaError::Custom(msg.to_string())
	}
}

#[cfg(feature = "serde")]
impl LuaError {
	pub(crate) fn serde_error<E: serde::de::Error>(self) -> E {
		E::custom(self)
	}

	pub(crate) fn serde_error_ser<E: serde::ser::Error>(self) -> E {
		E::custom(self)
	}
}
