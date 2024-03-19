macro_rules! define_log_variant {
	({$dollar:tt} $($name:ident($int_name:ident) = ($($level:tt)*),)*) => {
		$(
			#[macro_export]
			macro_rules! $name {
				(@$dollar self:expr, $dollar($dollar args:tt)*) => {
					$crate::log::$name! { self:$dollar self, $dollar($dollar args)* }
				};
				(self: $dollar self:expr, domain: $dollar log_domain:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { self:$dollar self, domain: $dollar log_domain, $($level)*, $dollar format $dollar($dollar args)* }
				};
				(self: $dollar self:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { self:$dollar self, $($level)*, $dollar format $dollar($dollar args)* }
				};
				($dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { $($level)*, $dollar format $dollar($dollar args)* }
				};
				(domain: $dollar log_domain:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { domain: $dollar log_domain, $($level)*, $dollar format $dollar($dollar args)* }
				};
			}
			pub use $name;

			#[allow(unused_macros)]
			macro_rules! $int_name {
				(@$dollar self:expr, $dollar($dollar args:tt)*) => {
					$crate::log::$int_name! { self:$dollar self, $dollar($dollar args)* }
				};
				(self: $dollar self:expr, $dollar($dollar args:tt)*) => {
					$crate::log::$name! { self:$dollar self, domain: $dollar crate::log::TOPIC_BINDINGS, $dollar($dollar args)* }
				};
				($dollar($dollar args:tt)*) => {
					$crate::log::$name! { domain: $dollar crate::log::TOPIC_BINDINGS, $dollar($dollar args)* }
				};
			}
			#[allow(unused_imports)]
			pub(crate) use $int_name;
		)*
	};
}

define_log_variant! { {$}
	critical(wp_critical) = ($crate::lib::glib::LogLevelFlags::LEVEL_CRITICAL),
	warning(wp_warning) = ($crate::lib::glib::LogLevelFlags::LEVEL_WARNING),
	message(wp_message) = ($crate::lib::glib::LogLevelFlags::LEVEL_MESSAGE),
	info(wp_info) = ($crate::lib::glib::LogLevelFlags::LEVEL_INFO),
	debug(wp_debug) = ($crate::lib::glib::LogLevelFlags::LEVEL_DEBUG),
	trace(wp_trace) = ($crate::Log::LEVEL_TRACE),
}

#[macro_export]
macro_rules! log {
	(self: $self:expr, domain: $log_domain:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { $log_domain, $log_level, None, Some($self), $format; $($args)* }
	};
	(self: $self:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { $crate::log::log_topic!(@FALLBACK), $log_level, None, Some($self), $format; $($args)* }
	};
	(domain: $log_domain:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { $log_domain, $log_level, None, None::<&$crate::lib::glib::Object>, $format; $($args)* }
	};
	($log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { $crate::log::log_topic!(@FALLBACK), $log_level, None, None::<&$crate::lib::glib::Object>, $format; $($args)* }
	};
}
pub use log;

#[macro_export]
macro_rules! log_topic {
	(@concat($($topic:tt)*)) => {
		$crate::log::LogTopicStorage::with_name(unsafe {
			$crate::lib::glib::GStr::from_str_with_nul_unchecked(
				concat!($($topic)*, "\0")
			)
		})
	};
	(@FALLBACK) => {
		// $crate::log::TOPIC_FALLBACK
		$crate::log::log_topic!(@concat(core::env!("CARGO_PKG_NAME")))
	};
	($name:literal) => {
		$crate::log::LogTopicStorage::with_name($crate::lib::glib::gstr!($name))
	};
	(
		$(#[$meta:meta])*
		$vis:vis static $id:ident = $name:literal
		$(;)?
	) => {
		$(#[$meta])*
		$vis static $id: $crate::log::LogTopicStorage<'static> = unsafe {
			$crate::log::LogTopicStorage::new_static($crate::lib::glib::gstr!($name))
		};
	};
}
pub use log_topic;

#[doc(hidden)]
#[macro_export]
macro_rules! _log_inner {
	($log_domain:expr, $log_level:expr, $obj_type:expr, $obj:expr, $fmt:expr; $($args:tt)*) => {
		{
			let log_topic = &$log_domain;
			let log_level = $log_level;
			if log_topic.is_enabled(log_level) {
				let log_topic: &$crate::log::LogTopic = log_topic.as_ref();
				let object = $obj;
				// TODO: allocation :<
				let file = $crate::lib::glib::GString::from(file!());
				let log_context = $crate::log::StructuredLogContext {
					caller: $crate::log::LogCallerContext {
						file: Some(file.as_gstr()),
						line: Some(Err(line!())),
						function: None, // TODO: this is possible: https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
					},
					object_type: None, // TODO: consider getting the static type from object
					object: object,
				};
				$crate::log::Log::log_args(Some(log_topic), log_level, log_context, format_args!($fmt $($args)*))
			}
		}
	};
}
pub use _log_inner;

#[cfg(test)]
mod tests {
	use {
		crate::{Core, InitFlags, Log},
		glib::gstr,
	};

	#[test]
	fn log() {
		Core::init_with_flags(InitFlags::SET_GLIB_LOG);
		Log::set_level(gstr!("8"));
		wp_critical!("internal crit");
		wp_warning!("internal warn");
		wp_message!("internal message");
		wp_info!("internal info");
		wp_debug!("internal debug");
		wp_trace!("internal trace");

		critical!("external crit");
		warning!("external warn");
		message!("external message");
		info!("external info");
		debug!("external debug");
		trace!("external trace");
	}
}
