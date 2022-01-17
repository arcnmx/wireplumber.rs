// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::ObjectManager;
use crate::Properties;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WpCore")]
    pub struct Core(Object<ffi::WpCore, ffi::WpCoreClass>);

    match fn {
        type_ => || ffi::wp_core_get_type(),
    }
}

impl Core {
    #[doc(alias = "wp_core_new")]
    pub fn new(context: Option<&glib::MainContext>, properties: Option<&Properties>) -> Core {
        unsafe {
            from_glib_full(ffi::wp_core_new(context.to_glib_none().0, properties.to_glib_full()))
        }
    }

    #[doc(alias = "wp_core_connect")]
    pub fn connect(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_core_connect(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_disconnect")]
    pub fn disconnect(&self) {
        unsafe {
            ffi::wp_core_disconnect(self.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_core_get_g_main_context")]
    #[doc(alias = "get_g_main_context")]
    pub fn g_main_context(&self) -> Option<glib::MainContext> {
        unsafe {
            from_glib_none(ffi::wp_core_get_g_main_context(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_get_properties")]
    #[doc(alias = "get_properties")]
    pub fn properties(&self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_core_get_properties(self.to_glib_none().0))
        }
    }

    //#[doc(alias = "wp_core_get_pw_context")]
    //#[doc(alias = "get_pw_context")]
    //pub fn pw_context(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:wp_core_get_pw_context() }
    //}

    //#[doc(alias = "wp_core_get_pw_core")]
    //#[doc(alias = "get_pw_core")]
    //pub fn pw_core(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:wp_core_get_pw_core() }
    //}

    #[doc(alias = "wp_core_get_remote_cookie")]
    #[doc(alias = "get_remote_cookie")]
    pub fn remote_cookie(&self) -> u32 {
        unsafe {
            ffi::wp_core_get_remote_cookie(self.to_glib_none().0)
        }
    }

    #[doc(alias = "wp_core_get_remote_host_name")]
    #[doc(alias = "get_remote_host_name")]
    pub fn remote_host_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_core_get_remote_host_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_get_remote_name")]
    #[doc(alias = "get_remote_name")]
    pub fn remote_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_core_get_remote_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_get_remote_properties")]
    #[doc(alias = "get_remote_properties")]
    pub fn remote_properties(&self) -> Option<Properties> {
        unsafe {
            from_glib_full(ffi::wp_core_get_remote_properties(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_get_remote_user_name")]
    #[doc(alias = "get_remote_user_name")]
    pub fn remote_user_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_core_get_remote_user_name(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_get_remote_version")]
    #[doc(alias = "get_remote_version")]
    pub fn remote_version(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_core_get_remote_version(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_idle_add")]
    pub fn idle_add<P: Fn() -> bool + 'static>(&self, function: P) -> glib::Source {
        let function_data: Box_<P> = Box_::new(function);
        unsafe extern "C" fn function_func<P: Fn() -> bool + 'static>(user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)();
            res.into_glib()
        }
        let function = Some(function_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn() -> bool + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = function_data;
        unsafe {
            let mut source = ptr::null_mut();
            ffi::wp_core_idle_add(self.to_glib_none().0, &mut source, function, Box_::into_raw(super_callback0) as *mut _, destroy_call4);
            from_glib_full(source)
        }
    }

    #[doc(alias = "wp_core_idle_add_closure")]
    pub fn idle_add_closure(&self, closure: &glib::Closure) -> glib::Source {
        unsafe {
            let mut source = ptr::null_mut();
            ffi::wp_core_idle_add_closure(self.to_glib_none().0, &mut source, closure.to_glib_none().0);
            from_glib_full(source)
        }
    }

    #[doc(alias = "wp_core_install_object_manager")]
    pub fn install_object_manager(&self, om: &ObjectManager) {
        unsafe {
            ffi::wp_core_install_object_manager(self.to_glib_none().0, om.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_core_is_connected")]
    pub fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ffi::wp_core_is_connected(self.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_load_component")]
    pub fn load_component(&self, component: &str, type_: &str, args: Option<&glib::Variant>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::wp_core_load_component(self.to_glib_none().0, component.to_glib_none().0, type_.to_glib_none().0, args.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "wp_core_sync")]
    pub fn sync<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) -> bool {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn sync_trampoline<Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::wp_core_sync_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = sync_trampoline::<Q>;
        unsafe {
            from_glib(ffi::wp_core_sync(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _))
        }
    }

    
    pub fn sync_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.sync(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[cfg(any(feature = "v0_4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_6")))]
    #[doc(alias = "wp_core_sync_closure")]
    pub fn sync_closure<P: IsA<gio::Cancellable>>(&self, cancellable: Option<&P>, closure: &glib::Closure) -> bool {
        unsafe {
            from_glib(ffi::wp_core_sync_closure(self.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, closure.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_core_timeout_add")]
    pub fn timeout_add<P: Fn() -> bool + 'static>(&self, timeout_ms: u32, function: P) -> glib::Source {
        let function_data: Box_<P> = Box_::new(function);
        unsafe extern "C" fn function_func<P: Fn() -> bool + 'static>(user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)();
            res.into_glib()
        }
        let function = Some(function_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn() -> bool + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call5 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = function_data;
        unsafe {
            let mut source = ptr::null_mut();
            ffi::wp_core_timeout_add(self.to_glib_none().0, &mut source, timeout_ms, function, Box_::into_raw(super_callback0) as *mut _, destroy_call5);
            from_glib_full(source)
        }
    }

    #[doc(alias = "wp_core_timeout_add_closure")]
    pub fn timeout_add_closure(&self, timeout_ms: u32, closure: &glib::Closure) -> glib::Source {
        unsafe {
            let mut source = ptr::null_mut();
            ffi::wp_core_timeout_add_closure(self.to_glib_none().0, &mut source, timeout_ms, closure.to_glib_none().0);
            from_glib_full(source)
        }
    }

    #[doc(alias = "wp_core_update_properties")]
    pub fn update_properties(&self, updates: &Properties) {
        unsafe {
            ffi::wp_core_update_properties(self.to_glib_none().0, updates.to_glib_full());
        }
    }

    #[doc(alias = "connected")]
    pub fn connect_connected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn connected_trampoline<F: Fn(&Core) + 'static>(this: *mut ffi::WpCore, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"connected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(connected_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "disconnected")]
    pub fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn disconnected_trampoline<F: Fn(&Core) + 'static>(this: *mut ffi::WpCore, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"disconnected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(disconnected_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "pw-core")]
    pub fn connect_pw_core_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pw_core_trampoline<F: Fn(&Core) + 'static>(this: *mut ffi::WpCore, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pw-core\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_pw_core_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
