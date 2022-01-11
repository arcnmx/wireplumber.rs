use crate::{ObjectInterest, ConstraintVerb, ConstraintType, InterestMatchFlags, InterestMatch, Properties};
use glib::{translate::IntoGlib, IsA, Object, Variant, ToVariant, ObjectExt, StaticType};
use std::str::FromStr;
use std::convert::TryFrom;

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
		self.matches_full::<Object>(InterestMatchFlags::CHECK_ALL, Properties::static_type(), None, Some(props), None) == InterestMatch::all()
	}
}

#[derive(Clone, Debug)]
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

	pub fn compare<S: Into<String>>(type_: ConstraintType, subject: S, value: Variant, equal: bool) -> Self {
		Self {
			type_,
			subject: subject.into(),
			verb: if equal { ConstraintVerb::Equals } else { ConstraintVerb::NotEquals },
			value: Some(value),
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
			value: Some((low.to_variant(), high.to_variant()).to_variant()),
		}
	}

	pub fn in_list<S: Into<String>, V: ToVariant, I: Iterator<Item=V>>(type_: ConstraintType, subject: S, one_of: I) -> Self {
		let values: Vec<_> = one_of.map(|v| v.to_variant()).collect();
		Self {
			type_,
			subject: subject.into(),
			verb: ConstraintVerb::InRange,
			value: Some(Variant::from_tuple(&values)),
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
