macro_rules! define_log_variant {
	({$dollar:tt} $($name:ident($int_name:ident) = $level:ident,)*) => {
		$(
			#[macro_export]
			macro_rules! $name {
				(@$dollar self:expr, $dollar($dollar args:tt)*) => {
					$crate::log::$name! { self:$dollar self, $dollar($dollar args)* }
				};
				(self: $dollar self:expr, domain: $dollar log_domain:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { self:$dollar self, domain: $dollar log_domain, $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
				(self: $dollar self:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { self:$dollar self, $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
				($dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
				(domain: $dollar log_domain:expr, $dollar format:literal $dollar($dollar args:tt)*) => {
					$crate::log::log! { domain: $dollar log_domain, $crate::lib::glib::LogLevelFlags::$level, $dollar format $dollar($dollar args)* }
				};
			}
			pub use $name;

			#[allow(unused_macros)]
			macro_rules! $int_name {
				(@$dollar self:expr, $dollar($dollar args:tt)*) => {
					$crate::log::$int_name! { self:$dollar self, $dollar($dollar args)* }
				};
				(self: $dollar self:expr, $dollar($dollar args:tt)*) => {
					$crate::log::$name! { self:$dollar self, domain: $dollar crate::Log::domain(), $dollar($dollar args)* }
				};
				($dollar($dollar args:tt)*) => {
					$crate::log::$name! { domain: $dollar crate::Log::domain(), $dollar($dollar args)* }
				};
			}
			#[allow(unused_imports)]
			pub(crate) use $int_name;
		)*
	};
}

define_log_variant! { {$}
	critical(wp_critical) = LEVEL_CRITICAL,
	warning(wp_warning) = LEVEL_WARNING,
	message(wp_message) = LEVEL_MESSAGE,
	info(wp_info) = LEVEL_INFO,
	debug(wp_debug) = LEVEL_DEBUG,
	trace(wp_trace) = LEVEL_TRACE,
}

#[macro_export]
macro_rules! log {
	(self: $self:expr, domain: $log_domain:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { $log_domain.into(), $log_level, None, Some($self), $format; $($args)* }
	};
	(self: $self:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { None, $log_level, None, Some($self), $format; $($args)* }
	};
	(domain: $log_domain:expr, $log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { $log_domain.into(), $log_level, None, None::<&$crate::lib::glib::Object>, $format; $($args)* }
	};
	($log_level:expr, $format:literal $($args:tt)*) => {
		$crate::log::_log_inner! { None, $log_level, None, None::<&$crate::lib::glib::Object>, $format; $($args)* }
	};
}
pub use log;

#[doc(hidden)]
#[macro_export]
macro_rules! _log_inner {
	($log_domain:expr, $log_level:expr, $obj_type:expr, $obj:expr, $fmt:expr; $($args:tt)*) => {
		{
			let log_level = $log_level;
			if $crate::log::Log::level_is_enabled(log_level) {
				let object = $obj;
				let log_context = $crate::log::StructuredLogContext {
					domain: $log_domain,
					file: Some(file!()),
					line: Some(line!()),
					function: None, // TODO: this is possible: https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
					object_type: None, // TODO: consider getting the static type from object
					object: object,
				};
				$crate::log::Log::log_args(log_level, log_context, format_args!($fmt $($args)*))
			}
		}
	};
}
pub use _log_inner;
