use {
	crate::{
		core::BaseDirsFlags, plugin::ComponentLoader, prelude::*, spa::json::SpaJson, Conf, Core, InitFlags, Properties,
	},
	glib::{MainContext, MainLoop},
	pipewire_sys::{pw_context, pw_core},
};

impl Core {
	#[doc(alias = "wp_init")]
	pub fn init_with_flags(flags: InitFlags) {
		unsafe { ffi::wp_init(flags.into_glib()) }
	}

	#[doc(alias = "wp_init")]
	pub fn init() {
		Self::init_with_flags(InitFlags::ALL)
	}

	#[cfg(feature = "v0_4_12")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_12")))]
	#[doc(alias = "wp_get_library_version")]
	pub fn library_version() -> String {
		unsafe { from_glib_full(ffi::wp_get_library_version()) }
	}

	#[cfg(feature = "v0_4_12")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_12")))]
	#[doc(alias = "wp_get_library_api_version")]
	pub fn library_api_version() -> String {
		unsafe { from_glib_full(ffi::wp_get_library_api_version()) }
	}

	#[doc(alias = "wp_base_dirs_find_file")]
	pub fn find_file(dirs: BaseDirsFlags, filename: &str, subdir: Option<&str>) -> Option<String> {
		unsafe {
			from_glib_full(ffi::wp_base_dirs_find_file(
				dirs.into_glib(),
				subdir.to_glib_none().0,
				filename.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "wp_base_dirs_new_files_iterator")]
	pub fn find_files(dirs: BaseDirsFlags, subdir: Option<&str>, suffix: Option<&str>) -> IntoValueIterator<String> {
		unsafe {
			IntoValueIterator::with_inner(from_glib_full(ffi::wp_base_dirs_new_files_iterator(
				dirs.into_glib(),
				subdir.to_glib_none().0,
				suffix.to_glib_none().0,
			)))
		}
	}

	#[doc(alias = "wp_core_clone")]
	pub fn clone_context(&self) -> Option<Self> {
		unsafe { from_glib_full(ffi::wp_core_clone(self.to_glib_none().0)) }
	}

	#[doc(alias = "wp_core_get_g_main_context")]
	#[doc(alias = "get_g_main_context")]
	pub fn default_context(&self) -> MainContext {
		self
			.g_main_context()
			.unwrap_or_else(|| MainContext::ref_thread_default())
	}

	#[doc(alias = "wp_core_get_pw_core")]
	#[doc(alias = "get_pw_core")]
	pub fn pw_core_raw(&self) -> *mut pw_core {
		unsafe { ffi::wp_core_get_pw_core(self.to_glib_none().0) }
	}

	#[doc(alias = "wp_core_get_pw_context")]
	#[doc(alias = "get_pw_context")]
	pub fn pw_context_raw(&self) -> NonNull<pw_context> {
		unsafe { NonNull::new(ffi::wp_core_get_pw_context(self.to_glib_none().0)).expect("pw_context for WpCore") }
	}

	#[cfg_attr(docsrs, doc(cfg(feature = "lua")))]
	#[doc(alias = "wp_core_load_component")]
	pub fn load_component_future<T: Into<Cow<'static, str>>>(
		&self,
		component: Option<Cow<'static, str>>,
		type_: T,
		args: Option<SpaJson>,
		provides: Option<String>,
	) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
		let type_ = type_.into();
		Box::pin(gio::GioFuture::new(self, move |this, cancellable, send| {
			this.load_component(
				component.as_ref().map(|c| &c[..]),
				&type_,
				args.as_ref(),
				provides.as_ref().map(|c| &c[..]),
				Some(cancellable),
				move |res| send.resolve(res),
			);
		}))
	}

	#[doc(alias = "wp_core_load_component")]
	pub fn load_lua_script<S: Into<Cow<'static, str>>>(
		&self,
		script_path: S,
		args: Option<SpaJson>,
	) -> impl Future<Output = Result<(), Error>> {
		let script_path = script_path.into();
		self.load_component_future(Some(script_path), ComponentLoader::TYPE_LUA_SCRIPT, args, None)
	}

	#[cfg(feature = "futures")]
	#[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
	#[doc(alias = "wp_core_connect")]
	#[doc(alias = "connect")]
	pub fn connect_future(&self) -> impl Future<Output = Result<(), Error>> {
		use crate::util::futures::signal_once;

		let connect = signal_once(match () {
			#[cfg(feature = "glib-signal")]
			() => self.signal_stream(Self::SIGNAL_CONNECTED),
			#[cfg(not(feature = "glib-signal"))]
			() => |handler| self.connect_connected(handler),
		});

		let res = match self.connect() {
			true => Ok(connect),
			false => Err(Error::new(
				LibraryErrorEnum::OperationFailed,
				"failed to connect to pipewire",
			)),
		};

		async move {
			match res {
				Ok(connect) => connect.await,
				Err(e) => Err(e),
			}
		}
	}

	pub fn run<F: FnOnce(&MainContext, MainLoop, Core)>(conf: Option<Conf>, props: Option<Properties>, setup: F) {
		let mainloop = MainLoop::new(None, false);
		let context = mainloop.context();
		let core = context
			.with_thread_default(|| {
				let core = Core::new(Some(&context), conf, props);
				let _disconnect_handler = core.connect_disconnected({
					let mainloop = mainloop.clone();
					move |_core| mainloop.quit()
				});

				setup(&context, mainloop.clone(), core.clone());

				mainloop.run();

				core
			})
			.unwrap();

		core.disconnect();
	}
}

#[test]
#[cfg(any(feature = "v0_4_2"))]
fn wp_new_files_iterator() {
	let file = Core::find_file(BaseDirsFlags::CONFIGURATION, "create-item.lua", Some("scripts/node"));
	assert!(file.is_some());

	let files = Core::find_files(BaseDirsFlags::CONFIGURATION, None, Some(".conf")).into_iter();
	assert_ne!(0, files.count());
}
