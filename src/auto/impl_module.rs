// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Core,Properties};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "WpImplModule")]
    pub struct ImplModule(Object<ffi::WpImplModule, ffi::WpImplModuleClass>);

    match fn {
        type_ => || ffi::wp_impl_module_get_type(),
    }
}

impl ImplModule {
    pub fn arguments(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "arguments")
    }

    //pub fn core(&self) -> /*Unimplemented*/Basic: Pointer {
    //    ObjectExt::property(self, "core")
    //}

    pub fn name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "name")
    }

    pub fn properties(&self) -> Option<Properties> {
        ObjectExt::property(self, "properties")
    }

    pub fn set_properties(&self, properties: Option<&Properties>) {
        ObjectExt::set_property(self,"properties", properties)
    }

    //#[doc(alias = "pw-impl-module")]
    //pub fn pw_impl_module(&self) -> /*Unimplemented*/Basic: Pointer {
    //    ObjectExt::property(self, "pw-impl-module")
    //}

    #[doc(alias = "wp_impl_module_load")]
    pub fn load(core: &Core, name: &str, arguments: Option<&str>, properties: Option<&Properties>) -> Option<ImplModule> {
        unsafe {
            from_glib_full(ffi::wp_impl_module_load(core.to_glib_none().0, name.to_glib_none().0, arguments.to_glib_none().0, properties.to_glib_none().0))
        }
    }

    #[cfg(feature = "v0_4_15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_4_15")))]
    #[doc(alias = "wp_impl_module_load_file")]
    pub fn load_file(core: &Core, name: &str, filename: &str, properties: Option<&Properties>) -> Option<ImplModule> {
        unsafe {
            from_glib_full(ffi::wp_impl_module_load_file(core.to_glib_none().0, name.to_glib_none().0, filename.to_glib_none().0, properties.to_glib_none().0))
        }
    }

    #[doc(alias = "properties")]
    pub fn connect_properties_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_properties_trampoline<F: Fn(&ImplModule) + 'static>(this: *mut ffi::WpImplModule, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::properties\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_properties_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "pw-impl-module")]
    pub fn connect_pw_impl_module_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pw_impl_module_trampoline<F: Fn(&ImplModule) + 'static>(this: *mut ffi::WpImplModule, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pw-impl-module\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_pw_impl_module_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
