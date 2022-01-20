use crate::{ObjectInterest, ConstraintVerb, ConstraintType, InterestMatchFlags, InterestMatch, Properties, PipewireObject, ValueIterator};
use crate::prelude::*;
use glib::{FromVariant, StaticVariantType, VariantTy, VariantClass};
use glib::{translate::{IntoGlib, from_glib}, Object, Variant};
use glib::prelude::*;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::str::FromStr;
use std::convert::TryFrom;
use std::borrow::{Cow, Borrow};
use std::ops::Deref;

impl ObjectInterest {
	/*#[doc(alias = "wp_object_interest_matches")]
	pub fn matches<O: IsA<Object>>(&self, object: O) -> bool {
		unsafe {
			from_glib(ffi::wp_object_interest_matches(self.to_glib_none().0, object.to_glib_none().0 as *mut _))
		}
	}*/
	#[doc(alias = "wp_object_interest_matches")]
	pub fn matches_object<O: IsA<Object>>(&self, object: &O) -> bool {
		let object = object.as_ref();
		self.matches_full(InterestMatchFlags::CHECK_ALL, object.type_(), Some(object), None, None) == InterestMatch::all()
	}

	#[doc(alias = "wp_object_interest_matches")]
	pub fn matches_props(&self, props: &Properties) -> bool {
		self.matches_full(InterestMatchFlags::CHECK_ALL, Properties::static_type(), None::<&Object>, Some(props), None) == InterestMatch::all()
	}

	#[doc(alias = "wp_object_interest_matches")]
	pub fn matches_pw_object<O: IsA<PipewireObject>>(&self, object: &O) -> bool {
		self.matches_props(&object.as_ref().properties().unwrap())
	}
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Interest<T: StaticType> {
	interest: ObjectInterest,
	_type: PhantomData<T>,
}

impl<T: StaticType> Interest<T> {
	pub fn new() -> Self {
		unsafe {
			Self::wrap_unchecked(ObjectInterest::new_type(T::static_type()))
		}
	}

	pub unsafe fn wrap_unchecked(interest: ObjectInterest) -> Self {
		Self {
			interest,
			_type: PhantomData,
		}
	}

	pub fn inner(&self) -> &ObjectInterest {
		&self.interest
	}

	pub fn into_inner(self) -> ObjectInterest {
		self.interest
	}

	pub fn matches_object<O: IsA<glib::Object>>(&self, object: &O) -> bool where T: IsA<O> {
		self.interest.matches_object(object)
	}

	pub fn filter<C: InterestContainer<T>>(&self, container: &C) -> ValueIterator::<T> {
		container.filter(self)
	}

	pub fn lookup<C: InterestContainer<T>>(&self, container: &C) -> Option<T> {
		container.lookup(self)
	}

	// TODO: helpers for adding constraints that skip the `Type` arg
	// TODO: wrapper types for each constraint verb that type-ifies the expected arguments?
}

pub trait InterestContainer<T: StaticType> {
	fn filter(&self, interest: &Interest<T>) -> ValueIterator<T>;
	fn lookup(&self, interest: &Interest<T>) -> Option<T>;
}

impl<T: StaticType> Deref for Interest<T> {
	type Target = ObjectInterest;

	fn deref(&self) -> &Self::Target {
		self.inner()
	}
}

impl<C: Borrow<Constraint>, T: StaticType> Extend<C> for Interest<T> {
	fn extend<I: IntoIterator<Item=C>>(&mut self, iter: I) {
		for constraint in iter {
			constraint.borrow().add_to(&self)
		}
	}
}

impl<C: Borrow<Constraint>, T: StaticType> FromIterator<C> for Interest<T> {
	fn from_iter<I: IntoIterator<Item=C>>(iter: I) -> Self {
		let mut interest = Self::new();
		interest.extend(iter);
		interest
	}
}

#[must_use]
#[derive(Clone, Debug, Variant)]
pub struct Constraint {
	pub type_: ConstraintType,
	pub subject: String,
	pub verb: ConstraintVerb,
	pub value: Option<Variant>,
}

impl Constraint {
	pub fn has<S: Into<String>>(type_: ConstraintType, subject: S, present: bool) -> Self {
		Self {
			type_,
			subject: subject.into(),
			verb: if present { ConstraintVerb::IsPresent } else { ConstraintVerb::IsAbsent },
			value: None,
		}
	}

	pub fn compare<S: Into<String>, V: ToVariant>(type_: ConstraintType, subject: S, value: V, equal: bool) -> Self {
		Self {
			type_,
			subject: subject.into(),
			verb: if equal { ConstraintVerb::Equals } else { ConstraintVerb::NotEquals },
			value: Some(value.to_variant()),
		}
	}

	pub fn matches<S: Into<String>>(type_: ConstraintType, subject: S, pattern: &str) -> Self {
		Self {
			type_,
			subject: subject.into(),
			verb: ConstraintVerb::Matches,
			value: Some(pattern.to_variant()),
		}
	}

	pub fn in_range<S: Into<String>, V: ToVariant>(type_: ConstraintType, subject: S, low: V, high: V) -> Self {
		Self {
			type_,
			subject: subject.into(),
			verb: ConstraintVerb::InRange,
			value: Some((low, high).to_variant()),
		}
	}

	pub fn in_list<S: Into<String>, V: ToVariant, I: Iterator<Item=V>>(type_: ConstraintType, subject: S, one_of: I) -> Self {
		let values = one_of.map(|v| v.to_variant());
		Self {
			type_,
			subject: subject.into(),
			verb: ConstraintVerb::InRange,
			value: Some(Variant::tuple_from_iter(values)),
		}
	}

	pub fn add_to(&self, interest: &ObjectInterest) {
		interest.add_constraint(self.type_, &self.subject, self.verb, self.value.as_ref())
	}
}

impl FromStr for ConstraintVerb {
	type Err = std::io::Error; // TODO: actual error type

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"equals" | "=" => ConstraintVerb::Equals,
			"not-equals" | "!" => ConstraintVerb::NotEquals,
			"in-list" | "c" => ConstraintVerb::InList,
			"in-range" | "~" => ConstraintVerb::InRange,
			"matches" | "#" => ConstraintVerb::Matches,
			"is-present" | "+" => ConstraintVerb::IsPresent,
			"is-absent" | "-" => ConstraintVerb::IsAbsent,
			_ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("unknown constraint verb {}", s))),
		})
	}
}

impl TryFrom<char> for ConstraintVerb {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'=' => ConstraintVerb::Equals,
			'!' => ConstraintVerb::NotEquals,
			'c' => ConstraintVerb::InList,
			'~' => ConstraintVerb::InRange,
			'#' => ConstraintVerb::Matches,
			'+' => ConstraintVerb::IsPresent,
			'-' => ConstraintVerb::IsAbsent,
			_ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("unknown constraint verb {}", value))),
		})
	}
}

impl StaticVariantType for ConstraintVerb {
	fn static_variant_type() -> Cow<'static, VariantTy> {
		<<Self as IntoGlib>::GlibType as StaticVariantType>::static_variant_type()
	}
}

impl FromVariant for ConstraintVerb {
	fn from_variant(variant: &Variant) -> Option<Self> {
		match variant.classify() {
			VariantClass::String =>
				variant.get::<String>()
				.and_then(|s| Self::from_str(&s).ok()),
			_ => unsafe {
				Some(from_glib(variant.get()?))
			},
		}
	}
}

impl ToVariant for ConstraintVerb {
	fn to_variant(&self) -> Variant {
		std::str::from_utf8(&[self.symbol() as u8])
			.unwrap()
			.to_variant()
	}
}

impl ConstraintVerb {
	pub fn value_type(&self) -> Option<()> {
		match self {
			ConstraintVerb::__Unknown(_) => panic!("unknown constraint verb"),
			ConstraintVerb::IsPresent | ConstraintVerb::IsAbsent => None,
			ConstraintVerb::Equals | ConstraintVerb::NotEquals => Some(/*one of [Type::STRING, Type::U32, Type::I32, Type::U64, Type::I64, Type::F64, Type::BOOL]*/()),
			ConstraintVerb::Matches => Some(/*Type::STRING*/()),
			ConstraintVerb::InRange => Some(/*(T, T) where T is one of [I/U32, I/U64, DOUBLE]*/()),
			ConstraintVerb::InList => Some(/*[T] where T is one of [I/U32, I/U64, DOUBLE] - also should be provided as a tuple and not an array/list???*/()),
		}
	}

	pub fn nickname(&self) -> &'static str {
		match self {
			ConstraintVerb::Equals => "equals",
			ConstraintVerb::NotEquals => "not-equals",
			ConstraintVerb::InList => "in-list",
			ConstraintVerb::InRange => "in-range",
			ConstraintVerb::Matches => "matches",
			ConstraintVerb::IsPresent => "is-present",
			ConstraintVerb::IsAbsent => "is-absent",
			ConstraintVerb::__Unknown(_) => panic!("unknown constraint verb"),
		}
	}

	pub fn symbol(&self) -> char {
		match self {
			ConstraintVerb::__Unknown(_) => panic!("unknown constraint verb"),
			_ => self.into_glib() as u8 as char,
		}
	}
}

impl Into<char> for ConstraintVerb {
	fn into(self) -> char {
		self.symbol()
	}
}

impl Default for ConstraintType {
	fn default() -> Self {
		ConstraintType::PwProperty
	}
}

impl FromStr for ConstraintType {
	type Err = std::io::Error; // TODO: actual error type

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"pw-global" => ConstraintType::PwGlobalProperty,
			"pw" => ConstraintType::PwProperty,
			"gobject" => ConstraintType::GProperty,
			_ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("unknown constraint type {}", s))),
		})
	}
}

impl StaticVariantType for ConstraintType {
	fn static_variant_type() -> Cow<'static, VariantTy> {
		<<Self as IntoGlib>::GlibType as StaticVariantType>::static_variant_type()
	}
}

impl FromVariant for ConstraintType {
	fn from_variant(variant: &Variant) -> Option<Self> {
		match variant.classify() {
			VariantClass::String =>
				variant.get::<String>()
				.and_then(|s| Self::from_str(&s).ok()),
			_ => unsafe {
				Some(from_glib(variant.get()?))
			},
		}
	}
}

impl ToVariant for ConstraintType {
	fn to_variant(&self) -> Variant {
		self.name().to_variant()
	}
}

impl ConstraintType {
	pub fn name(&self) -> &'static str {
		match self {
			ConstraintType::PwProperty => "pw",
			ConstraintType::PwGlobalProperty => "pw-global",
			ConstraintType::GProperty => "gobject",
			ConstraintType::None => panic!("no constraint type"),
			ConstraintType::__Unknown(_) => panic!("unknown constraint type"),
		}
	}
}
