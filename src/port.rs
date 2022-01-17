use crate::{Port, pw};
use crate::prelude::*;

impl Port {
	pub fn node_id(&self) -> crate::Result<u32> {
		self.pw_property(pw::PW_KEY_NODE_ID)
	}

	#[doc(alias = "port_id")]
	pub fn port_index(&self) -> crate::Result<u32> {
		self.pw_property(pw::PW_KEY_PORT_ID)
	}
}
