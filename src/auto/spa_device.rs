// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Core,Iterator,Object,Properties,Proxy};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "WpSpaDevice")]
    pub struct SpaDevice(Object<ffi::WpSpaDevice, ffi::WpSpaDeviceClass>) @extends Proxy, Object;

    match fn {
        type_ => || ffi::wp_spa_device_get_type(),
    }
}

impl SpaDevice {
    #[doc(alias = "wp_spa_device_new_from_spa_factory")]
    #[doc(alias = "new_from_spa_factory")]
    pub fn from_spa_factory(core: &Core, factory_name: &str, properties: Option<Properties>) -> Option<SpaDevice> {
        unsafe {
            from_glib_full(ffi::wp_spa_device_new_from_spa_factory(core.to_glib_none().0, factory_name.to_glib_none().0, properties.into_glib_ptr()))
        }
    }

    #[doc(alias = "wp_spa_device_get_managed_object")]
    #[doc(alias = "get_managed_object")]
    pub fn managed_object(&self, id: u32) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::wp_spa_device_get_managed_object(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "wp_spa_device_get_properties")]
    #[doc(alias = "get_properties")]
    pub fn properties(&self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_spa_device_get_properties(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_device_new_managed_object_iterator")]
    #[doc(alias = "new_managed_object_iterator")]
    pub fn managed_object_iterator(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_spa_device_new_managed_object_iterator(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_spa_device_store_managed_object")]
    pub fn store_managed_object(&self, id: u32, object: Option<impl IsA<glib::Object>>) {
        unsafe {
            ffi::wp_spa_device_store_managed_object(self.to_glib_none().0, id, object.map(|p| p.upcast()).into_glib_ptr());
        }
    }

    //#[doc(alias = "spa-device-handle")]
    //pub fn spa_device_handle(&self) -> /*Unimplemented*/Basic: Pointer {
    //    ObjectExt::property(self, "spa-device-handle")
    //}

    #[doc(alias = "create-object")]
    pub fn connect_create_object<F: Fn(&Self, u32, &str, &str, &Properties) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn create_object_trampoline<F: Fn(&SpaDevice, u32, &str, &str, &Properties) + 'static>(this: *mut ffi::WpSpaDevice, object: libc::c_uint, p0: *mut libc::c_char, p1: *mut libc::c_char, p2: *mut ffi::WpProperties, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, &glib::GString::from_glib_borrow(p0), &glib::GString::from_glib_borrow(p1), &from_glib_borrow(p2))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"create-object\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(create_object_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "object-removed")]
    pub fn connect_object_removed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn object_removed_trampoline<F: Fn(&SpaDevice, u32) + 'static>(this: *mut ffi::WpSpaDevice, object: libc::c_uint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"object-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(object_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
