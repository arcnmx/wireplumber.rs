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
//!   om.add_interest([
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
	#[doc(alias = "wp_object_manager_add_interest_full")]
	#[doc(alias = "add_interest_full")]
	pub fn add_interest<I: Into<ObjectInterest>>(&self, interest: I) {
		self.add_interest_full(interest.into())
	}

	#[doc(alias = "wp_object_manager_new_iterator")]
	#[doc(alias = "new_iterator")]
	pub fn objects<T: ObjectType>(&self) -> IntoValueIterator<T> {
		IntoValueIterator::with_inner(self.objects_iterator().unwrap())
	}

	#[doc(alias = "wp_object_manager_new_filtered_iterator")]
	#[doc(alias = "wp_object_manager_new_filtered_iterator_full")]
	#[doc(alias = "new_filtered_iterator")]
	#[doc(alias = "new_filtered_iterator_full")]
	pub fn filtered<T: ObjectType>(&self, interest: ObjectInterest) -> IntoValueIterator<T> {
		IntoValueIterator::with_inner(self.filtered_iterator(interest.into()).unwrap())
	}

	#[doc(alias = "wp_object_manager_lookup")]
	#[doc(alias = "wp_object_manager_lookup_full")]
	#[doc(alias = "lookup_full")]
	pub fn lookup<T: ObjectType>(&self, interest: Interest<T>) -> Option<T> {
		self
			.lookup_object(interest.into())
			.map(|obj| unsafe { obj.unsafe_cast() })
	}

	/// Wait until the ObjectManager [has been installed](Self::is_installed).
	///
	/// Note that the future does not take ownership over `self`, and will produce
	/// an error in cases where there are no more references keeping it alive.
	#[cfg(feature = "futures")]
	#[cfg_attr(docsrs, doc(feature = "futures"))]
	pub fn installed_future(&self) -> impl Future<Output = Result<(), Error>> {
		use crate::util::futures::signal_once;

		let signal_installed = if self.is_installed() {
			None
		} else {
			let signal_installed = signal_once(match () {
				#[cfg(feature = "glib-signal")]
				() => self.signal_stream(Self::SIGNAL_INSTALLED),
				#[cfg(not(feature = "glib-signal"))]
				() => |handler| self.connect_installed(handler),
			});
			if !self.is_installed() {
				Some(signal_installed)
			} else {
				None
			}
		};

		async move {
			if let Some(signal_installed) = signal_installed {
				let res = signal_installed.await;
				if res.is_err() {
					return Err(Error::new(
						LibraryErrorEnum::OperationFailed,
						"ObjectManager will never be installed",
					))
				}
			}

			Ok(())
		}
	}
}

impl<'a> IntoIterator for &'a ObjectManager {
	// TODO: crate::Object instead? or do factories not impl it?
	type Item = GObject;
	type IntoIter = ValueIterator<GObject>;

	fn into_iter(self) -> Self::IntoIter {
		self.objects().into_iter()
	}
}

impl<T: ObjectType> InterestContainer<T> for ObjectManager {
	fn filter(&self, interest: Interest<T>) -> IntoValueIterator<T> {
		self.filtered(interest.into())
	}

	fn lookup(&self, interest: Interest<T>) -> Option<T> {
		Self::lookup(self, interest)
	}
}
