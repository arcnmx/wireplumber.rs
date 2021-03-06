//! [glib_signal] types
//!
//! These should not need to be used directly. Instead, use an associated const:
//!
//! ```
//! # #[cfg(feature = "enable-futures")]
//! use futures_util::StreamExt;
//! use wireplumber::{
//!   prelude::*,
//!   registry::{ObjectManager, Interest},
//!   core::{Core, Object, ObjectFeatures},
//!   pw::Node,
//! };
//!
//! async fn watch_nodes(core: &Core) {
//!   let om = ObjectManager::new();
//!   om.add_interest_full(&Interest::<Node>::new());
//!
//!   // register a callback for a signal...
//!   om.handle(ObjectManager::SIGNAL_INSTALLED, |om, ()| {
//!     println!("{:?} installed", om);
//!   });
//!   // ... or receive events as an async Stream:
//!   # #[cfg(feature = "enable-futures")]
//!   let mut objects = om.signal_stream(ObjectManager::SIGNAL_OBJECT_ADDED);
//!
//!   om.request_object_features(Object::static_type(), ObjectFeatures::ALL);
//!   core.install_object_manager(&om);
//!
//!   # #[cfg(feature = "enable-futures")]
//!   while let Some((obj,)) = objects.next().await {
//!     let node = obj.dynamic_cast_ref::<Node>()
//!       .expect("we're only interested in nodes");
//!     println!("new object: {:?}", node);
//!   }
//! }
//! ```

use glib_signal::{def_signal, SignalFlags, Pointer};
use crate::prelude::*;

def_signal! {
	impl Notifies<"connected" as Connected> for crate::core::Core {
		impl {const SIGNAL_CONNECTED};
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"disconnected" as Disconnected> for crate::core::Core {
		impl {const SIGNAL_DISCONNECTED};
		fn(&self)
	}
}

def_signal! {
	impl Notifies<"installed" as Installed> for crate::registry::ObjectManager {
		impl {const SIGNAL_INSTALLED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"objects-changed" as ObjectsChanged> for crate::registry::ObjectManager {
		impl {const SIGNAL_OBJECTS_CHANGED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"object-added" as ObjectAdded> for crate::registry::ObjectManager {
		impl {const SIGNAL_OBJECT_ADDED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, GObject)
	}
}
def_signal! {
	impl Notifies<"object-removed" as ObjectRemoved> for crate::registry::ObjectManager {
		impl {const SIGNAL_OBJECT_REMOVED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, GObject)
	}
}

def_signal! {
	impl Notifies<"bound" as Bound> for crate::pw::Proxy {
		impl {const SIGNAL_BOUND};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, u32)
	}
}
def_signal! {
	impl Notifies<"pw-proxy-created" as PwProxyCreated> for crate::pw::Proxy {
		impl {const SIGNAL_PROXY_CREATED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, Pointer<pipewire_sys::pw_proxy>)
	}
}
def_signal! {
	impl Notifies<"pw-proxy-destroyed" as PwProxyDestroyed> for crate::pw::Proxy {
		impl {const SIGNAL_PROXY_DESTROYED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, Pointer<pipewire_sys::pw_proxy>)
	}
}
def_signal! {
	impl Notifies<"error" as Error> for crate::pw::Proxy {
		impl {const SIGNAL_ERROR};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, i32, i32, String)
	}
}

def_signal! {
	impl Notifies<"params-changed" as ParamsChanged> for crate::pw::PipewireObject {
		impl {const SIGNAL_PARAMS_CHANGED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, String)
	}
}

def_signal! {
	impl Notifies<"ports-changed" as PortsChanged> for crate::pw::Node {
		impl {const SIGNAL_PORTS_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"state-changed" as StateChanged> for crate::pw::Node {
		impl {const SIGNAL_STATE_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, crate::pw::NodeState, crate::pw::NodeState)
	}
}

def_signal! {
	impl Notifies<"changed" as Changed> for crate::pw::Metadata {
		impl {const SIGNAL_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, u32, String, String, String)
	}
}

def_signal! {
	impl Notifies<"create-object" as CreateObject> for crate::local::SpaDevice {
		impl {const SIGNAL_CREATE_OBJECT};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, u32, String, String, crate::pw::Properties)
	}
}
def_signal! {
	impl Notifies<"object-removed" as SpaObjectRemoved> for crate::local::SpaDevice {
		impl {const SIGNAL_OBJECT_REMOVED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, u32)
	}
}

def_signal! {
	impl Notifies<"endpoint-properties-changed" as EndpointPropertiesChanged> for crate::session::SiEndpoint {
		impl {const SIGNAL_ENDPOINT_PROPERTIES_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self)
	}
}
