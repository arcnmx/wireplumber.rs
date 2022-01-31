// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::Properties;
use glib::translate::*;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WpState")]
    pub struct State(Object<ffi::WpState, ffi::WpStateClass>);

    match fn {
        type_ => || ffi::wp_state_get_type(),
    }
}

impl State {
    #[doc(alias = "wp_state_new")]
    pub fn new(name: &str) -> State {
        unsafe {
            from_glib_full(ffi::wp_state_new(name.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_state_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::wp_state_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_state_get_location")]
    #[doc(alias = "get_location")]
    pub fn location(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_state_get_location(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_state_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_state_get_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_state_load")]
    pub fn load(&self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_state_load(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_state_save")]
    pub fn save(&self, props: &Properties) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::wp_state_save(self.to_glib_none().0, props.to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}