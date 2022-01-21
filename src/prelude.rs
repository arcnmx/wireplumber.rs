//! Extension traits
//!
//! Wildcard imports give you access to all the goodies:
//!
//! ```
//! use wireplumber::prelude::*;
//! use wireplumber::{PipewireObject, pw};
//!
//! fn get_id(obj: &PipewireObject) -> u32 {
//!   obj.pw_property(pw::PW_KEY_OBJECT_ID)
//!     .expect("how do you not know who you are")
//! }
//! ```

pub use crate::session::{
	SessionItemExt as _,
	SiAcquisitionExt as _, SiAcquisitionExt2 as _,
	SiAdapterExt as _, SiAdapterExt2 as _,
	SiEndpointExt as _,
	SiFactoryExt as _,
	SiLinkExt as _,
	SiLinkableExt as _,
};

pub use crate::pw::{
	PipewirePropertyStringIterExt as _,
	EndpointExt as _,
	GlobalProxyExt as _,
	MetadataExt as _,
	PipewireObjectExt as _, PipewireObjectExt2 as _,
	ProxyExt as _, ProxyExt2 as _,
};

pub use crate::plugin::PluginExt as _;

pub use crate::auto::traits::{
	ObjectExt as _,
	TransitionExt as _,
};

pub use crate::TransitionExt2 as _;

pub use glib_signal::ObjectSignalExt as _;
pub use glib::{
	Cast as _,
	IsA as _,
	StaticType as _,
};
