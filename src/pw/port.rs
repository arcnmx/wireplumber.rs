use crate::{Port, pw};
use crate::prelude::*;
use std::fmt;

impl Port {
	pub fn node_id(&self) -> crate::Result<u32> {
		self.pw_property(pw::PW_KEY_NODE_ID)
	}

	#[doc(alias = "port_id")]
	pub fn port_index(&self) -> crate::Result<u32> {
		self.pw_property(pw::PW_KEY_PORT_ID)
	}
}

impl fmt::Display for Port {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(res) = self.with_pw_property(pw::PW_KEY_PORT_ALIAS, |name| {
			f.write_str(name)
		}) {
			return res
		}

		if let Ok(node) = self.node_id() {
			write!(f, "{}", node)?;
		}
		if let Ok(index) = self.port_index() {
			write!(f, ".{}", index)?;
		}

		if let Some(res) = self.with_pw_property(pw::PW_KEY_PORT_NAME, |name| {
			write!(f, ":{}", name)
		}) {
			return res
		}

		write!(f, "pw.port({})", AsRef::<crate::PipewireObject>::as_ref(self))
	}
}
