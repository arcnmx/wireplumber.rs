use std::borrow::Cow;
use glib::VariantType;
use glib::variant::VariantTypeMismatchError;
use glib::variant::DictEntry;
use crate::lua::{LuaString, LuaError};
use crate::prelude::*;

newtype_wrapper! {
	/// A [Variant] that can be passed to a Lua script.
	///
	/// `LuaVariant` serves as a sort of adhoc ABI for WirePlumber modules
	/// and their configuration data.
	#[derive(Debug, Eq, Clone, Hash)]
	pub struct LuaVariant(Variant ;? LuaError);
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum LuaType {
	Nil,
	Boolean,
	Integer,
	Float,
	String,
	Table,
}

impl LuaType {
	pub fn with_variant_type(ty: &VariantTy) -> Option<Self> {
		Some(match ty {
			ty if ty == VariantTy::UNIT => LuaType::Nil,
			ty if ty == VariantTy::BOOLEAN => LuaType::Boolean,
			ty if ty == VariantTy::INT64 || ty == VariantTy::INT32 || ty == VariantTy::INT16
				|| ty == VariantTy::UINT32 || ty == VariantTy::UINT16
				=> LuaType::Integer,
			ty if ty == VariantTy::DOUBLE => LuaType::Float,
			ty if ty == VariantTy::STRING /*|| ty == VariantTy::BYTE_STRING*/ => LuaType::String,
			ty if ty.is_subtype_of(VariantTy::DICTIONARY) => match ty.element() {
				ty if Self::with_variant_type(ty.key()).is_some() && Self::with_variant_type(ty.value()).is_some() => LuaType::Table,
				_ => return None,
			},
			ty if ty.is_array() && Self::with_variant_type(ty.element()).is_some() => LuaType::Table,
			_ => return None,
		})
	}

	pub fn with_lua_variant(var: &LuaVariant) -> Self {
		const MAX_SAFE_INTEGER: u64 = i64::MAX as u64;
		match Self::with_variant_type(var.as_variant().type_()) {
			Some(ty) => ty,
			None => match var.as_variant().classify() {
				VariantClass::Uint64 => match var.as_variant().get::<u64>() {
					Some(0..=MAX_SAFE_INTEGER) => LuaType::Integer,
					_ => LuaType::Float,
				},
				VariantClass::Variant => Self::with_lua_variant(&var.flattened()),
				_ => LuaType::Table,
			},
		}
	}

	pub fn with_variant(var: &Variant) -> Result<Self, LuaError> {
		const MAX_SAFE_INTEGER: u64 = i64::MAX as u64;

		let ty = var.type_();
		if let Some(ty) = Self::with_variant_type(ty) {
			return Ok(ty)
		}

		Ok(match var.classify() {
			VariantClass::Uint64 => match var.get::<u64>() {
				Some(0..=MAX_SAFE_INTEGER) => LuaType::Integer,
				_ => LuaType::Float,
			},
			VariantClass::Variant => Self::with_variant(&var.as_variant().expect("VariantTy"))?,
			VariantClass::Array if ty.element().is_dict_entry() => match var.iter().map(|v|
				Self::with_variant(&v.child_value(0)).and_then(|_| Self::with_variant(&v.child_value(1)).map(drop))
			).collect() {
				Ok(()) => LuaType::Table,
				Err(e) => return Err(e),
			},
			VariantClass::Array if var.iter().all(|v| Self::with_variant(&v).is_ok()) => LuaType::Table,
			_ => return Err(LuaError::UnsupportedType(Cow::Owned(ty.to_owned()))),
		})
	}

	pub fn is_numeric(&self) -> bool {
		matches!(self, LuaType::Integer | LuaType::Float)
	}

	pub fn is_table(&self) -> bool {
		matches!(self, LuaType::Table)
	}

	pub fn is_nil(&self) -> bool {
		matches!(self, LuaType::Nil)
	}
}

pub enum LuaValue<'a> {
	Nil,
	Boolean(bool),
	Integer(i64),
	Float(f64),
	String(LuaString<'a>),
	Table(LuaTable<'a>),
}

impl<'a> LuaValue<'a> {
	pub fn lua_type(&self) -> LuaType {
		match self {
			LuaValue::Nil => LuaType::Nil,
			LuaValue::Boolean(..) => LuaType::Boolean,
			LuaValue::Integer(..) => LuaType::Integer,
			LuaValue::Float(..) => LuaType::Float,
			LuaValue::String(..) => LuaType::String,
			LuaValue::Table(..) => LuaType::Table,
		}
	}

	pub fn to_lua_string(&self) -> Result<LuaString, LuaError> {
		Ok(match self {
			LuaValue::Nil => b"nil"[..].into(),
			LuaValue::Boolean(v) => v.to_string().into(),
			LuaValue::Integer(v) => v.to_string().into(),
			LuaValue::Float(v) => v.to_string().into(),
			LuaValue::String(v) => v.borrowed(),
			LuaValue::Table(_) => return Err(LuaError::TypeMismatch(
				VariantTypeMismatchError::new(VariantTy::VARDICT.to_owned(), VariantTy::STRING.to_owned())
			)),
		})
	}

	pub fn to_integer(&self) -> Result<i64, LuaError> {
		match *self {
			LuaValue::Integer(v) => Ok(v),
			LuaValue::Float(v) => Ok(v as i64),
			LuaValue::String(ref s) => s.parse(),
			ref v => Err(LuaError::TypeMismatch(VariantTypeMismatchError::new(match v {
				LuaValue::Nil => VariantTy::UNIT,
				LuaValue::Boolean(..) => VariantTy::BOOLEAN,
				LuaValue::Table(t) if t.is_array().unwrap_or(false) => VariantTy::ARRAY,
				_ => VariantTy::VARDICT,
			}.to_owned(), VariantTy::INT64.to_owned()))),
		}
	}
}

impl<'v> LuaVariant<'v> {
	pub fn nil() -> Self {
		unsafe {
			Self::unsafe_from(().to_variant())
		}
	}

	fn check(v: &Variant) -> Result<(), LuaError> {
		// used by Self::wrap and Self::borrow
		LuaType::with_variant(v)
			.map(drop)
	}

	pub fn convert_from(v: &Variant) -> Result<Self, LuaError> {
		if Self::check(&v).is_ok() {
			return Ok(unsafe {
				Self::unsafe_from(v.clone())
			})
		}

		match v.classify() {
			VariantClass::Variant => LuaVariant::convert_from(
				&v.as_variant().expect("VariantClass")
			),
			VariantClass::Byte => Ok(Self::from(v.get::<u8>().expect("VariantClass") as u16)),
			VariantClass::Maybe => match v.as_maybe() {
				Some(v) => LuaVariant::convert_from(&v),
				None => Ok(().into()),
			},
			VariantClass::Array if v.type_().element().is_dict_entry() => v.iter()
				.map(|v| LuaVariant::convert_from(&v.child_value(0)).and_then(|k| LuaVariant::convert_from(&v.child_value(1)).map(|v| (k, v))))
				.collect(),
			VariantClass::Array | VariantClass::Tuple => v.iter()
				.map(|v| LuaVariant::convert_from(&v))
				.collect(),
			_ => Err(LuaError::UnsupportedType(Cow::Owned(v.type_().to_owned()))),
		}
	}

	pub fn with_bytes<B: AsRef<[u8]>>(bytes: B) -> Self {
		// TODO: LuaString::from(bytes.as_ref()).into() if BYTE_STRINGs are ever supported upstream
		bytes.as_ref().iter().map(|&v| LuaVariant::from(v as u16)).collect()
	}

	pub fn lua_type(&self) -> LuaType {
		LuaType::with_lua_variant(self)
	}

	pub fn lua_value(&self) -> LuaValue {
		match self.lua_type() {
			LuaType::Nil => Some(LuaValue::Nil),
			LuaType::Boolean => self.get_bool().map(LuaValue::Boolean),
			LuaType::Integer => self.get_integer().and_then(|i| i.ok()).map(LuaValue::Integer),
			LuaType::Float => self.get_float().and_then(|f| f.ok()).map(LuaValue::Float),
			LuaType::String => self.get_string().map(LuaValue::String),
			LuaType::Table => LuaTable::borrow(self.as_variant()).ok().map(LuaValue::Table),
		}.expect("LuaType")
	}

	pub fn flattened(&self) -> Self {
		match self.as_variant().classify() {
			VariantClass::Variant => unsafe {
				LuaVariant::unsafe_from(self.as_variant().as_variant().expect("VariantClass"))
			}.flattened(),
			_ => self.clone(),
		}
	}

	pub fn get_nil<'a>(&'a self) -> Option<()> {
		match self.as_variant().classify() {
			VariantClass::Tuple => self.as_variant().get(),
			VariantClass::Variant => self.flattened().get_nil(),
			_ => None,
		}
	}

	pub fn get_bool<'a>(&'a self) -> Option<bool> {
		match self.as_variant().classify() {
			VariantClass::Boolean => self.as_variant().get(),
			VariantClass::Variant => self.flattened().get_bool(),
			_ => None,
		}
	}

	pub fn get_table<'a>(&'a self) -> Option<LuaTable<'a>> {
		match self.lua_type() {
			LuaType::Table => Some(unsafe {
				LuaTable::unsafe_from(self.flattened().into_inner())
			}),
			_ => None,
		}
	}

	pub fn get_string(&self) -> Option<LuaString> {
		LuaString::try_from(self.as_variant()).ok()
	}

	pub fn get_float(&self) -> Option<Result<f64, LuaError>> {
		const MAX_SAFE_INTEGER: i64 = 2^53 - 1;
		const MAX_SAFE_INTEGER_UNSIGNED: u64 = 2^53 - 1;
		const MIN_SAFE_INTEGER: i64 = -(2^53 - 1);
		Some(match self.as_variant().classify() {
			VariantClass::Double => Ok(self.as_variant().get::<f64>().expect("VariantClass")),
			VariantClass::Boolean => u8::from(self.as_variant().get::<bool>().unwrap()).try_into().map_err(Into::into),
			VariantClass::Byte => self.as_variant().get::<u8>().unwrap().try_into().map_err(Into::into),
			VariantClass::Uint16 => self.as_variant().get::<u16>().unwrap().try_into().map_err(Into::into),
			VariantClass::Uint32 => self.as_variant().get::<u32>().unwrap().try_into().map_err(Into::into),
			VariantClass::Uint64 => match self.as_variant().get::<u64>().unwrap() {
				v @ 0..=MAX_SAFE_INTEGER_UNSIGNED => Ok(v as f64),
				v => Err(i32::try_from(v).unwrap_err().into()),
			},
			VariantClass::Int16 => self.as_variant().get::<i16>().unwrap().try_into().map_err(Into::into),
			VariantClass::Int32 => self.as_variant().get::<i32>().unwrap().try_into().map_err(Into::into),
			VariantClass::Int64 => match self.as_variant().get::<i64>().unwrap() {
				v @ MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER => Ok(v as f64),
				v => Err(u32::try_from(v).unwrap_err().into()),
			},
			VariantClass::Variant => return self.flattened().get_float(),
			_ => return None,
		})
	}

	pub fn get_integer<T>(&self) -> Option<Result<T, LuaError>> where
		T: TryFrom<bool>, <T as TryFrom<bool>>::Error: Into<LuaError>,
		T: TryFrom<u8>, <T as TryFrom<u8>>::Error: Into<LuaError>,
		T: TryFrom<u16>, <T as TryFrom<u16>>::Error: Into<LuaError>,
		T: TryFrom<u32>, <T as TryFrom<u32>>::Error: Into<LuaError>,
		T: TryFrom<u64>, <T as TryFrom<u64>>::Error: Into<LuaError>,
		T: TryFrom<i16>, <T as TryFrom<i16>>::Error: Into<LuaError>,
		T: TryFrom<i32>, <T as TryFrom<i32>>::Error: Into<LuaError>,
		T: TryFrom<i64>, <T as TryFrom<i64>>::Error: Into<LuaError>,
	{
		Some(match self.as_variant().classify() {
			VariantClass::Boolean => self.as_variant().get::<bool>().unwrap().try_into().map_err(Into::into),
			VariantClass::Byte => self.as_variant().get::<u8>().unwrap().try_into().map_err(Into::into),
			VariantClass::Uint16 => self.as_variant().get::<u16>().unwrap().try_into().map_err(Into::into),
			VariantClass::Uint32 => self.as_variant().get::<u32>().unwrap().try_into().map_err(Into::into),
			VariantClass::Uint64 => self.as_variant().get::<u64>().unwrap().try_into().map_err(Into::into),
			VariantClass::Int16 => self.as_variant().get::<i16>().unwrap().try_into().map_err(Into::into),
			VariantClass::Int32 => self.as_variant().get::<i32>().unwrap().try_into().map_err(Into::into),
			VariantClass::Int64 => self.as_variant().get::<i64>().unwrap().try_into().map_err(Into::into),
			VariantClass::Variant => return self.flattened().get_integer::<T>(),
			_ => return None,
		})
	}
}

impl<'a> StaticVariantType for LuaVariant<'a> {
	fn static_variant_type() -> Cow<'static, VariantTy> {
		VariantTy::VARIANT.into()
	}
}

impl<'a> ToVariant for LuaVariant<'a> {
	fn to_variant(&self) -> Variant {
		Variant::from_variant(self.as_variant())
	}
}

newtype_wrapper! {
	/// A specialized [LuaVariant] that is both an array and dictionary.
	#[derive(Debug, Eq, Clone, Hash)]
	pub struct LuaTable(Variant ;? LuaError);
}

impl<'v> LuaTable<'v> {
	fn check(v: &Variant) -> Result<(), LuaError> {
		// used by Self::wrap and Self::borrow
		LuaType::with_variant(v)
			.and_then(|ty| match ty {
				LuaType::Table => Ok(()),
				_ => Err(LuaError::TypeMismatch(
					VariantTypeMismatchError::new(v.type_().to_owned(), VariantTy::ARRAY.to_owned())
				)),
			})
	}

	pub fn into_lua_variant(self) -> LuaVariant<'v> {
		unsafe {
			LuaVariant::unsafe_from(self.into_inner())
		}
	}

	pub fn lua_variant(&self) -> LuaVariant {
		unsafe {
			LuaVariant::unsafe_from(self.as_variant())
		}
	}

	pub fn entry_len(&self) -> usize {
		self.as_variant().n_children()
	}

	pub fn is_empty(&self) -> bool {
		self.entry_len() == 0
	}

	/// Lua's `n` field, if it exists
	pub fn table_getn(&self) -> Option<u64> {
		self.by_key(&"n".into())
			.and_then(|v| v.get_integer())
			.and_then(|v| v.ok())
	}

	pub fn variant_is_dict(&self) -> bool {
		self.as_variant().type_().is_subtype_of(VariantTy::DICTIONARY)
	}

	pub fn iter_dict_entries<'a>(&'a self) -> impl Iterator<Item=DictEntry<LuaVariant<'static>, LuaVariant<'static>>> + 'a {
		self.as_variant().iter().enumerate()
			.map(move |(i, v)| unsafe { match self.variant_is_dict() {
				true => DictEntry::new(LuaVariant::unsafe_from(v.child_value(0)), LuaVariant::unsafe_from(v.child_value(1))),
				false => DictEntry::new((i as u64 + 1).into(), LuaVariant::unsafe_from(v)),
			} })
	}

	pub fn iter_array_indices<'a>(&'a self) -> impl Iterator<Item=Option<u64>> + 'a {
		self.as_variant().iter().enumerate()
			.map(move |(i, v)| match self.variant_is_dict() {
				true => unsafe { LuaVariant::unsafe_from(v.child_value(0)) }.lua_value().to_integer().ok()
					.and_then(|i| i.try_into().ok())
					.and_then(|idx: u64| idx.checked_sub(1)),
				false => Some(i as u64),
			})
	}

	pub fn array_indices(&self) -> Vec<(usize, u64)> {
		let mut indices: Vec<_> = self.iter_array_indices().enumerate()
			.filter_map(|(i, idx)| idx.map(|idx| (i, idx)))
			.collect();
		indices.sort_by_key(|&(_, idx)| idx);
		indices
	}

	pub fn iter_array_entries<'a>(&'a self) -> impl Iterator<Item=(u64, LuaVariant<'static>)> + 'a {
		self.array_indices().into_iter()
			.map(move |(i, idx)| (idx, self.value_at(i).expect("array_indices")))
	}

	pub fn iter_array<'a>(&'a self) -> impl Iterator<Item=Option<LuaVariant<'static>>> + 'a {
		let indices = self.array_indices();
		let last = self.table_getn()
			.or_else(|| indices.last().map(|&(_, idx)| idx));

		let mut indices = indices.into_iter().peekable();
		(0..=last.unwrap_or(0))
			.map(move |idx| match indices.peek() {
				Some(&(_, next)) if idx == next => {
					indices.next()
						.map(|(i, _)| self.value_at(i).expect("array_indices"))
				},
				Some(_) => None,
				None => indices.next().map(|_| unreachable!("array_indices")),
			})
	}

	pub fn array_len(&self) -> u64 {
		self.table_getn()
			.or_else(|| self.array_indices().last().map(|&(_, idx)| idx))
			.unwrap_or(0)
	}

	pub fn key_at(&self, index: usize) -> Option<LuaVariant<'static>> {
		match self.variant_is_dict() {
			true => self.as_variant().try_child_value(index)
				.map(|v| unsafe { LuaVariant::unsafe_from(v.child_value(0)) }),
			false => None,
		}
	}

	pub fn value_at(&self, index: usize) -> Option<LuaVariant<'static>> {
		self.as_variant().try_child_value(index)
			.map(|v| unsafe { LuaVariant::unsafe_from(match self.variant_is_dict() {
				true => v.child_value(1),
				false => v,
			}) })
	}

	pub fn by_key(&self, key: &LuaVariant) -> Option<LuaVariant<'static>> {
		self.iter_dict_entries().find(|e| e.key() == key)
			.map(|e| e.value().owned())
	}

	pub fn is_array(&self) -> Option<bool> {
		if self.table_getn().is_some() {
			return Some(true)
		}

		match self.iter_array_indices().all(|i| i.is_some()) {
			true if self.is_empty() => None,
			v => Some(v),
		}
	}

	pub fn is_vardict(&self) -> bool {
		self.as_variant().type_() == VariantTy::VARDICT
	}

	pub fn into_vardict(self) -> Result<Self, LuaError> {
		if self.is_vardict() {
			return Ok(self)
		}

		self.iter_dict_entries()
			.map(|e|
				e.key().lua_value().to_lua_string()
					.and_then(|s| s.into_string().map_err(Into::into))
					.map(|s| (s, e.value().owned()))
			).collect()
	}
}

impl StaticVariantType for LuaTable<'_> {
	fn static_variant_type() -> Cow<'static, VariantTy> {
		VariantTy::VARDICT.into()
	}
}

impl<'a, 'v, V: StaticVariantType + Into<LuaVariant<'v>>> FromIterator<V> for LuaTable<'a> {
	fn from_iter<T: IntoIterator<Item=V>>(iter: T) -> Self {
		unsafe {
			UnsafeFrom::unsafe_from(
				Variant::array_from_iter_with_type(&V::static_variant_type(), iter.into_iter().map(|v| v.into().to_variant()))
			)
		}
	}
}

impl<'a, 'v, K: StaticVariantType + Into<LuaVariant<'v>>, V: StaticVariantType + Into<LuaVariant<'v>>> FromIterator<(K, V)> for LuaTable<'a> {
	fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self {
		let entry_type = VariantType::new_dict_entry(&K::static_variant_type(), &V::static_variant_type());
		let entries = iter.into_iter()
			.map(|(k, v)| Variant::from_dict_entry(&k.into().into_variant(), &v.into().into_variant()));
		unsafe {
			UnsafeFrom::unsafe_from(
				Variant::array_from_iter_with_type(&entry_type, entries)
			)
		}
	}
}

impl<'a, 'v> FromIterator<LuaVariant<'v>> for LuaVariant<'a> {
	fn from_iter<T: IntoIterator<Item=LuaVariant<'v>>>(iter: T) -> Self {
		LuaTable::from_iter(iter)
			.into_lua_variant()
	}
}

impl<'a, 'v, K: StaticVariantType + Into<LuaVariant<'v>>, V: StaticVariantType + Into<LuaVariant<'v>>> FromIterator<(K, V)> for LuaVariant<'a> {
	fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self {
		LuaTable::from_iter(iter)
			.into_lua_variant()
	}
}

impl<'a> ToVariant for LuaTable<'a> {
	fn to_variant(&self) -> Variant {
		self.clone().into_vardict()
			.expect("VarDict requires UTF8 keys")
			.into_variant()
	}
}

impl<'a> From<LuaTable<'a>> for LuaVariant<'a> {
	fn from(v: LuaTable<'a>) -> Self {
		unsafe {
			LuaVariant::unsafe_from(v.into_inner())
		}
	}
}

impl<'a> UnsafeFrom<LuaVariant<'a>> for LuaTable<'a> {
	unsafe fn unsafe_from(v: LuaVariant<'a>) -> Self {
		Self::unsafe_from(v.into_inner())
	}
}

impl<'a> TryFrom<LuaVariant<'a>> for LuaTable<'a> {
	type Error = LuaError;

	fn try_from(v: LuaVariant<'a>) -> Result<Self, Self::Error> {
		Self::try_from(v.into_inner())
	}
}

macro_rules! lua_primitives {
	($($ty:ty,)*) => {
		$(
			impl<'a> From<$ty> for LuaVariant<'a> {
				fn from(v: $ty) -> Self {
					unsafe {
						Self::unsafe_from(v.to_variant())
					}
				}
			}
		)*
	};
}

lua_primitives! {
	i16, u16, i32, u32, i64, u64,
	bool, f64, (),
}

/*impl<'a, 'v> From<&'a str> for LuaVariant<'v> {
	fn from(s: &'a str) -> Self {
		unsafe {
			Self::unsafe_from(s.to_variant())
		}
	}
}*/

impl<'v, 's, S: Into<LuaString<'s>>> From<S> for LuaVariant<'v> {
	fn from(s: S) -> Self {
		unsafe {
			Self::unsafe_from(s.into().to_variant())
		}
	}
}
