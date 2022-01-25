use pipewire_sys::{pw_core, pw_context};
use glib::{MainContext, MainLoop};
use crate::prelude::*;
use crate::{Properties, Core, InitFlags, lua::ToLuaVariant};
#[cfg(any(feature = "v0_4_2"))]
use crate::plugin::LookupDirs;

impl Core {
	#[doc(alias = "wp_init")]
	pub fn init_with_flags(flags: InitFlags) {
		unsafe {
			ffi::wp_init(flags.into_glib())
		}
	}

	#[doc(alias = "wp_init")]
	pub fn init() {
		Self::init_with_flags(InitFlags::ALL)
	}

	#[doc(alias = "wp_get_module_dir")]
	pub fn module_dir() -> String {
		unsafe {
			from_glib_full(ffi::wp_get_module_dir())
		}
	}

	#[cfg_attr(feature = "v0_4_2", deprecated = "use `find_file` instead")]
	#[doc(alias = "wp_get_config_dir")]
	pub fn config_dir() -> String {
		unsafe {
			from_glib_full(ffi::wp_get_config_dir())
		}
	}

	#[cfg_attr(feature = "v0_4_2", deprecated = "use `find_file` instead")]
	#[doc(alias = "wp_get_data_dir")]
	pub fn data_dir() -> String {
		unsafe {
			from_glib_full(ffi::wp_get_data_dir())
		}
	}

	#[cfg(any(feature = "v0_4_2", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_2")))]
	#[doc(alias = "wp_find_file")]
	pub fn find_file(dirs: LookupDirs, filename: &str, subdir: Option<&str>) -> Option<String> {
		unsafe {
			from_glib_full(ffi::wp_find_file(dirs.into_glib(), filename.to_glib_none().0, subdir.to_glib_none().0))
		}
	}

	#[cfg(any(feature = "v0_4_2", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_2")))]
	#[doc(alias = "wp_new_files_iterator")]
	pub fn find_files(dirs: LookupDirs, subdir: Option<&str>, suffix: Option<&str>) -> ValueIterator<String> {
		unsafe {
			ValueIterator::with_inner(
				from_glib_full(ffi::wp_new_files_iterator(dirs.into_glib(), subdir.to_glib_none().0, suffix.to_glib_none().0))
			)
		}
	}

	#[doc(alias = "wp_core_clone")]
	pub fn clone_context(&self) -> Option<Self> {
		unsafe {
			from_glib_full(ffi::wp_core_clone(self.to_glib_none().0))
		}
	}

	#[doc(alias = "wp_core_get_pw_core")]
	#[doc(alias = "get_pw_core")]
	pub fn pw_core_raw(&self) -> *mut pw_core {
		unsafe {
			ffi::wp_core_get_pw_core(self.to_glib_none().0)
		}
	}

	#[doc(alias = "wp_core_get_pw_context")]
	#[doc(alias = "get_pw_context")]
	pub fn pw_context_raw(&self) -> NonNull<pw_context> {
		unsafe {
			NonNull::new(ffi::wp_core_get_pw_context(self.to_glib_none().0))
				.expect("pw_context for WpCore")
		}
	}

	#[doc(alias = "wp_core_load_component")]
	pub fn load_lua_script<A: ToLuaVariant>(&self, script_path: &str, args: A) -> Result<(), Error> {
		self.load_component(script_path, "script/lua", args.to_lua_variant()?.as_deref())
	}

	#[cfg(any(feature = "enable-futures", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "enable-futures")))]
	pub fn connect_future(&self) -> impl Future<Output=Result<(), Error>> {
		use futures_util::{TryFutureExt, future};

		let connect = self.signal_stream(Self::SIGNAL_CONNECTED);

		let res = if self.connect() {
			Ok(connect.once())
		} else {
			Err(Error::new(LibraryErrorEnum::OperationFailed, "failed to connect to pipewire"))
		};
		future::ready(res).and_then(|connect| connect.map_err(From::from).map_ok(drop))
	}

	pub fn run<F: FnOnce(&MainContext, MainLoop, Core)>(props: Option<&Properties>, setup: F) {
		let mainloop = MainLoop::new(None, false);
		let context = mainloop.context();
		let core = context.with_thread_default(|| {
			let core = Core::new(Some(&context), props);
			let _disconnect_handler = core.connect_disconnected({
				let mainloop = mainloop.clone();
				move |_core| mainloop.quit()
			});

			setup(&context, mainloop.clone(), core.clone());

			mainloop.run();

			core
		}).unwrap();

		core.disconnect();
	}
}

#[test]
#[cfg(any(feature = "v0_4_2"))]
fn wp_new_files_iterator() {
	let file = Core::find_file(LookupDirs::PREFIX_SHARE, "create-item.lua", Some("scripts"));
	assert!(file.is_some());

	let files = Core::find_files(LookupDirs::PREFIX_SHARE, None, Some(".conf"));
	assert_ne!(0, files.count());
}
