use {
	crate::{prelude::*, util::WpIterator},
	glib::ffi::GPtrArray,
	std::mem::transmute,
};

impl WpIterator {
	#[doc(alias = "wp_iterator_new")]
	pub unsafe fn with_impl_raw(methods: &'static ffi::WpIteratorMethods, userdata_size: usize) -> Self {
		from_glib_full(ffi::wp_iterator_new(methods, userdata_size))
	}

	#[doc(alias = "wp_iterator_new_ptr_array")]
	pub unsafe fn with_ptr_array(array: *mut GPtrArray, type_: Type) -> Self {
		from_glib_full(ffi::wp_iterator_new_ptr_array(array, type_.into_glib()))
	}

	pub fn empty(type_: Type) -> Self {
		unsafe {
			let empty = glib::ffi::g_ptr_array_new();
			Self::with_ptr_array(empty, type_)
		}
	}

	#[doc(alias = "wp_iterator_get_user_data")]
	#[doc(alias = "get_user_data")]
	pub fn user_data(&self) -> gpointer {
		unsafe { ffi::wp_iterator_get_user_data(self.to_glib_none().0) }
	}
}

impl<T: ObjectType> FromIterator<T> for WpIterator {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		unsafe {
			let free: unsafe extern "C" fn(gpointer) = transmute(glib::gobject_ffi::g_object_unref as gpointer);
			let array = glib::ffi::g_ptr_array_new_with_free_func(Some(free));
			for item in iter {
				glib::ffi::g_ptr_array_add(array, item.to_glib_full() as gpointer);
			}
			Self::with_ptr_array(array, T::static_type())
		}
	}
}

impl Iterator for WpIterator {
	type Item = Value;

	fn next(&mut self) -> Option<Self::Item> {
		Self::next(self)
	}
}

impl iter::FusedIterator for WpIterator {}

#[repr(transparent)]
pub struct ValueIterator<T> {
	iter: WpIterator,
	_data: PhantomData<fn() -> T>,
}

impl<T> ValueIterator<T> {
	pub fn new<I: IntoIterator>(iter: I) -> Self
	where
		Self: FromIterator<I::Item>,
	{
		FromIterator::from_iter(iter)
	}

	pub fn with_inner(iter: WpIterator) -> Self {
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

	pub fn into_inner(self) -> WpIterator {
		self.iter
	}

	pub fn inner(&self) -> &WpIterator {
		&self.iter
	}
}

impl<T: StaticType> Default for ValueIterator<T> {
	fn default() -> Self {
		Self::with_inner(WpIterator::empty(T::static_type()))
	}
}

impl<T: StaticType> fmt::Debug for ValueIterator<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let name = format!("ValueIterator<{}>", T::static_type().name());
		f.debug_tuple(&name).field(&self.iter).finish()
	}
}

impl<T: ObjectType> FromIterator<T> for ValueIterator<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self::with_inner(FromIterator::from_iter(iter))
	}
}

impl<T: for<'v> FromValue<'v>> ValueIterator<T> {
	fn with_next<R, F: FnOnce(T) -> R>(&mut self, f: F) -> Option<R> {
		let value = self.iter.next()?;
		let value = match value.get() {
			Ok(value) => value,
			Err(e) => panic!("iterator contained unexpected value type {e}"),
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

impl<T> iter::FusedIterator for ValueIterator<T> where Self: Iterator {}

#[test]
fn object_iterator() {
	use crate::ObjectManager;

	let value = ObjectManager::new();
	let iter = ValueIterator::new(iter::once(value.clone()));
	let values: Vec<_> = iter.collect();
	assert_eq!(vec![value], values);
}
