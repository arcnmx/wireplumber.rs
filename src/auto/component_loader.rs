// Generated by gir (https://github.com/gtk-rs/gir @ 0.14-2021-10-08)
// from /nix/store/7i7hi0ivv21w1n2n5b9gy7lfrhnkis9p-wireplumber.gir/share/gir-1.0 (@ ???)
// from /nix/store/l8nlsw7p6xi30lna4gq3mvd574njnmly-gobject-introspection-1.70.0-dev/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Object;
use crate::Plugin;

glib::wrapper! {
    #[doc(alias = "WpComponentLoader")]
    pub struct ComponentLoader(Object<ffi::WpComponentLoader, ffi::WpComponentLoaderClass>) @extends Plugin, Object;

    match fn {
        type_ => || ffi::wp_component_loader_get_type(),
    }
}

impl ComponentLoader {}

pub const NONE_COMPONENT_LOADER: Option<&ComponentLoader> = None;
