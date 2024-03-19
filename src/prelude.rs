//! Extension traits
//!
//! Wildcard imports give you access to all the goodies:
//!
//! ```
//! use wireplumber::prelude::*;
//! use wireplumber::pw::{self, PipewireObject};
//!
//! fn get_id(obj: &PipewireObject) -> u32 {
//!   obj.pw_property(pw::PW_KEY_OBJECT_ID)
//!     .expect("how do you not know who you are")
//! }
//! ```

#[cfg(feature = "glib-signal")]
#[cfg_attr(docsrs, doc(cfg(feature = "glib-signal")))]
#[doc(no_inline)]
pub use glib_signal::ObjectSignalExt as _;
#[doc(no_inline)]
pub use {
	crate::{
		core::Core,
		core::ObjectExt as _,
		core::ObjectExt2 as _,
		event::EventHookExt as _,
		log::log_topic,
		plugin::{AsyncPluginExt as _, PluginExt as _},
		pw::{
			GlobalProxyExt as _, MetadataExt as _, PipewireObjectExt as _, PipewireObjectExt2 as _,
			PipewirePropertyStringIterExt as _, ProxyExt as _, ProxyExt2 as _,
		},
		registry::InterestEventHookExt as _,
		session::{
			SessionItemExt as _, SiAcquisitionExt as _, SiAcquisitionExt2 as _, SiAdapterExt as _, SiAdapterExt2 as _,
			SiFactoryExt as _, SiLinkExt as _, SiLinkableExt as _,
		},
		util::{TransitionExt as _, TransitionExt2 as _},
	},
	glib::{
		object::{Cast as _, IsA as _},
		types::StaticType as _,
	},
};
/// this crate uses the prelude too!
#[allow(unused_imports)]
pub(crate) use {
	crate::{
		error::{Error, LibraryErrorEnum},
		log::{wp_critical, wp_debug, wp_info, wp_message, wp_trace, wp_warning},
		util::{IntoValueIterator, ValueIterator, WpIterator},
	},
	glib::{
		error::ErrorDomain,
		ffi::{gconstpointer, gpointer},
		gstr,
		object::{BorrowedObject, Cast, IsA, Object as GObject, ObjectExt as GObjectExt, ObjectType},
		translate::*,
		types::{Pointee, Pointer, StaticType},
		value::{FromValue, ToValue},
		variant::{FromVariant, StaticVariantType, ToVariant},
		GStr, GString, Type, Value, Variant, VariantClass, VariantTy,
	},
	std::{
		borrow::{Borrow, Cow},
		convert::{Infallible, TryFrom, TryInto},
		ffi::{CStr, CString},
		fmt::{self, Debug, Display, Write as _},
		future::Future,
		iter::{self, FromIterator},
		marker::PhantomData,
		mem,
		ops::{Deref, DerefMut},
		pin::Pin,
		ptr::{self, NonNull},
		slice,
		str::{self, FromStr},
	},
};
