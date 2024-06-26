// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Object,SessionItem,SpaPod};
#[cfg(feature = "v0_4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_10")))]
use crate::{SiAdapterPortsState};
use glib::{prelude::*,translate::*};
#[cfg(feature = "v0_4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_10")))]
use glib::{signal::{connect_raw, SignalHandlerId}};
#[cfg(feature = "v0_4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_10")))]
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "WpSiAdapter")]
    pub struct SiAdapter(Interface<ffi::WpSiAdapter, ffi::WpSiAdapterInterface>) @requires SessionItem, Object;

    match fn {
        type_ => || ffi::wp_si_adapter_get_type(),
    }
}

impl SiAdapter {
        pub const NONE: Option<&'static SiAdapter> = None;
    
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SiAdapter>> Sealed for T {}
}

pub trait SiAdapterExt: IsA<SiAdapter> + sealed::Sealed + 'static {
    #[doc(alias = "wp_si_adapter_get_ports_format")]
    #[doc(alias = "get_ports_format")]
    fn ports_format(&self) -> (SpaPod, Option<glib::GString>) {
        unsafe {
            let mut mode = std::ptr::null();
            let ret = from_glib_full(ffi::wp_si_adapter_get_ports_format(self.as_ref().to_glib_none().0, &mut mode));
            (ret, from_glib_full(mode))
        }
    }

    #[cfg(feature = "v0_4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_4_10")))]
    #[doc(alias = "wp_si_adapter_get_ports_state")]
    #[doc(alias = "get_ports_state")]
    fn ports_state(&self) -> SiAdapterPortsState {
        unsafe {
            from_glib(ffi::wp_si_adapter_get_ports_state(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v0_4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_4_10")))]
    #[doc(alias = "adapter-ports-state-changed")]
    fn connect_adapter_ports_state_changed<F: Fn(&Self, SiAdapterPortsState, SiAdapterPortsState) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn adapter_ports_state_changed_trampoline<P: IsA<SiAdapter>, F: Fn(&P, SiAdapterPortsState, SiAdapterPortsState) + 'static>(this: *mut ffi::WpSiAdapter, object: ffi::WpSiAdapterPortsState, p0: ffi::WpSiAdapterPortsState, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SiAdapter::from_glib_borrow(this).unsafe_cast_ref(), from_glib(object), from_glib(p0))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"adapter-ports-state-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(adapter_ports_state_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<SiAdapter>> SiAdapterExt for O {}
