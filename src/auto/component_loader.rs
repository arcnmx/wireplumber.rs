// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Object,Plugin};

glib::wrapper! {
    #[doc(alias = "WpComponentLoader")]
    pub struct ComponentLoader(Object<ffi::WpComponentLoader, ffi::WpComponentLoaderClass>) @extends Plugin, Object;

    match fn {
        type_ => || ffi::wp_component_loader_get_type(),
    }
}

impl ComponentLoader {
        pub const NONE: Option<&'static ComponentLoader> = None;
    
}
