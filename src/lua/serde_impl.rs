use std::fmt;
use glib::VariantDict;
use serde::{Serialize, Deserialize, ser::{self, SerializeSeq, SerializeMap}, de};
use crate::lua::{LuaVariant, LuaTable, LuaError, LuaValue, Deserializer};
use crate::prelude::*;

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
	type Value = LuaVariant<'static>;

	fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("LuaVariant")
	}

	fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
		Ok(().into())
	}

	fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
		self.visit_i16(v.into())
	}

	fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
		self.visit_u16(v.into())
	}

	fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
		self.visit_f64(v.into())
	}

	fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
		Ok(v.into())
	}

	fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
		let mut var: Vec<LuaVariant> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
		while let Some(v) = seq.next_element()? {
			var.push(v);
		}
		Ok(LuaVariant::from_iter(var))
	}

	fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
		let dict = VariantDict::default();
		while let Some((key, value)) = map.next_entry::<Cow<str>, _>()? {
			dict.insert_value(&key, LuaVariant::as_variant(&value));
		}

		dict.end().try_into()
			.map_err(LuaError::serde_error)
	}

	fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
		self.visit_unit()
	}

	fn visit_some<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
		deserializer.deserialize_any(self)
	}

	fn visit_enum<A: de::EnumAccess<'de>>(self, _data: A) -> Result<Self::Value, A::Error> {
		Err(LuaError::Custom("unimplemented: wireplumber::lua::Visitor::visit_enum".into()).serde_error())
	}

	fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
		Ok(LuaVariant::with_bytes(v))
	}
}

impl<'de, 'v> Deserialize<'de> for LuaVariant<'v> {
	fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_any(Visitor)
	}
}

struct TableVisitor;

impl<'de> de::Visitor<'de> for TableVisitor {
	type Value = LuaTable<'static>;

	fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("LuaTable")
	}

	fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
		Visitor.visit_map(map)
			.map(|v| unsafe {
				UnsafeFrom::unsafe_from(v)
			})
	}

	fn visit_seq<A: de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
		Visitor.visit_seq(seq)
			.map(|v| unsafe {
				UnsafeFrom::unsafe_from(v)
			})
	}
}

impl<'de, 'v> Deserialize<'de> for LuaTable<'v> {
	fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_any(TableVisitor)
	}
}

impl<'v> Serialize for LuaVariant<'v> {
	fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self.lua_value() {
			LuaValue::Nil => serializer.serialize_unit(),
			LuaValue::Boolean(v) => serializer.serialize_bool(v),
			LuaValue::Float(v) => serializer.serialize_f64(v),
			LuaValue::Integer(v) => serializer.serialize_i64(v),
			LuaValue::String(v) => match v.as_str() {
				Ok(s) => serializer.serialize_str(s),
				Err(_) => serializer.serialize_bytes(&v),
			},
			LuaValue::Table(t) => match t.is_array() {
				Some(true) | None => {
					let len = t.array_len();
					let len = len.try_into()
						.map_err(|e| ser::Error::custom(format_args!("Lua array length {} too large for usize: {}", len, e)))?;
					let mut ser = serializer.serialize_seq(Some(len))?;
					for v in t.iter_array() {
						match v {
							Some(v) => ser.serialize_element(&v),
							None => ser.serialize_element(&()),
						}?
					}
					ser.end()
				},
				_ => {
					let mut ser = serializer.serialize_map(Some(t.entry_len()))?;
					for v in t.iter_dict_entries() {
						ser.serialize_entry(v.key(), v.value())?;
					}
					ser.end()
				},
			},
		}
	}
}

impl<'v> Serialize for LuaTable<'v> {
	fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.lua_variant().serialize(serializer)
	}
}

impl<'de, 'v> de::IntoDeserializer<'de, LuaError> for LuaVariant<'v> {
	type Deserializer = Deserializer<'v>;
	fn into_deserializer(self) -> Self::Deserializer {
		self.into()
	}
}

impl<'a, 'de, 'v> de::IntoDeserializer<'de, LuaError> for &'a LuaVariant<'v> {
	type Deserializer = Deserializer<'v>;
	fn into_deserializer(self) -> Self::Deserializer {
		self.clone().into()
	}
}

impl<'de, 'v> de::IntoDeserializer<'de, LuaError> for LuaTable<'v> {
	type Deserializer = Deserializer<'v>;
	fn into_deserializer(self) -> Self::Deserializer {
		self.into()
	}
}

impl<'a, 'de, 'v> de::IntoDeserializer<'de, LuaError> for &'a LuaTable<'v> {
	type Deserializer = Deserializer<'v>;
	fn into_deserializer(self) -> Self::Deserializer {
		self.clone().into()
	}
}
