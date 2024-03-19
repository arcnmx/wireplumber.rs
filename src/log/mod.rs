//! Logging macros and utilities
//!
//! # See also
//!
//! [C API docs](https://pipewire.pages.freedesktop.org/wireplumber/library/c_api/log_api.html)

mod macros;

#[doc(hidden)]
pub use self::macros::_log_inner;
#[allow(unused_imports)]
pub(crate) use self::macros::{wp_critical, wp_debug, wp_info, wp_message, wp_trace, wp_warning};
pub use {
	self::macros::{critical, debug, info, log, message, trace, warning},
	crate::auto::LogTopicFlags,
};
use {
	crate::prelude::*,
	ffi::WpLogTopic,
	glib::{GString, GStringBuilder, LogLevelFlags},
	libspa_sys::spa_log,
	std::env,
	std::marker::PhantomPinned,
	std::mem::transmute,
};

pub struct Log(());

impl Log {
	#[doc(alias = "WP_LOG_LEVEL_TRACE")]
	pub const LEVEL_TRACE: LogLevelFlags = LogLevelFlags::from_bits_truncate(ffi::WP_LOG_LEVEL_TRACE as _);

	/*#[doc(alias = "WP_OBJECT_FORMAT")]
	pub const OBJECT_FORMAT: CStr = ffi::WP_OBJECT_FORMAT;*/

	pub fn domain() -> &'static str {
		LibraryErrorEnum::domain().as_str()
	}

	pub fn set_level(level: &str) {
		env::set_var("WIREPLUMBER_DEBUG", level); // XXX: this doesn't seem to work properly otherwise?
	}

	#[doc(alias = "wp_spa_log_get_instance")]
	pub fn spa_log() -> *mut spa_log {
		unsafe { ffi::wp_spa_log_get_instance() }
	}

	#[doc(alias = "wp_log_structured_standard")]
	pub fn log_string<M: Into<GString>, L: Into<LogLevelFlags>>(log_level: L, context: StructuredLogContext, message: M) {
		unsafe {
			// XXX: so much allocation, it burns...
			let domain = context.domain.unwrap_or_default().to_glib_none();
			let file = context.file.unwrap_or_default().to_glib_none();
			let line = context.line.as_ref().map(ToString::to_string);
			let line = line.as_ref().map(|l| &l[..]).unwrap_or_default().to_glib_none();
			let function = context.function.unwrap_or_default().to_glib_none();
			let obj_type = context.object_type.unwrap_or(match context.object {
				Some(obj) => obj.type_(),
				None => Type::INVALID,
			});
			let obj = context.object.as_ref().map(|obj| (*obj).to_glib_none());
			let message = message.into();
			ffi::wp_log_structured_standard(
				domain.0,
				log_level.into().into_glib(),
				file.0,
				line.0,
				function.0,
				obj_type.into_glib(),
				obj.as_ref().map(|o| o.0).unwrap_or(ptr::null()) as *mut _,
				b"%s\0".as_ptr() as *const _,
				message.as_ptr(),
			)
		}
	}

	pub fn log_args<O: AsRef<GObject>, L: Into<LogLevelFlags>>(
		log_level: L,
		context: StructuredLogContext<O>,
		args: fmt::Arguments,
	) {
		let mut message = GStringBuilder::default();
		let _ = write!(message, "{args}");
		Self::log_string(log_level.into(), context.to_object(), message.into_string())
	}

	#[doc(alias = "wp_log_writer_default")]
	pub unsafe fn writer_default<L: Into<LogLevelFlags>>(
		log_levels: L,
		fields: &[glib::ffi::GLogField],
		user_data: gpointer,
	) -> glib::ffi::GLogWriterOutput {
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
	fn to_object(&self) -> StructuredLogContext<'a, GObject>
	where
		O: AsRef<GObject>,
	{
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LogTopic {
	inner: WpLogTopic,
	_pinned: PhantomPinned,
}

impl LogTopic {
	pub fn new<N: Into<GString>, L: Into<LogLevelFlags>>(name: N, level: L) -> Self {
		Self {
			inner: WpLogTopic {
				topic_name: unsafe { from_glib_full(name.into()) },
				flags: LogTopicFlags::from(level.into()),
				_wp_padding: [ptr::null_mut(); 3],
			},
			_pinned: PhantomPinned,
		}
	}

	pub fn register(self) -> Pin<Box<Self>> {
		let mut this = Box::pin(self);
		unsafe {
			this.as_mut().register_unchecked()
		}
		this
	}

	pub fn unregister(self: Pin<Box<Self>>) {
		drop(self);
	}

	pub unsafe fn register_unchecked(&mut self) {
		ffi::wp_log_topic_register(self.ffi_mut())
	}

	pub unsafe fn unregister_unchecked(&mut self) {
		ffi::wp_log_topic_unregister(&mut self.inner)
		// TODO: unset initialized flag?
	}

	pub fn ffi(&self) -> &WpLogTopic {
		&self.inner
	}

	pub unsafe fn ffi_mut(&self) -> &mut WpLogTopic {
		&mut self.inner
	}

	pub fn name(&self) -> &GStr {
		unsafe {
			GStr::from_utf8_with_nul_unchecked(self.inner.topic_name)
		}
	}

	pub unsafe fn flags_mut(&mut self) -> &mut LogTopicFlags {
		transmute(&mut self.inner.flags)
	}
}

impl Drop for LogTopic {
	fn drop(&mut self) {
		if self.flags().contains(LogTopicFlags::FLAG_INITIALIZED) {
			unsafe {
				self.unregister_unchecked()
			}
		}
	}
}

impl LogTopicFlags {
	pub fn with_levels<L: Into<LogLevelFlags>>(levels: L) -> Self {
		Self::from_bits_truncate(levels.into().bits() & Self::LEVEL_MASK)
	}

	pub fn levels(&self) -> LogLevelFlags {
		LogLevelFlags::from_bits_truncate(self.bits() & Self::LEVEL_MASK)
	}
}

impl From<LogLevelFlags> for LogTopicFlags {
	fn from(levels: LogLevelFlags) -> Self {
		LogTopicFlags::with_levels(levels)
	}
}
