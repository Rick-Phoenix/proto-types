use core::ops::{Add, Div, Mul, Sub};

use crate::{Duration, constants::NANOS_PER_SECOND};

impl Add for Duration {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    self
      .checked_add(&rhs)
      .expect("Duration addition overflowed")
  }
}

impl Sub for Duration {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    self
      .checked_sub(&other)
      .expect("Duration subtraction overflowed")
  }
}

impl Mul<i64> for Duration {
  type Output = Self;

  fn mul(self, rhs: i64) -> Self {
    self
      .checked_mul(rhs)
      .expect("Duration multiplication by i64 overflowed")
  }
}

impl Mul<i32> for Duration {
  type Output = Self;

  fn mul(self, rhs: i32) -> Self {
    self
      .checked_mul(i64::from(rhs)) // Simply cast to i64 and use the i64 implementation
      .expect("Duration multiplication by i32 overflowed")
  }
}

impl Div<i64> for Duration {
  type Output = Self;

  fn div(self, rhs: i64) -> Self {
    self
      .checked_div(rhs)
      .expect("Duration division by i64 overflowed or divided by zero")
  }
}

impl Div<i32> for Duration {
  type Output = Self;

  fn div(self, rhs: i32) -> Self {
    self
      .checked_div(i64::from(rhs))
      .expect("Duration division by i32 overflowed or divided by zero")
  }
}

impl Duration {
  #[inline]
  #[must_use]
  pub const fn total_nanos(&self) -> i128 {
    (self.seconds as i128) * (crate::constants::NANOS_PER_SECOND as i128) + (self.nanos as i128)
  }

  /// Automatically handles normalization and overflow checking for i64 seconds.
  #[must_use]
  pub fn from_total_nanos(total: i128) -> Option<Self> {
    let factor = i128::from(NANOS_PER_SECOND);

    // Integer division truncates towards zero (correct for seconds)
    let seconds_val = total / factor;
    let seconds = i64::try_from(seconds_val).ok()?;

    // Remainder has same sign as dividend
    let nanos_val = total % factor;
    // Remainder guaranteed to fit in i32
    #[allow(clippy::cast_possible_truncation)]
    let nanos = nanos_val as i32;

    Some(Self { seconds, nanos })
  }

  /// Multiplies the Duration by an i64 scalar, returning `Some(Duration)` or `None` on overflow.
  #[must_use]
  pub fn checked_mul(&self, rhs: i64) -> Option<Self> {
    let total = self.total_nanos().checked_mul(i128::from(rhs))?;
    Self::from_total_nanos(total)
  }

  /// Adds another Duration to this one, returning `Some(Duration)` or `None` on overflow.
  #[must_use]
  pub fn checked_add(&self, other: &Self) -> Option<Self> {
    let total = self
      .total_nanos()
      .checked_add(other.total_nanos())?;
    Self::from_total_nanos(total)
  }

  /// Subtracts another Duration from this one, returning `Some(Duration)` or `None` on overflow.
  #[must_use]
  pub fn checked_sub(&self, other: &Self) -> Option<Self> {
    let total = self
      .total_nanos()
      .checked_sub(other.total_nanos())?;
    Self::from_total_nanos(total)
  }

  /// Divides the Duration by an i64 scalar, returning `Some(Duration)` or `None` on overflow.
  #[must_use]
  pub fn checked_div(&self, rhs: i64) -> Option<Self> {
    if rhs == 0 {
      return None;
    }
    let total = self.total_nanos().checked_div(i128::from(rhs))?;
    Self::from_total_nanos(total)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use core::cmp::Ordering;

  fn dur(s: i64, n: i32) -> Duration {
    Duration {
      seconds: s,
      nanos: n,
    }
  }

  #[test]
  fn test_arithmetic_normalization_via_math() {
    // 1. Add causing carry
    let d1 = dur(1, 900_000_000);
    let d2 = dur(0, 200_000_000);
    let sum = d1 + d2;
    assert_eq!(sum.seconds, 2);
    assert_eq!(sum.nanos, 100_000_000);

    // 2. Sub causing borrow
    let d1 = dur(2, 100_000_000);
    let d2 = dur(0, 200_000_000);
    let diff = d1 - d2;
    assert_eq!(diff.seconds, 1);
    assert_eq!(diff.nanos, 900_000_000);

    // 3. Sub causing negative result
    let d1 = dur(1, 0);
    let d2 = dur(2, 0);
    let diff = d1 - d2;
    // -1s
    assert_eq!(diff.seconds, -1);
    assert_eq!(diff.nanos, 0);
  }

  #[test]
  fn test_mul_overflow_checks() {
    // 1. Basic
    let d = dur(10, 0);
    assert_eq!(d * 2, dur(20, 0));

    // 2. Overflow i64 seconds via huge multiplier
    let huge = dur(i64::MAX / 2 + 100, 0);
    assert!(huge.checked_mul(2).is_none());

    // 3. Nanos overflow contributing to seconds overflow
    let edge = dur(i64::MAX, 0);
    assert!(edge.checked_mul(1).is_some());

    // i64::MAX s + 1s via nanos carry -> Overflow
    let edge_nanos = dur(i64::MAX - 1, 600_000_000);
    // * 2 -> (MAX-1)*2 s + 1.2s -> Overflow
    assert!(edge_nanos.checked_mul(2).is_none());
  }

  #[test]
  fn test_div_bug_fix() {
    // Before the i128-based impl, this would have failed
    let numerator = dur(10_000_000_000, 0); // 10B seconds
    let divisor = 11_000_000_000; // 11B

    // Expected: 0 seconds, ~0.909s (909,090,909 nanos)
    let res = numerator.checked_div(divisor).unwrap();
    assert_eq!(res.seconds, 0);
    // 10B * 1e9 / 11B = 10 * 1e9 / 11 = 909090909
    assert_eq!(res.nanos, 909_090_909);
  }

  #[test]
  fn test_div_edge_cases() {
    // Division by zero
    let d = dur(1, 0);
    assert!(d.checked_div(0).is_none());

    // Clean division
    let d = dur(1, 0);
    let res: Duration = d / 2;
    assert_eq!(res.seconds, 0);
    assert_eq!(res.nanos, 500_000_000);

    // Negative division
    let d = dur(-1, 0);
    let res: Duration = d / 2;
    assert_eq!(res.seconds, 0);
    assert_eq!(res.nanos, -500_000_000);
  }

  #[test]
  fn test_ord() {
    let d1 = dur(1, 500);
    let d2 = dur(1, 600);
    let d3 = dur(2, 0);

    assert!(d1 < d2);
    assert!(d2 < d3);

    // Test normalization-independent comparison
    // 0s + 1.5B nanos vs 1s + 500M nanos (Should be Equal)
    let unnormalized = dur(0, 1_500_000_000);
    let normalized = dur(1, 500_000_000);
    assert_eq!(unnormalized.cmp(&normalized), Ordering::Equal);
  }

  #[test]
  fn test_get_data_smoke_test() {
    // Just verifying the math helper runs without panic
    // 1 Year + 1 Month + 1 Day ...
    let d = dur(31_557_600 + 1, 0); // Approx 1 year + 1 sec
    let data = d.get_data();
    assert_eq!(data.years.value, 1);
  }
}
