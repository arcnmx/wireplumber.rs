// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::Core;
use crate::GlobalProxy;
use crate::Object;
use crate::PipewireObject;
use crate::Properties;
use crate::Proxy;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    #[doc(alias = "WpLink")]
    pub struct Link(Object<ffi::WpLink, ffi::WpLinkClass>) @extends GlobalProxy, Proxy, Object, @implements PipewireObject;

    match fn {
        type_ => || ffi::wp_link_get_type(),
    }
}

impl Link {
    #[doc(alias = "wp_link_new_from_factory")]
    #[doc(alias = "new_from_factory")]
    pub fn from_factory(core: &Core, factory_name: &str, properties: Option<&Properties>) -> Option<Link> {
        unsafe {
            from_glib_full(ffi::wp_link_new_from_factory(core.to_glib_none().0, factory_name.to_glib_none().0, properties.to_glib_full()))
        }
    }

    #[doc(alias = "wp_link_get_linked_object_ids")]
    #[doc(alias = "get_linked_object_ids")]
    pub fn linked_object_ids(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut output_node = mem::MaybeUninit::uninit();
            let mut output_port = mem::MaybeUninit::uninit();
            let mut input_node = mem::MaybeUninit::uninit();
            let mut input_port = mem::MaybeUninit::uninit();
            ffi::wp_link_get_linked_object_ids(self.to_glib_none().0, output_node.as_mut_ptr(), output_port.as_mut_ptr(), input_node.as_mut_ptr(), input_port.as_mut_ptr());
            let output_node = output_node.assume_init();
            let output_port = output_port.assume_init();
            let input_node = input_node.assume_init();
            let input_port = input_port.assume_init();
            (output_node, output_port, input_node, input_port)
        }
    }
}
