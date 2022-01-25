pub use crate::auto::{
	Core, InitFlags,
	Object,
	FeatureActivationTransition,
	Factory,
	traits::ObjectExt,
};

mod core;

mod features;
pub use features::ObjectFeatures;

mod subclass;
pub use subclass::{ObjectImpl, ObjectImplExt};

impl Default for InitFlags {
	fn default() -> Self {
		Self::ALL
	}
}
