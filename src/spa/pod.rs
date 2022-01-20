use libspa_sys::{spa_pod, spa_rectangle, spa_fraction};
use glib::translate::{ToGlibPtr, from_glib, from_glib_full};
use glib::{Error, ffi::gconstpointer};
use glib::prelude::*;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::{mem, ptr};
use std::slice::from_raw_parts;
use crate::{SpaPod, SpaType, SpaIdValue, SpaPodParser, SpaPodBuilder, ValueIterator, LibraryErrorEnum};
use crate::prelude::*;

impl SpaPod {
	/// # Safety
	///
	/// Does not tie lifetime of `bytes` to Self, so requires caller guarantee
	pub unsafe fn with_pod_unchecked(bytes: &[u8]) -> Self {
		assert!(bytes.len() >= mem::size_of::<spa_pod>());
		let pod = bytes.as_ptr() as *const spa_pod;

		// TODO: complicated, are unaligned pointers that bad if it gets copied anyway..?
		assert_eq!(pod as usize % mem::align_of::<spa_pod>(), 0);
		assert!(bytes.len() >= (*pod).size as usize);

		Self::new_wrap_raw_const(pod)
	}

	/// # Safety
	///
	/// Does not tie lifetime of `bytes` to Self, so requires caller guarantee
	pub unsafe fn with_pod_mut_unchecked(bytes: &mut [u8]) -> Self {
		assert!(bytes.len() >= mem::size_of::<spa_pod>());
		let pod = bytes.as_ptr() as *mut spa_pod;

		// TODO: complicated, are unaligned pointers that bad if it gets copied anyway..?
		assert_eq!(pod as usize % mem::align_of::<spa_pod>(), 0);
		assert!(bytes.len() >= (*pod).size as usize);

		Self::new_wrap_raw_mut(pod)
	}

	pub fn with_copy(pod: &SpaPod) -> Self {
		pod.copy().unwrap()
	}

	pub fn with_pod(bytes: &[u8]) -> Self {
		unsafe {
			Self::with_copy(
				&Self::with_pod_unchecked(bytes)
			)
		}
	}

	pub unsafe fn as_bytes(&self) -> &[u8] {
		// TODO: this is unsafe because we cannot check if this is a FLAG_CONSTANT pod or not
		let pod = self.spa_pod_raw();
		from_raw_parts(pod as *const _ as *const u8, (*pod).size as usize)
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		unsafe {
			self.as_bytes().into()
		}
	}

	#[doc(alias = "get_spa_type")]
	pub fn spa_type(&self) -> Option<SpaType> {
		unsafe {
			from_glib(ffi::wp_spa_pod_get_spa_type(self.to_glib_none().0))
		}
	}

	/// borrows pod for the lifetime of the returned object
	#[doc(alias = "wp_spa_pod_new_wrap")]
	pub unsafe fn new_wrap_raw_mut(pod: *mut spa_pod) -> SpaPod {
		from_glib_full(ffi::wp_spa_pod_new_wrap(pod))
	}

	/// borrows pod for the lifetime of the returned object
	#[doc(alias = "wp_spa_pod_new_wrap_const")]
	pub unsafe fn new_wrap_raw_const(pod: *const spa_pod) -> SpaPod {
		from_glib_full(ffi::wp_spa_pod_new_wrap_const(pod))
	}

	#[doc(alias = "wp_spa_pod_new_bytes")]
	pub fn new_bytes(value: &[u8]) -> SpaPod {
		unsafe {
			from_glib_full(ffi::wp_spa_pod_new_bytes(value.as_ptr() as *const _, value.len() as _))
		}
	}

	#[doc(alias = "wp_spa_pod_new_pointer")]
	pub fn new_pointer(type_name: &str, value: gconstpointer) -> SpaPod {
		unsafe {
			from_glib_full(ffi::wp_spa_pod_new_pointer(type_name.to_glib_none().0, value))
		}
	}

	#[doc(alias = "wp_spa_pod_get_bytes")]
	#[doc(alias = "get_bytes")]
	pub fn bytes(&self) -> Option<&[u8]> {
		let mut value = ptr::null();
		let mut len = 0;
		unsafe {
			if from_glib(ffi::wp_spa_pod_get_bytes(self.to_glib_none().0, &mut value, &mut len)) {
				Some(from_raw_parts(value as *const _, len as usize))
			} else {
				None
			}
		}
	}

	#[doc(alias = "wp_spa_pod_get_choice_type")]
	#[doc(alias = "get_choice_type")]
	pub fn choice_type(&self) -> Option<SpaIdValue> {
		unsafe {
			from_glib(ffi::wp_spa_pod_get_choice_type(self.to_glib_none().0))
		}
	}

	#[doc(alias = "wp_spa_pod_get_pointer")]
	#[doc(alias = "get_pointer")]
	pub fn pointer(&self) -> Option<gconstpointer> {
		let mut res = ptr::null();
		unsafe {
			if from_glib(ffi::wp_spa_pod_get_pointer(self.to_glib_none().0, &mut res)) {
				Some(res)
			} else {
				None
			}
		}
	}

	#[doc(alias = "wp_spa_pod_set_pointer")]
	pub fn set_pointer(&self, type_name: &str, value: gconstpointer) -> bool {
		unsafe {
			from_glib(ffi::wp_spa_pod_set_pointer(self.to_glib_none().0, type_name.to_glib_none().0, value))
		}
	}

	pub fn iterator(&self) -> ValueIterator<SpaPod> {
		ValueIterator::with_inner(self.new_iterator().unwrap())
	}

	pub fn array_pointers(&self) -> impl Iterator<Item=glib::ffi::gpointer> {
		self.new_iterator().unwrap().map(|v| unsafe {
			glib::gobject_ffi::g_value_get_pointer(v.to_glib_none().0)
		})
	}

	#[doc(alias = "wp_spa_pod_get_spa_pod")]
	#[doc(alias = "get_spa_pod")]
	pub fn spa_pod_raw(&self) -> &spa_pod {
		unsafe {
			&*ffi::wp_spa_pod_get_spa_pod(self.to_glib_none().0)
		}
	}

	pub fn spa_rectangle(&self) -> Option<spa_rectangle> {
		self.rectangle().map(|(width, height)| spa_rectangle {
			width,
			height,
		})
	}

	pub fn spa_fraction(&self) -> Option<spa_fraction> {
		self.fraction().map(|(num, denom)| spa_fraction {
			num,
			denom,
		})
	}

	pub fn spa_properties(&self) -> impl Iterator<Item=(Result<SpaIdValue, ffi::WpSpaType>, SpaPod)> {
		let type_ = self.spa_type();
		let values = type_.and_then(|ty| ty.values_table());
		self.iterator().map(move |pod| pod.property().unwrap())
			.map(move |(key_name, pod)| (
				SpaIdValue::value_or_name(&type_, &key_name,
					values.and_then(|values| values.find_value_from_short_name(&key_name))
				),
				pod,
			))
	}
}
