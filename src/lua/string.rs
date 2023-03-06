use {
	crate::{lua::LuaError, prelude::*},
	glib::variant::VariantTypeMismatchError,
	std::{fmt, ops::Deref, str},
};

newtype_wrapper! {
	#[derive(Debug, Ord, Eq, Clone, Hash)]
	pub struct LuaString([u8] | Vec<u8> = Self) as_bytes into_bytes;
}

impl<'a> LuaString<'a> {
	pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
		str::from_utf8(self.as_bytes())
	}

	pub fn into_string(self) -> Result<String, str::Utf8Error> {
		match self.as_str() {
			Ok(..) => Ok(unsafe { String::from_utf8_unchecked(self.into_bytes()) }),
			Err(e) => Err(e),
		}
	}

	pub fn to_variant(&self) -> Variant {
		match self.as_str() {
			Ok(s) => s.to_variant(),
			Err(..) => self.as_bytes().to_variant(),
		}
	}

	pub fn parse<F: FromStr>(&self) -> Result<F, LuaError>
	where
		F::Err: Into<LuaError>,
	{
		self
			.as_str()
			.map_err(Into::into)
			.and_then(|s| s.parse().map_err(Into::into))
	}
}

impl<'a> Deref for LuaString<'a> {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		self.as_bytes()
	}
}

impl<'a> TryFrom<&'a Variant> for LuaString<'a> {
	type Error = LuaError;

	fn try_from(variant: &'a Variant) -> Result<Self, Self::Error> {
		match variant.classify() {
			VariantClass::Variant => Self::try_from(variant.as_variant().expect("VariantClass")),
			VariantClass::String => Ok(Self::from(variant.str().expect("VariantClass"))),
			VariantClass::Array if variant.type_() == VariantTy::BYTE_STRING =>
				Ok(Self::from(variant.fixed_array::<u8>().expect("VariantClass"))),
			_ => Err(LuaError::TypeMismatch(VariantTypeMismatchError::new(
				variant.type_().to_owned(),
				VariantTy::STRING.to_owned(),
			))),
		}
	}
}

impl<'a> TryFrom<Variant> for LuaString<'a> {
	type Error = LuaError;

	fn try_from(variant: Variant) -> Result<Self, Self::Error> {
		LuaString::try_from(&variant).map(|s| s.owned())
	}
}

impl<'a> Into<Variant> for LuaString<'a> {
	fn into(self) -> Variant {
		Into::into(&self)
	}
}

impl<'a> From<String> for LuaString<'a> {
	fn from(v: String) -> Self {
		Self::from(v.into_bytes())
	}
}

impl<'a> From<&'a str> for LuaString<'a> {
	fn from(v: &'a str) -> Self {
		Self::from(v.as_bytes())
	}
}

impl fmt::Display for LuaString<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.as_str() {
			Ok(s) => fmt::Display::fmt(s, f),
			Err(_) => todo!(),
		}
	}
}
