use crate::prelude::*;
use crate::pw::{self, Node, Port, PipewireObject};
use crate::registry::{Interest, InterestContainer, ObjectInterest};

impl Node {
	#[doc(alias = "wp_node_new_ports_iterator")]
	pub fn ports(&self) -> ValueIterator<Port> {
		ValueIterator::with_inner(self.new_ports_iterator().unwrap())
	}

	#[doc(alias = "wp_node_new_ports_filtered_iterator")]
	#[doc(alias = "wp_node_new_ports_filtered_iterator_full")]
	pub fn ports_filtered(&self, interest: &ObjectInterest) -> ValueIterator<Port> {
		ValueIterator::with_inner(self.new_ports_filtered_iterator_full(interest).unwrap())
	}

	#[doc(alias = "wp_node_lookup_port")]
	#[doc(alias = "wp_node_lookup_port_full")]
	pub fn port(&self, interest: &ObjectInterest) -> Option<Port> {
		self.lookup_port_full(interest)
	}

	pub fn device_index(&self) -> Result<Option<u32>, Error> {
		self.pw_property_optional("card.profile.device")
	}

	pub fn device_id(&self) -> Result<Option<u32>, Error> {
		self.pw_property_optional(pw::PW_KEY_DEVICE_ID)
	}

	pub fn device_details(&self) -> Result<Option<(u32, Option<u32>)>, Error> {
		self.device_id().and_then(|id| self.device_index().map(|index|
			id.map(|id| (id, index))
		))
	}

	pub fn name(&self) -> Option<String> {
		self.get_pw_property(pw::PW_KEY_NODE_NAME)
	}

	pub fn props(&self) -> ! {
		for param in self.enum_params_sync("Props", None).into_iter().flat_map(|p| p) {
			// if param.pod_type == "Object" && pod.properties.volume != null
			println!("out_params.init_from_pod({:?})", param);
		}
		todo!()
	}

	// TODO: props_future
}

impl Display for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(res) = self.with_pw_property(pw::PW_KEY_NODE_NAME, |name| {
			f.write_str(name)
		}) {
			return res
		}

		write!(f, "pw.node({})", AsRef::<PipewireObject>::as_ref(self))
	}
}

impl InterestContainer<Port> for Node {
	fn filter(&self, interest: &Interest<Port>) -> ValueIterator<Port> {
		self.ports_filtered(interest)
	}

	fn lookup(&self, interest: &Interest<Port>) -> Option<Port> {
		self.port(interest)
	}
}

impl<'a> IntoIterator for &'a Node {
	type Item = Port;
	type IntoIter = ValueIterator<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.ports()
	}
}

impl IntoIterator for Node {
	type Item = Port;
	type IntoIter = ValueIterator<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.ports()
	}
}

#[cfg(feature = "pipewire")]
mod pipewire_types {
	use crate::pw::NodeState;
	use pipewire::node::NodeState as PwNodeState;

	impl NodeState {
		#[cfg_attr(feature = "dox", doc(cfg(feature = "pipewire")))]
		pub fn from_pw(pw: PwNodeState) -> Result<NodeState, &str> {
			Ok(match pw {
				PwNodeState::Error(e) => return Err(e),
				PwNodeState::Creating => NodeState::Creating,
				PwNodeState::Suspended => NodeState::Suspended,
				PwNodeState::Idle => NodeState::Idle,
				PwNodeState::Running => NodeState::Running,
			})
		}

		#[cfg_attr(feature = "dox", doc(cfg(feature = "pipewire")))]
		pub fn to_pw(&self) -> Result<PwNodeState<'static>, i32> {
			Ok(match self {
				&NodeState::__Unknown(v) => return Err(v),
				NodeState::Error => PwNodeState::Error(Default::default()),
				NodeState::Creating => PwNodeState::Creating,
				NodeState::Suspended => PwNodeState::Suspended,
				NodeState::Idle => PwNodeState::Idle,
				NodeState::Running => PwNodeState::Running,
			})
		}
	}

	#[cfg_attr(feature = "dox", doc(cfg(feature = "pipewire")))]
	impl From<PwNodeState<'_>> for NodeState {
		fn from(pw: PwNodeState) -> Self {
			Self::from_pw(pw).unwrap_or(NodeState::Error)
		}
	}

	#[cfg_attr(feature = "dox", doc(cfg(feature = "pipewire")))]
	impl From<NodeState> for PwNodeState<'_> {
		fn from(state: NodeState) -> Self {
			state.to_pw().expect("Unsupported WpNodeState value")
		}
	}
}
