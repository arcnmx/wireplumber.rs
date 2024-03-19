use {
	crate::{
		error,
		prelude::*,
		spa::{
			json::{SpaJson, SpaJsonParser, SpaJsonRef},
			SpaType,
		},
	},
	glib::variant::DictEntry,
	std::error::Error as StdError,
};

impl SpaJsonParser {
	pub fn new_object(json: &SpaJson) -> SpaJsonParserRef {
		SpaJsonParserRef::with_json(json.borrow(), Some(SpaType::OBJECT))
	}

	pub fn new_array(json: &SpaJson) -> SpaJsonParserRef {
		SpaJsonParserRef::with_json(json.borrow(), Some(SpaType::ARRAY))
	}

	pub fn parse_json(&self) -> Option<SpaJsonRef> {
		unsafe { self.json_unchecked().map(|json| SpaJsonRef::new(json)) }
	}

	/// Gets the [SpaJson] value from a spa json parser object
	///
	/// # Safety
	/// The returned object borrows the underlying parser's data,
	/// which is not tracked by a lifetime.
	#[doc(alias = "wp_spa_json_parser_get_json")]
	#[doc(alias = "get_json")]
	pub unsafe fn json_unchecked(&self) -> Option<SpaJson> {
		from_glib_full(ffi::wp_spa_json_parser_get_json(self.to_glib_none().0))
	}

	#[doc(alias = "wp_spa_json_parser_get_json")]
	#[doc(alias = "get_json")]
	pub fn json(&self) -> Option<SpaJson> {
		unsafe { self.json_unchecked().map(|json| json.ensure_unique_owner()) }
	}

	pub fn parse_end(self) -> Result<(), ParseError> {
		match self.parse_json() {
			None => Ok(()),
			Some(..) => Err(ParseError::Trailing),
		}
	}
}

/// A safe wrapper around a [SpaJsonParser].
///
/// The parser internally borrows a [`&'r SpaJsonRef<'j>`](SpaJsonRef),
/// so isn't safe to use without a properly constructed reference wrapper
/// that keeps track of the underlying lifetimes.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpaJsonParserRef<'r, 'j> {
	pub(crate) parser: SpaJsonParser,
	_ref: PhantomData<&'r SpaJsonRef<'j>>,
}

impl<'r, 'j> SpaJsonParserRef<'r, 'j> {
	pub fn with_json(json: &'r SpaJsonRef<'j>, ty: Option<SpaType>) -> Self {
		unsafe { Self::with_json_unchecked(json.get_json(), ty) }
	}

	pub unsafe fn with_json_unchecked(json: &'r SpaJson, ty: Option<SpaType>) -> Self {
		let parser = match ty.or_else(|| json.spa_type().ok()) {
			Some(SpaType::OBJECT) => SpaJsonParser::new_object_unchecked(json),
			Some(SpaType::ARRAY) => SpaJsonParser::new_array_unchecked(json),
			_ => SpaJsonParser::new_undefined_unchecked(json),
		};
		Self::new(parser)
	}

	pub unsafe fn new(parser: SpaJsonParser) -> Self {
		Self {
			parser,
			_ref: PhantomData,
		}
	}

	pub fn parse_json(&mut self) -> Option<SpaJsonRef<'j>> {
		unsafe { self.parser.json_unchecked().map(|j| SpaJsonRef::new(j)) }
	}

	pub fn parse_property(&mut self) -> Option<(Cow<'j, str>, SpaJsonRef<'j>)> {
		let key = self.parse_str()?;
		self.parse_json().map(|value| (key, value))
	}

	pub fn parse_str(&mut self) -> Option<Cow<'j, str>> {
		self.parse_json().and_then(|j| j.parse_str())
	}

	pub fn parse_boolean(&mut self) -> Option<bool> {
		self.parser.boolean()
	}

	pub fn parse_int(&mut self) -> Option<i32> {
		self.parser.int()
	}

	pub fn parse_float(&mut self) -> Option<f32> {
		self.parser.float()
	}

	pub fn parse_null(&mut self) -> Option<()> {
		if self.parser.null() {
			Some(())
		} else {
			None
		}
	}

	pub fn parse_next<'a, T: TryFrom<&'a mut Self>>(&'a mut self) -> Result<T, T::Error> {
		T::try_from(self)
	}

	pub fn parse_end(self) -> Result<(), ParseError> {
		unsafe { self.into_parser().parse_end() }
	}

	pub fn into_variant_array(self) -> Variant {
		let props = self.map(|v| v.to_variant());
		Variant::array_from_iter::<SpaJsonRef<'j>>(props)
	}

	pub unsafe fn into_parser(self) -> SpaJsonParser {
		self.parser
	}

	pub unsafe fn parser(&self) -> &SpaJsonParser {
		&self.parser
	}

	pub unsafe fn parser_mut(&mut self) -> &mut SpaJsonParser {
		&mut self.parser
	}

	pub fn into_object_parser(self) -> SpaJsonObjectParser<'r, 'j> {
		SpaJsonObjectParser::new(self)
	}
}

impl<'r, 'j> Iterator for SpaJsonParserRef<'r, 'j> {
	type Item = SpaJsonRef<'j>;

	fn next(&mut self) -> Option<Self::Item> {
		self.parse_json()
	}
}

impl<'r, 'j> From<&'_ mut SpaJsonParserRef<'r, 'j>> for Option<SpaJsonRef<'j>> {
	fn from(json: &mut SpaJsonParserRef<'r, 'j>) -> Self {
		json.parse_json()
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpaJsonObjectParser<'r, 'j> {
	parser: SpaJsonParserRef<'r, 'j>,
}

impl<'r, 'j> SpaJsonObjectParser<'r, 'j> {
	pub fn with_json(json: &'r SpaJsonRef<'j>) -> Self {
		unsafe { Self::with_json_unchecked(json.get_json()) }
	}

	pub unsafe fn with_json_unchecked(json: &'r SpaJson) -> Self {
		let parser = match json.is_object() {
			true => SpaJsonParser::new_object_unchecked(json),
			false => SpaJsonParser::new_undefined_unchecked(json),
		};
		Self::new(SpaJsonParserRef::new(parser))
	}

	pub const fn new(parser: SpaJsonParserRef<'r, 'j>) -> Self {
		Self { parser }
	}

	pub fn parse_end(self) -> Result<(), ParseError> {
		self.into_inner().parse_end()
	}

	pub fn into_vardict_variant(mut self) -> Variant {
		let props = iter::from_fn(|| self.parse_dict_entry(|e| e.to_variant()));
		Variant::array_from_iter::<DictEntry<&str, Variant>>(props)
	}

	pub unsafe fn into_parser(self) -> SpaJsonParser {
		self.into_inner().into_parser()
	}

	#[inline]
	pub fn into_inner(self) -> SpaJsonParserRef<'r, 'j> {
		self.parser
	}

	pub const fn inner_ref(&self) -> &SpaJsonParserRef<'r, 'j> {
		&self.parser
	}

	pub fn inner_mut(&mut self) -> &mut SpaJsonParserRef<'r, 'j> {
		&mut self.parser
	}

	fn parse_dict_entry<R, F: for<'a> FnOnce(DictEntry<&'a str, SpaJsonRef<'j>>) -> R>(&mut self, f: F) -> Option<R> {
		let (k, v) = self.parse_property()?;
		let dict_entry = DictEntry::new(&k[..], v);
		Some(f(dict_entry))
	}
}

impl<'r, 'j> Iterator for SpaJsonObjectParser<'r, 'j> {
	type Item = (Cow<'j, str>, SpaJsonRef<'j>);

	fn next(&mut self) -> Option<Self::Item> {
		self.parse_property()
	}
}

impl<'r, 'j> From<SpaJsonObjectParser<'r, 'j>> for SpaJsonParserRef<'r, 'j> {
	#[inline]
	fn from(value: SpaJsonObjectParser<'r, 'j>) -> Self {
		value.into_inner()
	}
}

impl<'r, 'j> Deref for SpaJsonObjectParser<'r, 'j> {
	type Target = SpaJsonParserRef<'r, 'j>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.inner_ref()
	}
}

impl<'r, 'j> DerefMut for SpaJsonObjectParser<'r, 'j> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.inner_mut()
	}
}

#[derive(Debug)]
pub enum ParseError {
	Error(Error),
	Custom(String),
	Char {
		string: GString,
	},
	/// Parse error
	InvalidType,
	/// Missing map value
	MapValue,
	/// Expected string key for enum map
	EnumKey,
	/// Expected EOF
	Trailing,
	TypeMismatch {
		expected: SpaType,
		found: Option<SpaType>,
	},
}

#[cfg(feature = "serde")]
impl serde::de::Error for ParseError {
	fn custom<T: Display>(msg: T) -> Self {
		Self::Custom(msg.to_string())
	}
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Error(e) => Display::fmt(e, f),
			Self::Custom(msg) => f.write_str(msg),
			Self::Char { string } => write!(f, "Deserializer expected one character but found {string:?}"),
			Self::InvalidType => f.write_str("Failed to parse JSON type"),
			Self::EnumKey => f.write_str("Expected JSON map key for enum"),
			Self::Trailing => f.write_str("Expected EOF but found trailing data"),
			Self::MapValue => f.write_str("Missing JSON map value"),
			Self::TypeMismatch { expected, found: None } =>
				write!(f, "Deserializer expected {expected:?} but found invalid JSON"),
			Self::TypeMismatch { expected, found } => write!(f, "Deserializer expected {expected:?} but found {found:?}"),
		}
	}
}

impl From<ParseError> for Error {
	fn from(error: ParseError) -> Error {
		match error {
			ParseError::Error(e) => e,
			_ => error::invalid_argument(format_args!("{error}")),
		}
	}
}

impl StdError for ParseError {}

#[cfg(feature = "serde")]
mod serde_impl {
	use {
		crate::{
			error,
			prelude::*,
			spa::{
				json::{ParseError, SpaJson, SpaJsonObjectParser, SpaJsonParser, SpaJsonParserRef, SpaJsonRef},
				SpaType,
			},
		},
		serde::{
			de::{
				self, Deserialize, DeserializeSeed, Deserializer, EnumAccess, Expected, IntoDeserializer, MapAccess, SeqAccess,
				Unexpected, VariantAccess, Visitor,
			},
			ser::{self, Serialize, SerializeMap, SerializeSeq, Serializer},
		},
	};

	// TODO: stash the name from visit_enum's argument or would that be weird?
	const ENUM_STRUCT_NAME: &'static str = "Enum";

	impl ParseError {
		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn to_ser_error<E: ser::Error>(&self) -> E {
			E::custom(self)
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn to_serde_error<E: de::Error>(&self) -> E {
			match *self {
				Self::Char { ref string } => return E::invalid_value(Unexpected::Str(&string), &"one character"),
				Self::TypeMismatch {
					expected,
					found: Some(found),
				} => match Self::unexpected_type(found) {
					Some(unexpected) => return E::invalid_type(unexpected, &Self::expected_type(expected)),
					_ => (),
				},
				_ => (),
			}

			E::custom(self)
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn into_ser_error<E: ser::Error>(self) -> E {
			self.to_ser_error()
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn into_serde_error<E: de::Error>(self) -> E {
			self.to_serde_error()
		}

		pub(crate) fn unexpected_type<'u>(ty: SpaType) -> Option<Unexpected<'u>> {
			Some(match ty {
				SpaType::NONE => Unexpected::Unit,
				SpaType::OBJECT => Unexpected::Map,
				SpaType::ARRAY | SpaType::BYTES => Unexpected::Seq,
				SpaType::CHOICE => Unexpected::Enum,
				_ => return None,
			})
		}

		pub(crate) fn expected_type(ty: SpaType) -> impl Expected {
			match ty {
				SpaType::NONE => "SpaJson null",
				SpaType::INT => "SpaJson integer",
				SpaType::FLOAT => "SpaJson floating point number",
				SpaType::STRING => "SpaJson string",
				SpaType::BOOL => "SpaJson boolean",
				SpaType::OBJECT => "SpaJson object",
				SpaType::ARRAY => "SpaJson array",
				SpaType::BYTES => "SpaJson byte array",
				SpaType::CHOICE => "SpaJson enum",
				_ => unreachable!(),
			}
		}
	}

	fn error_expected(json: &SpaJsonRef, expected: SpaType) -> ParseError {
		ParseError::TypeMismatch {
			expected,
			found: json.spa_type().ok(),
		}
	}

	impl SpaJson {
		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn deserialize_from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, Error> {
			let json = SpaJsonRef::with_str(s);
			T::deserialize(json).map_err(Into::into)
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn deserialize_from_string<'a, T: Deserialize<'a>>(s: &str) -> Result<T, Error> {
			let json = unsafe { Self::wrap_string(s) };
			T::deserialize(json).map_err(Into::into)
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn deserialize_to<'a, T: Deserialize<'a>>(&'a self) -> Result<T, Error> {
			T::deserialize(self).map_err(Into::into)
		}
	}

	impl<'j> SpaJsonRef<'j> {
		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn to_owned_deserializer<'a>(&'a self) -> impl for<'de> Deserializer<'de, Error = ParseError> + 'a + 'j {
			// TODO: this should be a newtype wrapper instead!
			self.clone().into_owned_deserializer()
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn into_owned_deserializer(self) -> impl for<'de> Deserializer<'de, Error = ParseError> + 'j {
			unsafe { self.into_json_unchecked() }
		}
	}

	macro_rules! proxy_impls {
		(fn deserialize_ignored_any;) => {
			fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
				self.spa_type()
					.and_then(|_| visitor.visit_unit())
			}
		};
		(fn deserialize_any;) => {
			fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
				match self.spa_type()? {
					SpaType::NONE => self.deserialize_unit(visitor),
					SpaType::INT => self.deserialize_i32(visitor),
					SpaType::FLOAT => self.deserialize_f32(visitor),
					SpaType::STRING => self.deserialize_str(visitor),
					SpaType::BOOL => self.deserialize_bool(visitor),
					SpaType::OBJECT => self.deserialize_map(visitor),
					SpaType::ARRAY => self.deserialize_seq(visitor),
					_ => Err(ParseError::Error(error::invariant(
						"SpaJson deserializer unhandled type",
					))),
				}
			}
		};
		(fn deserialize_char;) => {
			proxy_impls! { fn deserialize_char(self => self); }
		};
		(fn deserialize_char($this:ident => $json:expr);) => {
			fn deserialize_char<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let c = json.parse_char()?;
				visitor.visit_char(c)
			}
		};
		(fn deserialize_string;) => {
			proxy_impls! { fn deserialize_string(self => self); }
		};
		(fn deserialize_string($this:ident => $json:expr);) => {
			fn deserialize_string<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				match json.parse_string() {
					Some(s) => visitor.visit_string(s.into()),
					None => Err(error_expected(json, SpaType::STRING)),
				}
			}
		};
		(fn deserialize_byte_buf;) => {
			proxy_impls! { fn deserialize_byte_buf(self => self); }
		};
		(fn deserialize_byte_buf($this:ident => $json:expr);) => {
			fn deserialize_byte_buf<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				match json.parse_array() {
					Some(mut parser) => {
						let mut bytes = Vec::new();
						while let Some(b) = parser.parse_int() {
							let b = b.try_into().map_err(|_| ParseError::TypeMismatch {
								expected: SpaType::BYTES,
								found: Some(SpaType::INT),
							});
							bytes.push(b?);
						}
						parser.parse_end().and_then(|()| visitor.visit_byte_buf(bytes))
					},
					None => Err(error_expected(json, SpaType::BYTES)),
				}
			}
		};
		(fn deserialize_option($this:ident => $json:expr);) => {
			fn deserialize_option<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				match json.parse_null() {
					Some(()) => visitor.visit_some($this),
					None => visitor.visit_none(),
				}
			}
		};
		(fn deserialize_enum;) => {
			proxy_impls! { fn deserialize_enum(self, parser => self; parser); }
		};
		(fn deserialize_enum($this:ident, $parser:ident => $json:expr; $enum_access:expr);) => {
			fn deserialize_enum<V: Visitor<'de>>(
				$this,
				_name: &'static str,
				_variants: &'static [&'static str],
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				let json = $json;
				let $parser = match json.spa_type()? {
					SpaType::STRING => return visitor.visit_enum($this),
					found@SpaType::ARRAY =>
						return Err(ParseError::TypeMismatch {
							expected: SpaType::CHOICE,
							found: Some(found),
						}),
					_ => SpaJsonObjectParser::with_json(json),
				};
				visitor.visit_enum($enum_access)
			}
		};
		(fn deserialize@containers;) => {
			proxy_impls! { fn deserialize@containers(self, parser => self; &mut parser); }
		};
		(fn deserialize@containers($this:ident, $parser:ident => $json:expr; $access:expr);) => {
			proxy_impls! { fn deserialize_option($this => $json); }

			fn deserialize_seq<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let mut $parser = json.parse_array()
					.ok_or_else(|| error_expected(json, SpaType::ARRAY))?;
				let res = visitor.visit_seq($access)?;
				$parser.parse_end().map(|()| res)
			}

			fn deserialize_map<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let mut $parser = json.parse_object()
					.ok_or_else(|| error_expected(json, SpaType::OBJECT))?;
				let res = visitor.visit_map($access)?;
				$parser.parse_end().map(|()| res)
			}
		};
		(fn deserialize_unit;) => {
			proxy_impls! { fn deserialize_unit(self => self); }
		};
		(fn deserialize_unit($this:ident => $json:expr);) => {
			fn deserialize_unit<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				match json.parse_null() {
					Some(()) => visitor.visit_unit(),
					// TODO: could an empty map count too?
					None => Err(error_expected(json, SpaType::NONE)),
				}
			}
		};
		(fn deserialize@primitives;) => {
			proxy_impls! { fn deserialize@primitives(self => self); }
		};
		(fn deserialize_int_from@$deserialize:ident($visit:ident)($this:ident => $json:expr);) => {
			fn $deserialize<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let value = json.parse_int().ok_or_else(|| error_expected(json, SpaType::INT))?;
				match value.try_into() {
					Ok(i) => visitor.$visit(i),
					Err(..) => visitor.visit_i32(value),
				}
			}
		};
		(fn deserialize_int_into@$deserialize:ident($visit:ident)($this:ident => $json:expr);) => {
			fn $deserialize<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let value = json.parse_int().ok_or_else(|| error_expected(json, SpaType::INT));
				visitor.$visit(value?.into())
			}
		};
		(fn deserialize@primitives($this:ident => $json:expr);) => {
			proxy_impls! { fn deserialize_unit($this => $json); }
			proxy_impls! { fn deserialize_int_from@deserialize_u8(visit_u8)($this => $json); }
			proxy_impls! { fn deserialize_int_from@deserialize_u16(visit_u16)($this => $json); }
			proxy_impls! { fn deserialize_int_from@deserialize_u32(visit_u32)($this => $json); }
			proxy_impls! { fn deserialize_int_from@deserialize_u64(visit_u64)($this => $json); }
			proxy_impls! { fn deserialize_int_from@deserialize_i8(visit_i8)($this => $json); }
			proxy_impls! { fn deserialize_int_from@deserialize_i16(visit_i16)($this => $json); }
			proxy_impls! { fn deserialize_int_into@deserialize_i32(visit_i32)($this => $json); }
			proxy_impls! { fn deserialize_int_into@deserialize_i64(visit_i64)($this => $json); }

			fn deserialize_bool<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let value = json.parse_boolean().ok_or_else(|| error_expected(json, SpaType::BOOL));
				visitor.visit_bool(value?)
			}

			fn deserialize_f32<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let value = json.parse_float().ok_or_else(|| error_expected(json, SpaType::FLOAT));
				visitor.visit_f32(value?)
			}

			fn deserialize_f64<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				let json = $json;
				let value = json.parse_float().ok_or_else(|| error_expected(json, SpaType::FLOAT));
				visitor.visit_f64(value?.into())
			}
		};
		(fn(*);) => {
			fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
				self.deserialize_byte_buf(visitor)
			}

			fn deserialize_unit_struct<V: Visitor<'de>>(
				self,
				_name: &'static str,
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				self.deserialize_unit(visitor)
			}

			fn deserialize_newtype_struct<V: Visitor<'de>>(
				self,
				_name: &'static str,
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				visitor.visit_newtype_struct(self)
			}

			fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
				self.deserialize_seq(visitor)
			}

			fn deserialize_tuple_struct<V: Visitor<'de>>(
				self,
				_name: &'static str,
				_len: usize,
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				self.deserialize_seq(visitor)
			}

			fn deserialize_struct<V: Visitor<'de>>(
				self,
				_name: &'static str,
				_fields: &'static [&'static str],
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				self.deserialize_map(visitor)
			}

			fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
				self.deserialize_str(visitor)
			}
		};
		(fn@VariantAccess(*);) => {
			proxy_impls! { fn@VariantAccess(self => &mut self); }
		};
		(fn@VariantAccess($this:ident => $parser:expr$(; $parse_end:expr)?);) => {
			#[allow(unused_mut)]
			fn unit_variant(mut $this) -> Result<(), Self::Error> {
				let parser = $parser;
				let res = VariantAccess::unit_variant(parser)?;
				$($parse_end?;)?
				Ok(res)
			}

			#[allow(unused_mut)]
			fn newtype_variant_seed<T: DeserializeSeed<'de>>(mut $this, seed: T) -> Result<T::Value, Self::Error> {
				let parser = $parser;
				let res = VariantAccess::newtype_variant_seed(parser, seed)?;
				$($parse_end?;)?
				Ok(res)
			}

			#[allow(unused_mut)]
			fn tuple_variant<V: Visitor<'de>>(mut $this, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
				let parser = $parser;
				let res = VariantAccess::tuple_variant(parser, len, visitor)?;
				$($parse_end?;)?
				Ok(res)
			}

			#[allow(unused_mut)]
			fn struct_variant<V: Visitor<'de>>(
				mut $this,
				fields: &'static [&'static str],
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				let parser = $parser;
				let res = VariantAccess::struct_variant(parser, fields, visitor)?;
				$($parse_end?;)?
				Ok(res)
			}
		};
		(fn@EnumAccess(*);) => {
			proxy_impls! { fn@EnumAccess(self => &mut self); }
		};
		(fn@EnumAccess($this:ident => $parser:expr);) => {
			#[allow(unused_mut)]
			fn variant_seed<V: DeserializeSeed<'de>>(mut $this, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
				let (res, _) = EnumAccess::variant_seed($parser, seed)?;
				Ok((res, $this))
			}
		};
		(fn deserialize@all($this:ident => $de:expr);) => {
			fn deserialize_any<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_any($de, visitor)
			}

			fn deserialize_bool<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_bool($de, visitor)
			}

			fn deserialize_i8<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_i8($de, visitor)
			}

			fn deserialize_i16<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_i16($de, visitor)
			}

			fn deserialize_i32<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_i32($de, visitor)
			}

			fn deserialize_i64<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_i64($de, visitor)
			}

			fn deserialize_u8<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_u8($de, visitor)
			}

			fn deserialize_u16<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_u16($de, visitor)
			}

			fn deserialize_u32<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_u32($de, visitor)
			}

			fn deserialize_u64<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_u64($de, visitor)
			}

			fn deserialize_f32<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_f32($de, visitor)
			}

			fn deserialize_f64<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_f64($de, visitor)
			}

			fn deserialize_char<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_char($de, visitor)
			}

			fn deserialize_str<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_str($de, visitor)
			}

			fn deserialize_string<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_string($de, visitor)
			}

			fn deserialize_bytes<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_bytes($de, visitor)
			}

			fn deserialize_byte_buf<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_byte_buf($de, visitor)
			}

			fn deserialize_option<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_option($de, visitor)
			}

			fn deserialize_unit<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_unit($de, visitor)
			}

			fn deserialize_unit_struct<V: Visitor<'de>>($this, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_unit_struct($de, name, visitor)
			}

			fn deserialize_newtype_struct<V: Visitor<'de>>(
				$this,
				name: &'static str,
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_newtype_struct($de, name, visitor)
			}

			fn deserialize_seq<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_seq($de, visitor)
			}

			fn deserialize_tuple<V: Visitor<'de>>($this, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_tuple($de, len, visitor)
			}

			fn deserialize_tuple_struct<V: Visitor<'de>>(
				$this,
				name: &'static str,
				len: usize,
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_tuple_struct($de, name, len, visitor)
			}

			fn deserialize_map<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_map($de, visitor)
			}

			fn deserialize_struct<V: Visitor<'de>>(
				$this,
				name: &'static str,
				fields: &'static [&'static str],
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_struct($de, name, fields, visitor)
			}

			fn deserialize_enum<V: Visitor<'de>>(
				$this,
				name: &'static str,
				variants: &'static [&'static str],
				visitor: V,
			) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_enum($de, name, variants, visitor)
			}

			fn deserialize_identifier<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_identifier($de, visitor)
			}

			fn deserialize_ignored_any<V: Visitor<'de>>($this, visitor: V) -> Result<V::Value, Self::Error> {
				Deserializer::deserialize_ignored_any($de, visitor)
			}
		};
		(fn unit_variant;) => {
			fn unit_variant(self) -> Result<(), Self::Error> {
				match self.parse_json() {
					Some(json) => <()>::deserialize(json),
					// TODO: an empty container isn't necessarily correct here...
					None => Ok(()),
				}
			}
		};
	}

	impl<'j> Serialize for SpaJsonRef<'j> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			match self.spa_type().ok() {
				Some(SpaType::NONE) => serializer.serialize_unit(),
				Some(SpaType::INT) => self
					.parse_int()
					.ok_or_else(|| ser::Error::custom("SpaJson failed to parse int"))
					.and_then(|value| serializer.serialize_i32(value)),
				Some(SpaType::FLOAT) => self
					.parse_float()
					.ok_or_else(|| ser::Error::custom("SpaJson failed to parse float"))
					.and_then(|value| serializer.serialize_f32(value)),
				Some(SpaType::STRING) => serializer.serialize_str(&self.parse_str().expect("type checked")),
				Some(SpaType::BOOL) => self
					.parse_boolean()
					.ok_or_else(|| ser::Error::custom("SpaJson failed to parse bool"))
					.and_then(|value| serializer.serialize_bool(value)),
				Some(SpaType::OBJECT) => {
					let mut map = serializer.serialize_map(None)?;
					let mut parser = SpaJsonObjectParser::with_json(self);
					while let Some(key) = parser.parse_json() {
						let value = parser
							.parse_json()
							.ok_or_else(|| ser::Error::custom("SpaJson map value missing"))?;
						map.serialize_entry(&key, &value)?;
					}
					let res = map.end()?;
					parser.parse_end().map_err(ParseError::into_ser_error)?;
					Ok(res)
				},
				ty @ Some(SpaType::ARRAY) => {
					let mut seq = serializer.serialize_seq(None)?;
					let mut parser = SpaJsonParserRef::with_json(self, ty);
					while let Some(item) = parser.parse_json() {
						seq.serialize_element(&item)?
					}
					let res = seq.end()?;
					parser.parse_end().map_err(ParseError::into_ser_error)?;
					Ok(res)
				},
				_ => Err(ser::Error::custom("SpaJson failed to parse type")),
			}
		}
	}

	impl<'de> Deserializer<'de> for &'_ SpaJsonRef<'de> {
		type Error = ParseError;

		proxy_impls! { fn deserialize_char; }
		proxy_impls! { fn deserialize_string; }
		proxy_impls! { fn deserialize_byte_buf; }
		proxy_impls! { fn deserialize@primitives; }
		proxy_impls! { fn deserialize@containers; }
		proxy_impls! { fn deserialize_enum; }
		proxy_impls! { fn deserialize_ignored_any; }
		proxy_impls! { fn deserialize_any; }
		proxy_impls! { fn(*); }

		fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
			match self.parse_str() {
				Some(Cow::Borrowed(s)) => visitor.visit_borrowed_str(s),
				Some(Cow::Owned(s)) => visitor.visit_string(s),
				None => Err(error_expected(self, SpaType::STRING)),
			}
		}
	}

	impl<'de> Deserializer<'de> for SpaJsonRef<'de> {
		type Error = ParseError;

		proxy_impls! { fn deserialize@all(self => &self); }
	}

	impl<'de> EnumAccess<'de> for &'_ SpaJsonRef<'de> {
		type Error = ParseError;
		type Variant = Self;

		fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
			seed.deserialize(self).map(|v| (v, self))
		}
	}

	impl<'de> VariantAccess<'de> for &'_ SpaJsonRef<'de> {
		type Error = ParseError;

		fn unit_variant(self) -> Result<(), Self::Error> {
			Ok(())
		}

		fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
			seed.deserialize(SpaJsonRef::new_null())
		}

		fn tuple_variant<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> {
			Err(ParseError::MapValue)
		}

		fn struct_variant<V: Visitor<'de>>(
			self,
			_fields: &'static [&'static str],
			_visitor: V,
		) -> Result<V::Value, Self::Error> {
			Err(ParseError::MapValue)
		}
	}

	impl Serialize for SpaJson {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			Serialize::serialize(self.borrow(), serializer)
		}
	}

	impl<'de> Deserializer<'de> for SpaJson {
		type Error = ParseError;

		proxy_impls! { fn(*); }
		proxy_impls! { fn deserialize_char(self => self.borrow()); }
		proxy_impls! { fn deserialize_string(self => self.borrow()); }
		proxy_impls! { fn deserialize_byte_buf(self => self.borrow()); }
		proxy_impls! { fn deserialize@primitives(self => self.borrow()); }
		proxy_impls! { fn deserialize@containers(self, parser => self.borrow(); unsafe { parser.parser_mut() }); }
		proxy_impls! { fn deserialize_enum(self, parser => self.borrow(); unsafe { parser.into_parser() }); }
		proxy_impls! { fn deserialize_ignored_any; }
		proxy_impls! { fn deserialize_any; }

		fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
			self
				.parse_with_str(|s| visitor.visit_str(&s))
				.ok_or_else(|| error_expected(self.borrow(), SpaType::STRING))
				.and_then(|res| res)
		}
	}

	impl<'de> Deserializer<'de> for &'de SpaJson {
		type Error = ParseError;

		proxy_impls! { fn deserialize@all(self => self.borrow()); }
	}

	impl<'de> Deserializer<'de> for &'de mut SpaJson {
		type Error = ParseError;

		proxy_impls! { fn deserialize@all(self => &*self); }
	}

	impl<'de> EnumAccess<'de> for SpaJson {
		type Error = ParseError;
		type Variant = Self;

		fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
			seed.deserialize(self.clone()).map(|v| (v, self))
		}
	}

	impl<'de> VariantAccess<'de> for SpaJson {
		type Error = ParseError;

		fn unit_variant(self) -> Result<(), Self::Error> {
			Ok(())
		}

		fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
			seed.deserialize(SpaJsonRef::new_null())
		}

		fn tuple_variant<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> {
			Err(ParseError::MapValue)
		}

		fn struct_variant<V: Visitor<'de>>(
			self,
			_fields: &'static [&'static str],
			_visitor: V,
		) -> Result<V::Value, Self::Error> {
			Err(ParseError::MapValue)
		}
	}

	impl<'de> SeqAccess<'de> for SpaJsonParserRef<'_, 'de> {
		type Error = ParseError;

		fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
			self.parse_json().map(|json| seed.deserialize(json)).transpose()
		}
	}

	impl<'de> MapAccess<'de> for SpaJsonParserRef<'_, 'de> {
		type Error = ParseError;

		fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
			self.parse_json().map(|json| seed.deserialize(json)).transpose()
		}

		fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
			self
				.parse_json()
				.ok_or(ParseError::MapValue)
				.and_then(|next| seed.deserialize(next))
		}

		fn next_entry_seed<K: DeserializeSeed<'de>, V: DeserializeSeed<'de>>(
			&mut self,
			kseed: K,
			vseed: V,
		) -> Result<Option<(K::Value, V::Value)>, Self::Error> {
			let (k, v) = match self.parse_property() {
				Some(p) => p,
				None => return Ok(None),
			};
			let key = kseed.deserialize(k.into_deserializer())?;
			vseed.deserialize(v).map(|v| Some((key, v)))
		}
	}

	impl<'de> MapAccess<'de> for SpaJsonObjectParser<'_, 'de> {
		type Error = ParseError;

		fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
			MapAccess::next_key_seed(self.inner_mut(), seed)
		}

		fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
			MapAccess::next_value_seed(self.inner_mut(), seed)
		}

		fn next_entry_seed<K: DeserializeSeed<'de>, V: DeserializeSeed<'de>>(
			&mut self,
			kseed: K,
			vseed: V,
		) -> Result<Option<(K::Value, V::Value)>, Self::Error> {
			let (k, v) = match self.parse_property() {
				Some(p) => p,
				None => return Ok(None),
			};
			let key = kseed.deserialize(k.into_deserializer())?;
			vseed.deserialize(v).map(|v| Some((key, v)))
		}
	}

	impl<'de> EnumAccess<'de> for SpaJsonParserRef<'_, 'de> {
		type Error = ParseError;
		type Variant = Self;

		proxy_impls! { fn@EnumAccess(*); }
	}

	impl<'de> VariantAccess<'de> for SpaJsonParserRef<'_, 'de> {
		type Error = ParseError;

		proxy_impls! { fn@VariantAccess(self => &mut self; self.parse_end()); }
	}

	impl<'de, 'a> EnumAccess<'de> for &'a mut SpaJsonParserRef<'_, 'de> {
		type Error = ParseError;
		type Variant = Self;

		fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
			self
				.parse_json()
				.ok_or(ParseError::EnumKey)
				.and_then(|next| seed.deserialize(next))
				.map(|v| (v, self))
		}
	}

	impl<'de, 'a> VariantAccess<'de> for &'a mut SpaJsonParserRef<'_, 'de> {
		type Error = ParseError;

		proxy_impls! { fn unit_variant; }

		fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
			self
				.parse_json()
				.ok_or(ParseError::MapValue)
				.and_then(|next| seed.deserialize(next))
		}

		fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
			self
				.parse_json()
				.ok_or(ParseError::MapValue)
				.and_then(|next| next.deserialize_tuple(len, visitor))
		}

		fn struct_variant<V: Visitor<'de>>(
			self,
			fields: &'static [&'static str],
			visitor: V,
		) -> Result<V::Value, Self::Error> {
			self
				.parse_json()
				.ok_or(ParseError::MapValue)
				.and_then(|next| next.deserialize_struct(ENUM_STRUCT_NAME, fields, visitor))
		}
	}

	/// This parser cannot borrow data, use a [SpaJsonParserRef] instead if possible!
	impl<'de> SeqAccess<'de> for &'_ mut SpaJsonParser {
		type Error = ParseError;

		fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
			unsafe { self.json_unchecked() }
				.map(|next| seed.deserialize(next))
				.transpose()
		}
	}

	impl<'de> MapAccess<'de> for &'_ mut SpaJsonParser {
		type Error = ParseError;

		fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
			unsafe { self.json_unchecked() }
				.map(|next| seed.deserialize(next))
				.transpose()
		}

		fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
			self
				.parse_json()
				.ok_or(ParseError::MapValue)
				.and_then(|next| unsafe { seed.deserialize(next.into_json_unchecked()) })
		}
	}

	impl<'de> EnumAccess<'de> for &'_ mut SpaJsonParser {
		type Error = ParseError;
		type Variant = Self;

		fn variant_seed<V: DeserializeSeed<'de>>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
			MapAccess::next_key_seed(&mut self, seed)
				.and_then(|next| next.ok_or(ParseError::EnumKey))
				.map(|v| (v, self))
		}
	}

	impl<'de> VariantAccess<'de> for &'_ mut SpaJsonParser {
		type Error = ParseError;

		proxy_impls! { fn unit_variant; }

		fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
			let next = self.parse_json().ok_or(ParseError::MapValue)?;
			seed.deserialize(next.into_owned_deserializer())
		}

		fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
			let next = self.parse_json().ok_or(ParseError::MapValue)?;
			next.into_owned_deserializer().deserialize_tuple(len, visitor)
		}

		fn struct_variant<V: Visitor<'de>>(
			self,
			fields: &'static [&'static str],
			visitor: V,
		) -> Result<V::Value, Self::Error> {
			let next = self.parse_json().ok_or(ParseError::MapValue)?;
			next
				.into_owned_deserializer()
				.deserialize_struct(ENUM_STRUCT_NAME, fields, visitor)
		}
	}

	impl<'de> EnumAccess<'de> for SpaJsonParser {
		type Error = ParseError;
		type Variant = Self;

		proxy_impls! { fn@EnumAccess(*); }
	}

	impl<'de> VariantAccess<'de> for SpaJsonParser {
		type Error = ParseError;

		proxy_impls! { fn@VariantAccess(self => &mut self; self.parse_end()); }
	}

	impl<'de> EnumAccess<'de> for SpaJsonObjectParser<'_, 'de> {
		type Error = ParseError;
		type Variant = Self;

		proxy_impls! { fn@EnumAccess(*); }
	}

	impl<'de> VariantAccess<'de> for SpaJsonObjectParser<'_, 'de> {
		type Error = ParseError;

		proxy_impls! { fn@VariantAccess(self => &mut self; self.parse_end()); }
	}

	impl<'de> EnumAccess<'de> for &'_ mut SpaJsonObjectParser<'_, 'de> {
		type Error = ParseError;
		type Variant = Self;

		proxy_impls! { fn@EnumAccess(self => self.inner_mut()); }
	}

	impl<'de> VariantAccess<'de> for &'_ mut SpaJsonObjectParser<'_, 'de> {
		type Error = ParseError;

		proxy_impls! { fn@VariantAccess(self => self.inner_mut()); }
	}
}
