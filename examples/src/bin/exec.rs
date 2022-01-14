//! # wpexec example
//!
//! Based on [wpexec.c](https://gitlab.freedesktop.org/pipewire/wireplumber/-/blob/master/src/tools/wpexec.c).

use anyhow::Context;
use glib::Variant;
use anyhow::{Result, format_err};
use std::cell::RefCell;
use std::rc::Rc;
use std::env;

use wireplumber::prelude::*;
use wireplumber::{Core, Properties, Plugin, PluginFeatures, Log, pw};

async fn main_async(core: &Core, exec_script: &str, args: Option<&Variant>) -> Result<()> {
	core.load_component("libwireplumber-module-lua-scripting", "module", None)
		.context("failed to load the lua-scripting module")?;
	core.load_lua_script(exec_script, args)
		.context("failed to load the lua script")?;

	core.connect_future().await?;

	let p = Plugin::find(&core, "lua-scripting").unwrap();
	p.activate_future(PluginFeatures::ENABLED).await
		.context("failed to activate lua-scripting module")?;
	Ok(())
}

fn main() -> Result<()> {
	let mut argv = env::args().skip(1);
	let exec_script = argv.next().unwrap_or_else(||
		concat!(env!("CARGO_MANIFEST_DIR"), "/script.lua").into()
	);
	let args = match argv.next() {
		None => None,
		Some(args) => todo!(),
	};

	Log::set_level("3");
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
			let res = main_async(&core, &exec_script, args).await;
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
