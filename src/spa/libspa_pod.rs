use {
	crate::{
		prelude::*,
		spa::{SpaBool, SpaIdTable, SpaIdValue, SpaPod, SpaPodBuilder, SpaPrimitive, SpaType, SpaValue},
	},
	libspa::{
		pod::{
			deserialize::{DeserializeError, PodDeserialize, PodDeserializer},
			serialize::{GenError, PodSerialize, PodSerializer},
			CanonicalFixedSizedPod, ChoiceValue, Object, Property, PropertyFlags, Value, ValueArray,
		},
		utils::{Choice, ChoiceEnum, ChoiceFlags, Fd, Id},
	},
};

impl SpaPod {
	#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
	pub fn spa_id(&self) -> Option<Id> {
		self.id().map(Id)
	}

	#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
	pub fn spa_fd(&self) -> Option<Fd> {
		self.fd().map(Fd)
	}

	#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
	pub fn deserialize<'de, P: PodDeserialize<'de>>(&'de self) -> Result<P, DeserializeError<&'de [u8]>> {
		unsafe { PodDeserializer::deserialize_ptr(NonNull::new_unchecked(self.spa_pod_raw() as *const _ as *mut _)) }
	}

	#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
	pub fn serialize<P: PodSerialize>(value: &P) -> Result<Self, GenError> {
		use std::io::Cursor;

		let mut data = Vec::new();
		let len = {
			let write = Cursor::new(&mut data);
			let (_, len) = PodSerializer::serialize(write, value)?;
			len
		} as usize;
		Ok(Self::with_pod(&data[..len]))
	}

	#[cfg(none)]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
	pub fn to_pod_value(&self) -> Result<Value, DeserializeError<&[u8]>> {
		// TODO: broken due to https://gitlab.freedesktop.org/pipewire/pipewire-rs/-/issues/31
		self.deserialize()
	}

	fn pod_choice<T: SpaPrimitive + CanonicalFixedSizedPod>(&self, choice_type: SpaIdValue) -> Result<Choice<T>, Error> {
		let mut values = self.array_iterator::<T>();
		let mut next_value = || {
			values.next().ok_or_else(|| {
				Error::new(
					LibraryErrorEnum::InvalidArgument,
					&format!("wrong number of values for {:?} choice", choice_type),
				)
			})
		};
		let flags = ChoiceFlags::empty();
		let default = next_value()?;
		Ok(Choice(flags, match choice_type.number() {
			libspa_sys::spa_choice_type_SPA_CHOICE_None => ChoiceEnum::None(default),
			libspa_sys::spa_choice_type_SPA_CHOICE_Range => ChoiceEnum::Range {
				default,
				min: next_value()?,
				max: next_value()?,
			},
			libspa_sys::spa_choice_type_SPA_CHOICE_Step => ChoiceEnum::Step {
				default,
				min: next_value()?,
				max: next_value()?,
				step: next_value()?,
			},
			libspa_sys::spa_choice_type_SPA_CHOICE_Enum => ChoiceEnum::Enum {
				default,
				alternatives: values.collect(),
			},
			libspa_sys::spa_choice_type_SPA_CHOICE_Flags => ChoiceEnum::Flags {
				default,
				flags: values.collect(),
			},
			_ =>
				return Err(Error::new(
					LibraryErrorEnum::InvalidArgument,
					&format!("unknown choice type: {:?}", choice_type),
				)),
		}))
	}

	#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
	pub fn to_pod_value(&self) -> Result<Value, Error> {
		Ok(match () {
			_ if self.is_none() => Value::None,
			_ if self.is_boolean() => Value::Bool(self.boolean().unwrap()),
			_ if self.is_id() => Value::Id(self.spa_id().unwrap()),
			_ if self.is_int() => Value::Int(self.int().unwrap()),
			_ if self.is_long() => Value::Long(self.long().unwrap()),
			_ if self.is_float() => Value::Float(self.float().unwrap()),
			_ if self.is_double() => Value::Double(self.double().unwrap()),
			_ if self.is_string() => Value::String(self.string().unwrap().into()),
			_ if self.is_bytes() => Value::Bytes(self.bytes().unwrap().into()),
			_ if self.is_pointer() => Value::Pointer(0, self.pointer().unwrap()),
			_ if self.is_fd() => Value::Fd(self.spa_fd().unwrap()),
			_ if self.is_rectangle() => Value::Rectangle(self.spa_rectangle().unwrap()),
			_ if self.is_fraction() => Value::Fraction(self.spa_fraction().unwrap()),
			_ if self.is_struct() => self.parse_struct(|parser| {
				self
					.iterator()
					.map(|pod| pod.to_pod_value())
					.collect::<Result<_, Error>>()
					.map(Value::Struct)
			})?,
			_ if self.is_object() => {
				let type_ = self.spa_type().unwrap();
				let ids = type_.object_id_values_table().unwrap();
				let values = type_.values_table().unwrap();
				self.parse_object(|parser, id_name| {
					let value_id = ids.find_value_from_short_name(id_name.unwrap()).unwrap(); // TODO: parse "id-%08x" strings .-.
					Ok::<_, Error>(Value::Object(Object {
						type_: type_.into_glib(),
						id: value_id.number(),
						properties: self
							.spa_properties()
							.map(|(id, pod)| {
								pod.to_pod_value().map(|value| Property {
									key: SpaIdValue::result_number(id),
									flags: PropertyFlags::empty(),
									value,
								})
							})
							.collect::<Result<_, Error>>()?,
					}))
				})?
			},
			_ if self.is_sequence() => {
				wp_warning!("unsupported sequence spa type for {:?}", self);
				Value::Struct(
					self
						.iterator()
						.map(|pod| pod.control().unwrap())
						.map(|(offset, type_name, value)| {
							wp_warning!(
								"discarding sequence context ({}, {}) for {:?}",
								offset,
								type_name,
								value
							);
							value.to_pod_value()
						})
						.collect::<Result<_, _>>()?,
				)
			},
			_ if self.is_array() => {
				let child = self.array_child().unwrap();
				let type_ = child.spa_type().unwrap();
				Value::ValueArray(match type_ {
					_ if child.is_none() => ValueArray::None(self.array_pointers().map(drop).collect()),
					_ if child.is_boolean() => ValueArray::Bool(self.array_iterator::<SpaBool>().map(Into::into).collect()),
					_ if child.is_int() => ValueArray::Int(self.array_iterator().collect()),
					_ if child.is_long() => ValueArray::Long(self.array_iterator().collect()),
					_ if child.is_float() => ValueArray::Float(self.array_iterator().collect()),
					_ if child.is_double() => ValueArray::Double(self.array_iterator().collect()),
					_ if child.is_id() => ValueArray::Id(self.array_iterator().collect()),
					_ if child.is_fd() => ValueArray::Fd(self.array_iterator().collect()),
					_ if child.is_rectangle() => ValueArray::Rectangle(self.array_iterator().collect()),
					_ if child.is_fraction() => ValueArray::Fraction(self.array_iterator().collect()),
					type_ =>
						return Err(Error::new(
							LibraryErrorEnum::InvalidArgument,
							&format!("unsupported SPA array child type {:?}", type_),
						)),
				})
			},
			_ if self.is_choice() => {
				let child = self.choice_child().unwrap();
				let type_ = child.spa_type();
				let choice_type = self.choice_type().ok_or_else(|| {
					Error::new(
						LibraryErrorEnum::InvalidArgument,
						&format!("unknown choice type for {:?}", child),
					)
				})?;
				Value::Choice(match type_ {
					_ if child.is_int() => ChoiceValue::Int(child.pod_choice(choice_type)?),
					_ if child.is_long() => ChoiceValue::Long(child.pod_choice(choice_type)?),
					_ if child.is_float() => ChoiceValue::Float(child.pod_choice(choice_type)?),
					_ if child.is_double() => ChoiceValue::Double(child.pod_choice(choice_type)?),
					_ if child.is_id() => ChoiceValue::Id(child.pod_choice(choice_type)?),
					_ if child.is_fd() => ChoiceValue::Fd(child.pod_choice(choice_type)?),
					_ if child.is_rectangle() => ChoiceValue::Rectangle(child.pod_choice(choice_type)?),
					_ if child.is_fraction() => ChoiceValue::Fraction(child.pod_choice(choice_type)?),
					type_ =>
						return Err(Error::new(
							LibraryErrorEnum::InvalidArgument,
							&format!("unsupported SPA choice child type {:?}", type_),
						)),
				})
			},
			_ =>
				return Err(Error::new(
					LibraryErrorEnum::InvalidArgument,
					&format!("unsupported SPA type {:?}", self.spa_type()),
				)),
		})
	}

	pub fn debug(&self) -> DebugValue<'static, 'static> {
		DebugValue::with_value(self.to_pod_value().unwrap())
	}
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl TryInto<Value> for SpaPod {
	type Error = Error;

	fn try_into(self) -> Result<Value, Error> {
		self.to_pod_value()
	}
}

#[derive(Copy, Clone)]
struct DebugProperty<'a> {
	object: &'a Object,
	property: &'a Property,
}

impl<'a> DebugProperty<'a> {
	fn parent_values_table(&self) -> Option<SpaIdTable> {
		let type_ = SpaType::from_id(self.object.type_);
		type_.and_then(|ty| ty.values_table())
	}

	fn key_value(&self) -> Option<SpaIdValue> {
		self
			.parent_values_table()
			.and_then(|table| table.find_value(self.property.key))
	}

	fn key_table(&self) -> Option<SpaIdTable> {
		let key = self.key_value()?;
		let (key_type, table) = key.value_type();
		let key_type = key_type?;
		if key_type.into_glib() == libspa_sys::SPA_TYPE_Array {
			let (_, table) = key.array_item_type();
			table
		} else {
			table
		}
	}
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
pub struct DebugValue<'v, 'o> {
	container: Option<DebugProperty<'o>>,
	value: Cow<'v, Value>,
}

impl<'v, 'o> DebugValue<'v, 'o> {
	pub fn new(value: &'v Value) -> Self {
		Self {
			container: None,
			value: Cow::Borrowed(value),
		}
	}

	fn with_value(value: Value) -> Self {
		Self {
			container: None,
			value: Cow::Owned(value),
		}
	}
}

impl<'v, 'o> Debug for DebugValue<'v, 'o> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		struct DebugType(SpaType);
		impl Debug for DebugType {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				if let Some(name) = self.0.name() {
					write!(f, "{} ({:?})", name, self.0.into_glib())
				} else {
					write!(f, "{}", self.0.into_glib())
				}
			}
		}
		struct DebugIdValue(SpaIdValue);
		impl Debug for DebugIdValue {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				if let Some(short_name) = self.0.short_name() {
					write!(f, "{:?} ({:?})", short_name, self.0.number())
				} else {
					write!(f, "{}", self.0.number())
				}
			}
		}

		struct DebugValueList<I>(I);
		impl<'v, 'o, I: Clone + Iterator<Item = DebugValue<'v, 'o>>> Debug for DebugValueList<I> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_list().entries(self.0.clone()).finish()
			}
		}

		impl<'a> Debug for DebugProperty<'a> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				let mut f = f.debug_struct("Property");

				if let Some(key) = self.key_value() {
					f.field("key", &DebugIdValue(key));
				} else {
					f.field("key", &self.property.key);
				}
				f.field("flags", &self.property.flags)
					.field("value", &DebugValue {
						container: Some(DebugProperty {
							object: self.object,
							property: &self.property,
						}),
						value: Cow::Borrowed(&self.property.value),
					})
					.finish()
			}
		}
		struct DebugPropertyList<'a> {
			object: &'a Object,
		}
		impl<'a> Debug for DebugPropertyList<'a> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_list()
					.entries(self.object.properties.iter().map(|property| DebugProperty {
						object: self.object,
						property,
					}))
					.finish()
			}
		}
		struct DebugId<'a> {
			container: Option<DebugProperty<'a>>,
			id: Id,
		}
		impl<'a> Debug for DebugId<'a> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				let table = self.container.and_then(|prop| prop.key_table());
				if let Some(id) = table.and_then(|table| table.find_value(self.id.0)) {
					write!(f, "{:?}", DebugIdValue(id))
				} else {
					write!(f, "{:?}", self.id)
				}
			}
		}
		struct DebugIdList<'a> {
			container: Option<DebugProperty<'a>>,
			ids: &'a [Id],
		}
		impl<'a> Debug for DebugIdList<'a> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_list()
					.entries(self.ids.iter().copied().map(|id| DebugId {
						container: self.container,
						id,
					}))
					.finish()
			}
		}

		match &*self.value {
			&Value::Id(id) => f
				.debug_tuple("Id")
				.field(&DebugId {
					container: self.container,
					id,
				})
				.finish(),
			Value::ValueArray(ValueArray::Id(ids)) => f
				.debug_tuple("Ids")
				.field(&DebugIdList {
					container: self.container,
					ids,
				})
				.finish(),
			Value::Struct(values) => f
				.debug_tuple("Struct")
				.field(&DebugValueList(values.iter().map(|value| DebugValue {
					container: self.container,
					value: Cow::Borrowed(value),
				})))
				.finish(),
			Value::Object(obj) => {
				let type_ = SpaType::from_id(obj.type_);
				let id_table = type_.and_then(|ty| ty.object_id_values_table());

				let mut f = f.debug_struct("Object");
				if let Some(type_) = type_ {
					f.field("type", &DebugType(type_));
				} else {
					f.field("type", &obj.type_);
				}
				if let Some(id) = id_table.and_then(|table| table.find_value(obj.id)) {
					f.field("id", &DebugIdValue(id));
				} else {
					f.field("id", &obj.id);
				}
				f.field("properties", &DebugPropertyList { object: obj }).finish()
			},
			Value::Choice(choice) => {
				let mut f = f.debug_tuple("Choice");
				match choice {
					ChoiceValue::Id(id) => todo!(),
					choice => f.field(choice),
				}
				.finish()
			},
			value => write!(f, "{:?}", value),
		}
	}
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl SpaPrimitive for Id {
	const TYPE: SpaType = SpaType::ID;
}
#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl SpaValue for Id {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_id(self.0)
	}

	type Owned = Self;
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl<'a> TryFrom<&'a SpaPod> for Id {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.id().map(Id).ok_or(GlibNoneError)
	}
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl SpaPrimitive for Fd {
	const TYPE: SpaType = SpaType::FD;
}
#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl SpaValue for Fd {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_fd(self.0)
	}

	type Owned = Self;
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl<'a> TryFrom<&'a SpaPod> for Fd {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.fd().map(Fd).ok_or(GlibNoneError)
	}
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl SpaValue for Value {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_pod(todo!())
	}

	type Owned = Self;
}

#[cfg_attr(feature = "dox", doc(cfg(feature = "libspa")))]
impl<'a> TryFrom<&'a SpaPod> for Value {
	type Error = Error;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.to_pod_value()
	}
}
