use crate::plugin::ComponentLoader;

impl ComponentLoader {
	pub const TYPE_LUA_SCRIPT: &'static str = "script/lua";
	pub const TYPE_LUA_CONFIG: &'static str = "config/lua";
	pub const TYPE_WIREPLUMBER_MODULE: &'static str = "module";
	pub const TYPE_PIPEWIRE_MODULE: &'static str = "pw_module";

	pub const DIR_WIREPLUMBER_MODULE: &'static str = "WIREPLUMBER_MODULE_DIR";
	pub const DIR_PIPEWIRE_MODULE: &'static str = "PIPEWIRE_MODULE_DIR";

	pub const MODULE_LOADER_LUA: &'static str = "libwireplumber-module-lua-scripting";

	pub const PLUGIN_LOADER_LUA: &'static str = "lua-scripting";
}
