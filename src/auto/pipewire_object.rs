// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Iterator;
use crate::Object;
use crate::Properties;
use crate::Proxy;
use crate::SpaPod;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WpPipewireObject")]
    pub struct PipewireObject(Interface<ffi::WpPipewireObject, ffi::WpPipewireObjectInterface>) @requires Proxy, Object;

    match fn {
        type_ => || ffi::wp_pipewire_object_get_type(),
    }
}

pub const NONE_PIPEWIRE_OBJECT: Option<&PipewireObject> = None;

pub trait PipewireObjectExt: 'static {
    #[doc(alias = "wp_pipewire_object_enum_params")]
    fn enum_params<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Option<Iterator>, glib::Error>) + Send + 'static>(&self, id: Option<&str>, filter: Option<&SpaPod>, cancellable: Option<&P>, callback: Q);

    
    fn enum_params_future(&self, id: Option<&str>, filter: Option<&SpaPod>) -> Pin<Box_<dyn std::future::Future<Output = Result<Option<Iterator>, glib::Error>> + 'static>>;

    #[doc(alias = "wp_pipewire_object_enum_params_sync")]
    fn enum_params_sync(&self, id: &str, filter: Option<&SpaPod>) -> Option<Iterator>;

    //#[doc(alias = "wp_pipewire_object_get_native_info")]
    //#[doc(alias = "get_native_info")]
    //fn native_info(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "wp_pipewire_object_get_param_info")]
    #[doc(alias = "get_param_info")]
    fn param_info(&self) -> Option<glib::Variant>;

    #[doc(alias = "wp_pipewire_object_get_properties")]
    #[doc(alias = "get_properties")]
    fn properties(&self) -> Option<Properties>;

    #[doc(alias = "wp_pipewire_object_get_property")]
    #[doc(alias = "get_property")]
    fn property(&self, key: &str) -> Option<glib::GString>;

    #[doc(alias = "wp_pipewire_object_new_properties_iterator")]
    fn new_properties_iterator(&self) -> Option<Iterator>;

    #[doc(alias = "wp_pipewire_object_set_param")]
    fn set_param(&self, id: &str, flags: u32, param: &SpaPod) -> bool;

    #[doc(alias = "params-changed")]
    fn connect_params_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "native-info")]
    fn connect_native_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "param-info")]
    fn connect_param_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "properties")]
    fn connect_properties_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PipewireObject>> PipewireObjectExt for O {
    fn enum_params<P: IsA<gio::Cancellable>, Q: FnOnce(Result<Option<Iterator>, glib::Error>) + Send + 'static>(&self, id: Option<&str>, filter: Option<&SpaPod>, cancellable: Option<&P>, callback: Q) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn enum_params_trampoline<Q: FnOnce(Result<Option<Iterator>, glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::wp_pipewire_object_enum_params_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = enum_params_trampoline::<Q>;
        unsafe {
            ffi::wp_pipewire_object_enum_params(self.as_ref().to_glib_none().0, id.to_glib_none().0, filter.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn enum_params_future(&self, id: Option<&str>, filter: Option<&SpaPod>) -> Pin<Box_<dyn std::future::Future<Output = Result<Option<Iterator>, glib::Error>> + 'static>> {

        let id = id.map(ToOwned::to_owned);
        let filter = filter.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.enum_params(
                id.as_ref().map(::std::borrow::Borrow::borrow),
                filter.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    fn enum_params_sync(&self, id: &str, filter: Option<&SpaPod>) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_pipewire_object_enum_params_sync(self.as_ref().to_glib_none().0, id.to_glib_none().0, filter.to_glib_none().0))
        }
    }

    //fn native_info(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:wp_pipewire_object_get_native_info() }
    //}

    fn param_info(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::wp_pipewire_object_get_param_info(self.as_ref().to_glib_none().0))
        }
    }

    fn properties(&self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_pipewire_object_get_properties(self.as_ref().to_glib_none().0))
        }
    }

    fn property(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_pipewire_object_get_property(self.as_ref().to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn new_properties_iterator(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::wp_pipewire_object_new_properties_iterator(self.as_ref().to_glib_none().0))
        }
    }

    fn set_param(&self, id: &str, flags: u32, param: &SpaPod) -> bool {
        unsafe {
            from_glib(ffi::wp_pipewire_object_set_param(self.as_ref().to_glib_none().0, id.to_glib_none().0, flags, param.to_glib_full()))
        }
    }

    fn connect_params_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn params_changed_trampoline<P: IsA<PipewireObject>, F: Fn(&P, &str) + 'static>(this: *mut ffi::WpPipewireObject, object: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PipewireObject::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"params-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(params_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_native_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_native_info_trampoline<P: IsA<PipewireObject>, F: Fn(&P) + 'static>(this: *mut ffi::WpPipewireObject, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PipewireObject::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::native-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_native_info_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_param_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_param_info_trampoline<P: IsA<PipewireObject>, F: Fn(&P) + 'static>(this: *mut ffi::WpPipewireObject, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PipewireObject::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::param-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_param_info_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_properties_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_properties_trampoline<P: IsA<PipewireObject>, F: Fn(&P) + 'static>(this: *mut ffi::WpPipewireObject, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PipewireObject::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::properties\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_properties_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}
