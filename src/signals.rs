//! [glib_signal] types
//!
//! These should not need to be used directly. Instead, use an associated const:
//!
//! ```
//! # #[cfg(feature = "futures")]
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
//!   om.add_interest(Interest::<Node>::new());
//!
//!   // register a callback for a signal...
//!   om.handle(ObjectManager::SIGNAL_INSTALLED, |om, ()| {
//!     println!("{om:?} installed");
//!   });
//!   // ... or receive events as an async Stream:
//!   # #[cfg(feature = "futures")]
//!   let mut objects = om.signal_stream(ObjectManager::SIGNAL_OBJECT_ADDED);
//!
//!   om.request_object_features(Object::static_type(), ObjectFeatures::ALL);
//!   core.install_object_manager(&om);
//!
//!   # #[cfg(feature = "futures")]
//!   while let Some((obj,)) = objects.next().await {
//!     let node = obj.dynamic_cast_ref::<Node>()
//!       .expect("we're only interested in nodes");
//!     println!("new object: {node:?}");
//!   }
//! }
//! ```

use {
	crate::prelude::*,
	glib_signal::{def_signal, Pointer, SignalFlags},
};

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
	impl Notifies<"installed" as ManagerInstalled> for crate::registry::ObjectManager {
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
	impl Notifies<"bound" as PwProxyBound> for crate::pw::Proxy {
		impl {const SIGNAL_BOUND};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, u32)
	}
}
def_signal! {
	impl Notifies<"pw-proxy-created" as PwProxyCreated> for crate::pw::Proxy {
		impl {const SIGNAL_PW_PROXY_CREATED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, Pointer<pipewire_sys::pw_proxy>)
	}
}
def_signal! {
	impl Notifies<"pw-proxy-destroyed" as PwProxyDestroyed> for crate::pw::Proxy {
		impl {const SIGNAL_PW_PROXY_DESTROYED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, Pointer<pipewire_sys::pw_proxy>)
	}
}
def_signal! {
	impl Notifies<"error" as PwProxyError> for crate::pw::Proxy {
		impl {const SIGNAL_ERROR};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, i32, i32, String)
	}
}

def_signal! {
	impl Notifies<"params-changed" as PwObjectParamsChanged> for crate::pw::PipewireObject {
		impl {const SIGNAL_PARAMS_CHANGED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, String)
	}
}

def_signal! {
	impl Notifies<"ports-changed" as NodePortsChanged> for crate::pw::Node {
		impl {const SIGNAL_PORTS_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"state-changed" as NodeStateChanged> for crate::pw::Node {
		impl {const SIGNAL_STATE_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, crate::pw::NodeState, crate::pw::NodeState)
	}
}

#[cfg(feature = "v0_4_11")]
def_signal! {
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_11")))]
	impl Notifies<"state-changed" as LinkStateChanged> for crate::pw::Link {
		impl {const SIGNAL_STATE_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, crate::pw::LinkState, crate::pw::LinkState)
	}
}

def_signal! {
	impl Notifies<"changed" as MetadataChanged> for crate::pw::Metadata {
		impl {const SIGNAL_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, u32, String, Option<String>, Option<String>)
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

#[cfg(feature = "v0_4_10")]
def_signal! {
	#[cfg_attr(docsrs, doc(cfg(feature = "v0_4_10")))]
	impl Notifies<"adapter-ports-state-changed" as AdapterPortsStateChanged> for crate::session::SiAdapter {
		impl {const SIGNAL_ADAPTER_PORTS_STATE_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, crate::session::SiAdapterPortsState, crate::session::SiAdapterPortsState)
	}
}
