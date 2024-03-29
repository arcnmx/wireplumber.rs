// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use crate::{Core,GlobalProxy,Metadata,Object,Properties,Proxy};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "WpImplMetadata")]
    pub struct ImplMetadata(Object<ffi::WpImplMetadata, ffi::WpImplMetadataClass>) @extends Metadata, GlobalProxy, Proxy, Object;

    match fn {
        type_ => || ffi::wp_impl_metadata_get_type(),
    }
}

impl ImplMetadata {
    #[doc(alias = "wp_impl_metadata_new")]
    pub fn new(core: &Core) -> ImplMetadata {
        unsafe {
            from_glib_full(ffi::wp_impl_metadata_new(core.to_glib_none().0))
        }
    }

    #[cfg(feature = "v0_4_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_4_3")))]
    #[doc(alias = "wp_impl_metadata_new_full")]
    #[doc(alias = "new_full")]
    pub fn with_properties(core: &Core, name: Option<&str>, properties: Option<Properties>) -> ImplMetadata {
        unsafe {
            from_glib_full(ffi::wp_impl_metadata_new_full(core.to_glib_none().0, name.to_glib_none().0, properties.into_glib_ptr()))
        }
    }

    pub fn name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "name")
    }

    pub fn properties(&self) -> Option<Properties> {
        ObjectExt::property(self, "properties")
    }
}
