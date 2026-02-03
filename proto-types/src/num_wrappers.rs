use crate::*;

use core::ops::*;
use num_traits::{Num, One, Zero};

/// Trait for numbers that represent protobuf integer types, such as `sint32` or `fixed64`.
pub trait ProtoIntWrapper:
  Num
  + Clone
  + Copy
  + Display
  + Debug
  + Eq
  + Ord
  + Hash
  + Default
  + Add<Self::Target>
  + Sub<Self::Target>
  + Mul<Self::Target>
  + Div<Self::Target>
  + Rem<Self::Target>
{
  type Target: Num + Clone + Copy + Display + Debug + Eq + Ord + Hash + Default;

  /// The name of the associated type (i.e. "sfixed32")
  fn name() -> &'static str;
}

macro_rules! impl_wrapper {
  ($name:ident, $target:ty) => {
    #[doc = "Wrapper struct that represents the protobuf type `"]
    #[doc = stringify!($name)]
    #[doc = "`. It implements [`ProtoIntWrapper`]."]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct $name(pub $target);

    impl ProtoIntWrapper for $name {
      type Target = $target;

      #[inline]
      fn name() -> &'static str {
        paste::paste! {
          stringify!([< $name:lower >])
        }
      }
    }

    impl Display for $name {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
      }
    }

    impl core::ops::Deref for $name {
      type Target = $target;
      #[inline]
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl core::cmp::PartialEq<$target> for $name {
      #[inline]
      fn eq(&self, other: &$target) -> bool {
        self.0 == *other
      }
    }

    impl core::ops::Add for $name {
      type Output = Self;

      #[inline]
      fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
      }
    }

    impl core::ops::Add<$target> for $name {
      type Output = $target;

      #[inline]
      fn add(self, rhs: $target) -> Self::Output {
        self.0 + rhs
      }
    }

    impl Zero for $name {
      #[inline]
      fn zero() -> Self {
        Self(0)
      }

      #[inline]
      fn is_zero(&self) -> bool {
        self.0 == 0
      }
    }

    impl One for $name {
      #[inline]
      fn one() -> Self {
        Self(1)
      }
    }

    impl core::ops::Mul for $name {
      type Output = Self;

      #[inline]
      fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
      }
    }

    impl core::ops::Mul<$target> for $name {
      type Output = $target;

      #[inline]
      fn mul(self, rhs: $target) -> Self::Output {
        self.0 * rhs
      }
    }

    impl core::ops::Div for $name {
      type Output = Self;

      #[inline]
      fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
      }
    }

    impl core::ops::Div<$target> for $name {
      type Output = $target;

      #[inline]
      fn div(self, rhs: $target) -> Self::Output {
        self.0 / rhs
      }
    }

    impl core::ops::Rem for $name {
      type Output = Self;

      #[inline]
      fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
      }
    }

    impl core::ops::Rem<$target> for $name {
      type Output = $target;

      #[inline]
      fn rem(self, rhs: $target) -> Self::Output {
        self.0 % rhs
      }
    }

    impl core::ops::Sub for $name {
      type Output = Self;

      #[inline]
      fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
      }
    }

    impl core::ops::Sub<$target> for $name {
      type Output = $target;

      #[inline]
      fn sub(self, rhs: $target) -> Self::Output {
        self.0 - rhs
      }
    }

    impl core::ops::DerefMut for $name {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
      }
    }

    impl Num for $name {
      type FromStrRadixErr = <$target as num_traits::Num>::FromStrRadixErr;

      #[inline]
      fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(<$target>::from_str_radix(str, radix)?))
      }
    }

    impl From<$name> for $target {
      #[inline]
      fn from(value: $name) -> $target {
        value.0
      }
    }

    impl AsRef<$target> for $name {
      #[inline]
      fn as_ref(&self) -> &$target {
        self
      }
    }

    impl core::borrow::Borrow<$target> for $name {
      #[inline]
      fn borrow(&self) -> &$target {
        self
      }
    }

    impl core::borrow::BorrowMut<$target> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut $target {
        self
      }
    }
  };
}

impl_wrapper!(Sint64, i64);
impl_wrapper!(Sint32, i32);
impl_wrapper!(Sfixed64, i64);
impl_wrapper!(Sfixed32, i32);
impl_wrapper!(Fixed64, u64);
impl_wrapper!(Fixed32, u32);
