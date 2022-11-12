use {
	crate::{
		lua::{LuaError, LuaVariant},
		prelude::*,
	},
	glib::{variant::VariantTypeMismatchError, ToVariant, Variant, VariantTy, VariantType},
	serde::{ser, Serialize},
};

#[cfg_attr(feature = "dox", doc(cfg(feature = "serde")))]
pub fn to_variant<S: ser::Serialize>(v: S) -> Result<LuaVariant<'static>, LuaError> {
	v.serialize(Serializer::new())
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "serde")))]
#[derive(Debug, Copy, Clone)]
pub struct Serializer {
	humanize: bool,
}

impl Default for Serializer {
	fn default() -> Self {
		Self::new()
	}
}

impl Serializer {
	pub fn new() -> Self {
		Self { humanize: true }
	}

	pub fn inhuman(self) -> Self {
		Self { humanize: false }
	}
}

impl ser::Serializer for Serializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;
	type SerializeSeq = SeqSerializer;
	type SerializeTuple = SeqSerializer;
	type SerializeTupleStruct = Self::SerializeTuple;
	type SerializeTupleVariant = VariantSerializer<Self::SerializeTuple>;
	type SerializeMap = MapSerializer;
	type SerializeStruct = StructSerializer;
	type SerializeStructVariant = VariantSerializer<Self::SerializeStruct>;

	fn is_human_readable(&self) -> bool {
		self.humanize
	}

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		self.serialize_i16(v.into())
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.serialize_u16(v.into())
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		self.serialize_f64(v as f64)
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(&v.to_string())
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		v.to_variant().try_into()
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		Ok(LuaVariant::with_bytes(v))
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}

	fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok(LuaVariant::nil())
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		if self.is_human_readable() {
			self.serialize_str(variant)
		} else {
			self.serialize_u32(variant_index)
		}
	}

	fn serialize_newtype_struct<T: Serialize + ?Sized>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: Serialize + ?Sized>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		VariantSerializer::new(self, value, name, variant_index, variant).serialize()
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Ok(SeqSerializer::new(self, len))
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		self.serialize_tuple(len)
	}

	fn serialize_tuple_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		self
			.serialize_tuple(len)
			.map(|ser| VariantSerializer::new(self, ser, name, variant_index, variant))
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Ok(MapSerializer::new(self, None, len))
	}

	fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		Ok(if self.is_human_readable() {
			StructSerializer::Map(MapSerializer::new(self, Some(VariantTy::STRING), Some(len)))
		} else {
			StructSerializer::Seq(SeqSerializer::new(self, Some(len)))
		})
	}

	fn serialize_struct_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		self
			.serialize_struct(name, len)
			.map(|ser| VariantSerializer::new(self, ser, name, variant_index, variant))
	}
}

pub struct SeqSerializer {
	ser: Serializer,
	size: Option<usize>,
	variants: Vec<LuaVariant<'static>>,
}

impl SeqSerializer {
	fn new(ser: Serializer, size: Option<usize>) -> Self {
		Self {
			variants: Vec::with_capacity(size.unwrap_or_default()),
			ser,
			size,
		}
	}
}

impl ser::SerializeSeq for SeqSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_element<S: Serialize + ?Sized>(&mut self, value: &S) -> Result<(), Self::Error> {
		value.serialize(self.ser).map(|v| self.variants.push(v))
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		match self.size {
			Some(size) if size != self.variants.len() =>
				return Err(LuaError::LengthMismatch {
					actual: self.variants.len(),
					expected: size,
				}),
			_ => (),
		}
		Ok(LuaVariant::from_iter(self.variants))
	}
}

impl ser::SerializeTuple for SeqSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		ser::SerializeSeq::end(self)
	}
}

impl ser::SerializeTupleStruct for SeqSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		ser::SerializeTuple::serialize_element(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		ser::SerializeTuple::end(self)
	}
}

impl ser::SerializeStruct for SeqSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_field<T: Serialize + ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error> {
		ser::SerializeTupleStruct::serialize_field(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		ser::SerializeTupleStruct::end(self)
	}
}

pub struct MapSerializer {
	ser: Serializer,
	key_type: Option<&'static VariantTy>,
	keys: Vec<Variant>,
	values: Vec<Variant>,
	size: Option<usize>,
}

impl MapSerializer {
	fn new(ser: Serializer, key_type: Option<&'static VariantTy>, size: Option<usize>) -> Self {
		Self {
			ser,
			key_type,
			keys: Vec::with_capacity(size.unwrap_or_default()),
			values: Vec::with_capacity(size.unwrap_or_default()),
			size,
		}
	}
}

impl ser::SerializeMap for MapSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> {
		let key = key.serialize(self.ser)?;
		match self.key_type {
			Some(key_type) if !key.as_variant().type_().is_subtype_of(key_type) =>
				return Err(LuaError::TypeMismatch(VariantTypeMismatchError::new(
					key.as_variant().type_().to_owned(),
					key_type.to_owned(),
				))),
			_ => (),
		}
		Ok(self.keys.push(key.into_variant()))
	}

	fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		value.serialize(self.ser).map(|v| self.values.push(v.into_variant()))
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		match self.size {
			Some(expected) if expected != self.values.len() =>
				return Err(LuaError::LengthMismatch {
					actual: self.values.len(),
					expected,
				}),
			_ => (),
		}

		let key_type = self.key_type;
		let elem_ty = VariantType::new_dict_entry(&self.key_type.unwrap_or(VariantTy::VARIANT), VariantTy::VARIANT);

		let entries = self
			.keys
			.into_iter()
			.map(|k| if key_type.is_some() { k } else { k.to_variant() })
			.zip(self.values.into_iter())
			.map(|(k, v)| Variant::from_dict_entry(&k, &v));

		Variant::array_from_iter_with_type(&elem_ty, entries).try_into()
	}
}

impl ser::SerializeStruct for MapSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		ser::SerializeMap::serialize_entry(self, key, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		ser::SerializeMap::end(self)
	}
}

pub enum StructSerializer {
	Map(MapSerializer),
	Seq(SeqSerializer),
}

impl ser::SerializeStruct for StructSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		match self {
			StructSerializer::Map(s) => ser::SerializeStruct::serialize_field(s, key, value),
			StructSerializer::Seq(s) => ser::SerializeStruct::serialize_field(s, key, value),
		}
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		match self {
			StructSerializer::Map(s) => ser::SerializeStruct::end(s),
			StructSerializer::Seq(s) => ser::SerializeStruct::end(s),
		}
	}
}

impl ser::SerializeTupleStruct for StructSerializer {
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		match self {
			StructSerializer::Map(_) => unimplemented!(),
			StructSerializer::Seq(s) => ser::SerializeTupleStruct::serialize_field(s, value),
		}
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		match self {
			StructSerializer::Map(s) => ser::SerializeStruct::end(s),
			StructSerializer::Seq(s) => ser::SerializeStruct::end(s),
		}
	}
}

struct VariantSerializerState {
	ser: Serializer,
	_name: &'static str,
	variant_index: u32,
	variant: &'static str,
}

impl VariantSerializerState {
	fn tag<V: AsRef<Variant> + Into<LuaVariant<'static>>>(self, variant: V) -> Result<LuaVariant<'static>, LuaError> {
		let variant = LuaVariant::convert_from(variant.as_ref())?;
		Ok(match ser::Serializer::is_human_readable(&self.ser) {
			true => iter::once((self.variant, variant)).collect(),
			false => iter::once((self.variant_index, variant)).collect(),
		})
	}
}

pub struct VariantSerializer<S> {
	inner: S,
	state: VariantSerializerState,
}

impl<S> VariantSerializer<S> {
	fn new(ser: Serializer, inner: S, _name: &'static str, variant_index: u32, variant: &'static str) -> Self {
		Self {
			inner,
			state: VariantSerializerState {
				ser,
				_name,
				variant_index,
				variant,
			},
		}
	}

	fn serialize(self) -> Result<LuaVariant<'static>, LuaError>
	where
		S: Serialize,
	{
		self.inner.serialize(self.state.ser).and_then(|v| self.state.tag(v))
	}
}

impl<S: ser::SerializeTuple> ser::SerializeTupleVariant for VariantSerializer<S>
where
	S::Ok: AsRef<Variant> + Into<LuaVariant<'static>>,
	S::Error: Into<LuaError>,
{
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		self.inner.serialize_element(value).map_err(Into::into)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let state = self.state;
		self.inner.end().map_err(Into::into).and_then(|ok| state.tag(ok))
	}
}

impl<S: ser::SerializeStruct> ser::SerializeStructVariant for VariantSerializer<S>
where
	S::Ok: AsRef<Variant> + Into<LuaVariant<'static>>,
	S::Error: Into<LuaError>,
{
	type Ok = LuaVariant<'static>;
	type Error = LuaError;

	fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
		self.inner.serialize_field(key, value).map_err(Into::into)
	}

	fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
		self.inner.skip_field(key).map_err(Into::into)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let state = self.state;
		self.inner.end().map_err(Into::into).and_then(|ok| state.tag(ok))
	}
}
