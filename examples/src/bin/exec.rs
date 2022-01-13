//! wpexec example
//!
//! An example showing how to write a simple WirePlumber service.
//! Following along with the [source code](../src/exec/exec.rs.html) is recommended.
//!
//! Roughly based on the original [wpexec.c](https://gitlab.freedesktop.org/pipewire/wireplumber/-/blob/master/src/tools/wpexec.c)

use anyhow::Context;
use glib::Variant;
use clap::{Parser, ArgEnum};
use anyhow::{Result, format_err};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use std::{env, fs};

use wireplumber::{
	Core, Log, info, warning,
	pw::{self, Properties},
	plugin::{Plugin, PluginFeatures},
};

/// [GLib logging domain](glib::g_log)
const LOG_DOMAIN: &'static str = "wpexec.rs";

/// The type of module to be loaded
#[derive(ArgEnum, Copy, Clone, Debug)]
enum ModuleType {
	/// A [Lua WirePlumber script](https://pipewire.pages.freedesktop.org/wireplumber/lua_api/lua_introduction.html)
	Lua,
	/// A native WirePlumber module
	Wireplumber,
	/// A [PipeWire module](https://docs.pipewire.org/page_pipewire_modules.html)
	Pipewire,
}

#[cfg_attr(doc, doc = "Command-line arguments parsed via [clap](https://docs.rs/clap/latest/clap/)")]
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
	#[clap(arg_enum, short = 't', long = "type", default_value = "lua")]
	module_type: ModuleType,

	/// Arguments to pass to the loaded module
	///
	/// Lua scripts only support arrays and dictionary maps.
	#[clap(short = 'J', long = "json")]
	json_arg: Option<String>,

	/// Associated plugins to load, provided by the module
	#[clap(short, long = "plugin")]
	plugins: Vec<String>,

	/// Name or full path of the module or script to load
	module: Option<String>,
}

/// Load the script specified by [Args] and connect to the PipeWire daemon.
///
/// Once this function returns, the script will continue to run in the background until
/// the service is interrupted.
async fn main_async(core: &Core, args: &Args) -> Result<()> {
	let path = args.module().unwrap(); // NOTE: already checked this in main()
	let path = {
		let path = Path::new(path);
		if !args.module_type.is_script() && path.is_absolute() {
			let path = fs::canonicalize(path)?;
			let (dir, file) = (path.parent().unwrap(), path.file_name().unwrap());
			env::set_var("WIREPLUMBER_MODULE_DIR", dir);
			Some(file.to_string_lossy().into_owned())
		} else {
			None
		}
	}.unwrap_or(path.into());

	let variant_args = args.variant()?;
	if let Some(module) = args.module_type.loader_module() {
		core.load_component(module, "module", None)
			.with_context(|| format!("failed to load the {:?} scripting module", args.module_type))?;
	}
	if args.module_type.is_lua() {
		core.load_lua_script(&path, variant_args)
			.context("failed to load the lua script")?;
	} else {
		core.load_component(&path, args.module_type.loader_type(), variant_args.as_ref())
			.with_context(|| format!("failed to load {} as a {}", path, args.module_type.loader_type()))?;
	}

	core.connect_future().await?;

	let plugin_names = args.plugins();
	for plugin_name in &plugin_names {
		let p = Plugin::find(&core, plugin_name)
			.ok_or_else(|| format_err!("plugin {} not found", plugin_name))?;
		p.activate_future(PluginFeatures::ENABLED).await
			.with_context(|| format!("failed to activate {:?} plugin", plugin_name))?;
	}
	if plugin_names.is_empty() {
		info!(domain: LOG_DOMAIN, "skipped activation, no plugin specified");
	}

	Ok(())
}

/// Main entry point responsible for WirePlumber and PipeWire initialization.
///
/// This sets up the process with logging and SIGINT handlers before passing
/// a [Core](wireplumber::Core) on to [main_async] to run the application logic.
///
/// See also: [Core::run]
fn main() -> Result<()> {
	// info logging by default so we can see what's going on
	Log::set_default_level("3");

	// let clap build a CLI from argv for us
	let args = Args::parse();

	if args.module().is_none() {
		return Err(format_err!("no default module available for {:?}", args.module_type))
	}

	// bail out early if invalid args are provided
	let _ = args.variant()?;

	// initialize the wireplumber and pipewire libraries
	Core::init();

	// set up a cell to store the result of our main operation in
	let main_res = Rc::new(RefCell::new(None));

	// tell the pipewire daemon a little bit about ourselves
	let props = Properties::new_empty();
	props.insert(pw::PW_KEY_APP_NAME, LOG_DOMAIN);

	// run a (blocking) glib::MainLoop with associated core
	Core::run(Some(&props), |context, mainloop, core| {
		ctrlc::set_handler({
			// exit this loop if we receive a SIGINT
			let mainloop = mainloop.clone();
			move || mainloop.quit()
		}).unwrap();

		let main_res = main_res.clone();
		context.spawn_local(async move {
			// set up the requested module or script...
			let res = main_async(&core, &args).await;
			if res.is_err() {
				// bail out if we couldn't successfully load it after all
				mainloop.quit();
			}
			*main_res.borrow_mut() = Some(res)
		});
	});

	let main_res = main_res.borrow_mut().take();
	match main_res {
		Some(res) => res,
		// we didn't get far enough to store the result; likely it timed out or something
		None => Err(format_err!("could not connect to pipewire")),
	}
}

impl ModuleType {
	fn is_lua(&self) -> bool {
		matches!(self, ModuleType::Lua)
	}

	fn is_script(&self) -> bool {
		self.is_lua()
	}

	/// A module that provides the necessary loader
	///
	/// While wireplumber has built-in support for loading modules, the Lua scripting engine is itself
	/// implemented as a module, which must be loaded first. Afterwards, lua scripts may be loaded
	/// as modules (well, components) themselves.
	fn loader_module(&self) -> Option<&'static str> {
		match self {
			ModuleType::Lua => Some("libwireplumber-module-lua-scripting"),
			_ => None,
		}
	}

	/// The type name expected by a [wireplumber::plugin::ComponentLoader]
	///
	/// This is passed on to [Core::load_component].
	fn loader_type(&self) -> &'static str {
		match self {
			ModuleType::Lua => "script/lua",
			ModuleType::Wireplumber => "module",
			ModuleType::Pipewire => "pw_module",
		}
	}
}

impl Args {
	/// The [Plugin] names to load as part of initialization
	///
	/// For example, the [ModuleType::Lua] module provides a `lua-scripting` plugin that's responsible
	/// for loading and running lua scripts.
	fn plugins(&self) -> Vec<&str> {
		match self.module_type {
			ModuleType::Lua => vec!["lua-scripting"],
			ModuleType::Wireplumber if self.plugins.is_empty() && self.module.is_none() => vec!["static-link"],
			ModuleType::Wireplumber => self.plugins.iter().map(|s| s.as_str()).collect(),
			ModuleType::Pipewire => todo!(),
		}
	}

	/// The module or script to load
	///
	/// This can be a full path or often just be a name if WirePlumber knows where to find it
	/// (the rules for this are convoluted though, so I won't get into that here)
	fn module(&self) -> Option<&str> {
		match self.module {
			Some(ref module) => Some(module),
			None => match self.module_type {
				ModuleType::Lua =>
					Some(concat!(env!("CARGO_MANIFEST_DIR"), "/script.lua")),
				ModuleType::Wireplumber => {
					let module_path = concat!(env!("OUT_DIR"), "/../../../examples/libstatic_link_module.so"); // TODO: .so is so linux
					if fs::metadata(module_path).is_err() {
						warning!(domain: LOG_DOMAIN, "example module not found, try: cargo build -p wp-examples --example static-link-module");
					}
					Some(module_path)
				},
				_ => None,
			},
		}
	}

	/// A JSON-like blob of data to pass as an argument to the script or module to be loaded
	///
	/// In practice this must be a dictionary or array because lua scripts can't work with other
	/// types of data as the top-level container, but more specific types may be usable by other
	/// modules. See [wireplumber::lua] for a more detailed explanation.
	fn variant(&self) -> Result<Option<Variant>> {
		Ok(match self.json_arg {
			None => None,
			Some(ref json) => {
				let variant: Variant = serde_json::from_str::<glib_serde::AnyVariant>(json)?.into();
				Some(variant)
			},
		})
	}
}
