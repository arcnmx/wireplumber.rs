#[cfg(feature = "v0_4_2")]
use crate::pw::PropertiesItem;
use {
	crate::{
		prelude::*,
		pw::{Properties, ToPipewirePropertyString},
	},
	libspa_sys::spa_dict,
	pipewire_sys::pw_properties,
};

impl Properties {
	pub fn new_clone(props: &Self) -> Properties {
		unsafe { Self::new_wrap_mut(props.to_pw_properties().as_ptr()) }
	}

	#[doc(alias = "wp_properties_new_copy")]
	pub unsafe fn new_copy(props: &pw_properties) -> Properties {
		from_glib_full(ffi::wp_properties_new_copy(props))
	}

	#[doc(alias = "wp_properties_new_copy_dict")]
	pub unsafe fn new_copy_dict(dict: &spa_dict) -> Properties {
		from_glib_full(ffi::wp_properties_new_copy_dict(dict))
	}

	#[doc(alias = "wp_properties_new_wrap")]
	pub unsafe fn new_wrap(props: *const pw_properties) -> Properties {
		from_glib_full(ffi::wp_properties_new_wrap(props))
	}

	#[doc(alias = "wp_properties_new_take")]
	pub unsafe fn new_wrap_mut(props: *mut pw_properties) -> Properties {
		from_glib_full(ffi::wp_properties_new_take(props))
	}

	#[doc(alias = "wp_properties_new_wrap_dict")]
	pub unsafe fn new_wrap_dict(dict: *const spa_dict) -> Properties {
		from_glib_full(ffi::wp_properties_new_wrap_dict(dict))
	}

	#[doc(alias = "wp_properties_add_from_dict")]
	pub unsafe fn add_from_dict(&self, dict: &spa_dict) -> usize {
		ffi::wp_properties_add_from_dict(self.to_glib_none().0, dict) as usize
	}

	#[doc(alias = "wp_properties_peek_dict")]
	pub fn peek_dict(&self) -> &spa_dict {
		unsafe { &*ffi::wp_properties_peek_dict(self.to_glib_none().0) }
	}

	#[doc(alias = "wp_properties_to_pw_properties")]
	pub fn to_pw_properties(&self) -> NonNull<pw_properties> {
		unsafe { NonNull::new_unchecked(ffi::wp_properties_to_pw_properties(self.to_glib_none().0)) }
	}

	#[doc(alias = "wp_properties_unref_and_take_pw_properties")]
	pub fn unref_and_take_pw_properties(self) -> NonNull<pw_properties> {
		unsafe { NonNull::new_unchecked(ffi::wp_properties_unref_and_take_pw_properties(self.to_glib_full())) }
	}

	#[doc(alias = "wp_properties_update_from_dict")]
	pub unsafe fn update_from_dict(&self, dict: &spa_dict) -> usize {
		ffi::wp_properties_update_from_dict(self.to_glib_none().0, dict) as usize
	}

	pub fn remove(&self, key: &str) -> bool {
		self.set(key, None) != 0
	}

	pub fn insert<V: ToPipewirePropertyString>(&self, key: &str, value: V) {
		self.set(key, Some(value.pipewire_string().as_ref()));
	}

	#[cfg(feature = "v0_4_2")]
	pub fn iter(&self) -> iter::Map<ValueIterator<PropertiesItem>, fn(PropertiesItem) -> (String, String)> {
		self.items().map(PropertiesItem::into)
	}

	#[cfg(feature = "v0_4_2")]
	pub fn items(&self) -> ValueIterator<PropertiesItem> {
		ValueIterator::with_inner(self.new_iterator().unwrap())
	}

	#[cfg(feature = "v0_4_2")]
	pub fn keys(&self) -> impl Iterator<Item = String> {
		self.items().map(|kv| kv.key_string())
	}

	#[cfg(feature = "v0_4_2")]
	pub fn values(&self) -> impl Iterator<Item = String> {
		self.items().map(|kv| kv.value_string())
	}
}

#[cfg(feature = "v0_4_2")]
mod properties_item {
	use crate::PropertiesItem;

	impl PropertiesItem {
		pub fn key_string(&self) -> String {
			self.key().unwrap().into()
		}

		pub fn value_string(&self) -> String {
			self.value().unwrap().into()
		}

		pub fn key_value(&self) -> (String, String) {
			(self.key_string(), self.value_string())
		}
	}

	impl Into<(String, String)> for PropertiesItem {
		fn into(self) -> (String, String) {
			self.key_value()
		}
	}

	impl<'a> Into<(String, String)> for &'a PropertiesItem {
		fn into(self) -> (String, String) {
			self.key_value()
		}
	}
}

impl FromIterator<(String, String)> for Properties {
	fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
		let mut props = Self::new_empty();
		props.extend(iter);
		props
	}
}

impl<K: AsRef<str>, V: ToPipewirePropertyString> Extend<(K, V)> for Properties {
	fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
		for (k, v) in iter {
			self.insert(k.as_ref(), v)
		}
	}
}

#[cfg(feature = "v0_4_2")]
impl<'a> IntoIterator for &'a Properties {
	type Item = (String, String);
	type IntoIter = iter::Map<ValueIterator<PropertiesItem>, fn(PropertiesItem) -> Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[cfg(feature = "v0_4_2")]
impl IntoIterator for Properties {
	type Item = (String, String);
	type IntoIter = iter::Map<ValueIterator<PropertiesItem>, fn(PropertiesItem) -> Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[cfg(feature = "v0_4_2")]
impl Debug for Properties {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		struct DebugProps<'a>(&'a Properties);
		impl Debug for DebugProps<'_> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_map().entries(self.0).finish()
			}
		}
		let props = DebugProps(self);
		f.debug_tuple("wp::Properties").field(&props).finish()
	}
}

#[cfg(not(feature = "v0_4_2"))]
impl Debug for Properties {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO?
		f.debug_tuple("wp::Properties").finish()
	}
}

impl Default for Properties {
	fn default() -> Self {
		Self::new_empty()
	}
}
