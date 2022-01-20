use libspa_sys::{spa_pod, spa_rectangle, spa_fraction};
use crate::pw::{SpaPropertyKey, PipewireObject};
use crate::spa::{SpaPod, SpaType, SpaIdValue, SpaPodParser, SpaPodBuilder, SpaPrimitive, SpaValue};
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

	fn parse_<R, F: FnOnce(&SpaPodParser, Option<&str>) -> R>(&self, f: F) -> Result<R, Error> {
		let (parser, id_name) = match () {
			_ if self.is_object() => Ok(SpaPodParser::new_object(self)),
			_ if self.is_struct() => Ok((SpaPodParser::new_struct(self), None)),
			_ => Err(Error::new(LibraryErrorEnum::InvalidArgument, &format!("unsupported SPA type {:?}", self.spa_type()))),
		}?;
		let res = f(&parser, id_name);
		parser.end();
		Ok(res)
	}

	pub(crate) fn parse_struct<R, F: FnOnce(&SpaPodParser) -> R>(&self, f: F) -> R {
		self.parse_(|parser, _| f(parser)).unwrap()
	}

	pub(crate) fn parse_object<R, F: FnOnce(&SpaPodParser, Option<&str>) -> R>(&self, f: F) -> R {
		self.parse_(|parser, id_name| f(parser, id_name)).unwrap()
	}

	pub unsafe fn as_bytes(&self) -> &[u8] {
		// TODO: this is unsafe because we cannot check if this is a FLAG_CONSTANT pod or not
		let pod = self.spa_pod_raw();
		slice::from_raw_parts(pod as *const _ as *const u8, (*pod).size as usize)
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
				Some(slice::from_raw_parts(value as *const _, len as usize))
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

	pub fn array_pointers(&self) -> impl Iterator<Item=Pointer> {
		self.new_iterator().unwrap().map(|v| v.get().unwrap())
	}

	pub fn array_iterator<T: SpaPrimitive>(&self) -> impl Iterator<Item=T> {
		// TODO: assert type via T!!!
		self.array_pointers().map(|p| unsafe {
			*(p as *const T)
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

	#[cfg(any(feature = "experimental", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "experimental")))]
	pub fn struct_fields(&self, length_prefix: bool) -> crate::Result<std::vec::IntoIter<(String, SpaPod)>> {
		let mut params = self.iterator();

		let length: Option<i32> = if length_prefix {
			Some(params.next()
				.ok_or_else(|| Error::new(
					LibraryErrorEnum::InvalidArgument,
					&format!("pod struct {:?} is missing expected length prefix", self),
				)).and_then(|pod| (&pod).try_into()
					.map_err(|e| Error::new(
						LibraryErrorEnum::InvalidArgument,
						&format!("pod struct {:?} length could not be parsed from {:?}: {:?}", self, pod, e),
					))
				)?)
		} else {
			None
		};
		let length = match length {
			Some(len) if len < 0 => return Err(Error::new(
				LibraryErrorEnum::InvalidArgument,
				&format!("pod struct {:?} has invalid length {}", self, len),
			)),
			Some(len) => Some(len as usize),
			None => None,
		};
		let mut values = Vec::with_capacity(length.unwrap_or_default());

		// TODO: use a proper chunks(2) iterator here?
		while let Some(key) = params.next() {
			if let Some(length) = length {
				if values.len() >= length {
					return Err(Error::new(
						LibraryErrorEnum::InvalidArgument,
						&format!("too many entries in pod struct {:?}: {:?}", self, values),
					))
				}
			}

			let key: String = (&key).try_into()
				.map_err(|e| Error::new(
					LibraryErrorEnum::InvalidArgument,
					&format!("key {:?} was not a string: {:?}", key, e),
				))?;
			let value = match params.next() {
				Some(v) => v,
				None => return Err(Error::new(
					LibraryErrorEnum::InvalidArgument,
					&format!("unexpected key {:?} due to uneven amount of params on {:?}", key, self),
				)),
			};
			values.push((key, value));
		}
		Ok(values.into_iter())
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

	pub fn find_spa_property<K: SpaPropertyKey>(&self, key: &K) -> Option<SpaPod> {
		let values = self.spa_type().and_then(|ty| ty.values_table());
		let find_id = match key.spa_property_key_with_table(values) {
			Ok(id) => id,
			Err(e) => {
				wp_warning!("unknown spa key {:?} for {:?}: {:?}", key, self, e);
				return None
			},
		};
		self.spa_properties().find(|&(id, ..)| SpaIdValue::result_number(id) == find_id)
			.map(|(_, pod)| pod)
	}

	pub fn spa_property<T, K: SpaPropertyKey>(&self, key: &K) -> Option<T> where
		for<'a> &'a SpaPod: TryInto<T>,
		for<'a> <&'a SpaPod as TryInto<T>>::Error: Debug,
	{
		self.find_spa_property(key)
			.and_then(|pod| match TryInto::try_into(&pod) {
				Ok(v) => Some(v),
				Err(e) => {
					wp_warning!("failed to convert spa key {:?} for {:?}: {:?}", key, self, e);
					None
				},
			})
	}

	pub fn set_spa_property<K: SpaPropertyKey>(&self, key: &K, value: &SpaPod) -> Option<SpaPod> {
		let pod = match self.find_spa_property(key) {
			Some(pod) => pod,
			None => todo!(),
		};
		if pod.set_pod(value) {
			Some(pod)
		} else {
			wp_warning!("failed to set spa key {:?} of type {:?} to {:?}", key, pod, value);
			None
		}
	}

	#[cfg(any(feature = "experimental", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "experimental")))]
	pub fn apply<O: IsA<PipewireObject>>(&self, obj: &O) -> crate::Result<()> {
		if !self.is_object() {
			return Err(Error::new(
				LibraryErrorEnum::InvalidArgument,
				&format!("failed to apply spa pod to {:?}: {:?} is not an object", obj, self),
			))
		}

		let type_ = self.spa_type().unwrap();
		let name = match type_.number() {
			libspa_sys::SPA_TYPE_OBJECT_Props => "Props",
			libspa_sys::SPA_TYPE_OBJECT_ParamRoute => "Route",
			_ => return Err(Error::new(
				LibraryErrorEnum::InvalidArgument,
				&format!("could not apply unknown spa type {:?} to {:?}", type_, obj),
			)),
		};

		let flags = Default::default();
		if obj.set_param(name, flags, self) {
			Ok(())
		} else {
			Err(Error::new(
				LibraryErrorEnum::InvalidArgument,
				&format!("failed to apply param {} to {:?}", name, obj),
			))
		}
	}
}

impl<T: SpaValue> FromIterator<T> for SpaPod {
	fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
		SpaPodBuilder::from_iter(iter).end().unwrap()
	}
}
