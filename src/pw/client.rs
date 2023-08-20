use {
	crate::{prelude::*, pw::Client},
	pipewire_sys::pw_permission,
};

impl Client {
	#[doc(alias = "wp_client_update_permissions_array")]
	#[doc(alias = "wp_client_update_permissions")]
	#[doc(alias = "update_permissions_array")]
	pub fn update_permissions(&self, permissions: &[pw_permission]) {
		let n_perm = permissions.len() as _;
		unsafe { ffi::wp_client_update_permissions_array(self.to_glib_none().0, n_perm, permissions.as_ptr()) }
	}
}
