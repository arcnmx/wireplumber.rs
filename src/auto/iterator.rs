// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Iterator(Shared<ffi::WpIterator>);

    match fn {
        ref => |ptr| ffi::wp_iterator_ref(ptr),
        unref => |ptr| ffi::wp_iterator_unref(ptr),
        type_ => || ffi::wp_iterator_get_type(),
    }
}

impl Iterator {
    //#[doc(alias = "wp_iterator_new_ptr_array")]
    //pub fn new_ptr_array(items: /*Unimplemented*/&[&Fundamental: Pointer], item_type: glib::types::Type) -> Iterator {
    //    unsafe { TODO: call ffi:wp_iterator_new_ptr_array() }
    //}

    //#[doc(alias = "wp_iterator_fold")]
    //pub fn fold(&self, func: /*Unimplemented*/FnMut(&glib::Value, &glib::Value, /*Unimplemented*/Option<Fundamental: Pointer>) -> bool, ret: /*Unimplemented*/glib::Value, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:wp_iterator_fold() }
    //}

    //#[doc(alias = "wp_iterator_foreach")]
    //pub fn foreach(&self, func: /*Unimplemented*/FnMut(&glib::Value, /*Unimplemented*/Option<Fundamental: Pointer>), data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:wp_iterator_foreach() }
    //}

    //#[doc(alias = "wp_iterator_get_user_data")]
    //#[doc(alias = "get_user_data")]
    //pub fn user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:wp_iterator_get_user_data() }
    //}

    #[doc(alias = "wp_iterator_next")]
    pub fn next(&self) -> Option<glib::Value> {
        unsafe {
            let mut item = glib::Value::uninitialized();
            let ret = from_glib(ffi::wp_iterator_next(self.to_glib_none().0, item.to_glib_none_mut().0));
            if ret { Some(item) } else { None }
        }
    }

    #[doc(alias = "wp_iterator_reset")]
    pub fn reset(&self) {
        unsafe {
            ffi::wp_iterator_reset(self.to_glib_none().0);
        }
    }
}
