use glib::{LogLevelFlags, translate::{IntoGlib, ToGlibPtr, from_glib}};
use libspa_sys::spa_log;

pub struct Log(());

impl Log {
	#[doc(alias = "WP_LOG_LEVEL_TRACE")]
	pub const LEVEL_TRACE: i32 = ffi::WP_LOG_LEVEL_TRACE;

	/*#[doc(alias = "WP_OBJECT_FORMAT")]
	pub const OBJECT_FORMAT: CStr = ffi::WP_OBJECT_FORMAT;*/

	#[doc(alias = "wp_log_level_is_enabled")]
	pub fn level_is_enabled(flags: LogLevelFlags) -> bool {
		unsafe {
			from_glib(ffi::wp_log_level_is_enabled(flags.into_glib()))
		}
	}

	#[doc(alias = "wp_log_set_level")]
	pub fn set_level(level: &str) {
		std::env::set_var("WIREPLUMBER_DEBUG", level); // XXX: this doesn't seem to work properly otherwise?
		unsafe {
			ffi::wp_log_set_level(level.to_glib_none().0)
		}
	}

	#[doc(alias = "wp_spa_log_get_instance")]
	pub fn spa_log() -> *mut spa_log {
		unsafe {
			ffi::wp_spa_log_get_instance()
		}
	}

	// TODO: wp_log_writer_default, wp_log_structured_standard
}
