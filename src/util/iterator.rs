use glib::{value::FromValue, StaticType};
use std::{iter::{self, FusedIterator, FromIterator}, marker::PhantomData, fmt};

use glib::{translate::{from_glib_full, ToGlibPtr, IntoGlib}, ffi::gpointer, ObjectType, Value, Type};

impl crate::Iterator {
	#[doc(alias = "wp_iterator_new")]
	pub unsafe fn with_impl_raw(methods: &'static ffi::WpIteratorMethods, userdata_size: usize) -> Self {
		from_glib_full(ffi::wp_iterator_new(methods, userdata_size))
	}

	#[doc(alias = "wp_iterator_new_ptr_array")]
	pub unsafe fn with_pointers<I: IntoIterator<Item=gpointer>>(items: I, type_: Type) -> Self {
		let array = glib::ffi::g_ptr_array_new();
		for item in items {
			glib::ffi::g_ptr_array_add(array, item);
		}
		from_glib_full(ffi::wp_iterator_new_ptr_array(array, type_.into_glib()))
	}

	pub fn empty(type_: Type) -> Self {
		unsafe {
			Self::with_pointers(iter::empty(), type_)
		}
	}

	#[doc(alias = "wp_iterator_get_user_data")]
	#[doc(alias = "get_user_data")]
	pub fn user_data(&self) -> gpointer {
		unsafe {
			ffi::wp_iterator_get_user_data(self.to_glib_none().0)
		}
	}
}

impl<T: ObjectType> FromIterator<T> for crate::Iterator {
	fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
		unsafe {
			Self::with_pointers(iter.into_iter().map(|o| o.to_glib_full() as *mut _), T::static_type())
		}
	}
}

impl Iterator for crate::Iterator {
	type Item = Value;

	fn next(&mut self) -> Option<Self::Item> {
		Self::next(self)
	}
}

impl FusedIterator for crate::Iterator { }

#[repr(transparent)]
pub struct ValueIterator<T> {
	iter: crate::Iterator,
	_data: PhantomData<fn() -> T>,
}

impl<T> ValueIterator<T> {
	pub fn new<I: IntoIterator>(iter: I) -> Self where Self: FromIterator<I::Item> {
		FromIterator::from_iter(iter)
	}

	pub fn with_inner(iter: crate::Iterator) -> Self {
		// XXX: work around wireplumber bug where iterators do not start in a usable state
		// known affected methods: wp_new_files_iterator
		iter.reset();

		Self {
			iter,
			_data: PhantomData,
		}
	}

	pub fn reset(&mut self) {
		self.iter.reset()
	}

	pub fn into_inner(self) -> crate::Iterator {
		self.iter
	}

	pub fn inner(&self) -> &crate::Iterator {
		&self.iter
	}
}

impl<T: StaticType> Default for ValueIterator<T> {
	fn default() -> Self {
		Self::with_inner(crate::Iterator::empty(T::static_type()))
	}
}

impl<T: StaticType> fmt::Debug for ValueIterator<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let name = format!("ValueIterator<{}>", T::static_type().name());
		f.debug_tuple(&name)
			.field(&self.iter)
			.finish()
	}
}

impl<T: ObjectType> FromIterator<T> for ValueIterator<T> {
	fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
		Self::with_inner(FromIterator::from_iter(iter))
	}
}

impl<T: for<'v> FromValue<'v>> ValueIterator<T> {
	fn with_next<R, F: FnOnce(T) -> R>(&mut self, f: F) -> Option<R> {
		let value = self.iter.next()?;
		let value = match value.get() {
			Ok(value) => value,
			Err(e) => panic!("iterator contained unexpected value type {}", e),
		};
		Some(f(value))
	}
}

impl<T: for<'v> FromValue<'v>> Iterator for ValueIterator<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.with_next(|value| value)
	}
}

impl<T> FusedIterator for ValueIterator<T> where Self: Iterator { }

#[test]
fn object_iterator() {
	use crate::ObjectManager;

	let value = ObjectManager::new();
	let iter = ValueIterator::new(iter::once(value.clone()));
	let values: Vec<_> = iter.collect();
	assert_eq!(vec![value], values);
}
