use {
	crate::{
		prelude::*,
		spa::{SpaPod, SpaPodBuilder, SpaType},
	},
	glib::{ffi::gboolean, GString},
};

pub trait SpaPrimitive: SpaValue + Copy + Into<<Self as SpaValue>::Owned> {
	const TYPE: SpaType;
}

pub trait SpaValue {
	fn add_to_builder(&self, builder: &SpaPodBuilder);

	type Owned: for<'a> TryFrom<&'a SpaPod>;
}

impl SpaPrimitive for () {
	const TYPE: SpaType = SpaType::NONE;
}
impl SpaValue for () {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_none()
	}

	type Owned = Self;
}

impl<'a> TryFrom<&'a SpaPod> for () {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		if pod.is_none() {
			Ok(())
		} else {
			Err(GlibNoneError)
		}
	}
}

impl SpaValue for bool {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_boolean(*self)
	}

	type Owned = Self;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SpaBool(gboolean);

impl From<SpaBool> for bool {
	fn from(v: SpaBool) -> Self {
		unsafe { from_glib(v.0) }
	}
}

impl From<bool> for SpaBool {
	fn from(v: bool) -> Self {
		Self(v.into_glib())
	}
}

impl SpaPrimitive for SpaBool {
	const TYPE: SpaType = SpaType::BOOL;
}
impl SpaValue for SpaBool {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_boolean((*self).into())
	}

	type Owned = bool;
}

impl<'a> TryFrom<&'a SpaPod> for SpaBool {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.boolean().map(Into::into).ok_or(GlibNoneError)
	}
}

impl<'a> TryFrom<&'a SpaPod> for bool {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.boolean().ok_or(GlibNoneError)
	}
}

impl SpaPrimitive for i32 {
	const TYPE: SpaType = SpaType::INT;
}
impl SpaValue for i32 {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_int(*self)
	}

	type Owned = Self;
}

impl<'a> TryFrom<&'a SpaPod> for i32 {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.int().ok_or(GlibNoneError)
	}
}

impl<'a> TryFrom<&'a SpaPod> for u32 {
	type Error = Error;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		i32::try_from(pod)
			.map_err(|e| Error::new(LibraryErrorEnum::InvalidArgument, &format!("{e:?}")))
			.and_then(|v| {
				v.try_into()
					.map_err(|e| Error::new(LibraryErrorEnum::InvalidArgument, &format!("{e:?}")))
			})
	}
}

impl SpaPrimitive for i64 {
	const TYPE: SpaType = SpaType::LONG;
}
impl SpaValue for i64 {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_long(*self)
	}

	type Owned = Self;
}

impl<'a> TryFrom<&'a SpaPod> for i64 {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.long().ok_or(GlibNoneError)
	}
}

impl<'a> TryFrom<&'a SpaPod> for u64 {
	type Error = Error;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		i64::try_from(pod)
			.map_err(|e| Error::new(LibraryErrorEnum::InvalidArgument, &format!("{e:?}")))
			.and_then(|v| {
				v.try_into()
					.map_err(|e| Error::new(LibraryErrorEnum::InvalidArgument, &format!("{e:?}")))
			})
	}
}

impl SpaPrimitive for f32 {
	const TYPE: SpaType = SpaType::FLOAT;
}
impl SpaValue for f32 {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_float(*self)
	}

	type Owned = Self;
}

impl<'a> TryFrom<&'a SpaPod> for f32 {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.float().ok_or(GlibNoneError)
	}
}

impl SpaPrimitive for f64 {
	const TYPE: SpaType = SpaType::DOUBLE;
}
impl SpaValue for f64 {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_double(*self)
	}

	type Owned = f64;
}

impl<'a> TryFrom<&'a SpaPod> for f64 {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.double().ok_or(GlibNoneError)
	}
}

impl SpaValue for str {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_string(self)
	}

	type Owned = GString;
}

impl<'a> TryFrom<&'a SpaPod> for GString {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.string().ok_or(GlibNoneError)
	}
}

impl<'a> TryFrom<&'a SpaPod> for String {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		<GString as TryFrom<&'a SpaPod>>::try_from(pod).map(Into::into)
	}
}

impl SpaValue for [u8] {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_bytes(self)
	}

	type Owned = Vec<u8>;
}

impl<'a> TryFrom<&'a SpaPod> for Vec<u8> {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.bytes().map(Into::into).ok_or(GlibNoneError)
	}
}

impl SpaPrimitive for libspa_sys::spa_rectangle {
	const TYPE: SpaType = SpaType::RECTANGLE;
}
impl SpaValue for libspa_sys::spa_rectangle {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_rectangle(self.width, self.height)
	}

	type Owned = Self;
}

impl<'a> TryFrom<&'a SpaPod> for libspa_sys::spa_rectangle {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.spa_rectangle().ok_or(GlibNoneError)
	}
}

impl SpaPrimitive for libspa_sys::spa_fraction {
	const TYPE: SpaType = SpaType::FRACTION;
}
impl SpaValue for libspa_sys::spa_fraction {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_fraction(self.num, self.denom)
	}

	type Owned = Self;
}

impl<'a> TryFrom<&'a SpaPod> for libspa_sys::spa_fraction {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		pod.spa_fraction().ok_or(GlibNoneError)
	}
}

impl SpaValue for SpaPod {
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		builder.add_pod(self)
	}

	type Owned = Self;
}

impl<'a> TryFrom<&'a SpaPod> for SpaPod {
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		Ok(pod.copy().unwrap())
	}
}

impl<T: SpaPrimitive> SpaValue for [T]
where
	Vec<T::Owned>: for<'a> TryFrom<&'a SpaPod>,
{
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		let struct_ = SpaPodBuilder::new_array();
		for item in self {
			item.add_to_builder(&struct_);
		}
		if let Some(pod) = struct_.end() {
			builder.add_pod(&pod)
		} else {
			wp_critical!("failed to build spa struct with {struct_:?}")
		}
	}

	type Owned = Vec<T::Owned>;
}

impl<'a, T: for<'f> TryFrom<&'f SpaPod>> TryFrom<&'a SpaPod> for Vec<T>
where
	for<'f> <T as TryFrom<&'f SpaPod>>::Error: Into<GlibNoneError>,
{
	type Error = GlibNoneError;

	fn try_from(pod: &'a SpaPod) -> Result<Self, Self::Error> {
		if !pod.is_struct() {
			return Err(GlibNoneError)
		}

		pod
			.iterator()
			.into_iter()
			.map(|pod| T::try_from(&pod).map_err(Into::into))
			.collect()
	}
}

/// Struct
impl<'a, T: Sized> SpaValue for [&'a dyn SpaValue<Owned = T>]
where
	Vec<T>: for<'f> TryFrom<&'f SpaPod>,
	T: for<'f> TryFrom<&'f SpaPod>,
{
	fn add_to_builder(&self, builder: &SpaPodBuilder) {
		let struct_ = SpaPodBuilder::new_struct();
		for &item in self {
			SpaValue::add_to_builder(item, &struct_);
		}
		if let Some(pod) = struct_.end() {
			builder.add_pod(&pod)
		} else {
			wp_critical!("failed to build spa struct with {struct_:?}")
		}
	}

	type Owned = Vec<T>;
}
