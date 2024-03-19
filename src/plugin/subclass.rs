use {
	crate::{
		core::{Core, Object, ObjectImpl},
		plugin::{Plugin, PluginFeatures},
		prelude::*,
		util::Transition,
	},
	glib::{
		object::{BorrowedObject, ObjectSubclassIs},
		subclass::prelude::*,
		MainContext, SourceId,
	},
	std::{cell::RefCell, panic::catch_unwind},
};

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
			f(
				plugin.unsafe_cast_ref::<Plugin>().to_glib_none().0,
				error_handler.to_glib_none().0,
			)
		}
	}

	fn parent_disable(&self, plugin: &Self::Type) {
		let parent = PluginImplExt::parent_class(self);
		let f = parent.disable.expect("No parent class implementation for \"disable\"");
		unsafe { f(plugin.unsafe_cast_ref::<Plugin>().to_glib_none().0) }
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
	type EnableFuture: Future<Output = Result<(), Error>>;
	fn enable(&self, plugin: Self::Type) -> Self::EnableFuture;

	fn disable(&self) {}

	fn register_source(&self, source: SourceId) {
		let _ = source;
	}
}

pub trait AsyncPluginExt: IsA<Plugin> {
	fn as_plugin(&self) -> &Plugin;

	fn plugin_core(&self) -> Core;

	fn plugin_context(&self) -> MainContext;

	fn spawn_local<F: Future<Output = ()> + 'static>(&self, f: F);
}

impl Plugin {
	pub fn core(&self) -> Core {
		self
			.upcast_ref::<Object>()
			.core()
			.expect("plugin requires an active Core")
	}
}

impl<T: IsA<Plugin> + ObjectSubclassIsExt> AsyncPluginExt for T
where
	<T as ObjectSubclassIs>::Subclass: AsyncPluginImpl,
{
	fn as_plugin(&self) -> &Plugin {
		self.upcast_ref()
	}

	fn plugin_core(&self) -> Core {
		self.as_plugin().core()
	}

	fn plugin_context(&self) -> MainContext {
		self.plugin_core().default_context()
	}

	fn spawn_local<F: Future<Output = ()> + 'static>(&self, f: F) {
		let source = self.plugin_context().spawn_local(f);
		self.imp().register_source(source.into_source_id().unwrap());
	}
}

impl<T: AsyncPluginImpl + ObjectImpl> PluginImpl for T
where
	<T as ObjectSubclass>::Type: AsyncPluginExt,
{
	fn enable(&self, this: &Self::Type, error_handler: Transition) {
		let plugin = this.clone();
		let enable = self.enable(plugin.clone());

		let enable_handle = this.plugin_context().spawn_local(async move {
			match enable.await {
				Ok(()) => plugin
					.as_plugin()
					.update_features(PluginFeatures::ENABLED, PluginFeatures::empty()),
				Err(e) => error_handler.return_error(e),
			}
		});
		self.register_source(enable_handle.into_source_id().unwrap());
		// TODO: prevent multiple calls?
	}

	fn disable(&self, _plugin: &Self::Type) {
		self.disable();
	}
}

#[derive(Debug)]
pub struct SourceHandles {
	context: MainContext,
	handles: Vec<SourceId>,
}

impl SourceHandles {
	pub fn new(context: MainContext) -> Self {
		Self {
			context,
			handles: Vec::new(),
		}
	}

	pub fn push(&mut self, source: SourceId) {
		self.handles.push(source);
	}

	pub fn clear(&mut self) {
		for source in self.handles.drain(..) {
			if let Some(source) = self.context.find_source_by_id(&source) {
				// TODO: will completed future sources be considered destroyed?
				// or will they never exist on the context to begin with?
				// if !source.is_destroyed() { }
				source.destroy();
			}
		}
	}
}

#[derive(Default, Debug)]
pub struct SourceHandlesCell(RefCell<Option<SourceHandles>>);

impl SourceHandlesCell {
	pub fn init(&self, context: MainContext) {
		*self.cell().borrow_mut() = Some(SourceHandles::new(context));
	}

	pub fn try_init(&self, context: MainContext) -> Result<(), MainContext> {
		match &mut *self.cell().borrow_mut() {
			&mut Some(..) => Err(context),
			opt @ None => {
				*opt = Some(SourceHandles::new(context));
				Ok(())
			},
		}
	}

	pub fn push(&self, source: SourceId) {
		self.borrow_mut(|handles| handles.push(source))
	}

	pub fn clear(&self) {
		let res = self.borrow_mut(|handles| handles.clear());
		self.cell().replace(None);
		res
	}

	#[inline]
	pub fn cell(&self) -> &RefCell<Option<SourceHandles>> {
		&self.0
	}

	pub fn borrow_mut<R, F: FnOnce(&mut SourceHandles) -> R>(&self, f: F) -> R {
		match *self.cell().borrow_mut() {
			Some(ref mut handles) => f(handles),
			None => panic!("SourceHandles cell uninitialized"),
		}
	}
}

pub trait SimplePlugin: ObjectSubclass {
	type Args;

	fn instance_ref(&self) -> BorrowedObject<Self::Type> {
		self.obj()
	}

	fn decode_args(args: Option<Variant>) -> Result<Self::Args, Error>;

	fn init_args(&self, args: Self::Args) {
		let _ = args;
		unimplemented!()
	}
	fn new_plugin(core: &Core, args: Self::Args) -> Self::Type
	where
		Self::Type: IsA<GObject>,
	{
		let res = GObject::with_mut_values(Self::Type::static_type(), &mut [
			("name", Self::NAME.to_value()),
			("core", core.to_value()),
		]);
		let res: Self::Type = unsafe { res.unsafe_cast() };
		res.imp().init_args(args);
		res
	}
}

glib::wrapper! {
	pub struct SimplePluginObject<T: SimplePlugin>(ObjectSubclass<T>) @extends Plugin, Object;
}

impl<T> Deref for SimplePluginObject<T>
where
	T: SimplePlugin + ObjectSubclass<Type = Self>,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.imp()
	}
}

/// Implements [`glib::ObjectSubclass`](glib::subclass::types::ObjectSubclass),
/// [`glib::ObjectImpl`](glib::subclass::object::ObjectImpl), and
/// [`ObjectImpl`](crate::core::ObjectImpl) for your plugin.
///
/// The plugin type must also manually impl [SimplePlugin], and will be wrapped as
/// [`SimplePluginObject<T>`](SimplePluginObject). See the
/// [module documentation](super#implementing-and-exporting-a-plugin) for a full example.
#[macro_export]
macro_rules! simple_plugin_subclass {
	(impl ObjectSubclass for $name:tt as $ty:ident { $($subclass:tt)* }) => {
		$crate::plugin::simple_plugin_subclass! {
			impl ObjectSubclass<$crate::plugin::Plugin> for $name as $ty { $($subclass)* }
		}
	};
	(impl ObjectSubclass<$parent:ty> for $name:tt as $ty:ident { $($subclass:tt)* }) => {
		#[$crate::lib::glib::object_subclass]
		impl $crate::lib::glib::subclass::types::ObjectSubclass for $ty {
			type Type = $crate::plugin::SimplePluginObject<Self>;
			type ParentType = $parent;
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
		let res = catch_unwind(|| T::init(core, args));
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

impl<T: SimplePlugin> ModuleExport for T
where
	T::Type: IsA<GObject> + IsA<Plugin>,
{
	fn init(core: Core, args: Option<Variant>) -> Result<(), Error> {
		// TODO: support optional args? annoying to do this properly though...
		let args = T::decode_args(args)?;
		let plugin = T::new_plugin(&core, args);
		plugin.register();
		Ok(())
	}
}

/// Exports a [ModuleExport] as the wireplumber plugin entry point.
///
/// Using this properly requires that your crate be built as a `cdylib`. See the
/// [module documentation](super#implementing-and-exporting-a-plugin) for a full example.
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
			error: std::ptr::NonNull<*mut $crate::lib::glib::ffi::GError>,
		) -> glib::ffi::gboolean {
			use $crate::lib::glib::translate::{IntoGlib, IntoGlibPtr};

			let core = unsafe { glib::translate::from_glib_none(core.as_ptr()) };
			let args = unsafe { glib::translate::from_glib_none(args) };
			match <$desc as $crate::plugin::ModuleExport>::init(core, args) {
				Ok(()) => true.into_glib(),
				Err(e) => {
					*error.as_ptr() = e.into_glib_ptr();
					false.into_glib()
				},
			}
		}
	};
}
pub use plugin_export;
