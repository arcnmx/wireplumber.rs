use glib::subclass::prelude::*;
use glib::variant::VariantTypeMismatchError;
use std::panic::catch_unwind;
use crate::plugin::{Plugin, PluginFeatures};
use crate::core::{Core, Object, ObjectImpl};
use crate::util::Transition;
use crate::prelude::*;

pub trait PluginImpl: ObjectImpl + PluginImplExt {
	fn enable(&self, plugin: &Self::Type, error_handler: Transition) {
		self.parent_enable(plugin, error_handler)
	}

	fn disable(&self, plugin: &Self::Type) {
		self.parent_disable(plugin)
	}
}

pub trait PluginImplExt: ObjectSubclass {
	fn parent_class(&self) -> &ffi::WpPluginClass;
	fn parent_enable(&self, plugin: &Self::Type, error_handler: Transition);
	fn parent_disable(&self, plugin: &Self::Type);
}

impl<T: PluginImpl> PluginImplExt for T {
	fn parent_class(&self) -> &ffi::WpPluginClass {
		unsafe {
			let data = T::type_data();
			let parent_class = data.as_ref().parent_class() as *mut _;
			&*parent_class
		}
	}

	fn parent_enable(&self, plugin: &Self::Type, error_handler: Transition) {
		let parent = PluginImplExt::parent_class(self);
		let f = parent.enable.expect("No parent class implementation for \"enable\"");
		unsafe {
			f(plugin.unsafe_cast_ref::<Plugin>().to_glib_none().0, error_handler.to_glib_none().0)
		}
	}

	fn parent_disable(&self, plugin: &Self::Type) {
		let parent = PluginImplExt::parent_class(self);
		let f = parent.disable.expect("No parent class implementation for \"disable\"");
		unsafe {
			f(plugin.unsafe_cast_ref::<Plugin>().to_glib_none().0)
		}
	}
}

unsafe impl<T: PluginImpl> IsSubclassable<T> for Plugin {
	fn class_init(class: &mut glib::Class<Self>) {
		Self::parent_class_init::<T>(class);

		unsafe extern "C" fn enable<T: PluginImpl>(plugin: *mut ffi::WpPlugin, error_handler: *mut ffi::WpTransition) {
			let this = &*(plugin as *mut T::Instance);
			let this = this.imp();
			let plugin: Borrowed<Plugin> = from_glib_borrow(plugin);

			// TODO: transition ownership
			this.enable(plugin.unsafe_cast_ref(), from_glib_none(error_handler))
		}

		unsafe extern "C" fn disable<T: PluginImpl>(plugin: *mut ffi::WpPlugin) {
			let this = &*(plugin as *mut T::Instance);
			let this = this.imp();
			let plugin: Borrowed<Plugin> = from_glib_borrow(plugin);

			this.disable(plugin.unsafe_cast_ref())
		}

		let klass = class.as_mut();
		klass.enable = Some(enable::<T>);
		klass.disable = Some(disable::<T>);
	}
}

pub trait AsyncPluginImpl: ObjectSubclass {
	type EnableFuture: Future<Output=Result<(), Error>>;
	fn enable(&self, plugin: Self::Type) -> Self::EnableFuture;

	fn disable(&self) { }
}

impl<T: AsyncPluginImpl + ObjectImpl> PluginImpl for T where
	<T as ObjectSubclass>::Type: IsA<Plugin>,
{
	fn enable(&self, plugin: &Self::Type, error_handler: Transition) {
		let core = plugin.as_ref().core().expect("enable requires an active Core");
		let context = core.g_main_context().expect("async enable requires a MainContext");

		let plugin = plugin.clone();
		let enable = self.enable(plugin.clone());

		let enable_handle = context.spawn_local(async move {
			match enable.await {
				Ok(()) => plugin.as_ref().update_features(PluginFeatures::ENABLED, PluginFeatures::empty()),
				Err(e) => error_handler.return_error(e),
			}
		});
		// TODO: store enable_handle in self, and prevent multiple calls?
	}

	fn disable(&self, _plugin: &Self::Type) {
		self.disable();
	}
}

pub trait SimplePlugin: ObjectSubclass {
	type Args: FromVariant;

	fn instance_ref(&self) -> Self::Type {
		// TODO: use glib_signal::BorrowedObject?
		self.instance()
	}

	fn init_args(&self, args: Self::Args) { let _ = args; unimplemented!() }
	fn new_plugin(core: &Core, args: Self::Args) -> Self::Type where
		Self::Type: IsA<GObject>,
	{
		let res: Self::Type = GObject::new(&[
			("name", &Self::NAME),
			("core", core),
		]).unwrap();
		res.imp().init_args(args);
		res
	}
}

glib::wrapper! {
	pub struct SimplePluginObject<T: SimplePlugin>(ObjectSubclass<T>) @extends Plugin, Object;
}

impl<T> Deref for SimplePluginObject<T> where
	T: SimplePlugin + ObjectSubclass<Type=Self>,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.imp()
	}
}

#[macro_export]
macro_rules! simple_plugin_subclass {
	(impl ObjectSubclass for $name:tt as $ty:ty { $($subclass:tt)* }) => {
		#[$crate::lib::glib::object_subclass]
		impl $crate::lib::glib::subclass::types::ObjectSubclass for $ty {
			type Type = $crate::plugin::SimplePluginObject<Self>;
			type ParentType = $crate::plugin::Plugin;
			const NAME: &'static str = $name;
			$($subclass)*
		}

		impl $crate::lib::glib::subclass::object::ObjectImpl for $ty { }
		impl $crate::core::ObjectImpl for $ty { }
	};
}
pub use simple_plugin_subclass;

pub trait ModuleExport {
	fn init(core: Core, args: Option<Variant>) -> Result<(), Error>;
}

/// Catches panics from a [ModuleExport] initializer
pub struct ModuleWrapper<T>(PhantomData<T>);

impl<T: ModuleExport> ModuleExport for ModuleWrapper<T> {
	fn init(core: Core, args: Option<Variant>) -> Result<(), Error> {
		let res = catch_unwind(|| {
			T::init(core, args)
		});
		match res {
			Ok(res) => res,
			Err(panic) => Err({
				let op = LibraryErrorEnum::OperationFailed;
				if let Some(panic) = panic.downcast_ref::<String>() {
					Error::new(op, panic)
				} else if let Some(panic) = panic.downcast_ref::<&'static str>() {
					Error::new(op, *panic)
				} else {
					Error::new(op, "ModuleExport::init panicked")
				}
			}),
		}
	}
}

impl<T: SimplePlugin> ModuleExport for T where
	T::Type: IsA<GObject> + IsA<Plugin>,
	T::Args: FromAnyVariant<Error=Error>,
{
	fn init(core: Core, args: Option<Variant>) -> Result<(), Error> {
		// TODO: support optional args? annoying to do this properly though...
		let args = FromAnyVariant::from_a_variant(args.as_ref())?;
			//.map_err(|e| Error::new(LibraryErrorEnum::InvalidArgument, &format!("{:?}", e)))?;
		let plugin = T::new_plugin(&core, args);
		plugin.register();
		Ok(())
	}
}

pub trait FromAnyVariant: Sized {
	type Error: Debug;

	fn from_a_variant(args: Option<&Variant>) -> Result<Self, Self::Error>;
}

pub struct FromAnyVariantWrapVariant<T>(pub T);

impl<T: FromVariant> FromAnyVariant for FromAnyVariantWrapVariant<T> {
	type Error = VariantTypeMismatchError;

	fn from_a_variant(args: Option<&Variant>) -> Result<Self, Self::Error> {
		match args {
			Some(args) => args.try_get(),
			None => ().to_variant().try_get(),
		}.map(Self)
	}
}

impl<T: FromVariant> FromAnyVariant for T {
	type Error = Error;

	fn from_a_variant(args: Option<&Variant>) -> Result<Self, Self::Error> {
		let ty = args.map(|v| v.type_());

		let args_var: Result<FromAnyVariantWrapVariant<T>, _> = FromAnyVariant::from_a_variant(args);
		match args_var {
			Ok(args) => Ok(args.0),
			Err(e_var) => {
				// TODO: fallback to human-readable "jsonish" decoding via serde instead
				Err(
					Error::new(LibraryErrorEnum::InvalidArgument, &format!("{:?}", e_var))
				)
			},
		}
	}
}

#[macro_export]
macro_rules! plugin_export {
	($desc:ty) => {
		$crate::plugin::plugin_export! { @nowrap $crate::plugin::ModuleWrapper::<$desc> }
	};
	(@nowrap $desc:ty) => {
		#[no_mangle]
		pub unsafe extern "C" fn wireplumber__module_init(
			core: std::ptr::NonNull<$crate::ffi::WpCore>,
			args: *mut $crate::lib::glib::ffi::GVariant,
			error: std::ptr::NonNull<*mut $crate::lib::glib::ffi::GError>
		) -> glib::ffi::gboolean {
			use $crate::lib::glib::translate::{IntoGlib, ToGlibPtr};

			let core = unsafe { glib::translate::from_glib_none(core.as_ptr()) };
			let args = unsafe { glib::translate::from_glib_none(args) };
			match <$desc as $crate::plugin::ModuleExport>::init(core, args) {
				Ok(()) => true.into_glib(),
				Err(e) => {
					*error.as_ptr() = e.to_glib_full() as *mut _;
					false.into_glib()
				},
			}
		}
	};
}
pub use plugin_export;
