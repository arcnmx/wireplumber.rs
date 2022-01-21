use glib::translate::{FromGlib, IntoGlib, ToGlibPtr, Borrowed, from_glib, from_glib_none, from_glib_borrow};
use glib::prelude::*;
use glib::subclass::prelude::{ObjectImpl as GObjectImpl, *};

use crate::{Object, FeatureActivationTransition};

#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ObjectFeatures(pub u32); // TODO: consider keeping this as u32, and just keep the inherent impls (requires no changes to `auto`)

impl FromGlib<u32> for ObjectFeatures {
	unsafe fn from_glib(features: u32) -> Self {
		Self::with_bits(features)
	}
}

impl IntoGlib for ObjectFeatures {
	type GlibType = u32;

	fn into_glib(self) -> Self::GlibType {
		self.bits()
	}
}

impl Into<u32> for ObjectFeatures {
	fn into(self) -> u32 {
		self.bits()
	}
}

impl From<u32> for ObjectFeatures {
	fn from(features: u32) -> Self {
		Self::with_bits(features)
	}
}

impl ObjectFeatures {
	pub const NONE: Self = Self(0);
	pub const ALL: Self = Self(ffi::WP_OBJECT_FEATURES_ALL);

	pub fn with_bits(bits: u32) -> Self {
		Self(bits)
	}

	pub fn bits(&self) -> u32 {
		self.0
	}
}

pub trait ObjectImpl: GObjectImpl + ObjectImplExt {
	fn supported_features(&self, object: &Self::Type) -> ObjectFeatures {
		self.parent_supported_features(object)
	}

	fn activate_get_next_step(&self, object: &Self::Type, transition: FeatureActivationTransition, step: u32, features: ObjectFeatures) -> u32 {
		self.parent_activate_get_next_step(object, transition, step, features)
	}

	fn activate_execute_step(&self, object: &Self::Type, transition: FeatureActivationTransition, step: u32, features: ObjectFeatures) {
		self.parent_activate_execute_step(object, transition, step, features)
	}

	fn deactivate(&self, object: &Self::Type, features: ObjectFeatures) {
		self.parent_deactivate(object, features)
	}
}

pub trait ObjectImplExt: ObjectSubclass {
	fn parent_class(&self) -> &ffi::WpObjectClass;
	fn parent_supported_features(&self, object: &Self::Type) -> ObjectFeatures;
	fn parent_deactivate(&self, object: &Self::Type, features: ObjectFeatures);
	fn parent_activate_get_next_step(&self, object: &Self::Type, transition: FeatureActivationTransition, step: u32, features: ObjectFeatures) -> u32;
	fn parent_activate_execute_step(&self, object: &Self::Type, transition: FeatureActivationTransition, step: u32, features: ObjectFeatures);
}

unsafe impl<T: ObjectImpl> IsSubclassable<T> for Object {
	fn class_init(class: &mut glib::Class<Self>) {
		Self::parent_class_init::<T>(class);

		unsafe extern "C" fn get_supported_features<T: ObjectImpl>(object: *mut ffi::WpObject) -> ffi::WpObjectFeatures {
			let this = &*(object as *mut T::Instance);
			let this = this.imp();
			let object: Borrowed<Object> = from_glib_borrow(object);

			this.supported_features(object.unsafe_cast_ref()).into_glib()
		}

		unsafe extern "C" fn deactivate<T: ObjectImpl>(object: *mut ffi::WpObject, features: ffi::WpObjectFeatures) {
			let this = &*(object as *mut T::Instance);
			let this = this.imp();
			let object: Borrowed<Object> = from_glib_borrow(object);

			this.deactivate(object.unsafe_cast_ref(), from_glib(features))
		}

		unsafe extern "C" fn activate_get_next_step<T: ObjectImpl>(object: *mut ffi::WpObject,
			transition: *mut ffi::WpFeatureActivationTransition,
			step: u32, features: ffi::WpObjectFeatures
		) -> u32 {
			let this = &*(object as *mut T::Instance);
			let this = this.imp();
			let object: Borrowed<Object> = from_glib_borrow(object);

			// TODO: check ownership of transition
			this.activate_get_next_step(object.unsafe_cast_ref(), from_glib_none(transition), step, from_glib(features))
		}

		unsafe extern "C" fn activate_execute_step<T: ObjectImpl>(object: *mut ffi::WpObject,
			transition: *mut ffi::WpFeatureActivationTransition,
			step: u32, features: ffi::WpObjectFeatures
		) {
			let this = &*(object as *mut T::Instance);
			let this = this.imp();
			let object: Borrowed<Object> = from_glib_borrow(object);

			// TODO: check ownership of transition
			this.activate_execute_step(object.unsafe_cast_ref(), from_glib_none(transition), step, from_glib(features))
		}

		let klass = class.as_mut();
		klass.get_supported_features = Some(get_supported_features::<T>);
		klass.activate_get_next_step = Some(activate_get_next_step::<T>);
		klass.activate_execute_step = Some(activate_execute_step::<T>);
		klass.deactivate = Some(deactivate::<T>);
	}
}

impl<T: ObjectImpl> ObjectImplExt for T {
	fn parent_class(&self) -> &ffi::WpObjectClass {
		unsafe {
			let data = T::type_data();
			let parent_class = data.as_ref().parent_class() as *mut _;
			&*parent_class
		}
	}

	fn parent_supported_features(&self, object: &Self::Type) -> ObjectFeatures {
		let parent = self.parent_class();
		let f = parent.get_supported_features.expect("No parent class implementation for \"get_supported_features\"");
		unsafe {
			from_glib(f(object.unsafe_cast_ref::<Object>().to_glib_none().0))
		}
	}

	fn parent_deactivate(&self, object: &Self::Type, features: ObjectFeatures) {
		let parent = self.parent_class();
		let f = parent.deactivate.expect("No parent class implementation for \"deactivate\"");
		unsafe {
			f(object.unsafe_cast_ref::<Object>().to_glib_none().0, features.into_glib())
		}
	}

	fn parent_activate_get_next_step(&self, object: &Self::Type, transition: FeatureActivationTransition, step: u32, features: ObjectFeatures) -> u32 {
		let parent = self.parent_class();
		let f = parent.activate_get_next_step.expect("No parent class implementation for \"activate_get_next_step\"");
		unsafe {
			f(
				object.unsafe_cast_ref::<Object>().to_glib_none().0,
				transition.to_glib_none().0,
				step,
				features.into_glib(),
			)
		}
	}

	fn parent_activate_execute_step(&self, object: &Self::Type, transition: FeatureActivationTransition, step: u32, features: ObjectFeatures) {
		let parent = self.parent_class();
		let f = parent.activate_execute_step.expect("No parent class implementation for \"activate_execute_step\"");
		unsafe {
			f(
				object.unsafe_cast_ref::<Object>().to_glib_none().0,
				transition.to_glib_none().0,
				step,
				features.into_glib(),
			)
		}
	}
}

macro_rules! impl_object_features {
	($($id:ident:$ty:ident,)*) => {
		$(
			impl From<crate::$id> for ObjectFeatures {
				fn from(features: crate::$id) -> Self {
					Self::with_bits(features.bits())
				}
			}

			impl From<ObjectFeatures> for crate::$id {
				fn from(features: ObjectFeatures) -> crate::$id {
					crate::$id::from_bits_truncate(features.bits())
				}
			}

			impl $crate::$ty {
				#[doc(alias = "wp_object_activate")]
				pub fn activate<P, Q>(&self, features: $crate::$id, cancellable: Option<&P>, callback: Q) where
					P: IsA<::gio::Cancellable>,
					Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
				{
					crate::traits::ObjectExt::activate(self, features.into(), cancellable, callback)
				}

				#[doc(alias = "wp_object_activate_closure")]
				pub fn activate_closure<P>(&self, features: $crate::$id, cancellable: Option<&P>, closure: &glib::Closure) where
					P: IsA<gio::Cancellable>,
				{
					crate::traits::ObjectExt::activate_closure(self, features.into(), cancellable, closure)
				}

				#[doc(alias = "wp_object_activate")]
				pub fn activate_future(&self, features: $crate::$id) -> impl std::future::Future<Output=Result<(), glib::Error>> + Unpin {
					crate::traits::ObjectExt::activate_future(self, features.into())
				}

				#[doc(alias = "wp_object_deactivate")]
				pub fn deactivate(&self, features: $crate::$id) {
					crate::traits::ObjectExt::deactivate(self, features.into())
				}

				#[doc(alias = "wp_object_get_active_features")]
				pub fn active_features(&self) -> $crate::$id {
					crate::traits::ObjectExt::active_features(self)
						.into()
				}

				#[doc(alias = "wp_object_get_supported_features")]
				pub fn supported_features(&self) -> $crate::$id {
					crate::traits::ObjectExt::supported_features(self)
						.into()
				}

				#[doc(alias = "wp_object_update_features")]
				pub fn update_features(&self, activated: $crate::$id, deactivated: $crate::$id) {
					crate::traits::ObjectExt::update_features(self, activated.into(), deactivated.into())
				}
			}
		)*
	};
}

impl_object_features! {
	MetadataFeatures:Metadata, NodeFeatures:Node, PluginFeatures:Plugin, ProxyFeatures:Proxy, SessionItemFeatures:SessionItem, SpaDeviceFeatures:SpaDevice,
}
