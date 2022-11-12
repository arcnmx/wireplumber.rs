pub use pipewire_sys as ffi;
use {
	crate::{
		prelude::*,
		spa::{SpaIdTable, SpaIdValue},
	},
	::ffi::WpSpaType,
};

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PipewireKey(&'static [u8]);

impl PipewireKey {
	#[inline]
	pub fn as_cstr(&self) -> &'static CStr {
		unsafe { CStr::from_bytes_with_nul_unchecked(self.0) }
	}

	#[inline]
	pub fn as_str<'a>(&self) -> &'a str {
		unsafe { str::from_utf8_unchecked(self.as_cstr().to_bytes()) }
	}
}

impl Deref for PipewireKey {
	type Target = str;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl AsRef<str> for PipewireKey {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<'a> Into<&'a str> for PipewireKey {
	fn into(self) -> &'a str {
		self.as_str()
	}
}

impl<'a, 'b> Into<&'a str> for &'b PipewireKey {
	fn into(self) -> &'a str {
		self.as_str()
	}
}

impl Into<String> for PipewireKey {
	fn into(self) -> String {
		self.as_str().into()
	}
}

impl<'a> Into<String> for &'a PipewireKey {
	fn into(self) -> String {
		self.as_str().into()
	}
}

pub trait FromPipewirePropertyString: Sized {
	type Error: Debug;

	fn from_pipewire_string(value: &str) -> Result<Self, Self::Error>;
}

pub trait ToPipewirePropertyString {
	type Output: AsRef<str>;

	fn pipewire_string(self) -> Self::Output;
}

pub struct PipewirePropertyStringIter<T>(pub T);

pub trait PipewirePropertyStringIterExt: Sized {
	fn pipewire_string_iter(self) -> PipewirePropertyStringIter<Self> {
		PipewirePropertyStringIter(self)
	}
}

impl<T: IntoIterator<Item = I>, I: ToPipewirePropertyString> PipewirePropertyStringIterExt
	for PipewirePropertyStringIter<T>
{
}

impl<T: IntoIterator<Item = I>, I: ToPipewirePropertyString> ToPipewirePropertyString
	for PipewirePropertyStringIter<T>
{
	type Output = String;

	fn pipewire_string(self) -> Self::Output {
		let mut out = String::new();
		for (i, v) in self.0.into_iter().enumerate() {
			let prefix = if i == 0 { "" } else { "," };
			let item = v.pipewire_string();
			let _ = write!(&mut out, "{}{}", prefix, item.as_ref());
		}
		out
	}
}

impl<T: ToPipewirePropertyString> ToPipewirePropertyString for Vec<T> {
	type Output = String;

	fn pipewire_string(self) -> Self::Output {
		PipewirePropertyStringIter(self).pipewire_string()
	}
}

impl<'a, T> ToPipewirePropertyString for &'a Vec<T>
where
	&'a T: ToPipewirePropertyString,
{
	type Output = String;

	fn pipewire_string(self) -> Self::Output {
		PipewirePropertyStringIter(self.iter()).pipewire_string()
	}
}

impl<'a, T> ToPipewirePropertyString for &'a [T]
where
	&'a T: ToPipewirePropertyString,
{
	type Output = String;

	fn pipewire_string(self) -> Self::Output {
		PipewirePropertyStringIter(self).pipewire_string()
	}
}

impl ToPipewirePropertyString for bool {
	type Output = &'static str;

	fn pipewire_string(self) -> Self::Output {
		if self {
			"true"
		} else {
			"false"
		}
	}
}

macro_rules! pipewire_primitives {
	($($ty:ty,)*) => {
		$(
			impl ToPipewirePropertyString for $ty {
				type Output = String;

				fn pipewire_string(self) -> Self::Output {
					self.to_string()
				}
			}
			impl FromPipewirePropertyString for $ty {
				type Error = <$ty as FromStr>::Err;

				fn from_pipewire_string(value: &str) -> Result<Self, Self::Error> {
					FromStr::from_str(value)
				}
			}
		)*
	};
	(@AsRef $($ty:ty,)*) => {
		$(
			impl ToPipewirePropertyString for $ty {
				type Output = Self;

				fn pipewire_string(self) -> Self::Output {
					self
				}
			}
			impl FromPipewirePropertyString for $ty {
				type Error = Infallible;

				fn from_pipewire_string(value: &str) -> Result<Self, Self::Error> {
					Ok(value.into())
				}
			}
		)*
	};
	(@&AsRef $($ty:ty,)*) => {
		$(
			impl<'a> ToPipewirePropertyString for &'a $ty {
				type Output = Self;

				fn pipewire_string(self) -> Self::Output {
					self
				}
			}
		)*
	};
}

pipewire_primitives! {
	u8, i8,
	u16, i16,
	u32, i32,
	u64, i64,
	f32, f64,
	usize, isize,
}
pipewire_primitives! { @AsRef
	String, glib::GString,
}
pipewire_primitives! { @&AsRef
	str,
}

macro_rules! pipewire_keys {
	($($(#[$attr:meta])*$key:ident,)+) => {
		$(
			$(
				#[$attr]
			)*
			pub const $key: &'static PipewireKey = &PipewireKey(&*pipewire_sys::$key);
		)+
	};
}

pipewire_keys! {
	PW_KEY_PROTOCOL,
	PW_KEY_ACCESS,
	PW_KEY_CLIENT_ACCESS,
	PW_KEY_SEC_PID, PW_KEY_SEC_UID, PW_KEY_SEC_GID, PW_KEY_SEC_LABEL,
	PW_KEY_LIBRARY_NAME_SYSTEM, PW_KEY_LIBRARY_NAME_LOOP, PW_KEY_LIBRARY_NAME_DBUS,
	PW_KEY_OBJECT_PATH, PW_KEY_OBJECT_ID, #[cfg(pw = "0.3.41")] PW_KEY_OBJECT_SERIAL, #[cfg(pw = "0.3.32")] PW_KEY_OBJECT_LINGER, #[cfg(pw = "0.3.32")] PW_KEY_OBJECT_REGISTER,
	#[cfg(pw = "0.3.22")] PW_KEY_CONFIG_PREFIX, #[cfg(pw = "0.3.22")] PW_KEY_CONFIG_NAME,
	#[cfg(pw = "0.3.22")]PW_KEY_CONTEXT_PROFILE_MODULES,
	#[cfg(pw = "0.3.7")] PW_KEY_USER_NAME,
	#[cfg(pw = "0.3.7")] PW_KEY_HOST_NAME,
	#[cfg(pw = "0.3.7")] PW_KEY_CORE_NAME, #[cfg(pw = "0.3.7")] PW_KEY_CORE_VERSION, #[cfg(pw = "0.3.7")] PW_KEY_CORE_DAEMON, PW_KEY_CORE_ID, PW_KEY_CORE_MONITORS,
	PW_KEY_CPU_MAX_ALIGN, PW_KEY_CPU_CORES,
	PW_KEY_PRIORITY_SESSION, #[cfg(pw = "0.3.10")] PW_KEY_PRIORITY_DRIVER, PW_KEY_REMOTE_NAME, PW_KEY_REMOTE_INTENTION,
	PW_KEY_APP_NAME, PW_KEY_APP_ID, PW_KEY_APP_VERSION, PW_KEY_APP_ICON, PW_KEY_APP_ICON_NAME, PW_KEY_APP_LANGUAGE, PW_KEY_APP_PROCESS_ID, PW_KEY_APP_PROCESS_BINARY, PW_KEY_APP_PROCESS_USER, PW_KEY_APP_PROCESS_HOST, PW_KEY_APP_PROCESS_MACHINE_ID, PW_KEY_APP_PROCESS_SESSION_ID,
	PW_KEY_WINDOW_X11_DISPLAY,
	PW_KEY_CLIENT_ID, PW_KEY_CLIENT_NAME, PW_KEY_CLIENT_API,
	PW_KEY_NODE_ID, PW_KEY_NODE_NAME, PW_KEY_NODE_NICK, PW_KEY_NODE_DESCRIPTION, PW_KEY_NODE_PLUGGED, PW_KEY_NODE_SESSION, #[cfg(pw = "0.3.10")] PW_KEY_NODE_GROUP, PW_KEY_NODE_EXCLUSIVE, PW_KEY_NODE_AUTOCONNECT, PW_KEY_NODE_TARGET, PW_KEY_NODE_LATENCY, #[cfg(pw = "0.3.23")] PW_KEY_NODE_MAX_LATENCY, #[cfg(pw = "0.3.33")] PW_KEY_NODE_LOCK_QUANTUM, #[cfg(pw = "0.3.33")] PW_KEY_NODE_RATE, #[cfg(pw = "0.3.33")] PW_KEY_NODE_LOCK_RATE, PW_KEY_NODE_DONT_RECONNECT, PW_KEY_NODE_ALWAYS_PROCESS, #[cfg(pw = "0.3.33")] PW_KEY_NODE_WANT_DRIVER, PW_KEY_NODE_PAUSE_ON_IDLE, #[cfg(pw = "0.3.18")] PW_KEY_NODE_CACHE_PARAMS, PW_KEY_NODE_DRIVER, PW_KEY_NODE_STREAM, #[cfg(pw = "0.3.26")] PW_KEY_NODE_VIRTUAL, #[cfg(pw = "0.3.27")] PW_KEY_NODE_PASSIVE, #[cfg(pw = "0.3.32")] PW_KEY_NODE_LINK_GROUP, #[cfg(pw = "0.3.39")] PW_KEY_NODE_NETWORK, #[cfg(pw = "0.3.41")] PW_KEY_NODE_TRIGGER,
	PW_KEY_PORT_ID, PW_KEY_PORT_NAME, PW_KEY_PORT_DIRECTION, PW_KEY_PORT_ALIAS, PW_KEY_PORT_PHYSICAL, PW_KEY_PORT_TERMINAL, PW_KEY_PORT_CONTROL, PW_KEY_PORT_MONITOR, #[cfg(pw = "0.3.18")] PW_KEY_PORT_CACHE_PARAMS, #[cfg(pw = "0.3.22")] PW_KEY_PORT_EXTRA,
	PW_KEY_LINK_ID, PW_KEY_LINK_INPUT_NODE, PW_KEY_LINK_INPUT_PORT, PW_KEY_LINK_OUTPUT_NODE, PW_KEY_LINK_OUTPUT_PORT, PW_KEY_LINK_PASSIVE, #[cfg(pw = "0.3.19")] PW_KEY_LINK_FEEDBACK,
	PW_KEY_DEVICE_ID, PW_KEY_DEVICE_NAME, PW_KEY_DEVICE_PLUGGED, PW_KEY_DEVICE_NICK, PW_KEY_DEVICE_STRING, PW_KEY_DEVICE_API, PW_KEY_DEVICE_DESCRIPTION, PW_KEY_DEVICE_BUS_PATH, PW_KEY_DEVICE_SERIAL, PW_KEY_DEVICE_VENDOR_ID, PW_KEY_DEVICE_VENDOR_NAME, PW_KEY_DEVICE_PRODUCT_ID, PW_KEY_DEVICE_PRODUCT_NAME, PW_KEY_DEVICE_CLASS, PW_KEY_DEVICE_FORM_FACTOR, PW_KEY_DEVICE_BUS, PW_KEY_DEVICE_SUBSYSTEM, PW_KEY_DEVICE_ICON, PW_KEY_DEVICE_ICON_NAME, PW_KEY_DEVICE_INTENDED_ROLES, #[cfg(pw = "0.3.18")] PW_KEY_DEVICE_CACHE_PARAMS,
	PW_KEY_MODULE_ID, PW_KEY_MODULE_NAME, PW_KEY_MODULE_AUTHOR, PW_KEY_MODULE_DESCRIPTION, PW_KEY_MODULE_USAGE, PW_KEY_MODULE_VERSION,
	PW_KEY_FACTORY_ID, PW_KEY_FACTORY_NAME, PW_KEY_FACTORY_USAGE, PW_KEY_FACTORY_TYPE_NAME, PW_KEY_FACTORY_TYPE_VERSION,
	PW_KEY_STREAM_IS_LIVE, PW_KEY_STREAM_LATENCY_MIN, PW_KEY_STREAM_LATENCY_MAX, PW_KEY_STREAM_MONITOR, #[cfg(pw = "0.3.8")] PW_KEY_STREAM_DONT_REMIX, #[cfg(pw = "0.3.14")] PW_KEY_STREAM_CAPTURE_SINK,
	PW_KEY_MEDIA_TYPE, PW_KEY_MEDIA_CATEGORY, PW_KEY_MEDIA_ROLE, PW_KEY_MEDIA_CLASS, PW_KEY_MEDIA_NAME, PW_KEY_MEDIA_TITLE, PW_KEY_MEDIA_ARTIST, PW_KEY_MEDIA_COPYRIGHT, PW_KEY_MEDIA_SOFTWARE, PW_KEY_MEDIA_LANGUAGE, PW_KEY_MEDIA_FILENAME, PW_KEY_MEDIA_ICON, PW_KEY_MEDIA_ICON_NAME, #[cfg(pw = "0.3.14")] PW_KEY_MEDIA_COMMENT, #[cfg(pw = "0.3.14")] PW_KEY_MEDIA_DATE, #[cfg(pw = "0.3.14")] PW_KEY_MEDIA_FORMAT,
	PW_KEY_FORMAT_DSP,
	PW_KEY_AUDIO_CHANNEL, PW_KEY_AUDIO_RATE, PW_KEY_AUDIO_CHANNELS, PW_KEY_AUDIO_FORMAT,
	PW_KEY_VIDEO_RATE, PW_KEY_VIDEO_FORMAT, PW_KEY_VIDEO_SIZE,
}

pub trait SpaPropertyKey: Debug {
	type Error: Debug;

	fn spa_property_key_with_table(&self, table: Option<SpaIdTable>) -> Result<WpSpaType, Self::Error>;
}

pub trait SpaPropertyKeyId: SpaPropertyKey<Error = Infallible> {
	fn spa_property_key(&self) -> WpSpaType;
}

impl<T: SpaPropertyKeyId> SpaPropertyKey for T {
	type Error = Infallible;

	fn spa_property_key_with_table(&self, _table: Option<SpaIdTable>) -> Result<WpSpaType, Self::Error> {
		Ok(self.spa_property_key())
	}
}

impl SpaPropertyKeyId for WpSpaType {
	fn spa_property_key(&self) -> WpSpaType {
		*self
	}
}

impl SpaPropertyKeyId for SpaIdValue {
	fn spa_property_key(&self) -> WpSpaType {
		self.number()
	}
}

impl SpaPropertyKey for str {
	// TODO
	type Error = ();

	fn spa_property_key_with_table(&self, table: Option<SpaIdTable>) -> Result<WpSpaType, Self::Error> {
		table
			.and_then(|table| table.find_value_from_short_name(self))
			.map(|v| v.number())
			.or_else(|| SpaIdValue::parse_unknown_name(self))
			.ok_or(())
	}
}

impl<'a, T: SpaPropertyKeyId> SpaPropertyKeyId for &'a T {
	fn spa_property_key(&self) -> WpSpaType {
		T::spa_property_key(*self)
	}
}
