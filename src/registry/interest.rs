use crate::registry::{ObjectInterest, ConstraintVerb, ConstraintType, InterestMatchFlags, InterestMatch};
use crate::pw::{PipewireObject, Properties};
use crate::prelude::*;

impl ObjectInterest {
	/*#[doc(alias = "wp_object_interest_matches")]
	pub fn matches<O: IsA<GObject>>(&self, object: O) -> bool {
		unsafe {
			from_glib(ffi::wp_object_interest_matches(self.to_glib_none().0, object.to_glib_none().0 as *mut _))
		}
	}*/
	#[doc(alias = "wp_object_interest_matches")]
	pub fn matches_object<O: IsA<GObject>>(&self, object: &O) -> bool {
		let object = object.as_ref();
		self.matches_full(InterestMatchFlags::CHECK_ALL, object.type_(), Some(object), None, None) == InterestMatch::all()
	}

	#[doc(alias = "wp_object_interest_matches")]
	pub fn matches_props(&self, props: &Properties) -> bool {
		self.matches_full(InterestMatchFlags::CHECK_ALL, Properties::static_type(), None::<&GObject>, Some(props), None) == InterestMatch::all()
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

	pub fn matches_object<O: IsA<GObject>>(&self, object: &O) -> bool where T: IsA<O> {
		self.interest.matches_object(object)
	}

	pub fn constrain<'o, O: IsA<GObject>>(&self, object: &'o O) -> Option<&'o T> where T: IsA<O> {
		if self.matches_object(object) {
			Some(unsafe {
				object.unsafe_cast_ref()
			})
		} else {
			None
		}
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
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"equals" | "=" => ConstraintVerb::Equals,
			"not-equals" | "!" => ConstraintVerb::NotEquals,
			"in-list" | "c" => ConstraintVerb::InList,
			"in-range" | "~" => ConstraintVerb::InRange,
			"matches" | "#" => ConstraintVerb::Matches,
			"is-present" | "+" => ConstraintVerb::IsPresent,
			"is-absent" | "-" => ConstraintVerb::IsAbsent,
			_ => return Err(Error::new(LibraryErrorEnum::InvalidArgument, &format!("unknown constraint verb {}", s))),
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
			_ => return Err(Error::new(LibraryErrorEnum::InvalidArgument, &format!("unknown constraint verb {}", value))),
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
		str::from_utf8(&[self.symbol() as u8])
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
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"pw-global" => ConstraintType::PwGlobalProperty,
			"pw" => ConstraintType::PwProperty,
			"gobject" => ConstraintType::GProperty,
			_ => return Err(Error::new(LibraryErrorEnum::InvalidArgument, &format!("unknown constraint type {}", s))),
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

#[cfg(feature = "serde")]
mod impl_serde {
	use super::{Constraint, ConstraintVerb, ConstraintType};
	use glib_serde::{prelude::*, AnyVariant};
	use glib::{Variant, ToVariant};
	use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{self, Error as _, Visitor, SeqAccess, MapAccess, Unexpected}, ser::SerializeStruct};
	use std::str::FromStr;
	use std::borrow::Cow;
	use std::fmt;

	impl<'de> Deserialize<'de> for ConstraintVerb {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			<Cow<String>>::deserialize(deserializer)
				.and_then(|s| Self::from_str(&s).map_err(D::Error::custom))
		}
	}

	impl Serialize for ConstraintVerb {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			self.symbol().serialize(serializer)
		}
	}

	impl<'de> Deserialize<'de> for ConstraintType {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			<Cow<String>>::deserialize(deserializer)
				.and_then(|s| Self::from_str(&s).map_err(D::Error::custom))
		}
	}

	impl Serialize for ConstraintType {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			self.name().serialize(serializer)
		}
	}

	impl Serialize for Constraint {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let compact = !serializer.is_human_readable();
			let mut state = serializer.serialize_struct("Constraint", 4)?;
			state.serialize_field("type", &self.type_)?;
			state.serialize_field("subject", &self.subject)?;
			state.serialize_field("verb", &self.verb)?;
			state.serialize_field("value", &self.value.as_ref().map(|v| v.as_serializable()))?;
			state.end()
		}
	}

	impl<'de> Deserialize<'de> for Constraint {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			enum Field { Type, Subject, Verb, Value }

			impl<'de> Deserialize<'de> for Field {
				fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error> {
					struct FieldVisitor;

					impl<'de> Visitor<'de> for FieldVisitor {
						type Value = Field;

						fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
							formatter.write_str("`type` or `subject` or `verb` or `value`")
						}

						fn visit_str<E: de::Error>(self, value: &str) -> Result<Field, E> {
							match value {
								"type" => Ok(Field::Type),
								"subject" => Ok(Field::Subject),
								"verb" => Ok(Field::Verb),
								"value" => Ok(Field::Value),
								_ => Err(E::unknown_field(value, FIELDS)),
							}
						}
					}

					deserializer.deserialize_identifier(FieldVisitor)
				}
			}

			struct ConstraintVisitor;

			impl<'de> Visitor<'de> for ConstraintVisitor {
				type Value = Constraint;

				fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
					formatter.write_str("struct Constraint")
				}

				fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Self::Value, V::Error> {
					let mut len = 0;

					let mut subject: String = seq.next_element()?
						.ok_or_else(|| V::Error::invalid_length(len, &self))?; len += 1;

					let type_ = match ConstraintType::from_str(&subject) {
						Ok(type_) => {
							subject = seq.next_element()?
								.ok_or_else(|| V::Error::invalid_length(len, &self))?; len += 1;
							type_
						},
						Err(_) => ConstraintType::default(),
					};

					let verb = seq.next_element()?
						.ok_or_else(|| V::Error::invalid_length(len, &self))?; len += 1;

					let value = match verb {
						ConstraintVerb::__Unknown(v) => return Err(V::Error::invalid_value(Unexpected::Signed(v.into()), &"constraint verb")),
						ConstraintVerb::IsPresent | ConstraintVerb::IsAbsent => None,
						ConstraintVerb::Equals | ConstraintVerb::NotEquals => Some(
							seq.next_element::<AnyVariant>()?
								.ok_or_else(|| V::Error::invalid_length(len, &"constraint value"))?
								.into()
						),
						ConstraintVerb::Matches => Some(
							seq.next_element::<&str>()?
								.ok_or_else(|| V::Error::invalid_length(len, &"constraint match pattern"))?
								.to_variant()
						),
						ConstraintVerb::InRange => Some(Variant::tuple_from_iter([
								seq.next_element::<AnyVariant>()?
									.ok_or_else(|| V::Error::invalid_length(len, &"constraint range min"))?
									.into_variant(),
								seq.next_element::<AnyVariant>()?
									.ok_or_else(|| V::Error::invalid_length(len + 1, &"constraint range max"))?
									.into_variant(),
						])),
						ConstraintVerb::InList => {
							let mut values: Vec<glib::Variant> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
							while let Some(value) = seq.next_element::<AnyVariant>()? {
								values.push(value.into());
							}
							Some(glib::Variant::tuple_from_iter(values))
						},
					};

					Ok(Constraint {
						type_,
						subject,
						verb,
						value,
					})
				}

				fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
					let mut type_ = None;
					let mut subject = None;
					let mut verb = None;
					let mut value = None::<Option<AnyVariant>>;
					while let Some(key) = map.next_key()? {
						match key {
							Field::Type => {
								if type_.is_some() {
									return Err(V::Error::duplicate_field("type"));
								}
								type_ = Some(map.next_value()?);
							},
							Field::Subject => {
								if subject.is_some() {
									return Err(V::Error::duplicate_field("subject"));
								}
								subject = Some(map.next_value()?);
							},
							Field::Verb => {
								if verb.is_some() {
									return Err(V::Error::duplicate_field("verb"));
								}
								verb = Some(map.next_value()?);
							},
							Field::Value => {
								if value.is_some() {
									return Err(V::Error::duplicate_field("value"));
								}
								value = Some(map.next_value()?);
							},
						}
					}
					Ok(Constraint {
						type_: type_.ok_or_else(|| V::Error::missing_field("type"))?,
						subject: subject.ok_or_else(|| V::Error::missing_field("subject"))?,
						verb: verb.ok_or_else(|| V::Error::missing_field("verb"))?,
						value: value.unwrap_or_default().map(Into::into),
					})
				}
			}

			const FIELDS: &'static [&'static str] = &["type", "subject", "verb", "value"];
			deserializer.deserialize_struct("Constraint", FIELDS, ConstraintVisitor)
		}
	}
}
