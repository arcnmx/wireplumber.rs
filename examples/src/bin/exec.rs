//! # wpexec example
//!
//! Based on [wpexec.c](https://gitlab.freedesktop.org/pipewire/wireplumber/-/blob/master/src/tools/wpexec.c).

use anyhow::Context;
use glib::{Variant, g_info};
use clap::{Parser, ArgEnum};
use anyhow::{Result, format_err};
use std::cell::RefCell;
use std::rc::Rc;
use std::env;

use wireplumber::prelude::*;
use wireplumber::{Core, Properties, Plugin, PluginFeatures, Log, pw};

#[derive(ArgEnum, Copy, Clone, Debug)]
enum ModuleType {
	Lua,
	Wireplumber,
	Pipewire,
}

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

async fn main_async(core: &Core, args: &Args) -> Result<()> {
	let path = args.module().unwrap(); // NOTE: already checked this in main()
	let variant_args = args.variant()?;
	if args.module_type.is_lua() {
		core.load_component("libwireplumber-module-lua-scripting", "module", None)
			.context("failed to load the lua-scripting module")?;
		core.load_lua_script(path, variant_args)
			.context("failed to load the lua script")?;
	} else {
		core.load_component(path, args.module_type.loader_type(), variant_args.as_ref())
			.with_context(|| format!("failed to load the {} {}", path, args.module_type.loader_type()))?;
	}

	core.connect_future().await?;

	let plugin_names = args.plugins();
	for plugin_name in &plugin_names {
		let p = Plugin::find(&core, plugin_name).unwrap();
		p.activate_future(PluginFeatures::ENABLED).await
			.with_context(|| format!("failed to activate {:?} plugin", plugin_name))?;
	}
	if plugin_names.is_empty() {
		g_info!("wpexec", "skipped activation, no plugin specified");
	}

	Ok(())
}

fn main() -> Result<()> {
	let args = Args::parse();

	if args.module().is_none() {
		return Err(format_err!("no default module available for {:?}", args.module_type))
	}

	let _ = args.variant()?; // bail out early if invalid args provided

	Log::set_default_level("3");
	Core::init(Default::default());

	let main_res = Rc::new(RefCell::new(None));

	let props = Properties::new_empty();
	props.set(pw::PW_KEY_APP_NAME, Some("wpexec"));

	Core::run(Some(&props), |context, mainloop, core| {
		ctrlc::set_handler({
			let mainloop = mainloop.clone();
			move || mainloop.quit()
		}).unwrap();

		let main_res = main_res.clone();
		context.spawn_local(async move {
			let res = main_async(&core, &args).await;
			if res.is_err() {
				mainloop.quit();
			}
			*main_res.borrow_mut() = Some(res)
		});
	});

	let main_res = main_res.borrow_mut().take();
	match main_res {
		Some(res) => res,
		None => Err(format_err!("could not connect to pipewire")),
	}
}

impl ModuleType {
	fn is_lua(&self) -> bool {
		matches!(self, ModuleType::Lua)
	}

	fn loader_type(&self) -> &'static str {
		match self {
			ModuleType::Lua => "script/lua",
			ModuleType::Wireplumber => "module",
			ModuleType::Pipewire => "pw_module",
		}
	}
}

impl Args {
	fn plugins(&self) -> Vec<&str> {
		match self.module_type {
			ModuleType::Lua => vec!["lua-scripting"],
			ModuleType::Wireplumber => self.plugins.iter().map(|s| s.as_str()).collect(),
			ModuleType::Pipewire => todo!(),
		}
	}

	fn module(&self) -> Option<&str> {
		match self.module {
			Some(ref module) => Some(module),
			None if self.module_type.is_lua() =>
				Some(concat!(env!("CARGO_MANIFEST_DIR"), "/script.lua").into()),
			_ => None,
		}
	}

	fn variant(&self) -> Result<Option<Variant>> {
		Ok(match self.json_arg {
			None => None,
			Some(ref json) => {
				let variant: Variant = todo!();
				Some(variant)
			},
		})
	}
}
