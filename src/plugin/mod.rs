//! Externally loaded modules
//!
//! [Plugins](Plugin) can be provided via [externally loaded components](crate::core::Core::load_component).
//!
//! # Loading a Plugin
//!
//! Once a component is [loaded](crate::Core::load_component), it can be found by its plugin name:
//!
//! ```
//! use wireplumber::plugin::{Plugin, PluginFeatures};
//!
//! # async fn load_lua(core: wireplumber::core::Core) -> wireplumber::Result<()> {
//! core.load_component("libwireplumber-module-lua-scripting", "module", None)?;
//! core.load_lua_script("create-item.lua", ())?;
//! if let Some(p) = Plugin::find(&core, "lua-scripting") {
//!   p.activate_future(PluginFeatures::ENABLED).await?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Implementing and Exporting a Plugin
//!
//! Implementing the [PluginImpl] trait enables a type to be loaded externally as a dynamic library
//! and exported to the wireplumber daemon. [SimplePlugin] is also provided as a utility to ease
//! implementation of the module entry point loader and reduce boilerplate.
//!
//! The plugin module must be compiled as a `cdylib` in your `Cargo.toml`:
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//! ```no_run
//! use wireplumber::prelude::*;
//! use wireplumber::plugin::{self, SimplePlugin, AsyncPluginImpl};
//! use wireplumber::error;
//! use std::future::Future;
//! use std::cell::RefCell;
//! use std::pin::Pin;
//!
//! const DOMAIN: &'static str = "my-plugin";
//!
//! #[derive(Debug, Default)]
//! struct MyPlugin {
//!   arg: RefCell<Option<i32>>,
//! }
//!
//! impl AsyncPluginImpl for MyPlugin {
//!   type EnableFuture = Pin<Box<dyn Future<Output=wireplumber::Result<()>>>>;
//!   fn enable(&self, this: Self::Type) -> Self::EnableFuture {
//!     let core = this.core().unwrap();
//!     Box::pin(async move {
//!       let arg = this.arg.borrow().unwrap_or_default();
//!       wireplumber::info!(domain: DOMAIN, "enabling on {:?} with {}...", core, arg);
//!       Ok(())
//!     })
//!   }
//!
//!   fn disable(&self) {
//!     wireplumber::info!(domain: DOMAIN, "disabling...");
//!   }
//! }
//!
//! impl SimplePlugin for MyPlugin {
//!   type Args = Option<i32>;
//!
//!   fn init_args(&self, args: Self::Args) {
//!     self.arg.replace(args);
//!   }
//!
//!   fn decode_args(args: Option<glib::Variant>) -> Result<Self::Args, error::Error> {
//!     args.map(|args| match () {
//!       #[cfg(feature = "serde")]
//!       _ => wireplumber::lua::from_variant(&args),
//!       #[cfg(not(feature = "serde"))]
//!       _ => args.try_get(),
//!     }).transpose().map_err(error::invalid_argument)
//!   }
//! }
//!
//! plugin::simple_plugin_subclass! {
//!   impl ObjectSubclass for DOMAIN as MyPlugin { }
//! }
//!
//! plugin::plugin_export!(MyPlugin);
//! ```
//!
//! The source code of the automatic [PluginImpl](PluginImpl#impl-PluginImpl) for
//! [`T: AsyncPluginImpl`](AsyncPluginImpl) can also serve as an example implementation of a plugin
//! without using async futures.
//!
//! # See also
//!
//! C API docs for:
//!
//! - [Plugin](https://pipewire.pages.freedesktop.org/wireplumber/c_api/plugin_api.html)
//! - [Component Loader](https://pipewire.pages.freedesktop.org/wireplumber/c_api/component_loader_api.html)

pub use crate::auto::{
	Plugin, PluginFeatures,
	ComponentLoader, LookupDirs,
	traits::PluginExt,
};

mod loader;

mod subclass;
pub use subclass::{
	PluginImpl, PluginImplExt,
	AsyncPluginImpl, AsyncPluginExt,
	SourceHandles, SourceHandlesCell,
	SimplePlugin, simple_plugin_subclass,
	SimplePluginObject,
	ModuleExport, ModuleWrapper, plugin_export,
};
