// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Object;
use crate::SessionItem;
use crate::SiAcquisition;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "WpSiLinkable")]
    pub struct SiLinkable(Interface<ffi::WpSiLinkable, ffi::WpSiLinkableInterface>) @requires SessionItem, Object;

    match fn {
        type_ => || ffi::wp_si_linkable_get_type(),
    }
}

pub const NONE_SI_LINKABLE: Option<&SiLinkable> = None;

pub trait SiLinkableExt: 'static {
    #[doc(alias = "wp_si_linkable_get_acquisition")]
    #[doc(alias = "get_acquisition")]
    fn acquisition(&self) -> Option<SiAcquisition>;

    #[doc(alias = "wp_si_linkable_get_ports")]
    #[doc(alias = "get_ports")]
    fn ports(&self, context: Option<&str>) -> Option<glib::Variant>;
}

impl<O: IsA<SiLinkable>> SiLinkableExt for O {
    fn acquisition(&self) -> Option<SiAcquisition> {
        unsafe {
            from_glib_none(ffi::wp_si_linkable_get_acquisition(self.as_ref().to_glib_none().0))
        }
    }

    fn ports(&self, context: Option<&str>) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::wp_si_linkable_get_ports(self.as_ref().to_glib_none().0, context.to_glib_none().0))
        }
    }
}
