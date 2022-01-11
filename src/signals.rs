use glib_signal::{def_signal, SignalFlags, Pointer};

def_signal! {
	impl Notifies<"connected" as Connected> for crate::Core {
		impl {const SIGNAL_CONNECTED};
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"disconnected" as Disconnected> for crate::Core {
		impl {const SIGNAL_DISCONNECTED};
		fn(&self)
	}
}

def_signal! {
	impl Notifies<"installed" as Installed> for crate::ObjectManager {
		impl {const SIGNAL_INSTALLED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"objects-changed" as ObjectsChanged> for crate::ObjectManager {
		impl {const SIGNAL_OBJECTS_CHANGED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"object-added" as ObjectAdded> for crate::ObjectManager {
		impl {const SIGNAL_OBJECT_ADDED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, crate::Object)
	}
}
def_signal! {
	impl Notifies<"object-removed" as ObjectRemoved> for crate::ObjectManager {
		impl {const SIGNAL_OBJECT_REMOVED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, crate::Object)
	}
}

def_signal! {
	impl Notifies<"bound" as Bound> for crate::Proxy {
		impl {const SIGNAL_BOUND};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, u32)
	}
}
def_signal! {
	impl Notifies<"pw-proxy-created" as PwProxyCreated> for crate::Proxy {
		impl {const SIGNAL_PROXY_CREATED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, Pointer<pipewire_sys::pw_proxy>)
	}
}
def_signal! {
	impl Notifies<"pw-proxy-destroyed" as PwProxyDestroyed> for crate::Proxy {
		impl {const SIGNAL_PROXY_DESTROYED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, Pointer<pipewire_sys::pw_proxy>)
	}
}
def_signal! {
	impl Notifies<"error" as Error> for crate::Proxy {
		impl {const SIGNAL_ERROR};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, i32, i32, String)
	}
}

def_signal! {
	impl Notifies<"params-changed" as ParamsChanged> for crate::PipewireObject {
		impl {const SIGNAL_PARAMS_CHANGED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, String)
	}
}

def_signal! {
	impl Notifies<"ports-changed" as PortsChanged> for crate::Node {
		impl {const SIGNAL_PORTS_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self)
	}
}
def_signal! {
	impl Notifies<"state-changed" as StateChanged> for crate::Node {
		impl {const SIGNAL_STATE_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, crate::NodeState, crate::NodeState)
	}
}

def_signal! {
	impl Notifies<"changed" as Changed> for crate::Metadata {
		impl {const SIGNAL_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self, u32, String, String, String)
	}
}

def_signal! {
	impl Notifies<"create-object" as CreateObject> for crate::SpaDevice {
		impl {const SIGNAL_CREATE_OBJECT};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, u32, String, String, crate::Properties)
	}
}
def_signal! {
	impl Notifies<"object-removed" as SpaObjectRemoved> for crate::SpaDevice {
		impl {const SIGNAL_OBJECT_REMOVED};
		FLAGS = SignalFlags::RUN_FIRST;
		fn(&self, u32)
	}
}

def_signal! {
	impl Notifies<"endpoint-properties-changed" as EndpointPropertiesChanged> for crate::SiEndpoint {
		impl {const SIGNAL_ENDPOINT_PROPERTIES_CHANGED};
		FLAGS = SignalFlags::RUN_LAST;
		fn(&self)
	}
}
