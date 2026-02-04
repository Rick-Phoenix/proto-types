use core::{
	cmp::Ordering,
	ops::{Add, Sub},
	time::Duration as StdDuration,
};

use crate::{Duration, Timestamp};

impl<'b> Sub<&'b Duration> for &Timestamp {
	type Output = Timestamp;

	fn sub(self, rhs: &'b Duration) -> Self::Output {
		let duration = rhs.normalized();

		let mut new = Timestamp {
			seconds: self.seconds.saturating_sub(duration.seconds),
			nanos: self.nanos - duration.nanos,
		};

		new.normalize();

		new
	}
}

impl Sub<StdDuration> for Timestamp {
	type Output = Self;

	fn sub(self, rhs: StdDuration) -> Self::Output {
		let mut new = Self {
			seconds: self
				.seconds
				.saturating_sub(rhs.as_secs().cast_signed()),
			nanos: self
				.nanos
				.saturating_sub(rhs.subsec_nanos().cast_signed()),
		};

		new.normalize();

		new
	}
}

#[cfg(feature = "chrono")]
impl Sub<chrono::TimeDelta> for Timestamp {
	type Output = Self;

	fn sub(self, rhs: chrono::TimeDelta) -> Self::Output {
		let mut new = Self {
			seconds: self.seconds.saturating_sub(rhs.num_seconds()),
			nanos: self.nanos.saturating_sub(rhs.subsec_nanos()),
		};

		new.normalize();

		new
	}
}

impl Sub<Duration> for Timestamp {
	type Output = Self;
	#[inline]
	fn sub(self, rhs: Duration) -> Self::Output {
		<&Self as Sub<&Duration>>::sub(&self, &rhs)
	}
}

impl<'b> Sub<&'b Duration> for Timestamp {
	type Output = Self;
	#[inline]
	fn sub(self, rhs: &'b Duration) -> Self::Output {
		<&Self as Sub<&Duration>>::sub(&self, rhs)
	}
}

impl<'a> Sub<Duration> for &'a Timestamp {
	type Output = Timestamp;
	#[inline]
	fn sub(self, rhs: Duration) -> Self::Output {
		<&'a Timestamp as Sub<&Duration>>::sub(self, &rhs)
	}
}

impl<'b> Add<&'b Duration> for &Timestamp {
	type Output = Timestamp;

	fn add(self, rhs: &'b Duration) -> Self::Output {
		let duration = rhs.normalized();

		let mut new = Timestamp {
			seconds: self.seconds.saturating_add(duration.seconds),
			nanos: self.nanos + duration.nanos,
		};

		new.normalize();

		new
	}
}

impl Add<StdDuration> for Timestamp {
	type Output = Self;

	fn add(self, rhs: StdDuration) -> Self::Output {
		let mut new = Self {
			seconds: self
				.seconds
				.saturating_add(rhs.as_secs().cast_signed()),
			nanos: self
				.nanos
				.saturating_add(rhs.subsec_nanos().cast_signed()),
		};

		new.normalize();

		new
	}
}

#[cfg(feature = "chrono")]
impl Add<chrono::TimeDelta> for Timestamp {
	type Output = Self;

	fn add(self, rhs: chrono::TimeDelta) -> Self::Output {
		let mut new = Self {
			seconds: self.seconds.saturating_add(rhs.num_seconds()),
			nanos: self.nanos.saturating_add(rhs.subsec_nanos()),
		};

		new.normalize();

		new
	}
}

impl<'b> Add<&'b Duration> for Timestamp {
	type Output = Self;
	#[inline]
	fn add(self, rhs: &'b Duration) -> Self::Output {
		<&Self as Add<&Duration>>::add(&self, rhs)
	}
}

impl Add<Duration> for &Timestamp {
	type Output = Timestamp;
	#[inline]
	fn add(self, rhs: Duration) -> Self::Output {
		<Self as Add<&Duration>>::add(self, &rhs)
	}
}

impl Add<Duration> for Timestamp {
	type Output = Self;

	#[inline]
	fn add(self, rhs: Duration) -> Self::Output {
		&self + &rhs
	}
}

impl PartialOrd for Timestamp {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Timestamp {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		(self.seconds, self.nanos).cmp(&(other.seconds, other.nanos))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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
			fn test_simple_addition() {
				let t = Timestamp::new(100, 500);
				let d = get_duration!($duration, 50, 100);
				let res = t + d;
				assert_eq!(res, Timestamp::new(150, 600));
			}

			#[test]
			fn test_nano_overflow_addition() {
				let t = Timestamp::new(100, 900_000_000);
				let d = get_duration!($duration, 0, 200_000_000);
				let res = t + d;
				// 900M + 200M = 1.1B -> 1s + 100M
				assert_eq!(res, Timestamp::new(101, 100_000_000));
			}

			#[test]
			fn test_simple_subtraction() {
				let t = Timestamp::new(100, 500);
				let d = get_duration!($duration, 50, 100);
				let res = t - d;
				assert_eq!(res, Timestamp::new(50, 400));
			}

			#[test]
			fn test_subtraction_crossing_zero() {
				let t = Timestamp::new(100, 100);
				let d = get_duration!($duration, 200, 0);
				let res = t - d;
				// 100 - 200 = -100
				assert_eq!(res, Timestamp::new(-100, 100));
			}

			#[test]
			fn test_subtraction_borrowing_nanos() {
				// Case: (10s, 100ns) - (0s, 200ns)
				// Raw: 10s, -100ns
				// Normalized: 9s, 999_999_900ns
				let t = Timestamp::new(10, 100);
				let d = get_duration!($duration, 0, 200);
				let res = t - d;
				assert_eq!(res, Timestamp::new(9, 999_999_900));
			}

			#[test]
			fn test_add_saturation_max() {
				// i64::MAX + 1s should stay at MAX, NOT wrap to MIN
				let t = Timestamp::new(i64::MAX, 0);
				let d = get_duration!($duration, 1, 0);
				let res = t + d;

				assert_eq!(res.seconds, i64::MAX);
			}

			#[test]
			fn test_sub_saturation_min() {
				// i64::MIN - 1s should stay at MIN, NOT wrap to MAX
				let t = Timestamp::new(i64::MIN, 0);
				let d = get_duration!($duration, 1, 0);
				let res = t - d;

				assert_eq!(res.seconds, i64::MIN);
			}
		};
	}

	macro_rules! test_saturation {
		($duration:ident) => {
			#[test]
			fn test_add_saturation_with_nanos() {
				// i64::MAX + (0s, 2B nanos) -> i64::MAX + 2s -> Saturation
				let t = Timestamp::new(i64::MAX, 0);
				// Duration > 1s via nanos
				let d = get_duration!($duration, 0, 2_000_000_000);
				let res = t + d;

				assert_eq!(res.seconds, i64::MAX);
			}
		};
	}

	#[cfg(feature = "chrono")]
	mod chrono_test {
		use super::*;

		use chrono::TimeDelta;

		test_ops!(chrono);
	}

	test_ops!(duration);
	test_saturation!(duration);

	#[test]
	fn test_sub_double_negative_saturation() {
		// i64::MAX - (-1s) is effectively i64::MAX + 1s -> Should saturate at MAX
		let t = Timestamp::new(i64::MAX, 0);
		let d = Duration::new(-1, 0);
		let res = t - d;

		assert_eq!(res.seconds, i64::MAX);
	}

	#[test]
	fn test_add_negative_duration() {
		let t = Timestamp::new(100, 0);
		let d = Duration::new(-50, 0);
		let res = t + d;
		assert_eq!(res, Timestamp::new(50, 0));
	}

	mod std_duration {
		use super::*;

		test_ops!(std);
		test_saturation!(std);
	}
}
