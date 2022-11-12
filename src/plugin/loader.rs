use {
	crate::{
		error,
		plugin::{ComponentLoader, PluginImpl},
		prelude::*,
	},
	glib::subclass::prelude::*,
	std::ptr,
};

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

pub trait ComponentLoaderImpl: ObjectImpl + ComponentLoaderImplExt {
	fn supports_type(&self, loader: &Self::Type, type_: String) -> bool {
		self.parent_supports_type(loader, type_)
	}

	fn load(&self, loader: &Self::Type, component: String, type_: String, args: Option<Variant>) -> Result<(), Error> {
		self.parent_load(loader, component, type_, args)
	}
}

pub trait ComponentLoaderImplExt: ObjectSubclass {
	fn parent_class(&self) -> &ffi::WpComponentLoaderClass;
	fn parent_supports_type(&self, loader: &Self::Type, type_: String) -> bool;
	fn parent_load(
		&self,
		loader: &Self::Type,
		component: String,
		type_: String,
		args: Option<Variant>,
	) -> Result<(), Error>;
}

impl<T: ComponentLoaderImpl> ComponentLoaderImplExt for T {
	fn parent_class(&self) -> &ffi::WpComponentLoaderClass {
		unsafe {
			let data = T::type_data();
			let parent_class = data.as_ref().parent_class() as *mut _;
			&*parent_class
		}
	}

	fn parent_supports_type(&self, loader: &Self::Type, type_: String) -> bool {
		let parent = ComponentLoaderImplExt::parent_class(self);
		let f = parent
			.supports_type
			.expect("No parent class implementation for \"supports_type\"");
		unsafe {
			from_glib(f(
				loader.unsafe_cast_ref::<ComponentLoader>().to_glib_none().0,
				type_.to_glib_none().0,
			))
		}
	}

	fn parent_load(
		&self,
		loader: &Self::Type,
		component: String,
		type_: String,
		args: Option<Variant>,
	) -> Result<(), Error> {
		let parent = ComponentLoaderImplExt::parent_class(self);
		let f = parent.load.expect("No parent class implementation for \"load\"");
		unsafe {
			let mut error = ptr::null_mut();
			let res = from_glib(f(
				loader.unsafe_cast_ref::<ComponentLoader>().to_glib_none().0,
				component.to_glib_none().0,
				type_.to_glib_none().0,
				args.to_glib_none().0,
				&mut error,
			));
			match (res, error.is_null()) {
				(true, _) => Ok(()),
				(false, false) => Err(from_glib_full(error)),
				(false, true) => Err(error::invariant("GError NULL")),
			}
		}
	}
}

unsafe impl<T: ComponentLoaderImpl + PluginImpl> IsSubclassable<T> for ComponentLoader {
	fn class_init(class: &mut glib::Class<Self>) {
		Self::parent_class_init::<T>(class);

		unsafe extern "C" fn supports_type<T: ComponentLoaderImpl>(
			loader: *mut ffi::WpComponentLoader,
			type_: *const libc::c_char,
		) -> glib::ffi::gboolean {
			let this = &*(loader as *mut T::Instance);
			let this = this.imp();
			let loader: Borrowed<ComponentLoader> = from_glib_borrow(loader);

			this
				.supports_type(loader.unsafe_cast_ref(), from_glib_none(type_))
				.into_glib()
		}

		unsafe extern "C" fn load<T: ComponentLoaderImpl>(
			loader: *mut ffi::WpComponentLoader,
			component: *const libc::c_char,
			type_: *const libc::c_char,
			args: *mut glib::ffi::GVariant,
			error: *mut *mut glib::ffi::GError,
		) -> glib::ffi::gboolean {
			let this = &*(loader as *mut T::Instance);
			let this = this.imp();
			let loader: Borrowed<ComponentLoader> = from_glib_borrow(loader);

			match this.load(
				loader.unsafe_cast_ref(),
				from_glib_none(component),
				from_glib_none(type_),
				from_glib_none(args),
			) {
				Ok(()) => true,
				Err(e) => {
					*error = e.to_glib_full() as *mut _;
					false
				},
			}
			.into_glib()
		}

		let klass = class.as_mut();
		klass.supports_type = Some(supports_type::<T>);
		klass.load = Some(load::<T>);
	}
}
