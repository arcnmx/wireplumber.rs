use ffi::WpSpaType;
use crate::spa::{SpaType, SpaIdTable};
use crate::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpaIdValue {
	inner: NonNull<libc::c_void>,
}

impl SpaIdValue {
	pub fn number(&self) -> WpSpaType {
		unsafe {
			ffi::wp_spa_id_value_number(self.into_glib())
		}
	}

	pub fn name(&self) -> Option<String> {
		unsafe {
			from_glib_none(ffi::wp_spa_id_value_name(self.into_glib()))
		}
	}

	pub fn short_name(&self) -> Option<String> {
		unsafe {
			from_glib_none(ffi::wp_spa_id_value_short_name(self.into_glib()))
		}
	}

	pub fn value_type(&self) -> (Option<SpaType>, Option<SpaIdTable>) {
		unsafe {
			let mut table = ptr::null();
			let res = ffi::wp_spa_id_value_get_value_type(self.into_glib(), &mut table);
			(from_glib(res), from_glib(table))
		}
	}

	pub fn array_item_type(&self) -> (Option<SpaType>, Option<SpaIdTable>) {
		unsafe {
			let mut table = ptr::null();
			let res = ffi::wp_spa_id_value_array_get_item_type(self.into_glib(), &mut table);
			(from_glib(res), from_glib(table))
		}
	}

	pub fn parse_unknown_name(id_name: &str) -> Option<WpSpaType> {
		if id_name.starts_with("id-") {
			<WpSpaType>::from_str_radix(&id_name[2..], 16).ok()
		} else {
			None
		}
	}

	pub(crate) fn value_or_name<C: Debug>(context: &C, key_name: &str, v: Option<Self>) -> Result<Self, WpSpaType> {
		let raw = match v {
			Some(v) => return Ok(v),
			None => Self::parse_unknown_name(key_name),
		};
		Err(raw.unwrap_or_else(|| {
			wp_warning!("expected to find spa key {:?} of {:?}", key_name, context);
			ffi::WP_SPA_TYPE_INVALID
		}))
	}

	pub fn result_number(res: Result<Self, WpSpaType>) -> WpSpaType {
		res.map(|v| v.number()).unwrap_or_else(|e| e)
	}
}

unsafe impl<'a> FromValue<'a> for SpaIdValue {
	type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

	unsafe fn from_value(value: &'a Value) -> Self {
		let optional: Option<Self> = from_glib(
			value.get::<Pointer>().unwrap() as ffi::WpSpaIdValue
		);
		optional.unwrap() // TODO
	}
}

impl fmt::Debug for SpaIdValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut f = f.debug_struct("SpaIdValue");
		f.field("raw", &self.into_glib());
		f.field("number", &self.number());
		if let Some(name) = self.name() {
			f.field("name", &name);
		}
		f.finish()
	}
}

impl StaticType for SpaIdValue {
	fn static_type() -> Type {
		unsafe {
			from_glib(ffi::wp_spa_id_value_get_type())
		}
	}
}

impl UnsafeFrom<NonNull<libc::c_void>> for SpaIdValue {
	unsafe fn unsafe_from(inner: NonNull<libc::c_void>) -> Self {
		Self {
			inner,
		}
	}
}

impl UnsafeFrom<ffi::WpSpaIdValue> for SpaIdValue {
	unsafe fn unsafe_from(inner: ffi::WpSpaIdValue) -> Self {
		Self {
			inner: NonNull::new_unchecked(inner as *mut _),
		}
	}
}

impl TryFromGlib<ffi::WpSpaIdValue> for SpaIdValue {
	type Error = GlibNoneError;

	unsafe fn try_from_glib(val: ffi::WpSpaIdValue) -> Result<Self, Self::Error> {
		NonNull::new(val as *mut _).map(|ptr| Self::unsafe_from(ptr)).ok_or(GlibNoneError)
	}
}

impl IntoGlib for SpaIdValue {
	type GlibType = ffi::WpSpaIdValue;

	fn into_glib(self) -> Self::GlibType {
		self.inner.as_ptr() as *const _
	}
}

impl OptionIntoGlib for SpaIdValue {
	const GLIB_NONE: ffi::WpSpaIdValue = ptr::null();
}
