use crate::prelude::*;

pub use crate::auto::{
	ObjectManager,
	ObjectInterest,
	ConstraintType, ConstraintVerb,
	InterestMatch, InterestMatchFlags,
};

mod interest;
pub use interest::{Constraint, Interest, InterestContainer};

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
		self.lookup_full(interest)
			.map(|obj| unsafe { obj.unsafe_cast() })
	}
}

impl<'a> IntoIterator for &'a ObjectManager {
	type Item = GObject; // TODO: crate::Object instead? or do factories not impl it?
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
