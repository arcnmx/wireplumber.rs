#[cfg(feature = "serde")]
pub use self::serde_impl::{SpaJsonChildSerializer, SpaJsonPropertySerializer, SpaJsonSerializer};
use {
	crate::{
		error,
		prelude::*,
		spa::json::{SpaJson, SpaJsonBuilder, SpaJsonRef},
	},
	std::error::Error as StdError,
};

impl SpaJsonBuilder {
	#[doc(alias = "wp_spa_json_builder_add_json")]
	pub fn add_json<'j, J: AsRef<SpaJsonRef<'j>>>(&self, json: J) {
		self.add_spa_json(unsafe { json.as_ref().get_json() })
	}

	pub fn add<T>(&mut self, value: T)
	where
		Self: Extend<T>,
	{
		self.extend([value])
	}

	pub fn add_nullable<T>(&mut self, value: Option<T>)
	where
		Self: Extend<T>,
	{
		match value {
			Some(value) => self.add(value),
			None => self.add_null(),
		}
	}
}

macro_rules! spa_json_builder_impl {
	(@add $($add:ident($ty:ty),)*) => {
		$(
			impl Extend<$ty> for SpaJsonBuilder {
				fn extend<T: IntoIterator<Item = $ty>>(&mut self, iter: T) {
					for v in iter {
						self.$add(v);
					}
				}
			}
		)*
	};
}

spa_json_builder_impl! { @add
	add_boolean(bool),
	add_int(i32),
	add_float(f32),
}

impl Extend<()> for SpaJsonBuilder {
	fn extend<I: IntoIterator<Item = ()>>(&mut self, iter: I) {
		for () in iter {
			self.add_null();
		}
	}
}

impl<'j, J: AsRef<SpaJsonRef<'j>>> Extend<J> for SpaJsonBuilder {
	fn extend<I: IntoIterator<Item = J>>(&mut self, iter: I) {
		for v in iter {
			self.add_json(v.as_ref());
		}
	}
}

impl<'j> Extend<SpaJson> for SpaJsonBuilder {
	fn extend<I: IntoIterator<Item = SpaJson>>(&mut self, iter: I) {
		for ref v in iter {
			self.add_json(v);
		}
	}
}

impl<'j, T> Extend<Option<T>> for SpaJsonBuilder
where
	Self: Extend<T>,
{
	fn extend<I: IntoIterator<Item = Option<T>>>(&mut self, iter: I) {
		for v in iter {
			self.add_nullable(v);
		}
	}
}

impl<T> FromIterator<T> for SpaJsonBuilder
where
	Self: Extend<T>,
{
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut builder = Self::new_array();
		builder.extend(iter);
		builder
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpaJsonObjectBuilder {
	builder: SpaJsonBuilder,
}

impl SpaJsonObjectBuilder {
	pub fn new() -> Self {
		Self::with_builder(SpaJsonBuilder::new_object())
	}

	#[inline]
	pub fn with_builder(builder: SpaJsonBuilder) -> Self {
		Self { builder }
	}

	#[inline]
	pub fn with_builder_ref(builder: &SpaJsonBuilder) -> &Self {
		unsafe { mem::transmute(builder) }
	}

	#[inline]
	pub fn with_builder_mut(builder: &mut SpaJsonBuilder) -> &mut Self {
		unsafe { mem::transmute(builder) }
	}

	#[inline]
	pub fn into_inner(self) -> SpaJsonBuilder {
		self.builder
	}

	pub const fn inner_ref(&self) -> &SpaJsonBuilder {
		&self.builder
	}

	pub fn inner_mut(&mut self) -> &mut SpaJsonBuilder {
		&mut self.builder
	}

	pub fn add_entry<K: AsRef<str>, T>(&mut self, key: K, value: T)
	where
		SpaJsonBuilder: Extend<T>,
	{
		self.builder.add_property(key.as_ref());
		self.builder.add(value);
	}

	pub fn end(&self) -> SpaJson {
		self.builder.end()
	}
}

impl Default for SpaJsonObjectBuilder {
	fn default() -> Self {
		Self::new()
	}
}

impl<'j, K: AsRef<str>, T> Extend<(K, T)> for SpaJsonObjectBuilder
where
	SpaJsonBuilder: Extend<T>,
{
	fn extend<I: IntoIterator<Item = (K, T)>>(&mut self, iter: I) {
		for (key, value) in iter {
			self.add_entry(key, value);
		}
	}
}

impl<T> FromIterator<T> for SpaJsonObjectBuilder
where
	Self: Extend<T>,
{
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut builder = Self::new();
		builder.extend(iter);
		builder
	}
}

impl Deref for SpaJsonObjectBuilder {
	type Target = SpaJsonBuilder;

	fn deref(&self) -> &Self::Target {
		self.inner_ref()
	}
}

impl DerefMut for SpaJsonObjectBuilder {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.inner_mut()
	}
}

impl SpaJson {
	pub fn new_array<T, I: IntoIterator<Item = T>>(items: I) -> Self
	where
		SpaJsonBuilder: Extend<T>,
	{
		SpaJsonBuilder::from_iter(items).end()
	}

	pub fn new_object<T, K: AsRef<str>, I: IntoIterator<Item = (K, T)>>(props: I) -> Self
	where
		SpaJsonBuilder: Extend<T>,
	{
		SpaJsonObjectBuilder::from_iter(props).end()
	}

	pub(crate) fn try_new_int<I: TryInto<i32> + TryInto<i64> + Copy>(value: I) -> Result<Self, BuildError>
	where
		<I as TryInto<i32>>::Error: StdError + 'static,
	{
		BuildError::serialize_int(value).map(Self::new_int)
	}

	pub(crate) fn try_new_uint<I: TryInto<i32> + TryInto<u64> + Copy>(value: I) -> Result<Self, BuildError>
	where
		<I as TryInto<i32>>::Error: StdError + 'static,
	{
		BuildError::serialize_uint(value).map(Self::new_int)
	}

	pub fn try_from_variant(variant: &Variant) -> Result<Self, BuildError> {
		match variant.type_() {
			ty if ty.is_variant() => Self::try_from_variant(&variant.as_variant().unwrap()),
			ty if ty == VariantTy::UNIT => Ok(Self::new_null()),
			ty if ty == VariantTy::BOOLEAN => Ok(Self::new_boolean(variant.get::<bool>().unwrap())),
			ty if ty == VariantTy::BYTE => Ok(Self::new_int(variant.get::<u8>().unwrap().into())),
			ty if ty == VariantTy::INT16 => Ok(Self::new_int(variant.get::<i16>().unwrap().into())),
			ty if ty == VariantTy::INT32 => Ok(Self::new_int(variant.get::<i32>().unwrap())),
			ty if ty == VariantTy::INT64 => Self::try_new_int(variant.get::<i64>().unwrap()),
			ty if ty == VariantTy::UINT16 => Ok(Self::new_int(variant.get::<u16>().unwrap().into())),
			ty if ty == VariantTy::UINT32 => Self::try_new_uint(variant.get::<u32>().unwrap()),
			ty if ty == VariantTy::UINT64 => Self::try_new_uint(variant.get::<u64>().unwrap()),
			ty if ty == VariantTy::DOUBLE => Ok(Self::new_float(variant.get::<f64>().unwrap() as f32)),
			ty if ty == VariantTy::STRING => Ok(Self::new_string(variant.str().unwrap())),
			ty if ty.is_maybe() => match variant.as_maybe() {
				None => Ok(Self::new_null()),
				Some(v) => Self::try_from_variant(&v),
			},
			ty if ty.is_subtype_of(VariantTy::DICTIONARY) => {
				let mut builder = SpaJsonObjectBuilder::new();
				for entry in variant.iter() {
					debug_assert!(entry.is_type(VariantTy::DICT_ENTRY));
					let k = entry.child_value(0);
					let v = entry.child_value(1);
					let key = k.str().ok_or(BuildError::INVALID_KEY)?;
					let value = Self::try_from_variant(&v)?;
					builder.add_entry(key, value);
				}
				Ok(builder.end())
			},
			ty if ty.is_tuple() || ty.is_array() => variant.iter().map(|v| Self::try_from_variant(&v)).collect(),
			_ty => Err(BuildError::Error(error::invariant(format_args!(
				"unknown variant type {_ty} for SpaJson"
			)))),
		}
	}
}

impl<T> FromIterator<T> for SpaJson
where
	SpaJsonBuilder: Extend<T>,
{
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self::new_array(iter)
	}
}

#[derive(Debug)]
pub enum BuildError {
	Error(Error),
	Custom(String),
	IntConversion {
		error: Box<dyn StdError>,
		value: Option<i64>,
	},
	UIntConversion {
		error: Box<dyn StdError>,
		value: Option<u64>,
	},
	InvalidKey {
		#[cfg(feature = "serde")]
		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		unexpected: Option<serde::de::Unexpected<'static>>,
	},
}

impl BuildError {
	pub const INVALID_KEY: Self = Self::InvalidKey {
		#[cfg(feature = "serde")]
		unexpected: None,
	};

	pub(crate) fn serialize_int<I: TryInto<i32> + TryInto<i64> + Copy>(value: I) -> Result<i32, BuildError>
	where
		<I as TryInto<i32>>::Error: StdError + 'static,
	{
		value.try_into().map_err(|e| BuildError::IntConversion {
			error: Box::new(e),
			value: value.try_into().ok(),
		})
	}

	pub(crate) fn serialize_uint<I: TryInto<i32> + TryInto<u64> + Copy>(value: I) -> Result<i32, BuildError>
	where
		<I as TryInto<i32>>::Error: StdError + 'static,
	{
		value.try_into().map_err(|e| BuildError::UIntConversion {
			error: Box::new(e),
			value: value.try_into().ok(),
		})
	}
}

#[cfg(feature = "serde")]
impl serde::ser::Error for BuildError {
	fn custom<T: Display>(msg: T) -> Self {
		Self::Custom(msg.to_string())
	}
}

impl fmt::Display for BuildError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Error(e) => Display::fmt(e, f),
			Self::IntConversion { error, value: None } | Self::UIntConversion { error, value: None } =>
				Display::fmt(error, f),
			Self::IntConversion { error, value: Some(v) } => write!(f, "failed to convert {v} to i32: {error}"),
			Self::UIntConversion { error, value: Some(v) } => write!(f, "failed to convert {v} to i32: {error}"),
			#[cfg(feature = "serde")]
			Self::InvalidKey {
				unexpected: Some(unexpected),
			} => write!(f, "non-string key invalid for JSON map: {unexpected}"),
			Self::InvalidKey { .. } => f.write_str("non-string key invalid for JSON map"),
			Self::Custom(msg) => f.write_str(msg),
		}
	}
}

impl From<BuildError> for Error {
	fn from(error: BuildError) -> Error {
		match error {
			BuildError::Error(e) => e,
			_ => error::invalid_argument(format_args!("{error}")),
		}
	}
}

impl StdError for BuildError {}

#[cfg(feature = "serde")]
mod serde_impl {
	use {
		crate::{
			prelude::*,
			spa::json::{BuildError, SpaJson, SpaJsonBuilder, SpaJsonObjectBuilder},
		},
		serde::{
			de::{self, Unexpected, Visitor},
			ser::{self, Impossible, Serialize, Serializer},
			Deserialize, Deserializer,
		},
	};

	impl BuildError {
		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn to_ser_error<E: ser::Error>(&self) -> E {
			E::custom(self)
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn to_serde_error<E: de::Error>(&self) -> E {
			match *self {
				Self::IntConversion {
					ref error,
					value: Some(v),
				} => E::invalid_value(Unexpected::Signed(v), &&error.to_string()[..]),
				Self::UIntConversion {
					ref error,
					value: Some(v),
				} => E::invalid_value(Unexpected::Unsigned(v), &&error.to_string()[..]),
				Self::InvalidKey {
					unexpected: Some(unexpected),
				} => E::invalid_value(unexpected, &"SpaJson object property key"),
				_ => E::custom(self),
			}
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn into_ser_error<E: ser::Error>(self) -> E {
			self.to_ser_error()
		}

		#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
		pub fn into_serde_error<E: de::Error>(self) -> E {
			self.to_serde_error()
		}

		pub(crate) fn invalid_key(unexpected: Unexpected<'static>) -> Self {
			Self::InvalidKey {
				unexpected: Some(unexpected),
			}
		}
	}

	#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
	#[derive(Debug, Copy, Clone, Default)]
	pub struct SpaJsonSerializer;

	impl<'de> Deserialize<'de> for SpaJson {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			deserializer.deserialize_any(SpaJsonSerializer)
		}
	}

	impl<'de> Visitor<'de> for SpaJsonSerializer {
		type Value = SpaJson;

		fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.write_str("SpaJson")
		}

		fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
			Ok(SpaJson::new_int(v))
		}

		fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
			SpaJson::try_new_int(v).map_err(BuildError::into_serde_error)
		}

		fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
			SpaJson::try_new_uint(v).map_err(BuildError::into_serde_error)
		}

		fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
			SpaJson::try_new_uint(v).map_err(BuildError::into_serde_error)
		}

		fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
			Ok(SpaJson::new_float(v))
		}

		fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
			self.visit_f32(v as f32)
		}

		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			Ok(SpaJson::new_string(v))
		}

		fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
			Ok(SpaJson::new_boolean(v))
		}

		fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
			Ok(SpaJson::new_null())
		}

		fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
			self.visit_unit()
		}

		fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			deserializer.deserialize_any(self)
		}

		fn visit_newtype_struct<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			deserializer.deserialize_any(self)
		}

		fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
			//Ok(SpaJson::new_array(v.iter().map(|&v| v as i32)))
			let builder = SpaJsonBuilder::new_array();
			for &b in v {
				builder.add_int(b as i32);
			}
			Ok(builder.end())
		}

		fn visit_enum<A: de::EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
			use serde::de::VariantAccess;

			/*let (key, variant) = data.variant()?;
			let value = variant.newtype_variant()?;
			Ok(SpaJson::new_object([
				(key, value)
			]))*/

			let mut builder = SpaJsonObjectBuilder::new();
			let ((), variant) = data.variant_seed(SpaJsonPropertySerializer::new(&mut builder))?;
			let () = variant.newtype_variant_seed(builder.inner_mut())?;
			Ok(builder.end())
		}

		fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
			let mut builder = SpaJsonBuilder::new_array();
			/*while let Some(elem) = seq.next_element()? {
				builder.add_json(&elem)
			}*/
			while let Some(()) = seq.next_element_seed(&mut builder)? {}
			Ok(builder.end())
		}

		fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
			let mut builder = SpaJsonObjectBuilder::new();
			/*while let Some((key, value)) = map.next_entry::<String, SpaJson>()? {
				builder.add_entry(&key, value);
			}*/
			while let Some(()) = map.next_key_seed(SpaJsonPropertySerializer::new(&mut builder))? {
				let () = map.next_value_seed(builder.inner_mut())?;
			}
			Ok(builder.end())
		}

		// remaining default impls are fine: visit_char, visit_string
	}

	impl<'a, 'de> Visitor<'de> for &'a mut SpaJsonBuilder {
		type Value = ();

		fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.write_str("SpaJson")
		}

		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			Ok(self.add_string(v))
		}

		fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
			self.visit_i32(v.into())
		}

		fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
			Ok(self.add_int(v))
		}

		fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
			BuildError::serialize_int(v)
				.map_err(BuildError::into_serde_error)
				.and_then(|v| self.visit_i32(v))
		}

		fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
			BuildError::serialize_uint(v)
				.map_err(BuildError::into_serde_error)
				.and_then(|v| self.visit_i32(v))
		}

		fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
			BuildError::serialize_uint(v)
				.map_err(BuildError::into_serde_error)
				.and_then(|v| self.visit_i32(v))
		}

		fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
			Ok(self.add_float(v))
		}

		fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
			self.visit_f32(v as f32)
		}

		fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
			Ok(self.add_boolean(v))
		}

		fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
			Ok(self.add_null())
		}

		fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
			self.visit_unit()
		}

		fn visit_newtype_struct<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			deserializer.deserialize_any(self)
		}

		fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
			let json = SpaJsonSerializer.visit_bytes(v)?;
			Ok(self.add_json(&json))
		}

		fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			let json = SpaJsonSerializer.visit_some(deserializer)?;
			Ok(self.add_json(&json))
		}

		fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
			let json = SpaJsonSerializer.visit_map(map)?;
			Ok(self.add_json(&json))
		}

		fn visit_seq<A: de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
			let json = SpaJsonSerializer.visit_seq(seq)?;
			Ok(self.add_json(&json))
		}

		fn visit_enum<A: de::EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
			let json = SpaJsonSerializer.visit_enum(data)?;
			Ok(self.add_json(&json))
		}
	}

	impl<'a, 'de> de::DeserializeSeed<'de> for &'a mut SpaJsonBuilder {
		type Value = ();

		fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			deserializer.deserialize_any(self)
		}
	}

	impl<'a, 'de> Visitor<'de> for SpaJsonPropertySerializer<'a> {
		type Value = ();

		fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.write_str("SpaJson object key string")
		}

		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			Ok(self.builder.add_property(v))
		}

		fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			deserializer.deserialize_str(self)
		}
	}

	impl<'a, 'de> de::DeserializeSeed<'de> for SpaJsonPropertySerializer<'a> {
		type Value = ();

		fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
			deserializer.deserialize_str(self)
		}
	}

	#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
	pub struct SpaJsonPropertySerializer<'a> {
		pub builder: &'a mut SpaJsonObjectBuilder,
	}

	impl<'a> SpaJsonPropertySerializer<'a> {
		pub fn new(builder: &'a mut SpaJsonObjectBuilder) -> Self {
			Self { builder }
		}
	}

	#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
	pub struct SpaJsonChildSerializer<S, P> {
		pub serializer: S,
		pub parent: P,
	}

	impl<S, P> SpaJsonChildSerializer<S, P> {
		pub fn new(serializer: S, parent: P) -> Self {
			Self { serializer, parent }
		}
	}

	impl<'a> Serializer for SpaJsonSerializer {
		type Ok = SpaJson;
		type Error = BuildError;
		type SerializeSeq = SpaJsonBuilder;
		type SerializeTuple = Self::SerializeSeq;
		type SerializeTupleStruct = Self::SerializeTuple;
		type SerializeTupleVariant = SpaJsonChildSerializer<Self::SerializeTuple, SpaJsonBuilder>;
		type SerializeMap = SpaJsonObjectBuilder;
		type SerializeStruct = Self::SerializeMap;
		type SerializeStructVariant = SpaJsonChildSerializer<Self::SerializeStruct, SpaJsonBuilder>;

		fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
			Ok(SpaJson::new_boolean(v))
		}

		fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
			Ok(SpaJson::new_int(v))
		}

		fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(BuildError::serialize_int(v)?)
		}

		fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(BuildError::serialize_uint(v)?)
		}

		fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(BuildError::serialize_uint(v)?)
		}

		fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
			Ok(SpaJson::new_float(v))
		}

		fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
			self.serialize_f32(v as f32)
		}

		fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
			self.serialize_str(&v.to_string())
		}

		fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
			Ok(SpaJson::new_string(v))
		}

		fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
			let builder = SpaJsonBuilder::new_array();
			for &byte in v {
				builder.add_int(byte.into());
			}
			Ok(builder.end())
		}

		fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
			self.serialize_unit()
		}

		fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
			value.serialize(self)
		}

		fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
			Ok(SpaJson::new_null())
		}

		fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
			self.serialize_unit()
		}

		fn serialize_unit_variant(
			self,
			_name: &'static str,
			_variant_index: u32,
			variant: &'static str,
		) -> Result<Self::Ok, Self::Error> {
			self.serialize_str(variant)
		}

		fn serialize_newtype_struct<T: ?Sized + Serialize>(
			self,
			_name: &'static str,
			value: &T,
		) -> Result<Self::Ok, Self::Error> {
			value.serialize(self)
		}

		fn serialize_newtype_variant<T: ?Sized + Serialize>(
			self,
			_name: &'static str,
			_variant_index: u32,
			variant: &'static str,
			value: &T,
		) -> Result<Self::Ok, Self::Error> {
			let mut builder = SpaJsonObjectBuilder::new();
			builder.add_property(variant);
			value.serialize(builder.inner_mut())?;
			Ok(builder.end())
		}

		fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
			let builder = SpaJsonBuilder::new_array();
			Ok(builder)
		}

		fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
			self.serialize_seq(Some(len))
		}

		fn serialize_tuple_struct(
			self,
			_name: &'static str,
			len: usize,
		) -> Result<Self::SerializeTupleStruct, Self::Error> {
			self.serialize_tuple(len)
		}

		fn serialize_tuple_variant(
			self,
			_name: &'static str,
			_variant_index: u32,
			variant: &'static str,
			len: usize,
		) -> Result<Self::SerializeTupleVariant, Self::Error> {
			let tuple = SpaJsonSerializer.serialize_tuple(len)?;
			let builder = SpaJsonObjectBuilder::new();
			builder.add_property(variant);
			Ok(SpaJsonChildSerializer::new(tuple, builder.into_inner()))
		}

		fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
			Ok(SpaJsonObjectBuilder::new())
		}

		fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
			self.serialize_map(Some(len))
		}

		fn serialize_struct_variant(
			self,
			name: &'static str,
			_variant_index: u32,
			variant: &'static str,
			len: usize,
		) -> Result<Self::SerializeStructVariant, Self::Error> {
			let struct_ = SpaJsonSerializer.serialize_struct(name, len)?;
			let builder = SpaJsonObjectBuilder::new();
			builder.add_property(variant);
			Ok(SpaJsonChildSerializer::new(struct_, builder.into_inner()))
		}
	}

	impl ser::SerializeSeq for SpaJsonBuilder {
		type Ok = SpaJson;
		type Error = BuildError;

		fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
			value.serialize(self)
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			Ok(SpaJsonBuilder::end(&self))
		}
	}

	impl ser::SerializeMap for SpaJsonObjectBuilder {
		type Ok = SpaJson;
		type Error = BuildError;

		fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
			key.serialize(SpaJsonPropertySerializer::new(self))
		}

		fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
			value.serialize(self.inner_mut())
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			Ok(SpaJsonObjectBuilder::end(&self))
		}
	}

	impl<'a, S: ser::SerializeSeq<Ok = SpaJson>> ser::SerializeSeq for SpaJsonChildSerializer<S, &'a mut SpaJsonBuilder> {
		type Ok = ();
		type Error = S::Error;

		fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
			self.serializer.serialize_element(value)
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			let json = self.serializer.end()?;
			self.parent.add_json(&json);
			Ok(())
		}
	}

	impl<'a, S: ser::SerializeTupleVariant<Ok = SpaJson>> ser::SerializeTupleVariant
		for SpaJsonChildSerializer<S, &'a mut SpaJsonBuilder>
	{
		type Ok = ();
		type Error = S::Error;

		fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
			self.serializer.serialize_field(value)
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			let json = self.serializer.end()?;
			self.parent.add_json(&json);
			Ok(())
		}
	}

	impl<'a, S: ser::SerializeStructVariant<Ok = SpaJson>> ser::SerializeStructVariant
		for SpaJsonChildSerializer<S, &'a mut SpaJsonBuilder>
	{
		type Ok = ();
		type Error = S::Error;

		fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
			self.serializer.serialize_field(key, value)
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			let json = self.serializer.end()?;
			self.parent.add_json(&json);
			Ok(())
		}
	}

	impl<'a, S: ser::SerializeTuple<Ok = SpaJson>> ser::SerializeTupleVariant
		for SpaJsonChildSerializer<S, SpaJsonBuilder>
	{
		type Ok = SpaJson;
		type Error = S::Error;

		fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
			self.serializer.serialize_element(value)
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			let json = self.serializer.end()?;
			self.parent.add_json(&json);
			Ok(SpaJsonBuilder::end(&self.parent))
		}
	}

	impl<'a, S: ser::SerializeMap<Ok = SpaJson>> ser::SerializeMap for SpaJsonChildSerializer<S, &'a mut SpaJsonBuilder> {
		type Ok = ();
		type Error = S::Error;

		fn serialize_entry<K: Serialize + ?Sized, V: Serialize + ?Sized>(
			&mut self,
			key: &K,
			value: &V,
		) -> Result<(), Self::Error> {
			self.serializer.serialize_entry(key, value)
		}

		fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
			self.serializer.serialize_key(key)
		}

		fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
			self.serializer.serialize_value(value)
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			let json = self.serializer.end()?;
			self.parent.add_json(&json);
			Ok(())
		}
	}

	impl<'a, S: ser::SerializeStruct<Ok = SpaJson>> ser::SerializeStructVariant
		for SpaJsonChildSerializer<S, SpaJsonBuilder>
	{
		type Ok = SpaJson;
		type Error = S::Error;

		fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
			self.serializer.serialize_field(key, value)
		}

		fn end(self) -> Result<Self::Ok, Self::Error> {
			let json = self.serializer.end()?;
			self.parent.add_json(&json);
			Ok(SpaJsonBuilder::end(&self.parent))
		}
	}

	impl<'a> Serializer for &'a mut SpaJsonBuilder {
		type Ok = ();
		type Error = BuildError;
		type SerializeSeq = SpaJsonChildSerializer<<SpaJsonSerializer as Serializer>::SerializeSeq, &'a mut SpaJsonBuilder>;
		type SerializeTuple = Self::SerializeSeq;
		type SerializeTupleStruct = Self::SerializeTuple;
		type SerializeTupleVariant =
			SpaJsonChildSerializer<<SpaJsonSerializer as Serializer>::SerializeTupleVariant, &'a mut SpaJsonBuilder>;
		type SerializeMap = SpaJsonChildSerializer<<SpaJsonSerializer as Serializer>::SerializeMap, &'a mut SpaJsonBuilder>;
		type SerializeStruct = Self::SerializeMap;
		type SerializeStructVariant =
			SpaJsonChildSerializer<<SpaJsonSerializer as Serializer>::SerializeStructVariant, &'a mut SpaJsonBuilder>;

		fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
			self.add_boolean(v);
			Ok(())
		}

		fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
			self.add_int(v);
			Ok(())
		}

		fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(BuildError::serialize_int(v)?)
		}

		fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(v.into())
		}

		fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(BuildError::serialize_uint(v)?)
		}

		fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
			self.serialize_i32(BuildError::serialize_uint(v)?)
		}

		fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
			self.add_float(v);
			Ok(())
		}

		fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
			self.serialize_f32(v as f32)
		}

		fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
			self.add_string(&v.to_string());
			Ok(())
		}

		fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
			self.add_string(v);
			Ok(())
		}

		fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
			use serde::ser::SerializeSeq;
			let mut seq = self.serialize_seq(Some(v.len()))?;
			for byte in v {
				seq.serialize_element(byte)?;
			}
			seq.end()
		}

		fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
			self.serialize_unit()
		}

		fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
			value.serialize(self)
		}

		fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
			self.add_null();
			Ok(())
		}

		fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
			self.serialize_unit()
		}

		fn serialize_unit_variant(
			self,
			_name: &'static str,
			_variant_index: u32,
			variant: &'static str,
		) -> Result<Self::Ok, Self::Error> {
			self.serialize_str(variant)
		}

		fn serialize_newtype_struct<T: ?Sized + Serialize>(
			self,
			_name: &'static str,
			value: &T,
		) -> Result<Self::Ok, Self::Error> {
			value.serialize(self)
		}

		fn serialize_newtype_variant<T: ?Sized + Serialize>(
			self,
			_name: &'static str,
			_variant_index: u32,
			variant: &'static str,
			value: &T,
		) -> Result<Self::Ok, Self::Error> {
			use serde::ser::SerializeMap;

			let mut map = self.serialize_map(Some(1))?;
			map.serialize_entry(variant, value)?;
			map.end()
		}

		fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
			let seq = SpaJsonSerializer.serialize_seq(len)?;
			Ok(SpaJsonChildSerializer::new(seq, self))
		}

		fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
			self.serialize_seq(Some(len))
		}

		fn serialize_tuple_struct(
			self,
			_name: &'static str,
			len: usize,
		) -> Result<Self::SerializeTupleStruct, Self::Error> {
			self.serialize_tuple(len)
		}

		fn serialize_tuple_variant(
			self,
			name: &'static str,
			variant_index: u32,
			variant: &'static str,
			len: usize,
		) -> Result<Self::SerializeTupleVariant, Self::Error> {
			let variant = SpaJsonSerializer.serialize_tuple_variant(name, variant_index, variant, len)?;
			Ok(SpaJsonChildSerializer::new(variant, self))
		}

		fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
			let map = SpaJsonSerializer.serialize_map(len)?;
			Ok(SpaJsonChildSerializer::new(map, self))
		}

		fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
			self.serialize_map(Some(len))
		}

		fn serialize_struct_variant(
			self,
			name: &'static str,
			variant_index: u32,
			variant: &'static str,
			len: usize,
		) -> Result<Self::SerializeStructVariant, Self::Error> {
			let variant = SpaJsonSerializer.serialize_struct_variant(name, variant_index, variant, len)?;
			Ok(SpaJsonChildSerializer::new(variant, self))
		}
	}

	impl<'a> Serializer for SpaJsonPropertySerializer<'a> {
		type Ok = ();
		type Error = BuildError;
		type SerializeSeq = Impossible<Self::Ok, Self::Error>;
		type SerializeTuple = Self::SerializeSeq;
		type SerializeTupleStruct = Self::SerializeTuple;
		type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
		type SerializeMap = Impossible<Self::Ok, Self::Error>;
		type SerializeStruct = Self::SerializeMap;
		type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

		fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Bool(_v)))
		}

		fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Signed(_v.into())))
		}

		fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Signed(_v.into())))
		}

		fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Signed(_v.into())))
		}

		fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Signed(_v)))
		}

		fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Unsigned(_v.into())))
		}

		fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Unsigned(_v.into())))
		}

		fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Unsigned(_v.into())))
		}

		fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Unsigned(_v)))
		}

		fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Float(_v as f64)))
		}

		fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Float(_v)))
		}

		fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
			self.serialize_str(&v.to_string())
		}

		fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
			self.builder.add_property(v);
			Ok(())
		}

		fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
			// XXX: needs lifetime for `Unexpected::Bytes`
			Err(BuildError::invalid_key(Unexpected::Seq))
		}

		fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Option))
		}

		fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
			value.serialize(self)
		}

		fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Unit))
		}

		fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Unit))
		}

		fn serialize_unit_variant(
			self,
			_name: &'static str,
			_variant_index: u32,
			_variant: &'static str,
		) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::UnitVariant))
		}

		fn serialize_newtype_struct<T: ?Sized + Serialize>(
			self,
			_name: &'static str,
			_value: &T,
		) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::NewtypeStruct))
		}

		fn serialize_newtype_variant<T: ?Sized + Serialize>(
			self,
			_name: &'static str,
			_variant_index: u32,
			_variant: &'static str,
			_value: &T,
		) -> Result<Self::Ok, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::NewtypeVariant))
		}

		fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Seq))
		}

		fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Seq))
		}

		fn serialize_tuple_struct(
			self,
			_name: &'static str,
			_len: usize,
		) -> Result<Self::SerializeTupleStruct, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Seq))
		}

		fn serialize_tuple_variant(
			self,
			_name: &'static str,
			_variant_index: u32,
			_variant: &'static str,
			_len: usize,
		) -> Result<Self::SerializeTupleVariant, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::TupleVariant))
		}

		fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Map))
		}

		fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::Map))
		}

		fn serialize_struct_variant(
			self,
			_name: &'static str,
			_variant_index: u32,
			_variant: &'static str,
			_len: usize,
		) -> Result<Self::SerializeStructVariant, Self::Error> {
			Err(BuildError::invalid_key(Unexpected::StructVariant))
		}
	}

	// proxy serializer impls

	macro_rules! spa_json_builder_proxy_impls {
		(@seq impl($($imp:tt)*) for ($($ty:tt)*)) => {
			impl<$($imp)*> ser::SerializeTuple for $($ty)* where Self: ser::SerializeSeq {
				type Ok = <Self as ser::SerializeSeq>::Ok;
				type Error = <Self as ser::SerializeSeq>::Error;

				fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
					ser::SerializeSeq::serialize_element(self, value)
				}

				fn end(self) -> Result<Self::Ok, Self::Error> {
					ser::SerializeSeq::end(self)
				}
			}

			impl<$($imp)*> ser::SerializeTupleStruct for $($ty)* where Self: ser::SerializeTuple {
				type Ok = <Self as ser::SerializeTuple>::Ok;
				type Error = <Self as ser::SerializeTuple>::Error;

				fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
					ser::SerializeTuple::serialize_element(self, value)
				}

				fn end(self) -> Result<Self::Ok, Self::Error> {
					ser::SerializeTuple::end(self)
				}
			}
		};
		(@map impl($($imp:tt)*) for ($($ty:tt)*)) => {
			impl<$($imp)*> ser::SerializeStruct for $($ty)* where Self: ser::SerializeMap {
				type Ok = <Self as ser::SerializeMap>::Ok;
				type Error = <Self as ser::SerializeMap>::Error;

				fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
					ser::SerializeMap::serialize_entry(self, key, value)
				}

				fn end(self) -> Result<Self::Ok, Self::Error> {
					ser::SerializeMap::end(self)
				}
			}
		};
		(impl($($imp:tt)*) for ($($ty:tt)*)) => {
			spa_json_builder_proxy_impls! { @seq impl($($imp)*) for ($($ty)*) }
			spa_json_builder_proxy_impls! { @map impl($($imp)*) for ($($ty)*) }
		};
	}

	spa_json_builder_proxy_impls! { @seq impl() for(SpaJsonBuilder) }
	spa_json_builder_proxy_impls! { @map impl() for(SpaJsonObjectBuilder) }
	spa_json_builder_proxy_impls! { impl('a) for(&'a mut SpaJsonBuilder) }
	spa_json_builder_proxy_impls! { impl(S, P) for(SpaJsonChildSerializer<S, P>) }
}
