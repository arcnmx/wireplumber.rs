pub use crate::auto::{
	PipewireObject,
	Proxy, ProxyFeatures,
	GlobalProxy,
	Endpoint,
	Device,
	Node, NodeState, NodeFeatures,
	Port, Direction,
	Link,
	Metadata, MetadataFeatures,
	Properties,
	traits::{
		PipewireObjectExt,
		ProxyExt,
		GlobalProxyExt,
		EndpointExt,
		MetadataExt,
	},
};
#[cfg(any(feature = "v0_4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_2")))]
pub use crate::auto::PropertiesItem;

mod keys;
pub use keys::*;

mod proxy;
pub use proxy::{PipewireObjectExt2, ProxyExt2};
mod node;
mod port;
mod link;
pub use link::LinkTarget;
mod properties;
