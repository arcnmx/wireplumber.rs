//! Lua script execution
//!
//! The types in this module facilitate integration with the built-in
//! `libwireplumber-module-lua-scripting` plugin via
//! [Core::load_lua_script](crate::Core::load_lua_script).

mod variant;
pub use variant::{LuaTable, ToLuaTable};
