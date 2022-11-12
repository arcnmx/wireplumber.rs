use crate::{prelude::*, spa::SpaIdValue};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SpaIdTable {
	inner: NonNull<libc::c_void>,
}

impl SpaIdTable {
	pub fn from_name(name: &str) -> Option<Self> {
		unsafe { from_glib(ffi::wp_spa_id_table_from_name(name.to_glib_none().0)) }
	}

	pub fn new_iterator(&self) -> crate::Iterator {
		unsafe { from_glib_full(ffi::wp_spa_id_table_new_iterator(self.into_glib())) }
	}

	pub fn values(&self) -> ValueIterator<SpaIdValue> {
		ValueIterator::with_inner(self.new_iterator())
	}

	pub fn find_value(&self, value: u32) -> Option<SpaIdValue> {
		unsafe { from_glib(ffi::wp_spa_id_table_find_value(self.into_glib(), value)) }
	}

	pub fn find_value_from_name(&self, name: &str) -> Option<SpaIdValue> {
		unsafe {
			from_glib(ffi::wp_spa_id_table_find_value_from_name(
				self.into_glib(),
				name.to_glib_none().0,
			))
		}
	}

	pub fn find_value_from_short_name(&self, name: &str) -> Option<SpaIdValue> {
		unsafe {
			from_glib(ffi::wp_spa_id_table_find_value_from_short_name(
				self.into_glib(),
				name.to_glib_none().0,
			))
		}
	}
}

impl fmt::Debug for SpaIdTable {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		struct IdValues<'a>(&'a SpaIdTable);
		impl fmt::Debug for IdValues<'_> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_list().entries(self.0.values()).finish()
			}
		}

		let mut f = f.debug_struct("SpaIdTable");
		f.field("raw", &self.into_glib());
		f.field("values", &IdValues(self));
		f.finish()
	}
}

impl TryFromGlib<ffi::WpSpaIdTable> for SpaIdTable {
	type Error = GlibNoneError;

	unsafe fn try_from_glib(val: ffi::WpSpaIdTable) -> Result<Self, Self::Error> {
		NonNull::new(val as *mut _)
			.map(|ptr| Self::unsafe_from(ptr))
			.ok_or(GlibNoneError)
	}
}

impl IntoGlib for SpaIdTable {
	type GlibType = ffi::WpSpaIdTable;

	fn into_glib(self) -> Self::GlibType {
		self.inner.as_ptr() as *const _
	}
}

impl OptionIntoGlib for SpaIdTable {
	const GLIB_NONE: ffi::WpSpaIdTable = ptr::null();
}

impl StaticType for SpaIdTable {
	fn static_type() -> Type {
		unsafe { from_glib(ffi::wp_spa_id_table_get_type()) }
	}
}

impl UnsafeFrom<NonNull<libc::c_void>> for SpaIdTable {
	unsafe fn unsafe_from(inner: NonNull<libc::c_void>) -> Self {
		Self { inner }
	}
}

impl UnsafeFrom<ffi::WpSpaIdTable> for SpaIdTable {
	unsafe fn unsafe_from(inner: ffi::WpSpaIdTable) -> Self {
		Self {
			inner: NonNull::new_unchecked(inner as *mut _),
		}
	}
}

#[test]
fn id_table_values() {
	use crate::spa::SpaType;

	let ty = SpaType::OBJECT_PROP_INFO;
	let table = ty.values_table().unwrap();
	let values = table.values();
	assert!(values.count() > 0);
}
