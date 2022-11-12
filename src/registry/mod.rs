//! Remote object change notifications
//!
//! When combined with a [Core](crate::core::Core), an [ObjectManager] emits signals whenever
//! objects change on the remote service if they match any filters specified by the registered
//! [Interest]'s [Constraints](Constraint).
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "futures")]
//! use futures_util::StreamExt;
//! use wireplumber::{
//!   prelude::*,
//!   registry::{ObjectManager, Interest, Constraint, ConstraintType},
//!   core::{Core, Object, ObjectFeatures},
//!   pw::Node,
//! };
//!
//! async fn watch_nodes(core: &Core) {
//!   let om = ObjectManager::new();
//!   om.add_interest_full(&[
//!     Constraint::compare(ConstraintType::PwProperty, "media.class", "Audio/Sink", true),
//!   ].iter().collect::<Interest<Node>>());
//!
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
//!
//! # See also
//!
//! C API docs for:
//!
//! - [ObjectManager](https://pipewire.pages.freedesktop.org/wireplumber/c_api/obj_manager_api.html)
//! - [Interest](https://pipewire.pages.freedesktop.org/wireplumber/c_api/obj_interest_api.html)
use crate::prelude::*;
pub use {
	self::interest::{Constraint, Interest, InterestContainer},
	crate::auto::{ConstraintType, ConstraintVerb, InterestMatch, InterestMatchFlags, ObjectInterest, ObjectManager},
};

mod interest;

impl ObjectManager {
	#[doc(alias = "wp_object_manager_new_iterator")]
	pub fn objects<T: ObjectType>(&self) -> ValueIterator<T> {
		ValueIterator::with_inner(self.new_iterator().unwrap())
	}

	#[doc(alias = "wp_object_manager_new_filtered_iterator")]
	#[doc(alias = "wp_object_manager_new_filtered_iterator_full")]
	pub fn filtered<T: ObjectType>(&self, interest: &ObjectInterest) -> ValueIterator<T> {
		ValueIterator::with_inner(self.new_filtered_iterator_full(interest).unwrap())
	}

	#[doc(alias = "wp_object_manager_lookup")]
	#[doc(alias = "wp_object_manager_lookup_full")]
	pub fn lookup<T: ObjectType>(&self, interest: &Interest<T>) -> Option<T> {
		self.lookup_full(interest).map(|obj| unsafe { obj.unsafe_cast() })
	}
}

impl<'a> IntoIterator for &'a ObjectManager {
	// TODO: crate::Object instead? or do factories not impl it?
	type Item = GObject;
	type IntoIter = ValueIterator<GObject>;

	fn into_iter(self) -> Self::IntoIter {
		self.objects()
	}
}

impl<T: ObjectType> InterestContainer<T> for ObjectManager {
	fn filter(&self, interest: &Interest<T>) -> ValueIterator<T> {
		self.filtered(interest)
	}

	fn lookup(&self, interest: &Interest<T>) -> Option<T> {
		Self::lookup(self, interest)
	}
}
