use anyhow::Context;
use futures::channel::oneshot;
use glib::g_warning;
use wireplumber::Plugin;
use wireplumber::PluginFeatures;
use wireplumber::prelude::*;
use pipewire_sys as pw;
use glib::{MainLoop, Variant};
use anyhow::{Result, format_err};
use std::cell::RefCell;
use std::rc::Rc;
use std::env;

use wireplumber::{Core, Properties};

async fn load_script(core: &Core, exec_script: &str, args: Option<&Variant>) -> Result<()> {
	core.load_component("libwireplumber-module-lua-scripting", "module", None)
		.context("failed to load the lua-scripting module")?;
	core.load_component(exec_script, "script/lua", args)
		.context("failed to load the lua script")?;
	let (tx, rx) = oneshot::channel();
	let tx = Rc::new(RefCell::new(Some(tx)));
	core.connect_connected(move |_core| {
		match tx.borrow_mut().take() {
			Some(tx) => tx.send(()).unwrap_or_else(|e| {
				g_warning!("wpexec", "Failed to signal connected: {:?}", e)
			}),
			None => (),
		}
	});
	if !core.connect() {
		return Err(format_err!("failed to connect to pipewire"))
	}
	rx.await?;
	let p = Plugin::find(&core, "lua-scripting").unwrap();
	p.activate_future(PluginFeatures::ENABLED.bits()).await
		.context("failed to activate lua-scripting module")?;
	Ok(())
}

fn main() -> Result<()> {
	// roughly based on: https://gitlab.freedesktop.org/pipewire/wireplumber/-/blob/master/src/tools/wpexec.c
	// https://coaxion.net/blog/2018/04/glib-gio-async-operations-and-rust-futures-async-await/#futures-glib for reference

	let mut argv = env::args().skip(1);
	let exec_script = argv.next().unwrap_or_else(||
		concat!(env!("CARGO_MANIFEST_DIR"), "/script.lua").into()
	);
	let args = match argv.next() {
		None => None,
		Some(args) => todo!(),
	};

	env::set_var("WIREPLUMBER_DEBUG", "3");
	wireplumber::init(wireplumber::InitFlags::ALL);
	/*unsafe {
		wp::wp_log_set_level(std::ffi::CStr::from_bytes_with_nul(b"5\0").unwrap().as_ptr());
	}*/

	let main_res = Rc::new(RefCell::new(None));

	let mainloop = MainLoop::new(None, false);
	let context = mainloop.context();
	context.push_thread_default();

	ctrlc::set_handler({
		let mainloop = mainloop.clone();
		move || mainloop.quit()
	}).unwrap();

	// TODO: prop constructors? or just use the non-sys pipewire crate?
	let pw_key_app_name = std::ffi::CStr::from_bytes_with_nul(pw::PW_KEY_APP_NAME).unwrap();
	let props = Properties::new_empty();
	props.set(pw_key_app_name.to_str().unwrap(), Some("wpexec"));

	let core = Rc::new(Core::new(Some(&context), Some(&props)));
	let _disconnect_handler = core.connect_disconnected({
		let mainloop = mainloop.clone();
		move |_core| mainloop.quit()
	});

	context.spawn_local({
		let core = core.clone();
		let mainloop = mainloop.clone();
		let main_res = main_res.clone();
		async move {
			let res = load_script(&core, &exec_script, args).await;
			if res.is_err() {
				mainloop.quit();
			}
			*main_res.borrow_mut() = Some(res)
		}
	});
	mainloop.run();
	context.pop_thread_default();

	core.disconnect();

	let main_res = main_res.borrow_mut().take();
	match main_res {
		Some(res) => res,
		None => Err(format_err!("could not connect to pipewire")),
	}
}
