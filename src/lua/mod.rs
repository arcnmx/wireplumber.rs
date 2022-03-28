//! Lua script execution
//!
//! The types in this module facilitate integration with the built-in
//! `libwireplumber-module-lua-scripting` plugin via
//! [Core::load_lua_script](crate::Core::load_lua_script).
//!
//! libwireplumber-module-lua-scripting only supports a limited subset of `Variant` argument types:
//! - it must be of type `VariantDict`, so values must always be boxed
//! - arrays must be converted to dictionary form, with string keys beginning with "1"

#[macro_use]
mod macros;
mod variant;
mod string;
mod error;
mod traits;
pub use variant::{LuaVariant, LuaTable, LuaType, LuaValue};
pub use string::LuaString;
pub use error::LuaError;
pub use traits::ToLuaTable;

#[cfg(feature = "serde")]
mod serde_impl;
#[cfg(feature = "serde")]
mod deserializer;
#[cfg(feature = "serde")]
mod serializer;
#[cfg(feature = "serde")]
pub use self::{
	serializer::{Serializer, to_variant},
	deserializer::{Deserializer, from_variant},
};
