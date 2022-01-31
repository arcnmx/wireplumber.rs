// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::Object;
use crate::SessionItem;
use crate::SiLink;
use crate::SiLinkable;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WpSiAcquisition")]
    pub struct SiAcquisition(Interface<ffi::WpSiAcquisition, ffi::WpSiAcquisitionInterface>) @requires SessionItem, Object;

    match fn {
        type_ => || ffi::wp_si_acquisition_get_type(),
    }
}

impl SiAcquisition {
        pub const NONE: Option<&'static SiAcquisition> = None;
    
}

pub trait SiAcquisitionExt: 'static {
    #[doc(alias = "wp_si_acquisition_release")]
    fn release(&self, acquisitor: &impl IsA<SiLink>, item: &impl IsA<SiLinkable>);
}

impl<O: IsA<SiAcquisition>> SiAcquisitionExt for O {
    fn release(&self, acquisitor: &impl IsA<SiLink>, item: &impl IsA<SiLinkable>) {
        unsafe {
            ffi::wp_si_acquisition_release(self.as_ref().to_glib_none().0, acquisitor.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }
}