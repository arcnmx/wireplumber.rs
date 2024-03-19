use crate::{
	prelude::*,
	spa::{
		json::{ParseError, SpaJson, SpaJsonObjectParser, SpaJsonParserRef},
		SpaType,
	},
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpaJsonRef<'j> {
	pub(crate) json: SpaJson,
	_ref: PhantomData<&'j str>,
}

impl<'j> SpaJsonRef<'j> {
	#[cfg(feature = "v0_4_10")]
	pub fn with_str(json: &'j str) -> Self {
		unsafe { Self::new(SpaJson::wrap_string(json)) }
	}

	pub fn with_gstr(json: &'j GStr) -> Self {
		unsafe { Self::new(SpaJson::wrap_gstr(json)) }
	}

	pub fn with_json<'a>(json: &'a SpaJson) -> &'a Self
	where
		'j: 'a,
	{
		unsafe { Self::with_json_unchecked(json) }
	}

	pub unsafe fn with_json_unchecked<'a>(json: &'a SpaJson) -> &'a Self {
		unsafe { mem::transmute(json) }
	}

	pub unsafe fn new(json: SpaJson) -> Self {
		Self {
			json,
			_ref: PhantomData,
		}
	}

	pub fn into_data(self) -> &'j str {
		unsafe { &*self.json.data() }
	}

	pub fn data(&self) -> &str {
		unsafe { self.json.data_unchecked() }
	}

	/// Because this object borrows data of lifetime `'j`,
	/// it is *not* safe to [clone](SpaJson::clone) this reference.
	pub unsafe fn get_json(&self) -> &SpaJson {
		&self.json
	}

	pub unsafe fn get_json_mut(&mut self) -> &mut SpaJson {
		&mut self.json
	}

	pub unsafe fn into_json_unchecked(self) -> SpaJson {
		self.json
	}

	pub fn copy(&self) -> SpaJson {
		self.json.copy()
	}

	pub fn into_owned(self) -> SpaJson {
		self.json.ensure_unique_owner()
	}

	pub unsafe fn into_parts_unchecked(self) -> (SpaJson, &'j str) {
		let data = unsafe { &*self.json.data() };
		(self.json, data)
	}

	pub fn parse_char(&self) -> Result<char, ParseError> {
		self.json.parse_char()
	}

	pub fn parse_str(&self) -> Option<Cow<'j, str>> {
		if !self.json.is_string() {
			return None
		}

		match self.json.parse_str_ptr() {
			Some(s) => Some(Cow::Borrowed(unsafe { &*s })),
			None => Some(Cow::Owned(self.json.parse_string().into())),
		}
	}

	pub fn parse_string(&self) -> Option<GString> {
		match self.json.is_string() {
			true => Some(self.json.parse_string()),
			false => None,
		}
	}

	pub fn parse_boolean(&self) -> Option<bool> {
		self.json.parse_boolean()
	}

	pub fn parse_int(&self) -> Option<i32> {
		self.json.parse_int()
	}

	pub fn parse_float(&self) -> Option<f32> {
		self.json.parse_float()
	}

	pub fn parse_null(&self) -> Option<()> {
		self.json.parse_null()
	}

	pub fn parse_array<'a>(&'a self) -> Option<SpaJsonParserRef<'a, 'j>> {
		match self.json.is_array() {
			true => Some(SpaJsonParserRef::with_json(self, Some(SpaType::ARRAY))),
			false => None,
		}
	}

	pub fn parse_object<'a>(&'a self) -> Option<SpaJsonObjectParser<'a, 'j>> {
		match self.json.is_object() {
			true => Some(SpaJsonObjectParser::with_json(self)),
			false => None,
		}
	}

	pub fn parse_values<'a>(&'a self) -> impl Iterator<Item = SpaJsonRef<'j>> + 'static {
		let values = self.json.parse_values();
		values.map(|v| unsafe { Self::new(v) })
	}

	pub fn parse_variant(&self) -> Result<Variant, ParseError> {
		self.json.parse_variant()
	}

	pub fn parse_container<'a>(&'a self) -> Result<SpaJsonParserRef<'a, 'j>, ParseError> {
		match self.spa_type()? {
			ty @ SpaType::OBJECT | ty @ SpaType::ARRAY => Ok(SpaJsonParserRef::with_json(self, Some(ty))),
			found => Err(ParseError::TypeMismatch {
				expected: SpaType::OBJECT,
				found: Some(found),
			}),
		}
	}

	pub fn check_parse(&self) -> Result<(), ParseError> {
		self.json.check_parse()
	}

	pub fn spa_type(&self) -> Result<SpaType, ParseError> {
		self.json.spa_type()
	}
}

impl SpaJsonRef<'static> {
	pub fn new_null() -> Self {
		Self::with_gstr(gstr!("null"))
	}

	pub fn empty() -> Self {
		Self::with_gstr(gstr!(""))
	}

	pub fn empty_array() -> Self {
		Self::with_gstr(gstr!("[]"))
	}

	pub fn empty_object() -> Self {
		Self::with_gstr(gstr!("{}"))
	}

	pub fn into_json(self) -> SpaJson {
		self.json
	}
}

impl<'a, 'j> From<&'a SpaJson> for &'a SpaJsonRef<'j>
where
	'a: 'j,
{
	fn from(json: &'a SpaJson) -> Self {
		SpaJsonRef::with_json(json)
	}
}

impl<'j> AsRef<SpaJsonRef<'j>> for &'j SpaJson {
	fn as_ref(&self) -> &SpaJsonRef<'j> {
		SpaJsonRef::with_json(*self)
	}
}

impl<'a, 'j> AsRef<SpaJsonRef<'a>> for SpaJsonRef<'j>
where
	'j: 'a,
{
	fn as_ref(&self) -> &SpaJsonRef<'a> {
		self
	}
}

impl<'j> ToVariant for SpaJsonRef<'j> {
	fn to_variant(&self) -> Variant {
		let v = self.parse_variant().expect("valid SpaJson");
		Variant::from_variant(&v)
	}
}

impl<'j> StaticVariantType for SpaJsonRef<'j> {
	fn static_variant_type() -> Cow<'static, VariantTy> {
		SpaJson::static_variant_type()
	}
}

impl<'j> Display for SpaJsonRef<'j> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Display::fmt(self.data(), f)
	}
}

impl<'j> Debug for SpaJsonRef<'j> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("SpaJsonRef").field(&self.json).finish()
	}
}
