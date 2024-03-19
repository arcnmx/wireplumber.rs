//! Logging macros and utilities
//!
//! # Usage
//!
//! ```no_run
//! use glib::gstr;
//! use wireplumber::{
//!   prelude::*,
//!   log::{Log, info, log_topic},
//! };
//!
//! log_topic! {
//!   static TOPIC = "myapp";
//! }
//!
//! fn main() {
//!   Log::set_default_level(gstr!("4"));
//!   Core::init();
//!
//!   info!(domain: TOPIC, "hello world");
//! }
//! ```
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
	self::macros::{critical, debug, info, log, log_topic, message, trace, warning},
	crate::auto::LogTopicFlags,
};
use {
	crate::prelude::*,
	ffi::WpLogTopic,
	glib::{gformat, GString, GStringBuilder, LogLevelFlags},
	libspa_sys::spa_log,
	std::{cell::Cell, env, marker::PhantomPinned, mem::transmute},
};

log_topic! {
	/// Internal [LogTopic] used by this library.
	pub static TOPIC_BINDINGS = "wireplumber.rs";
}
#[doc(hidden)]
pub static TOPIC_FALLBACK: LogTopicStorage<'static> = LogTopicStorage::with_name(gstr!("default"));

pub struct Log(());

impl Log {
	#[doc(alias = "WP_LOG_LEVEL_TRACE")]
	pub const LEVEL_TRACE: LogLevelFlags = LogLevelFlags::from_bits_truncate(ffi::WP_LOG_LEVEL_TRACE as _);

	pub const ENV_LEVEL: &'static str = "WIREPLUMBER_DEBUG";

	/*#[doc(alias = "WP_OBJECT_FORMAT")]
	pub const OBJECT_FORMAT: GStr = ffi::WP_OBJECT_FORMAT;*/

	#[doc(alias("wp_log_set_level"))]
	pub fn set_level<L: IntoGStr>(level: L) {
		level.run_with_gstr(|level| {
			unsafe {
				ffi::wp_log_set_level(level.as_ptr());
			}
			// XXX: depending on library init order, prevent environment override
			env::set_var(Self::ENV_LEVEL, level);
		})
	}

	#[doc(alias("wp_log_set_level"))]
	pub fn set_default_level<L: IntoGStr>(level: L) {
		match env::var_os(Self::ENV_LEVEL) {
			None => Self::set_level(level),
			Some(_) => {
				// ffi::wp_log_set_level(level) ?
			},
		}
	}

	#[doc(alias = "wp_spa_log_get_instance")]
	pub fn spa_log() -> *mut spa_log {
		unsafe { ffi::wp_spa_log_get_instance() }
	}

	#[doc(alias = "wp_logt_checked")]
	#[doc(alias = "wp_log_checked")]
	pub fn log_string_checked(
		topic: Option<&LogTopic>,
		level: LogLevelFlags,
		file: Option<&GStr>,
		line: Option<&GStr>,
		function: Option<&GStr>,
		object_type: Option<Type>,
		object: Option<BorrowedObject<GObject>>,
		message: &GStr,
	) {
		let level = level.into_glib();
		let file = file.to_glib_none();
		let line = line.to_glib_none();
		let function = function.to_glib_none();
		let format = unsafe { GStr::from_str_with_nul_unchecked("%s\0") }.as_ptr();
		let message = message.as_ptr();

		let object_type = object_type
			.unwrap_or_else(|| match &object {
				Some(object) => object.type_(),
				None => Type::INVALID,
			})
			.into_glib();
		let object = object.as_ref().map(|object| object.to_glib_none());
		let object = object.as_ref().map(|object| object.0).unwrap_or(ptr::null()) as *const _;

		unsafe {
			match () {
				#[cfg(feature = "v0_5_3")]
				() => {
					let topic = topic.map(|topic| topic.ffi());
					let topic = topic.map(|topic| topic as *const _).unwrap_or(ptr::null());
					ffi::wp_logt_checked(
						topic,
						level,
						file.0,
						line.0,
						function.0,
						object_type,
						object,
						format,
						message,
					)
				},
				#[cfg(not(feature = "v0_5_3"))]
				() => {
					let topic_name = topic.map(|topic| topic.name());
					let topic_name = topic_name.to_glib_none();
					ffi::wp_log_checked(
						topic_name.0,
						level,
						file.0,
						line.0,
						function.0,
						object_type,
						object,
						format,
						message,
					)
				},
			}
		}
	}

	#[doc(alias = "wp_logt_checked")]
	#[doc(alias = "wp_log_checked")]
	pub fn log_string<L: Into<LogLevelFlags>, O: AsRef<GObject>, M: AsRef<GStr>>(
		topic: Option<&LogTopic>,
		level: L,
		context: LogCallerContext,
		object: Option<O>,
		message: M,
	) {
		let level = level.into();
		let object = object.as_ref().map(|object| object.as_ref().to_glib_none());
		let object = object.as_ref().map(|object| unsafe { BorrowedObject::new(object.0) });
		let line = context.line();
		let line = line.as_ref().map(|line| line.as_ref());
		Self::log_string_checked(
			topic,
			level,
			context.file,
			line,
			context.function,
			None,
			object,
			message.as_ref(),
		)
	}

	pub fn log_args<O: AsRef<GObject>, L: Into<LogLevelFlags>>(
		topic: Option<&LogTopic>,
		log_level: L,
		context: StructuredLogContext<O>,
		args: fmt::Arguments,
	) {
		let mut message = GStringBuilder::default();
		let _ = write!(message, "{args}");
		Self::log_string(topic, log_level, context.caller, context.object, message.into_string())
	}

	#[doc(alias = "wp_log_writer_default")]
	pub unsafe fn writer_default<L: Into<LogLevelFlags>>(
		log_levels: L,
		fields: &[glib::ffi::GLogField],
		user_data: gpointer,
	) -> glib::ffi::GLogWriterOutput {
		ffi::wp_log_writer_default(log_levels.into().into_glib(), fields.as_ptr(), fields.len(), user_data)
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct LogCallerContext<'a> {
	pub file: Option<&'a GStr>,
	pub line: Option<Result<&'a GStr, u32>>,
	pub function: Option<&'a GStr>,
}

impl<'a> LogCallerContext<'a> {
	pub fn line(&self) -> Option<Cow<'a, GStr>> {
		match self.line {
			None => None,
			Some(Ok(line)) => Some(Cow::Borrowed(line)),
			Some(Err(line)) => Some(Cow::Owned(gformat!("{}", line))),
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct StructuredLogContext<'a, 'o, O = GObject> {
	pub caller: LogCallerContext<'a>,
	pub object: Option<&'o O>,
	pub object_type: Option<Type>,
}

#[repr(transparent)]
pub struct LogTopic<'n> {
	inner: WpLogTopic,
	_pinned: PhantomPinned,
	_name: PhantomData<&'n GStr>,
}

impl<'n> LogTopic<'n> {
	pub const fn new(name: &'n GStr) -> Self {
		unsafe { Self::with_flags(name, LogTopicFlags::empty()) }
	}

	pub const unsafe fn with_flags(name: &'n GStr, flags: LogTopicFlags) -> Self {
		Self {
			inner: WpLogTopic {
				topic_name: name.as_ptr(),
				flags: flags.bits(),
				_wp_padding: [ptr::null_mut(); 3],
			},
			_pinned: PhantomPinned,
			_name: PhantomData,
		}
	}

	#[inline]
	pub fn register(self) -> Pin<Box<Self>> {
		let mut this = Box::pin(self);
		unsafe { this.as_mut().get_unchecked_mut().register_unchecked() }
		this
	}

	#[inline]
	pub fn unregister(self: Pin<Box<Self>>) {
		drop(self);
	}

	#[inline]
	pub unsafe fn register_unchecked(&mut self) {
		ffi::wp_log_topic_register(self.ffi_mut())
	}

	#[inline]
	pub fn unregister_unchecked(&mut self) {
		unsafe { ffi::wp_log_topic_unregister(self.ffi_mut()) }
	}

	fn unregister_checked(&mut self) {
		if self.flags().contains(LogTopicFlags::FLAG_INITIALIZED) {
			self.unregister_unchecked()
		}
	}

	#[inline]
	pub fn ffi(&self) -> &WpLogTopic {
		&self.inner
	}

	#[inline]
	pub unsafe fn ffi_mut(&mut self) -> &mut WpLogTopic {
		&mut self.inner
	}

	#[inline]
	pub fn name(&self) -> &'n GStr {
		unsafe { GStr::from_ptr(self.inner.topic_name) }
	}

	#[inline]
	pub unsafe fn flags_mut(&mut self) -> &mut LogTopicFlags {
		transmute(&mut self.inner.flags)
	}

	#[inline]
	pub fn flags(&self) -> &LogTopicFlags {
		unsafe { transmute(&self.inner.flags) }
	}

	pub fn is_enabled<L: Into<LogLevelFlags>>(&self, level: L) -> bool {
		let flags = self.flags();
		// if flags.contains(LogTopicFlags::FLAG_INITIALIZED) { return false }
		flags.levels().contains(level.into())
	}

	/// Initializes the topic [level flags](LogTopicFlags::levels).
	///
	/// If the topic has [static storage lifetime](LogTopicFlags::FLAG_STATIC),
	/// it will be [registered](Self::register).
	///
	/// Internal function, don't use it directly.
	/// Has no effect if topic has already been initialized.
	#[doc(alias = "wp_log_topic_init")]
	pub unsafe fn init(&mut self) {
		ffi::wp_log_topic_init(self.ffi_mut())
	}
}

impl LogTopic<'static> {
	pub fn mark_static(&'static mut self) {
		unsafe {
			self.flags_mut().insert(LogTopicFlags::FLAG_STATIC);
		}
	}
}

impl<'t> AsRef<Self> for LogTopic<'t> {
	#[inline]
	fn as_ref(&self) -> &Self {
		self
	}
}

impl<'t> fmt::Debug for LogTopic<'t> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("LogTopic")
			.field("name", &self.name())
			.field("flags", &self.flags())
			.finish()
	}
}

impl<'a> Drop for LogTopic<'a> {
	fn drop(&mut self) {
		self.unregister_checked();
	}
}

/// A [LogTopic] meant to be used with the [log_topic] macro.
pub struct LogTopicStorage<'t> {
	topic: Cell<LogTopic<'t>>,
	allocated_name: bool,
}

impl<'t> LogTopicStorage<'t> {
	pub const fn with_name(name: &'t GStr) -> Self {
		Self {
			topic: Cell::new(LogTopic::new(name)),
			allocated_name: false,
		}
	}

	pub fn new<N: Into<GString>>(name: N) -> Self {
		let name = name.into();
		Self {
			topic: Cell::new(LogTopic::new(unsafe { transmute(name.as_gstr()) })),
			allocated_name: true,
		}
	}

	#[inline]
	pub fn topic(&self) -> &LogTopic<'t> {
		let topic = self.topic.as_ptr() as *const LogTopic<'t>;
		unsafe { &*topic }
	}

	#[inline]
	pub fn topic_mut_unchecked(&self) -> &mut LogTopic<'t> {
		let topic = self.topic.as_ptr();
		unsafe { &mut *topic }
	}

	#[inline]
	pub fn topic_mut(&mut self) -> &mut LogTopic<'t> {
		self.topic.get_mut()
	}

	pub fn is_enabled<L: Into<LogLevelFlags>>(&self, level: L) -> bool {
		if !self.flags().contains(LogTopicFlags::FLAG_INITIALIZED) {
			unsafe {
				self.init_unchecked();
			}
		}

		self.topic().is_enabled(level)
	}

	pub fn mark_static(&'static self) {
		self.topic_mut_unchecked().mark_static();
	}

	pub unsafe fn init_unchecked(&self) {
		self.topic_mut_unchecked().init()
	}
}

impl LogTopicStorage<'static> {
	pub const unsafe fn new_static(name: &'static GStr) -> Self {
		Self {
			topic: Cell::new(LogTopic::with_flags(name, LogTopicFlags::FLAG_STATIC)),
			allocated_name: false,
		}
	}
}

impl<'t> Deref for LogTopicStorage<'t> {
	type Target = LogTopic<'t>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.topic()
	}
}

impl<'t> AsRef<LogTopic<'t>> for LogTopicStorage<'t> {
	#[inline]
	fn as_ref(&self) -> &LogTopic<'t> {
		self.topic()
	}
}

impl<'t> fmt::Debug for LogTopicStorage<'t> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("LogTopicStorage")
			.field("topic", self.topic())
			.field("allocated_name", &self.allocated_name)
			.finish()
	}
}

impl<'t> Drop for LogTopicStorage<'t> {
	fn drop(&mut self) {
		if self.allocated_name {
			self.allocated_name = false;
			let topic = self.topic_mut();
			topic.unregister_checked();
			drop(unsafe {
				let topic_name = mem::replace(&mut topic.ffi_mut().topic_name, ptr::null());
				GString::from_glib_full(topic_name)
			});
		}
	}
}

unsafe impl<'t> Sync for LogTopicStorage<'t> {}

impl LogTopicFlags {
	pub fn with_levels<L: Into<LogLevelFlags>>(levels: L) -> Self {
		Self::from_bits_truncate(levels.into().bits() & Self::LEVEL_MASK.bits())
	}

	pub fn levels(&self) -> LogLevelFlags {
		LogLevelFlags::from_bits_truncate(self.bits() & Self::LEVEL_MASK.bits())
	}
}

impl From<LogLevelFlags> for LogTopicFlags {
	fn from(levels: LogLevelFlags) -> Self {
		LogTopicFlags::with_levels(levels)
	}
}
