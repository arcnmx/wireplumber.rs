use {
	crate::lua::{LuaError, LuaTable, LuaType, LuaVariant},
	glib::{variant::VariantTypeMismatchError, Variant, VariantTy},
	serde::de::{self, IntoDeserializer, Visitor},
	std::borrow::Cow,
};

#[cfg_attr(feature = "dox", doc(cfg(feature = "serde")))]
pub fn from_variant<'de, D: de::Deserialize<'de>, V: AsRef<Variant>>(v: V) -> Result<D, LuaError> {
	LuaVariant::convert_from(v.as_ref()).and_then(|v| D::deserialize(v.into_deserializer()))
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "serde")))]
#[derive(Debug, Clone)]
pub struct Deserializer<'a> {
	variant: LuaVariant<'a>,
	humanize: bool,
}

impl<'a> Deserializer<'a> {
	pub fn new(variant: LuaVariant<'a>) -> Self {
		Self {
			variant,
			humanize: true,
		}
	}

	pub fn inhuman(self) -> Self {
		Self {
			variant: self.variant,
			humanize: false,
		}
	}

	fn change_variant<'v, V: Into<LuaVariant<'v>>>(&self, variant: V) -> Deserializer<'v> {
		Deserializer {
			variant: variant.into(),
			humanize: self.humanize,
		}
	}

	fn error(&self, wanted: &VariantTy) -> LuaError {
		LuaError::TypeMismatch(VariantTypeMismatchError::new(
			self.variant.flattened().as_variant().type_().to_owned(),
			wanted.to_owned(),
		))
	}

	fn unsupported(&self) -> LuaError {
		LuaError::UnsupportedType(Cow::Owned(self.variant.flattened().as_variant().type_().to_owned()))
	}

	fn try_num<T, E: Into<LuaError>>(&self, v: Option<Result<T, E>>, wanted: &VariantTy) -> Result<T, LuaError> {
		match v {
			Some(res) => res.map_err(Into::into),
			None => Err(self.error(wanted)),
		}
	}

	fn try_get_float(&self) -> Result<f64, LuaError> {
		match self.variant.flattened().get_float() {
			Some(res) => res,
			None => Err(self.error(VariantTy::DOUBLE)),
		}
	}

	fn str(&self) -> Result<Cow<str>, LuaError> {
		match self.variant.flattened().inner_ref() {
			Err(v) => v.str().map(|s| Cow::Owned(s.into())),
			Ok(v) => v.str().map(Cow::Borrowed),
		}
		.ok_or_else(|| self.error(VariantTy::STRING))
	}
}

impl<'a, 'de> de::Deserializer<'de> for Deserializer<'a> {
	type Error = LuaError;

	fn is_human_readable(&self) -> bool {
		self.humanize
	}

	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.variant.lua_type() {
			LuaType::Nil => self.deserialize_unit(visitor),
			LuaType::Boolean => self.deserialize_bool(visitor),
			LuaType::Integer => self.deserialize_i64(visitor),
			LuaType::Float => self.deserialize_f64(visitor),
			LuaType::String => self.deserialize_str(visitor),
			LuaType::Table if self.variant.get_table().expect("LuaType").is_array().unwrap_or(true) =>
				self.deserialize_seq(visitor),
			LuaType::Table => self.deserialize_map(visitor),
		}
	}

	fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_bool(self.variant.get_bool().ok_or_else(|| self.error(VariantTy::BOOLEAN))?)
	}

	fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i8(self.try_num(self.variant.get_integer(), VariantTy::INT16)?)
	}

	fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i16(self.try_num(self.variant.get_integer(), VariantTy::INT16)?)
	}

	fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i32(self.try_num(self.variant.get_integer(), VariantTy::INT32)?)
	}

	fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_i64(self.try_num(self.variant.get_integer(), VariantTy::INT64)?)
	}

	fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u8(self.try_num(self.variant.get_integer(), VariantTy::BYTE)?)
	}

	fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u16(self.try_num(self.variant.get_integer(), VariantTy::UINT16)?)
	}

	fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u32(self.try_num(self.variant.get_integer(), VariantTy::UINT32)?)
	}

	fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_u64(self.try_num(self.variant.get_integer(), VariantTy::UINT64)?)
	}

	fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_f32(self.try_get_float()? as f32)
	}

	fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_f64(self.try_get_float()?)
	}

	fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_str(visitor)
	}

	fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_str(&self.str()?)
	}

	fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_string(self.str()?.into_owned())
	}

	fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.variant.lua_type() {
			LuaType::String => self.deserialize_str(visitor),
			LuaType::Table => visitor.visit_seq(SeqDeserializer::new(
				self.clone().variant.get_table().unwrap().iter_array(),
				self,
			)),
			_ => Err(self.error(VariantTy::BYTE_STRING)),
		}
	}

	fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_bytes(visitor)
	}

	fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.variant.lua_type() {
			LuaType::Nil => visitor.visit_unit(),
			_ => visitor.visit_some(self),
		}
	}

	fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.variant.get_nil().ok_or_else(|| self.error(VariantTy::UNIT))?;
		match self.variant.lua_type() {
			LuaType::Nil => visitor.visit_unit(),
			_ => Err(self.error(VariantTy::UNIT)),
		}
	}

	fn deserialize_unit_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_unit(visitor)
	}

	fn deserialize_newtype_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.variant.lua_type() {
			LuaType::Table => visitor.visit_seq(SeqDeserializer::new(
				self.clone().variant.get_table().unwrap().iter_array(),
				self,
			)),
			_ => Err(self.error(VariantTy::ARRAY)),
		}
	}

	fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		match self.variant.lua_type() {
			LuaType::Nil => visitor.visit_unit(),
			LuaType::Table => match self.variant.get_table().expect("LuaType").array_len() {
				actual if actual != len as u64 => Err(LuaError::LengthMismatch {
					actual: actual as usize,
					expected: len,
				}),
				_ => self.deserialize_seq(visitor),
			},
			_ => Err(self.error(VariantTy::TUPLE)),
		}
	}

	fn deserialize_tuple_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		self.deserialize_tuple(len, visitor)
	}

	fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_map(MapDeserializer::new(self))
	}

	fn deserialize_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		let table = self
			.variant
			.get_table()
			.ok_or_else(|| self.error(VariantTy::DICTIONARY))?;
		if table.is_array().unwrap_or(false) {
			self.deserialize_seq(visitor)
		} else {
			self.deserialize_map(visitor)
		}
	}

	fn deserialize_enum<V: Visitor<'de>>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		match self.variant.lua_type() {
			LuaType::Table => visitor.visit_enum(EnumDeserializer::new(self)),
			_ => visitor.visit_enum(UnitEnumDeserializer::new(self)),
		}
	}

	fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_unit()
	}

	fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.variant.lua_type() {
			LuaType::Integer => self.deserialize_i64(visitor),
			LuaType::Float => self.deserialize_f64(visitor),
			LuaType::String => self.deserialize_str(visitor),
			_ => Err(self.error(VariantTy::OBJECT_PATH)),
		}
	}
}

#[repr(transparent)]
struct EnumDeserializer<'v> {
	_de: Deserializer<'v>,
}

impl<'v> EnumDeserializer<'v> {
	fn new(de: Deserializer<'v>) -> Self {
		Self { _de: de }
	}
}

impl<'v, 'de> de::EnumAccess<'de> for EnumDeserializer<'v> {
	type Error = LuaError;
	type Variant = Self;

	fn variant_seed<V: de::DeserializeSeed<'de>>(self, _seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
		Err(LuaError::Custom(format!(
			"unimplemented wireplumber::lua::EnumDeserializer::variant_seed"
		)))
	}
}

impl<'v, 'de> de::VariantAccess<'de> for EnumDeserializer<'v> {
	type Error = LuaError;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Err(LuaError::Custom(format!(
			"unimplemented wireplumber::lua::EnumDeserializer::unit_variant"
		)))
	}

	fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, _seed: T) -> Result<T::Value, Self::Error> {
		Err(LuaError::Custom(format!(
			"unimplemented wireplumber::lua::EnumDeserializer::newtype_variant_seed"
		)))
	}

	fn tuple_variant<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> {
		Err(LuaError::Custom(format!(
			"unimplemented wireplumber::lua::EnumDeserializer::tuple_variant"
		)))
	}

	fn struct_variant<V: Visitor<'de>>(
		self,
		_fields: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error> {
		Err(LuaError::Custom(format!(
			"unimplemented wireplumber::lua::EnumDeserializer::struct_variant"
		)))
	}
}

#[repr(transparent)]
struct UnitEnumDeserializer<'v> {
	de: Deserializer<'v>,
}

impl<'v> UnitEnumDeserializer<'v> {
	fn new(de: Deserializer<'v>) -> Self {
		Self { de }
	}
}

impl<'v, 'de> de::EnumAccess<'de> for UnitEnumDeserializer<'v> {
	type Error = LuaError;
	type Variant = Self;

	fn variant_seed<V: de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
		let value = seed.deserialize(self.de.clone())?;
		Ok((value, self))
	}
}

impl<'v, 'de> de::VariantAccess<'de> for UnitEnumDeserializer<'v> {
	type Error = LuaError;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, _seed: T) -> Result<T::Value, Self::Error> {
		Err(self.de.unsupported())
	}

	fn tuple_variant<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error> {
		Err(self.de.unsupported())
	}

	fn struct_variant<V: Visitor<'de>>(
		self,
		_fields: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error> {
		Err(self.de.unsupported())
	}
}

struct SeqDeserializer<'v, I> {
	de: Deserializer<'v>,
	iter: I,
}

impl<'v, I> SeqDeserializer<'v, I> {
	fn new(iter: I, de: Deserializer<'v>) -> Self {
		Self { iter, de }
	}
}

impl<'v, 'de, 'i, I: Iterator<Item = Option<LuaVariant<'i>>>> de::SeqAccess<'de> for SeqDeserializer<'v, I> {
	type Error = LuaError;

	fn next_element_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
		self
			.iter
			.next()
			.map(|v| seed.deserialize(self.de.change_variant(v.unwrap_or_else(|| LuaVariant::nil()))))
			.transpose()
	}

	fn size_hint(&self) -> Option<usize> {
		self.iter.size_hint().1
	}
}

struct MapDeserializer<'v> {
	de: Deserializer<'v>,
	index: usize,
}

impl<'v> MapDeserializer<'v> {
	fn new(de: Deserializer<'v>) -> Self {
		Self { de, index: 0 }
	}

	fn table(&self) -> LuaTable {
		self.de.variant.get_table().expect("MapDeserializer")
	}
}

impl<'v, 'de> de::MapAccess<'de> for MapDeserializer<'v> {
	type Error = LuaError;

	fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
		let table = self.table();
		if self.index >= table.entry_len() {
			return Ok(None)
		}
		match table.key_at(self.index) {
			Some(key) => seed.deserialize(key.into_deserializer()).map(Some),
			None if !table.variant_is_dict() => seed.deserialize((self.index + 1).into_deserializer()).map(Some),
			None => Ok(None),
		}
	}

	fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
		let value = self
			.table()
			.value_at(self.index)
			.ok_or_else(|| -> LuaError { todo!() })?;
		self.index += 1;
		seed.deserialize(self.de.change_variant(value))
	}

	/*fn next_entry_seed<K: de::DeserializeSeed<'de>, V: de::DeserializeSeed<'de>>(&mut self, kseed: K, vseed: V) -> Result<Option<(K::Value, V::Value)>, Self::Error> {
		todo!()
	}*/

	fn size_hint(&self) -> Option<usize> {
		self.table().entry_len().checked_sub(self.index)
	}
}

impl<'v> From<LuaVariant<'v>> for Deserializer<'v> {
	fn from(v: LuaVariant<'v>) -> Self {
		Self::new(v)
	}
}

impl<'v> From<LuaTable<'v>> for Deserializer<'v> {
	fn from(v: LuaTable<'v>) -> Self {
		Self::new(v.into())
	}
}
