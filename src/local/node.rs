use crate::prelude::*;
use pipewire_sys::pw_impl_node;

use crate::{
	Core,
	local::ImplNode,
};

impl ImplNode {
	#[doc(alias = "wp_impl_node_new_wrap")]
	pub fn new_wrap(core: &Core, node: *mut pw_impl_node) -> ImplNode {
		unsafe {
			from_glib_full(ffi::wp_impl_node_new_wrap(core.to_glib_none().0, node))
		}
	}

	#[doc(alias = "pw-impl-node")]
	pub fn pw_impl_node(&self) -> Option<NonNull<pw_impl_node>> {
		unsafe {
			let mut value = Value::from_type(Type::POINTER);
			glib::gobject_ffi::g_object_get_property(self.upcast_ref::<GObject>().to_glib_none().0, b"pw-impl-node\0".as_ptr() as *const _, value.to_glib_none_mut().0);
			value.get::<Option<NonNull<Pointee>>>()
				.expect("pw-impl-node property")
				.map(|p| p.cast())
		}
	}
}
