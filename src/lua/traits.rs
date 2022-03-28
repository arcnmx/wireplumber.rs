use crate::lua::{LuaTable, LuaError};
use crate::prelude::*;

pub trait ToLuaTable {
	fn to_lua_variant(self) -> Result<Option<LuaTable<'static>>, LuaError>;
}

impl ToLuaTable for () {
	fn to_lua_variant(self) -> Result<Option<LuaTable<'static>>, LuaError> {
		Ok(None)
	}
}

impl<T: TryInto<LuaTable<'static>>> ToLuaTable for T where
	T::Error: Into<LuaError>,
{
	fn to_lua_variant(self) -> Result<Option<LuaTable<'static>>, LuaError> {
		self.try_into().map(Some)
			.map_err(Into::into)
	}
}

impl<'a, T: TryInto<LuaTable<'a>>> ToLuaTable for Option<T> where
	T::Error: Into<LuaError>,
{
	fn to_lua_variant(self) -> Result<Option<LuaTable<'static>>, LuaError> {
		self.map(TryInto::try_into).transpose()
			.map_err(Into::into)
			.map(|v| v.map(|v| v.owned()))
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
