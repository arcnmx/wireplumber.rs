use crate::spa::SpaIdTable;
use crate::prelude::*;

glib::wrapper! {
	#[doc(alias = "WpSpaType")]
	#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct SpaType(BoxedInline<ffi::WpSpaType>);

	match fn {
		type_ => || ffi::wp_spa_type_get_type(),
	}
}

impl SpaType {
	pub const ARRAY: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Array) };
	pub const BITMAP: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Bitmap) };
	pub const BOOL: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Bool) };
	pub const BYTES: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Bytes) };
	pub const CHOICE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Choice) };
	pub const DOUBLE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Double) };
	pub const FD: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Fd) };
	pub const FLOAT: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Float) };
	pub const FRACTION: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Fraction) };
	pub const ID: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Id) };
	pub const INT: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Int) };
	pub const LONG: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Long) };
	pub const NONE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_None) };
	pub const OBJECT: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Object) };
	pub const OBJECT_FORMAT: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_Format) };
	pub const OBJECT_PARAM_BUFFERS: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamBuffers) };
	pub const OBJECT_PARAM_IO: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamIO) };
	pub const OBJECT_PARAM_LATENCY: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamLatency) };
	pub const OBJECT_PARAM_META: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamMeta) };
	pub const OBJECT_PARAM_PORT_CONFIG: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamPortConfig) };
	pub const OBJECT_PARAM_PROCESS_LATENCY: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamProcessLatency) };
	pub const OBJECT_PARAM_PROFILE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamProfile) };
	pub const OBJECT_PARAM_ROUTE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_ParamRoute) };
	pub const OBJECT_PROFILER: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_Profiler) };
	pub const OBJECT_PROP_INFO: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_PropInfo) };
	pub const OBJECT_PROPS: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_OBJECT_Props) };
	pub const POD: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Pod) };
	pub const POINTER: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Pointer) };
	pub const RECTANGLE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Rectangle) };
	pub const SEQUENCE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Sequence) };
	pub const STRING: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_String) };
	pub const STRUCT: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_Struct) };
	pub const POINTER_BUFFER: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_POINTER_Buffer) };
	pub const POINTER_DICT: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_POINTER_Dict) };
	pub const POINTER_META: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_POINTER_Meta) };
	pub const COMMAND_DEVICE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_COMMAND_Device) };
	pub const COMMAND_NODE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_COMMAND_Node) };
	pub const EVENT_DEVICE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_EVENT_Device) };
	pub const EVENT_NODE: Self = unsafe { SpaType::from_id_unchecked(libspa_sys::SPA_TYPE_EVENT_Node) };

	pub fn from_id(id: ffi::WpSpaType) -> Option<Self> {
		match id {
			ffi::WP_SPA_TYPE_INVALID => None,
			inner => Some(unsafe { Self::from_id_unchecked(inner) }),
		}
	}

	pub const unsafe fn from_id_unchecked(inner: ffi::WpSpaType) -> Self {
		Self {
			inner,
		}
	}

	pub fn number(&self) -> ffi::WpSpaType {
		self.into_glib()
	}

	#[cfg(libspa_linked)]
	pub fn root_types() -> impl Iterator<Item=Self> {
		unsafe {
			libspa_sys::spa_types.iter().map(|ty|
				Self::from_id_unchecked(ty.type_)
			)
		}
	}

	#[doc(alias = "wp_spa_type_get_object_id_values_table")]
	#[doc(alias = "get_object_id_values_table")]
	pub fn object_id_values_table(&self) -> Option<SpaIdTable> {
		unsafe {
			from_glib(ffi::wp_spa_type_get_object_id_values_table(self.into_glib()))
		}
	}

	#[doc(alias = "wp_spa_type_get_values_table")]
	#[doc(alias = "get_values_table")]
	pub fn values_table(&self) -> Option<SpaIdTable> {
		unsafe {
			from_glib(ffi::wp_spa_type_get_values_table(self.into_glib()))
		}
	}

	#[doc(alias = "wp_spa_type_is_fundamental")]
	pub fn is_fundamental(&self) -> bool {
		unsafe {
			from_glib(ffi::wp_spa_type_is_fundamental(self.into_glib()))
		}
	}

	#[doc(alias = "wp_spa_type_is_id")]
	pub fn is_id(&self) -> bool {
		unsafe {
			from_glib(ffi::wp_spa_type_is_id(self.into_glib()))
		}
	}

	#[doc(alias = "wp_spa_type_is_object")]
	pub fn is_object(&self) -> bool {
		unsafe {
			from_glib(ffi::wp_spa_type_is_object(self.into_glib()))
		}
	}

	#[doc(alias = "wp_spa_type_name")]
	pub fn name(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::wp_spa_type_name(self.into_glib()))
		}
	}

	#[doc(alias = "wp_spa_type_parent")]
	#[must_use]
	pub fn parent(&self) -> Option<SpaType> {
		unsafe {
			from_glib(ffi::wp_spa_type_parent(self.into_glib()))
		}
	}

	#[doc(alias = "wp_spa_type_from_name")]
	pub fn from_name(name: &str) -> Option<SpaType> {
		unsafe {
			from_glib(ffi::wp_spa_type_from_name(name.to_glib_none().0))
		}
	}
}

impl TryFromGlib<ffi::WpSpaType> for SpaType {
	type Error = GlibNoneError;

	unsafe fn try_from_glib(val: ffi::WpSpaType) -> Result<Self, Self::Error> {
		Self::try_from(val)
	}
}

impl TryFrom<ffi::WpSpaType> for SpaType {
	type Error = GlibNoneError;

	fn try_from(value: ffi::WpSpaType) -> Result<Self, Self::Error> {
		Self::from_id(value).ok_or(GlibNoneError)
	}
}

impl IntoGlib for SpaType {
	type GlibType = ffi::WpSpaType;
	fn into_glib(self) -> Self::GlibType {
		self.inner
	}
}

impl fmt::Debug for SpaType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut f = f.debug_struct("SpaType");
		f.field("id", &self.into_glib());
		if let Some(name) = self.name() {
			f.field("name", &name);
		}
		f.finish()
	}
}

#[test]
#[cfg(libspa_linked)]
fn all_spa_types() {
	let types = SpaType::root_types();
	assert!(types.count() > 0);
}

#[test]
#[cfg(todo)]
fn dynamic_spa_types() {
	use crate::{Core, InitFlags};
	Core::init_with_flags(InitFlags::SPA_TYPES);
	todo!()
}
