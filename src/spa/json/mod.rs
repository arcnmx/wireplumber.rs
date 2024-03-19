#[cfg(feature = "serde")]
pub use self::builder::{SpaJsonChildSerializer, SpaJsonPropertySerializer, SpaJsonSerializer};
pub use {
	self::{
		builder::{BuildError, SpaJsonObjectBuilder},
		parser::{ParseError, SpaJsonObjectParser, SpaJsonParserRef},
		r#ref::SpaJsonRef,
	},
	crate::auto::{SpaJson, SpaJsonBuilder, SpaJsonParser},
};
use {
	crate::{prelude::*, spa::SpaType},
	libspa_sys::spa_json,
	std::str::from_utf8_unchecked,
};

mod builder;
mod parser;
mod r#ref;

// TODO: consider renaming the gir struct `WpSpaJson`?
// and making `pub struct SpaJson(WpSpaJson)` a wrapper that can't be cloned
// (or otherwise clones a copy if the underlying json data is not owned).
// That way we can `impl Borrow<SpaJson> for SpaJsonRef`
// and fix a lot of awkward unsafe code here...

impl SpaJson {
	#[doc(alias = "wp_spa_json_new_wrap_string")]
	#[doc(alias = "new_wrap_string")]
	pub fn wrap_static_str(json: &'static str) -> Self {
		unsafe { Self::wrap_string(json) }
	}

	#[doc(alias = "wp_spa_json_new_wrap")]
	#[doc(alias = "new_wrap")]
	pub unsafe fn wrap(spa: NonNull<spa_json>) -> Self {
		from_glib_full(ffi::wp_spa_json_new_wrap(spa.as_ptr()))
	}

	#[doc(alias = "wp_spa_json_ensure_unique_owner")]
	#[doc(alias = "ensure_unique_owner")]
	pub fn make_unique(&mut self) {
		if !self.is_unique_owner() {
			let empty = SpaJson::wrap_static_str("");
			let this = mem::replace(self, empty);
			drop(mem::replace(self, this.ensure_unique_owner()))
		}
	}

	#[doc(alias = "wp_spa_json_parse_string")]
	#[doc(alias = "parse_string")]
	pub fn parse_str(&self) -> Cow<str> {
		self
			.parse_str_ptr()
			.map(|inner| unsafe { &*inner })
			.map(|inner| match self.is_unique_owner() {
				true => Cow::Borrowed(inner),
				false => Cow::Owned(String::from(inner)),
			})
			.unwrap_or_else(|| Cow::Owned(self.parse_string().into()))
	}

	#[doc(alias = "wp_spa_json_parse_string")]
	#[doc(alias = "parse_string")]
	pub fn parse_str_ptr(&self) -> Option<*const str> {
		let data = unsafe { self.data_unchecked() };
		let len = data.len();
		let inner = data.get(1..len.saturating_sub(1))?;
		if inner.contains(['\\', '"']) {
			None
		} else {
			Some(inner)
		}
	}

	#[doc(alias = "wp_spa_json_get_spa_json")]
	#[doc(alias = "get_spa_json")]
	pub fn spa_json(&self) -> *const spa_json {
		unsafe { ffi::wp_spa_json_get_spa_json(self.to_glib_none().0) }
	}

	pub fn borrow<'a>(&'a self) -> &'a SpaJsonRef<'a> {
		SpaJsonRef::with_json(self)
	}

	#[doc(alias = "wp_spa_json_get_data")]
	#[doc(alias = "get_data")]
	pub fn data(&self) -> *const str {
		let data = unsafe { ffi::wp_spa_json_get_data(self.to_glib_none().0) };
		let size = self.size();
		unsafe { from_utf8_unchecked(slice::from_raw_parts(data as *const _, size)) }
	}

	#[doc(alias = "wp_spa_json_get_data")]
	#[doc(alias = "get_data")]
	pub unsafe fn data_unchecked(&self) -> &str {
		&*self.data()
	}

	#[doc(alias = "wp_spa_json_get_data")]
	pub fn get_data(&mut self) -> &str {
		self.make_unique();

		unsafe { self.data_unchecked() }
	}

	#[doc(alias = "wp_spa_json_get_data")]
	#[doc(alias = "get_data")]
	pub fn run_with_data<R, F: FnOnce(&str) -> R>(&self, f: F) -> R {
		f(unsafe { self.data_unchecked() })
	}

	#[doc(alias = "wp_spa_json_parse_string")]
	#[doc(alias = "parse_string")]
	pub fn parse_with_str<R, F: FnOnce(Cow<str>) -> R>(&self, f: F) -> Option<R> {
		if !self.is_string() {
			return None
		};
		let s = self
			.parse_str_ptr()
			.map(|inner| Cow::Borrowed(unsafe { &*inner }))
			.unwrap_or_else(|| Cow::Owned(self.parse_string().into()));
		Some(f(s))
	}

	#[doc(alias = "wp_spa_json_get_data")]
	#[doc(alias = "wp_spa_json_to_string")]
	pub fn to_string(&self) -> String {
		String::from(unsafe { self.data_unchecked() })
	}

	#[doc(alias = "wp_spa_json_to_string")]
	#[doc(alias = "to_string")]
	#[cfg(feature = "v0_4_11")]
	pub fn to_gstring(&self) -> glib::GString {
		unsafe { from_glib_full(ffi::wp_spa_json_to_string(self.to_glib_none().0)) }
	}

	pub fn parse_char(&self) -> Result<char, ParseError> {
		let c = self.parse_with_str(|s| {
			// assert string contains one (and only one) char...
			let mut chars = s.chars();
			let c = match chars.next() {
				Some(c) => {
					let eof = chars.next().is_none();
					if eof {
						Some(c)
					} else {
						None
					}
				},
				None => None,
			};
			c.ok_or_else(|| ParseError::Char { string: s.into() })
		});

		c.ok_or_else(|| ParseError::TypeMismatch {
			expected: SpaType::STRING,
			found: self.spa_type().ok(),
		})
		.and_then(|res| res)
	}

	#[doc(alias = "wp_spa_json_is_null")]
	#[doc(alias = "is_null")]
	pub fn parse_null(&self) -> Option<()> {
		if self.is_null() {
			Some(())
		} else {
			None
		}
	}

	pub fn parse_array<'a>(&'a self) -> Option<SpaJsonParserRef<'a, 'a>> {
		self.borrow().parse_array()
	}

	pub fn parse_object<'a>(&'a self) -> Option<SpaJsonObjectParser<'a, 'a>> {
		self.borrow().parse_object()
	}

	#[doc(alias = "wp_spa_json_new_iterator")]
	pub fn parse_values(&self) -> ValueIterator<SpaJson> {
		ValueIterator::with_inner(self.new_iterator())
	}

	pub fn parse_variant(&self) -> Result<Variant, ParseError> {
		let ty = self.spa_type()?;
		match ty {
			SpaType::NONE => return Ok(().to_variant()),
			SpaType::INT => self.parse_int().map(|v| v.to_variant()),
			SpaType::FLOAT => self.parse_float().map(|v| f64::from(v).to_variant()),
			SpaType::BOOL => self.parse_boolean().map(|v| v.to_variant()),
			SpaType::STRING => return Ok(self.parse_string().to_variant()),
			SpaType::OBJECT => self.parse_object().map(|p| p.into_vardict_variant()),
			SpaType::ARRAY => self.parse_array().map(|p| p.into_variant_array()),
			_ => None,
		}
		.ok_or_else(|| ParseError::TypeMismatch {
			expected: ty,
			found: None,
		})
	}

	pub fn check_parse(&self) -> Result<(), ParseError> {
		match self.spa_type()? {
			SpaType::OBJECT => {
				let mut parser = self.parse_object().unwrap();
				while let Some((_k, value)) = parser.parse_property() {
					value.check_parse()?;
				}
				parser.parse_end()
			},
			SpaType::ARRAY => {
				let mut parser = self.parse_array().unwrap();
				while let Some(value) = parser.parse_json() {
					value.check_parse()?;
				}
				parser.parse_end()
			},
			// TODO: check for trailing JSON data...
			ty @ SpaType::INT => self.parse_int().map(drop).ok_or(ParseError::TypeMismatch {
				expected: ty,
				found: None,
			}),
			ty @ SpaType::FLOAT => self.parse_int().map(drop).ok_or(ParseError::TypeMismatch {
				expected: ty,
				found: None,
			}),
			ty @ SpaType::BOOL => self.parse_boolean().map(drop).ok_or(ParseError::TypeMismatch {
				expected: ty,
				found: None,
			}),
			SpaType::STRING | SpaType::NONE => Ok(()),
			ty => Err(ParseError::TypeMismatch {
				expected: ty,
				found: Some(ty),
			}),
		}
	}

	pub fn spa_type(&self) -> Result<SpaType, ParseError> {
		Ok(if self.is_null() {
			SpaType::NONE
		} else if self.is_int() {
			SpaType::INT
		} else if self.is_float() {
			SpaType::FLOAT
		} else if self.is_string() {
			SpaType::STRING
		} else if self.is_boolean() {
			SpaType::BOOL
		} else if self.is_object() {
			SpaType::OBJECT
		} else if self.is_array() {
			SpaType::ARRAY
		} else {
			return Err(ParseError::InvalidType)
		})
	}
}

impl From<()> for SpaJson {
	fn from(_: ()) -> Self {
		Self::new_null()
	}
}

impl From<bool> for SpaJson {
	fn from(value: bool) -> Self {
		Self::new_boolean(value)
	}
}

impl From<i32> for SpaJson {
	fn from(value: i32) -> Self {
		Self::new_int(value)
	}
}

impl From<f32> for SpaJson {
	fn from(value: f32) -> Self {
		Self::new_float(value)
	}
}

impl<'a, 'j> From<&'a SpaJsonRef<'j>> for SpaJson {
	fn from(json: &'a SpaJsonRef) -> Self {
		json.copy()
	}
}

impl<'j> From<SpaJsonRef<'j>> for SpaJson {
	fn from(json: SpaJsonRef<'j>) -> Self {
		json.copy()
	}
}

impl IntoIterator for SpaJson {
	type Item = SpaJson;
	type IntoIter = ValueIterator<SpaJson>;

	fn into_iter(self) -> Self::IntoIter {
		self.parse_values()
	}
}

impl<'a> IntoIterator for &'a SpaJson {
	type Item = SpaJsonRef<'a>;
	type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

	fn into_iter(self) -> Self::IntoIter {
		Box::new(self.borrow().parse_values())
	}
}

impl ToVariant for SpaJson {
	fn to_variant(&self) -> Variant {
		self.borrow().to_variant()
	}
}

impl FromVariant for SpaJson {
	fn from_variant(variant: &Variant) -> Option<Self> {
		Self::try_from_variant(variant).ok()
	}
}

impl StaticVariantType for SpaJson {
	fn static_variant_type() -> Cow<'static, VariantTy> {
		Cow::Borrowed(VariantTy::VARIANT)
	}
}

impl Display for SpaJson {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.run_with_data(|data| Display::fmt(data, f))
	}
}

impl Debug for SpaJson {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let stash: Stash<*mut ffi::WpSpaJson, Self> = self.to_glib_none();
		self.run_with_data(|data| {
			f.debug_struct("SpaJson")
				.field("inner", &stash.1)
				.field("data", &data)
				.finish()
		})
	}
}
