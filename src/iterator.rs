use glib::translate::from_glib_full;

pub struct IteratorMethods {
	raw: ffi::WpIteratorMethods,
}

impl crate::Iterator {
	#[doc(alias = "wp_iterator_new")]
	pub fn new(methods: &IteratorMethods, user_size: usize) -> Self {
		unsafe {
			from_glib_full(ffi::wp_iterator_new(&methods.raw, user_size))
		}
	}
}
