use glib::{GString, GStringBuilder, LogLevelFlags};
use libspa_sys::spa_log;
use std::env;
use crate::prelude::*;

mod macros;
pub use macros::{
	log,
	trace, debug, message, info, warning, critical,
};
#[doc(hidden)]
pub use macros::_log_inner;
#[allow(unused_imports)]
pub(crate) use macros::{wp_trace, wp_debug, wp_message, wp_info, wp_warning, wp_critical};

pub struct Log(());

impl Log {
	#[doc(alias = "WP_LOG_LEVEL_TRACE")]
	pub const LEVEL_TRACE: LogLevelFlags = LogLevelFlags::from_bits_truncate(ffi::WP_LOG_LEVEL_TRACE as _);

	/*#[doc(alias = "WP_OBJECT_FORMAT")]
	pub const OBJECT_FORMAT: CStr = ffi::WP_OBJECT_FORMAT;*/

	pub fn domain() -> &'static str {
		LibraryErrorEnum::domain().as_str()
	}

	#[doc(alias = "wp_log_level_is_enabled")]
	pub fn level_is_enabled<L: Into<LogLevelFlags>>(flags: L) -> bool {
		unsafe {
			from_glib(ffi::wp_log_level_is_enabled(flags.into().into_glib()))
		}
	}

	#[doc(alias = "wp_log_set_level")]
	pub fn set_level(level: &str) {
		env::set_var("WIREPLUMBER_DEBUG", level); // XXX: this doesn't seem to work properly otherwise?
		unsafe {
			ffi::wp_log_set_level(level.to_glib_none().0)
		}
	}

	pub fn set_default_level(level: &str) {
		if env::var_os("WIREPLUMBER_DEBUG").is_none() {
			Self::set_level(level)
		}
	}

	#[doc(alias = "wp_spa_log_get_instance")]
	pub fn spa_log() -> *mut spa_log {
		unsafe {
			ffi::wp_spa_log_get_instance()
		}
	}

	#[doc(alias = "wp_log_structured_standard")]
	pub fn log_string<M: Into<GString>, L: Into<LogLevelFlags>>(log_level: L, context: StructuredLogContext, message: M) {
		unsafe {
			// XXX: so much allocation, it burns...
			let domain = context.domain.to_glib_none();
			let file = context.file.to_glib_none();
			let line = context.line.as_ref().map(ToString::to_string);
			let line = line.to_glib_none();
			let function = context.function.to_glib_none();
			let obj_type = context.object_type.unwrap_or(match context.object {
				Some(obj) => obj.type_(),
				None => Type::INVALID,
			});
			let obj = context.object.as_ref().map(|obj| (*obj).to_glib_none());
			let message = message.into();
			ffi::wp_log_structured_standard(
				domain.0, log_level.into().into_glib(),
				file.0, line.0, function.0,
				obj_type.into_glib(), obj.as_ref().map(|o| o.0).unwrap_or(ptr::null()) as *mut _,
				b"%s\0".as_ptr() as *const _, message.as_ptr()
			)
		}
	}

	pub fn log_args<O: AsRef<GObject>, L: Into<LogLevelFlags>>(log_level: L, context: StructuredLogContext<O>, args: fmt::Arguments) {
		let mut message = GStringBuilder::default();
		let _ = write!(message, "{}", args);
		Self::log_string(log_level.into(), context.to_object(), message.into_string())
	}

	#[doc(alias = "wp_log_writer_default")]
	pub unsafe fn writer_default<L: Into<LogLevelFlags>>(log_levels: L, fields: &[glib::ffi::GLogField], user_data: gpointer) -> glib::ffi::GLogWriterOutput {
		ffi::wp_log_writer_default(log_levels.into().into_glib(), fields.as_ptr(), fields.len(), user_data)
	}
	// TODO: wp_log_writer_default, wp_log_structured_standard
}

#[derive(Debug, Clone, Default)]
pub struct StructuredLogContext<'a, O = GObject> {
	pub domain: Option<&'a str>,
	pub file: Option<&'static str>,
	pub line: Option<u32>,
	pub function: Option<&'a str>,
	pub object: Option<&'a O>,
	pub object_type: Option<Type>,
}

impl<'a, O> StructuredLogContext<'a, O> {
	fn to_object(&self) -> StructuredLogContext<'a, GObject> where O: AsRef<GObject> {
		StructuredLogContext {
			domain: self.domain,
			file: self.file,
			line: self.line,
			function: self.function,
			object: self.object.map(|o| o.as_ref()),
			object_type: self.object_type,
		}
	}
}
