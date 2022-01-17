use glib::Cast;
use glib::translate::{IntoGlib, ToGlibPtr, Borrowed, from_glib, from_glib_none, from_glib_borrow};
use glib::subclass::prelude::{ObjectImpl as GObjectImpl, *};

use crate::{Object, ObjectFeatures, FeatureActivationTransition};

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
