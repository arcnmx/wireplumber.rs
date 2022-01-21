pub use crate::auto::{
	Plugin, PluginFeatures,
	ComponentLoader, LookupDirs,
	traits::PluginExt,
};

mod subclass;
pub use subclass::{
	PluginImpl, PluginImplExt,
	AsyncPluginImpl,
	SimplePlugin, simple_plugin_subclass,
	SimplePluginObject,
	ModuleExport, ModuleWrapper, plugin_export,
	FromAnyVariant, FromAnyVariantWrapVariant,
};
