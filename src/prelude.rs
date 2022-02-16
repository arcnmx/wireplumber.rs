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

#[doc(no_inline)]
pub use crate::session::{
	SessionItemExt as _,
	SiAcquisitionExt as _, SiAcquisitionExt2 as _,
	SiAdapterExt as _, SiAdapterExt2 as _,
	SiEndpointExt as _,
	SiFactoryExt as _,
	SiLinkExt as _,
	SiLinkableExt as _,
};

#[doc(no_inline)]
pub use crate::pw::{
	PipewirePropertyStringIterExt as _,
	EndpointExt as _,
	GlobalProxyExt as _,
	MetadataExt as _,
	PipewireObjectExt as _, PipewireObjectExt2 as _,
	ProxyExt as _, ProxyExt2 as _,
};

#[doc(no_inline)]
pub use crate::plugin::{
	PluginExt as _,
	AsyncPluginExt as _,
};

#[doc(no_inline)]
pub use crate::core::ObjectExt as _;

#[doc(no_inline)]
pub use crate::util::{
	TransitionExt as _, TransitionExt2 as _,
};

#[doc(no_inline)]
pub use glib_signal::ObjectSignalExt as _;
#[doc(no_inline)]
pub use glib::{
	Cast as _,
	IsA as _,
	StaticType as _,
};

/// this crate uses the prelude too!
#[allow(unused_imports)]
pub(crate) use crate::{
	error::{LibraryErrorEnum, Error},
	util::{ValueIterator, WpIterator},
	log::{
		wp_trace, wp_debug, wp_message, wp_info, wp_warning, wp_critical,
	},
};
pub(crate) use glib::{
	Cast, IsA, ObjectType, StaticType,
	Object as GObject, ObjectExt as GObjectExt,
	error::ErrorDomain,
	translate::*,
	ffi::{gpointer, gconstpointer},
	types::{Pointer, Pointee},
	Type,
	Value, value::FromValue,
	Variant, ToVariant, FromVariant, StaticVariantType, VariantClass, VariantTy,
};
pub(crate) use std::{
	iter::{self, FromIterator},
	marker::PhantomData,
	future::Future,
	convert::{TryFrom, TryInto, Infallible},
	borrow::{Cow, Borrow},
	fmt::{self, Debug, Display, Write as _},
	str::{self, FromStr},
	ops::Deref,
	ffi::{CStr, CString},
	ptr::{self, NonNull},
	mem, slice,
	pin::Pin,
};
