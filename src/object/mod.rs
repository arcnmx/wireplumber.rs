pub use crate::auto::{
	Object,
	FeatureActivationTransition,
	Factory,
	traits::ObjectExt,
};

mod features;
pub use features::ObjectFeatures;

mod subclass;
pub use subclass::{ObjectImpl, ObjectImplExt};
