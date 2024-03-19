use {
	crate::{core::Conf, prelude::*},
	pipewire_sys::pw_context,
};

impl Conf {
	#[doc(alias = "wp_conf_parse_pw_context_sections")]
	pub unsafe fn parse_pw_context_sections(&self, context: ptr::NonNull<pw_context>) {
		ffi::wp_conf_parse_pw_context_sections(self.to_glib_none().0, context.as_ptr())
	}
}
