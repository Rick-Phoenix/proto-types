use crate::Duration;
use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Sub};
use core::time::Duration as StdDuration;

impl PartialEq<StdDuration> for Duration {
	#[inline]
	fn eq(&self, other: &StdDuration) -> bool {
		self.total_nanos() == other.as_nanos().cast_signed()
	}
}

impl PartialEq<Duration> for StdDuration {
	#[inline]
	fn eq(&self, other: &Duration) -> bool {
		other == self
	}
}

impl PartialOrd<StdDuration> for Duration {
	#[inline]
	fn partial_cmp(&self, other: &StdDuration) -> Option<Ordering> {
		Some(
			self.total_nanos()
				.cmp(&other.as_nanos().cast_signed()),
		)
	}
}

impl PartialOrd<Duration> for StdDuration {
	#[inline]
	fn partial_cmp(&self, other: &Duration) -> Option<Ordering> {
		other.partial_cmp(self).map(Ordering::reverse)
	}
}

#[cfg(feature = "chrono")]
impl PartialEq<chrono::TimeDelta> for Duration {
	#[inline]
	fn eq(&self, other: &chrono::TimeDelta) -> bool {
		let other_total = (i128::from(other.num_seconds()) * Self::NANOS_PER_SEC_I128)
			+ i128::from(other.subsec_nanos());

		self.total_nanos() == other_total
	}
}

#[cfg(feature = "chrono")]
impl PartialEq<Duration> for chrono::TimeDelta {
	#[inline]
	fn eq(&self, other: &Duration) -> bool {
		other == self
	}
}

#[cfg(feature = "chrono")]
impl PartialOrd<chrono::TimeDelta> for Duration {
	#[inline]
	fn partial_cmp(&self, other: &chrono::TimeDelta) -> Option<Ordering> {
		let other_total = (i128::from(other.num_seconds()) * Self::NANOS_PER_SEC_I128)
			+ i128::from(other.subsec_nanos());

		Some(self.total_nanos().cmp(&other_total))
	}
}

#[cfg(feature = "chrono")]
impl PartialOrd<Duration> for chrono::TimeDelta {
	#[inline]
	fn partial_cmp(&self, other: &Duration) -> Option<Ordering> {
		other.partial_cmp(self).map(Ordering::reverse)
	}
}

impl Add<StdDuration> for Duration {
	type Output = Self;

	#[inline]
	fn add(self, rhs: StdDuration) -> Self::Output {
		let rhs_s = i64::try_from(rhs.as_secs()).expect("overflow in duration addition");
		let rhs_n = i64::from(rhs.subsec_nanos());

		self.checked_add_raw(rhs_s, rhs_n)
			.expect("overflow in duration addition")
	}
}

impl Add for Duration {
	type Output = Self;
	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		self.checked_add(&rhs)
			.expect("overflow in duration addition")
	}
}

impl Sub for Duration {
	type Output = Self;

	#[inline]
	fn sub(self, other: Self) -> Self {
		self.checked_sub(&other)
			.expect("overflow in duration subtraction")
	}
}

impl Sub<StdDuration> for Duration {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: StdDuration) -> Self::Output {
		let rhs_s = i64::try_from(rhs.as_secs()).expect("overflow in duration subtraction");
		let rhs_n = i64::from(rhs.subsec_nanos());

		self.checked_sub_raw(rhs_s, rhs_n)
			.expect("overflow in duration subtraction")
	}
}

#[cfg(feature = "chrono")]
impl Add<chrono::TimeDelta> for Duration {
	type Output = Self;

	#[inline]
	fn add(self, rhs: chrono::TimeDelta) -> Self::Output {
		self.checked_add_raw(rhs.num_seconds(), i64::from(rhs.subsec_nanos()))
			.expect("overflow in duration addition")
	}
}

#[cfg(feature = "chrono")]
impl Sub<chrono::TimeDelta> for Duration {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: chrono::TimeDelta) -> Self::Output {
		self.checked_sub_raw(rhs.num_seconds(), i64::from(rhs.subsec_nanos()))
			.expect("overflow in duration subtraction")
	}
}

impl Mul<i64> for Duration {
	type Output = Self;

	#[inline]
	fn mul(self, rhs: i64) -> Self {
		self.checked_mul(rhs)
			.expect("Duration multiplication by i64 overflowed")
	}
}

impl Mul<i32> for Duration {
	type Output = Self;

	#[inline]
	fn mul(self, rhs: i32) -> Self {
		self.checked_mul(i64::from(rhs)) // Simply cast to i64 and use the i64 implementation
			.expect("Duration multiplication by i32 overflowed")
	}
}

impl Div<i64> for Duration {
	type Output = Self;

	#[inline]
	fn div(self, rhs: i64) -> Self {
		self.checked_div(rhs)
			.expect("Duration division by i64 overflowed or divided by zero")
	}
}

impl Div<i32> for Duration {
	type Output = Self;

	#[inline]
	fn div(self, rhs: i32) -> Self {
		self.checked_div(i64::from(rhs))
			.expect("Duration division by i32 overflowed or divided by zero")
	}
}

impl Duration {
	const NANOS_PER_SEC: i64 = 1_000_000_000;
	const NANOS_PER_SEC_I128: i128 = 1_000_000_000;

	fn align_signs(mut s: i64, mut n: i32) -> Option<Self> {
		if s > 0 && n < 0 {
			s = s.checked_sub(1)?;
			n += 1_000_000_000;
		} else if s < 0 && n > 0 {
			s = s.checked_add(1)?;
			n -= 1_000_000_000;
		}
		Some(Self {
			seconds: s,
			nanos: n,
		})
	}

	fn checked_add_raw(&self, rhs_s: i64, rhs_n: i64) -> Option<Self> {
		let mut s = self.seconds.checked_add(rhs_s)?;
		let mut n_total = i64::from(self.nanos) + rhs_n;

		if n_total >= 1_000_000_000 {
			s = s.checked_add(1)?;
			n_total -= 1_000_000_000;
		} else if n_total <= -1_000_000_000 {
			s = s.checked_sub(1)?;
			n_total += 1_000_000_000;
		}

		if s > 0 && n_total < 0 {
			s = s.checked_sub(1)?;
			n_total += 1_000_000_000;
		} else if s < 0 && n_total > 0 {
			s = s.checked_add(1)?;
			n_total -= 1_000_000_000;
		}

		Some(Self {
			seconds: s,
			#[allow(clippy::cast_possible_truncation)]
			nanos: n_total as i32,
		})
	}

	fn checked_sub_raw(&self, rhs_s: i64, rhs_n: i64) -> Option<Self> {
		let mut s = self.seconds.checked_sub(rhs_s)?;
		let mut n_total = i64::from(self.nanos) - rhs_n;

		if n_total >= Self::NANOS_PER_SEC {
			s = s.checked_add(1)?;
			n_total -= Self::NANOS_PER_SEC;
		} else if n_total <= -Self::NANOS_PER_SEC {
			s = s.checked_sub(1)?;
			n_total += Self::NANOS_PER_SEC;
		}

		#[allow(clippy::cast_possible_truncation)]
		Self::align_signs(s, n_total as i32)
	}

	/// Returns the total nanoseconds for this instance.
	#[inline]
	#[must_use]
	pub const fn total_nanos(&self) -> i128 {
		(self.seconds as i128) * (Self::NANOS_PER_SEC_I128) + (self.nanos as i128)
	}

	/// Creates a new normalized instance from a given amount of nanoseconds.
	#[must_use]
	#[inline]
	pub fn from_total_nanos(total: i128) -> Option<Self> {
		let factor = Self::NANOS_PER_SEC_I128;

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
	#[inline]
	pub fn checked_mul(&self, rhs: i64) -> Option<Self> {
		let total = self.total_nanos().checked_mul(i128::from(rhs))?;
		Self::from_total_nanos(total)
	}

	/// Adds another Duration to this one, returning `Some(Duration)` or `None` on overflow.
	#[must_use]
	#[inline]
	pub fn checked_add(&self, other: &Self) -> Option<Self> {
		self.checked_add_raw(other.seconds, other.nanos.into())
	}

	/// Subtracts another Duration from this one, returning `Some(Duration)` or `None` on overflow.
	#[must_use]
	#[inline]
	pub fn checked_sub(&self, other: &Self) -> Option<Self> {
		self.checked_sub_raw(other.seconds, other.nanos.into())
	}

	/// Divides the Duration by an i64 scalar, returning `Some(Duration)` or `None` on overflow.
	#[must_use]
	#[inline]
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

	macro_rules! get_duration {
		(duration, $secs:literal, $nanos:literal) => {
			Duration::new($secs, $nanos)
		};

		(std, $secs:literal, $nanos:literal) => {
			StdDuration::new($secs, $nanos)
		};

		(chrono, $secs:literal, $nanos:literal) => {
			TimeDelta::new($secs, $nanos).unwrap()
		};
	}

	macro_rules! test_ops {
		($duration:ident) => {
			#[test]
			fn test_add_sub() {
				// 1. Add causing carry
				let d1 = dur(1, 900_000_000);
				let d2 = get_duration!($duration, 0, 200_000_000);
				let sum = d1 + d2;
				assert_eq!(sum.seconds, 2);
				assert_eq!(sum.nanos, 100_000_000);

				// 2. Sub causing borrow
				let d1 = dur(2, 100_000_000);
				let d2 = get_duration!($duration, 0, 200_000_000);
				let diff = d1 - d2;
				assert_eq!(diff.seconds, 1);
				assert_eq!(diff.nanos, 900_000_000);

				// 3. Sub causing negative result
				let d1 = dur(1, 0);
				let d2 = get_duration!($duration, 2, 0);
				let diff = d1 - d2;
				// -1s
				assert_eq!(diff.seconds, -1);
				assert_eq!(diff.nanos, 0);
			}

			#[test]
			fn test_eq_ord() {
				let d1 = dur(1, 500);
				let d2 = get_duration!($duration, 1, 600);
				let d3 = get_duration!($duration, 2, 0);

				assert!(d1 < d2);
				assert!(d2 < d3);

				// Test normalization-independent comparison
				// 0s + 1.5B nanos vs 1s + 500M nanos (Should be Equal)
				let unnormalized = dur(0, 1_500_000_000);
				let normalized = get_duration!($duration, 1, 500_000_000);
				assert_eq!(unnormalized.partial_cmp(&normalized), Some(Ordering::Equal));
			}
		};
	}

	test_ops!(duration);

	mod std_test {
		use super::*;

		test_ops!(std);
	}

	#[cfg(feature = "chrono")]
	mod chrono_test {
		use super::*;
		use chrono::TimeDelta;

		test_ops!(chrono);
	}

	fn dur(s: i64, n: i32) -> Duration {
		Duration {
			seconds: s,
			nanos: n,
		}
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
	fn test_get_data_smoke_test() {
		// Just verifying the math helper runs without panic
		// 1 Year + 1 Month + 1 Day ...
		let d = dur(31_557_600 + 1, 0); // Approx 1 year + 1 sec
		let data = d.get_data();
		assert_eq!(data.years.value, 1);
	}
}
