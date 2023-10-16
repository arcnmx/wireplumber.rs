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
			let free = glib::gobject_ffi::g_object_unref as gpointer;
			let array = glib::ffi::g_ptr_array_new_with_free_func(transmute(free));
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

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntoValueIterator<T> {
	iter: WpIterator,
	_data: PhantomData<fn() -> T>,
}

impl<T> IntoValueIterator<T> {
	pub fn with_inner(iter: WpIterator) -> Self {
		Self {
			iter,
			_data: PhantomData,
		}
	}

	pub fn into_value_iterator(self) -> ValueIterator<T> {
		// XXX: work around wireplumber bug where iterators do not start in a usable state
		// known affected methods: wp_new_files_iterator
		self.iter.reset();

		ValueIterator::with_inner(self.iter)
	}

	pub fn upcast<U: ObjectType>(self) -> IntoValueIterator<U>
	where
		T: IsA<U>,
	{
		IntoValueIterator::with_inner(self.iter)
	}

	pub fn into_inner(self) -> WpIterator {
		self.iter
	}

	pub fn inner(&self) -> &WpIterator {
		&self.iter
	}
}

impl<T: for<'v> FromValue<'v>> IntoIterator for IntoValueIterator<T> {
	type IntoIter = ValueIterator<T>;
	type Item = T;

	fn into_iter(self) -> Self::IntoIter {
		self.into_value_iterator()
	}
}

impl<T> From<IntoValueIterator<T>> for ValueIterator<T> {
	fn from(iter: IntoValueIterator<T>) -> ValueIterator<T> {
		iter.into_value_iterator()
	}
}

impl<T: StaticType> Default for IntoValueIterator<T> {
	fn default() -> Self {
		Self::with_inner(WpIterator::empty(T::static_type()))
	}
}

impl<T: StaticType> fmt::Debug for IntoValueIterator<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let name = format!("IntoValueIterator<{}>", T::static_type().name());
		f.debug_tuple(&name).field(&self.iter).finish()
	}
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash)]
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
		Self {
			iter,
			_data: PhantomData,
		}
	}

	pub fn reset(&mut self) {
		self.iter.reset()
	}

	pub fn upcast<U: ObjectType>(self) -> ValueIterator<U>
	where
		T: IsA<U>,
	{
		ValueIterator::with_inner(self.iter)
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
	assert_eq!(value.ref_count(), 2);

	let values: Vec<_> = iter.collect();
	assert_eq!(vec![value.clone()], values);

	// now check that the iterator handles ownership transfer properly
	drop(values);
	assert_eq!(value.ref_count(), 1);
}
