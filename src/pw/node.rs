use crate::{
	prelude::*,
	pw::{self, Node, NodeState, PipewireObject, Port},
	registry::{Interest, InterestContainer, ObjectInterest},
};

impl Node {
	#[doc(alias = "wp_node_get_state")]
	#[doc(alias = "get_state")]
	pub fn state(&self) -> NodeState {
		unsafe { from_glib(ffi::wp_node_get_state(self.to_glib_none().0, ptr::null_mut())) }
	}

	#[doc(alias = "wp_node_get_state")]
	#[doc(alias = "get_state")]
	pub fn state_result(&self) -> Result<NodeState, Error> {
		unsafe {
			let mut error = ptr::null();
			match from_glib(ffi::wp_node_get_state(self.to_glib_none().0, &mut error)) {
				NodeState::Error => {
					let msg: Option<&glib::GStr> = from_glib_none(error);
					Err(Error::new(
						LibraryErrorEnum::OperationFailed,
						msg.map(|s| s.as_str()).unwrap_or("unspecified node state error"),
					))
				},
				state => Ok(state),
			}
		}
	}

	#[doc(alias = "wp_node_new_ports_iterator")]
	pub fn ports(&self) -> ValueIterator<Port> {
		ValueIterator::with_inner(self.new_ports_iterator().unwrap())
	}

	#[doc(alias = "wp_node_new_ports_filtered_iterator")]
	#[doc(alias = "wp_node_new_ports_filtered_iterator_full")]
	pub fn ports_filtered(&self, interest: ObjectInterest) -> ValueIterator<Port> {
		ValueIterator::with_inner(self.new_ports_filtered_iterator_full(interest).unwrap())
	}

	pub fn device_index(&self) -> Result<Option<u32>, Error> {
		self.pw_property_optional("card.profile.device")
	}

	pub fn device_id(&self) -> Result<Option<u32>, Error> {
		self.pw_property_optional(pw::PW_KEY_DEVICE_ID)
	}

	pub fn device_details(&self) -> Result<Option<(u32, Option<u32>)>, Error> {
		self
			.device_id()
			.and_then(|id| self.device_index().map(|index| id.map(|id| (id, index))))
	}

	pub fn name(&self) -> Option<String> {
		self.get_pw_property(pw::PW_KEY_NODE_NAME)
	}

	pub fn props(&self) -> ! {
		for param in self.enum_params_sync("Props", None).into_iter().flat_map(|p| p) {
			// if param.pod_type == "Object" && pod.properties.volume != null
			println!("out_params.init_from_pod({param:?})");
		}
		todo!()
	}

	// TODO: props_future
}

impl Display for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(res) = self.with_pw_property(pw::PW_KEY_NODE_NAME, |name| f.write_str(name)) {
			return res
		}

		write!(f, "pw.node({})", AsRef::<PipewireObject>::as_ref(self))
	}
}

impl InterestContainer<Port> for Node {
	fn filter(&self, interest: Interest<Port>) -> ValueIterator<Port> {
		self.ports_filtered(interest.into())
	}

	fn lookup(&self, interest: Interest<Port>) -> Option<Port> {
		self.lookup_port(interest.into())
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

impl<E> From<Result<NodeState, E>> for NodeState {
	fn from(res: Result<NodeState, E>) -> Self {
		match res {
			Ok(state) => state,
			Err(_) => NodeState::Error,
		}
	}
}
