// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Core;
use crate::SessionItem;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "WpSiFactory")]
    pub struct SiFactory(Object<ffi::WpSiFactory, ffi::WpSiFactoryClass>);

    match fn {
        type_ => || ffi::wp_si_factory_get_type(),
    }
}

impl SiFactory {
    #[doc(alias = "wp_si_factory_new_simple")]
    pub fn new_simple(factory_name: &str, si_type: glib::types::Type) -> SiFactory {
        unsafe {
            from_glib_full(ffi::wp_si_factory_new_simple(factory_name.to_glib_none().0, si_type.into_glib()))
        }
    }

    #[doc(alias = "wp_si_factory_find")]
    pub fn find(core: &Core, factory_name: &str) -> Option<SiFactory> {
        unsafe {
            from_glib_full(ffi::wp_si_factory_find(core.to_glib_none().0, factory_name.to_glib_none().0))
        }
    }

    #[doc(alias = "wp_si_factory_register")]
    pub fn register<P: IsA<SiFactory>>(core: &Core, factory: &P) {
        unsafe {
            ffi::wp_si_factory_register(core.to_glib_none().0, factory.as_ref().to_glib_full());
        }
    }
}

pub const NONE_SI_FACTORY: Option<&SiFactory> = None;

pub trait SiFactoryExt: 'static {
    #[doc(alias = "wp_si_factory_construct")]
    fn construct(&self, core: &Core) -> Option<SessionItem>;

    #[doc(alias = "wp_si_factory_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;
}

impl<O: IsA<SiFactory>> SiFactoryExt for O {
    fn construct(&self, core: &Core) -> Option<SessionItem> {
        unsafe {
            from_glib_full(ffi::wp_si_factory_construct(self.as_ref().to_glib_none().0, core.to_glib_none().0))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_si_factory_get_name(self.as_ref().to_glib_none().0))
        }
    }
}
