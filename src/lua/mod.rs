//! Lua script execution
//!
//! The types in this module facilitate integration with the built-in
//! `libwireplumber-module-lua-scripting` plugin via
//! [Core::load_lua_script](crate::Core::load_lua_script).
//!
//! libwireplumber-module-lua-scripting only supports a limited subset of `Variant` argument types:
//! - it must be of type `VariantDict`, so values must always be boxed
//! - arrays must be converted to dictionary form, with string keys beginning with "1"

#[cfg(feature = "serde")]
pub use self::{
	deserializer::{from_variant, Deserializer},
	serializer::{to_variant, Serializer},
};
pub use self::{
	error::LuaError,
	string::LuaString,
	traits::ToLuaTable,
	variant::{LuaTable, LuaType, LuaValue, LuaVariant},
};

#[macro_use]
mod macros;
mod error;
mod string;
mod traits;
mod variant;

#[cfg(feature = "serde")]
mod deserializer;
#[cfg(feature = "serde")]
mod serde_impl;
#[cfg(feature = "serde")]
mod serializer;
