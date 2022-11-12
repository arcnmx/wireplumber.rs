use crate::{
	prelude::*,
	pw::SpaPropertyKey,
	spa::{SpaPodBuilder, SpaValue},
};

impl SpaPodBuilder {
	#[doc(alias = "wp_spa_pod_builder_add_bytes")]
	pub fn add_bytes(&self, value: &[u8]) {
		unsafe { ffi::wp_spa_pod_builder_add_bytes(self.to_glib_none().0, value.as_ptr() as *const _, value.len() as _) }
	}

	#[doc(alias = "wp_spa_pod_builder_add_pointer")]
	pub fn add_pointer(&self, type_name: &str, value: gconstpointer) {
		unsafe { ffi::wp_spa_pod_builder_add_pointer(self.to_glib_none().0, type_name.to_glib_none().0, value) }
	}

	pub fn add_object_property<V: SpaValue, K: SpaPropertyKey>(&self, key: &K, value: V) -> bool {
		let table = None; // TODO: store from `new_object`?
		let id = match key.spa_property_key_with_table(table) {
			Ok(id) => id,
			Err(e) => return false,
		};
		self.add_property_id(id);
		value.add_to_builder(&self);
		true
	}
}

impl<T: SpaValue> FromIterator<T> for SpaPodBuilder {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut builder = Self::new_array();
		builder.extend(iter);
		builder
	}
}

impl<T: SpaValue> Extend<T> for SpaPodBuilder {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		for v in iter {
			v.add_to_builder(self)
		}
	}
}
