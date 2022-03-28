#[macro_export]
macro_rules! newtype_wrapper {
	(
		$(#[$meta:meta])*
		$vis:vis struct $id:ident($ty:ty ;? $err:ty);
	) => {
		newtype_wrapper! {
			$(#[$meta])*
			$vis struct $id($ty | $ty ;? $err) as_variant into_variant;
		}

		impl<'a> glib::FromVariant for $id<'a> {
			fn from_variant(variant: &glib::Variant) -> Option<Self> {
				Self::try_from(variant.clone()).ok()
			}
		}

		impl<'a> std::str::FromStr for $id<'a> {
			type Err = <$ty as std::str::FromStr>::Err;

			fn from_str(str: &str) -> Result<Self, Self::Err> {
				<$ty as std::str::FromStr>::from_str(str)
					.and_then(|v| v.try_into().map_err(Into::into))
			}
		}

		impl<'a> std::fmt::Display for $id<'a> {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Display::fmt(self.as_variant(), f)
			}
		}
	};
	(
		$(#[$meta:meta])*
		$vis:vis struct $id:ident($ty:ty | $owned:ty $(;? $err:ty)? $(= $self:ident)?) $inner:ident $into_inner:ident;
	) => {
		$(#[$meta])*
		#[repr(transparent)]
		$vis struct $id<'a>(std::borrow::Cow<'a, $ty>);

		impl<'a> $id<'a> {
			$(
				pub fn wrap(v: $owned) -> Result<Self, $err> {
					v.try_into()
				}

				pub fn borrow(v: &'a $ty) -> Result<Self, $err> {
					v.try_into()
				}
			)?

			$(
				pub fn wrap(v: $owned) -> $self {
					v.into()
				}

				pub fn borrow(v: &'a $ty) -> $self {
					v.into()
				}
			)?

			pub fn borrowed<'s>(&'s self) -> Self where 's: 'a {
				Self(std::borrow::Cow::Borrowed(self.$inner()))
			}

			pub fn owned(&self) -> $id<'static> {
				$id(std::borrow::Cow::Owned(self.$inner().to_owned()))
			}

			pub fn $inner(&self) -> &$ty {
				match self.0 {
					std::borrow::Cow::Borrowed(s) => s,
					std::borrow::Cow::Owned(ref s) => s,
				}
			}

			pub fn inner_ref(&self) -> Result<&'a $ty, &$ty> {
				match self.0 {
					std::borrow::Cow::Borrowed(s) => Ok(s),
					std::borrow::Cow::Owned(ref s) => Err(s),
				}
			}

			pub fn inner_mut(&mut self) -> &mut $ty {
				self.0.to_mut()
			}

			pub fn $into_inner(self) -> $owned {
				self.into_inner().into_owned()
			}

			pub fn into_inner(self) -> std::borrow::Cow<'a, $ty> {
				self.0
			}
		}

		$(
			impl<'a> TryFrom<std::borrow::Cow<'a, $ty>> for $id<'a> {
				type Error = $err;

				fn try_from(v: std::borrow::Cow<'a, $ty>) -> Result<Self, $err> {
					Self::check(&v)
						.map(|()| unsafe {
							Self::unsafe_from(v)
						})
				}
			}

			impl<'a> TryFrom<$owned> for $id<'a> {
				type Error = $err;

				fn try_from(v: $owned) -> Result<Self, $err> {
					Self::check(&v)
						.map(|()| unsafe {
							Self::unsafe_from(v)
						})
				}
			}

			impl<'a> TryFrom<&'a $ty> for $id<'a> {
				type Error = $err;

				fn try_from(v: &'a $ty) -> Result<Self, $err> {
					Self::check(v)
						.map(|()| unsafe {
							Self::unsafe_from(v)
						})
				}
			}

			impl<'a> UnsafeFrom<std::borrow::Cow<'a, $ty>> for $id<'a> {
				unsafe fn unsafe_from(v: std::borrow::Cow<'a, $ty>) -> Self {
					Self(v)
				}
			}

			impl<'a> UnsafeFrom<&'a $ty> for $id<'a> {
				unsafe fn unsafe_from(v: &'a $ty) -> Self {
					Self(std::borrow::Cow::Borrowed(v))
				}
			}

			impl<'a> UnsafeFrom<$owned> for $id<'a> {
				unsafe fn unsafe_from(v: $owned) -> Self {
					Self(std::borrow::Cow::Owned(v))
				}
			}
		)*
		$(
			impl<'a> From<std::borrow::Cow<'a, $ty>> for $id<'a> {
				fn from(v: std::borrow::Cow<'a, $ty>) -> $self {
					Self(v)
				}
			}

			impl<'a> From<&'a $ty> for $id<'a> {
				fn from(v: &'a $ty) -> $self {
					Self(std::borrow::Cow::Borrowed(v))
				}
			}

			impl<'a> From<$owned> for $id<'a> {
				fn from(v: $owned) -> $self {
					Self(std::borrow::Cow::Owned(v))
				}
			}
		)*

		impl<'a> Into<$owned> for $id<'a> {
			fn into(self) -> $owned {
				self.$into_inner()
			}
		}

		impl<'a> Into<std::borrow::Cow<'a, $ty>> for $id<'a> {
			fn into(self) -> std::borrow::Cow<'a, $ty> {
				self.into_inner()
			}
		}

		impl<'a> AsRef<$ty> for $id<'a> {
			fn as_ref(&self) -> &$ty {
				self.$inner()
			}
		}

		impl<'a, T: AsRef<$ty>> PartialOrd<T> for $id<'a> {
			fn partial_cmp(&self, rhs: &T) -> Option<std::cmp::Ordering> {
				PartialOrd::partial_cmp(self.$inner(), rhs.as_ref())
			}
		}

		impl<'a, T: AsRef<$ty>> PartialEq<T> for $id<'a> {
			fn eq(&self, rhs: &T) -> bool {
				PartialEq::eq(self.$inner(), rhs.as_ref())
			}
		}
	};
}
