use glib::variant::DictEntry;
use crate::prelude::*;

/// Arguments for use from lua scripts.
///
/// libwireplumber-module-lua-scripting only supports a limited subset of `Variant` arguments:
/// - it must be of type `VariantDict`, so values must always be boxed
/// - arrays must be converted to dictionary form, with string keys beginning with "1"
pub struct LuaTable(Variant); // TODO: Consider Cow<Variant> instead

impl LuaTable {
	pub fn new(variant: Variant) -> Result<Self, Error> {
		if variant.is::<Self>() {
			Ok(Self(variant))
		} else {
			Self::new_copy(&variant)
		}
	}

	pub fn new_copy(variant: &Variant) -> Result<Self, Error> {
		if variant.is::<Self>() {
			return Ok(Self(variant.clone()))
		}

		let ty = variant.type_();
		match variant.classify() {
			VariantClass::Variant => Self::new(variant.child_value(0)),
			VariantClass::Array if !ty.element().is_dict_entry() => {
				debug_assert!(ty.is_array());
				let ty = ty.element();
				Ok(Self(Variant::from_iter(
					variant.iter().enumerate()
					.map(|(i, v)| DictEntry::new(
						(i + 1).to_string(),
						if ty == VariantTy::VARIANT { v.child_value(0) } else { v },
					))
				)))
			},
			VariantClass::Tuple => Ok(Self(Variant::from_iter(
				variant.iter().enumerate()
					.map(|(i, v)| DictEntry::new((i + 1).to_string(), v))
			))),
			VariantClass::Array => {
				debug_assert!(ty.is_array());
				let ty = ty.element(); debug_assert!(ty.is_dict_entry());
				let key_type = ty.key();
				let value_type = ty.value();
				let mut res = Ok(());
				let variant = Variant::from_iter(
					variant.iter().map(|kv| {
						let key = kv.child_value(0); debug_assert_eq!(key_type, key.type_());
						let value = kv.child_value(1); debug_assert_eq!(value_type, value.type_());
						let key = match Self::to_lua_string(&key) {
							Ok(key) => key,
							Err(e) => {
								res = Err(e);
								String::new()
							},
						};
						DictEntry::new(
							key,
							if value_type == VariantTy::VARIANT { value.child_value(0) } else { value },
						)
					})
				);
				res.map(|_| Self(variant))
			},
			VariantClass::Maybe if variant.n_children() == 1 => Self::new(variant.child_value(0)),
			VariantClass::Maybe => todo!("empty maybe variant"),
			ty => todo!("unsupported variant type {:?}", ty),
		}
	}

	pub fn to_lua_string(variant: &Variant) -> Result<String, Error> {
		Ok(match variant.classify() {
			VariantClass::Boolean => variant.get::<bool>().unwrap().to_string(),
			VariantClass::Byte => variant.get::<u8>().unwrap().to_string(),
			VariantClass::Int16 => variant.get::<i16>().unwrap().to_string(),
			VariantClass::Uint16 => variant.get::<u16>().unwrap().to_string(),
			VariantClass::Int32 => variant.get::<i32>().unwrap().to_string(),
			VariantClass::Uint32 => variant.get::<u32>().unwrap().to_string(),
			VariantClass::Int64 => variant.get::<i64>().unwrap().to_string(),
			VariantClass::Uint64 => variant.get::<u64>().unwrap().to_string(),
			VariantClass::Double => variant.get::<f64>().unwrap().to_string(),
			VariantClass::String | VariantClass::ObjectPath | VariantClass::Signature => variant.get().unwrap(),
			VariantClass::Variant => Self::to_lua_string(&variant.child_value(0))?,
			VariantClass::Maybe if variant.n_children() == 1 => Self::to_lua_string(&variant.child_value(0))?,
			VariantClass::Maybe => todo!(),
			_ => todo!(),
		})
	}

	pub fn inner(&self) -> &Variant {
		&self.0
	}

	pub fn into_inner(self) -> Variant {
		self.0
	}
}

impl Deref for LuaTable {
	type Target = Variant;

	fn deref(&self) -> &Self::Target {
		self.inner()
	}
}

impl StaticVariantType for LuaTable {
	fn static_variant_type() -> std::borrow::Cow<'static, VariantTy> {
		unsafe {
			VariantTy::from_str_unchecked("a{sv}").into()
		}
	}
}

impl From<LuaTable> for Variant {
	fn from(variant: LuaTable) -> Self {
		variant.0
	}
}

impl TryFrom<Variant> for LuaTable {
	type Error = Error;

	fn try_from(value: Variant) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

impl<'a> TryFrom<&'a Variant> for LuaTable {
	type Error = Error;

	fn try_from(value: &'a Variant) -> Result<Self, Self::Error> {
		Self::new_copy(value)
	}
}

pub trait ToLuaTable {
	fn to_lua_variant(self) -> Result<Option<LuaTable>, Error>;
}

impl ToLuaTable for () {
	fn to_lua_variant(self) -> Result<Option<LuaTable>, Error> {
		Ok(None)
	}
}

impl<T: TryInto<LuaTable>> ToLuaTable for T where
	T::Error: Into<Error>,
{
	fn to_lua_variant(self) -> Result<Option<LuaTable>, Error> {
		self.try_into().map(Some)
			.map_err(Into::into)
	}
}

impl<T: TryInto<LuaTable>> ToLuaTable for Option<T> where
	T::Error: Into<Error>,
{
	fn to_lua_variant(self) -> Result<Option<LuaTable>, Error> {
		self.map(TryInto::try_into).transpose()
			.map_err(Into::into)
	}
}

#[test]
fn to_lua_variant() {
	fn assert_impl<T: ToLuaTable>() { }

	assert_impl::<LuaTable>();
	assert_impl::<Option<LuaTable>>();
	assert_impl::<Option<Variant>>();
	assert_impl::<Variant>();
	assert_impl::<&'static Variant>();
	assert_impl::<Option<&'static Variant>>();
}
