use core::cmp::Ordering;

use thiserror::Error;

use crate::{Duration, String, Timestamp, ToString, common::Interval, constants::NANOS_PER_SECOND};

/// Errors that can occur during the creation, conversion or validation of an [`Interval`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum IntervalError {
	#[error("Interval's end_time is before its start_time")]
	EndTimeBeforeStartTime,
	#[error("Interval conversion error: {0}")]
	ConversionError(String),
}

fn validate_interval(
	start: Option<Timestamp>,
	end: Option<Timestamp>,
) -> Result<(), IntervalError> {
	if start.is_some_and(|s| end.is_some_and(|e| e < s)) {
		Err(IntervalError::EndTimeBeforeStartTime)
	} else {
		Ok(())
	}
}

impl Interval {
	/// Creates a new [`Interval`] instance, checking that `end_time` is not before `start_time`.
	#[inline]
	pub fn new(
		start_time: Option<Timestamp>,
		end_time: Option<Timestamp>,
	) -> Result<Self, IntervalError> {
		validate_interval(start_time, end_time)?;

		Ok(Self {
			start_time,
			end_time,
		})
	}

	#[cfg(any(feature = "std", feature = "chrono-wasm"))]
	/// Creates an [`Interval`] going from now to the `end_time` specified.
	#[must_use]
	#[inline]
	pub fn from_now_to(end_time: Timestamp) -> Self {
		Self {
			start_time: Some(Timestamp::now()),
			end_time: Some(end_time),
		}
	}

	#[cfg(any(feature = "std", feature = "chrono-wasm"))]
	/// Creates a new [`Interval`] going from the specified `start_time` to the present moment.
	#[must_use]
	#[inline]
	pub fn from_start_to_now(start_time: Timestamp) -> Self {
		Self {
			start_time: Some(start_time),
			end_time: Some(Timestamp::now()),
		}
	}

	/// Checks that `end_time` is not before `start_time`.
	#[must_use]
	pub fn is_valid(&self) -> bool {
		validate_interval(self.start_time, self.end_time).is_ok()
	}

	/// Returns `true` if the `Interval` is empty (`start_time` equals `end_time`).
	#[must_use]
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.start_time
			.as_ref()
			.zip(self.end_time.as_ref())
			.map_or_else(|| false, |(start, end)| start == end)
	}

	/// Returns `true` if the `Interval` is unspecified (no `start_time` and no `end_time`)
	#[must_use]
	#[inline]
	pub const fn is_unspecified(&self) -> bool {
		self.start_time.is_none() && self.end_time.is_none()
	}
}

impl TryFrom<Interval> for Duration {
	type Error = IntervalError;
	fn try_from(value: Interval) -> Result<Self, Self::Error> {
		let result = value
			.start_time
			.zip(value.end_time)
			.map(|(start, end)| {
				let mut seconds_diff = end.seconds - start.seconds;
				let mut nanos_diff = end.nanos - start.nanos;

				if nanos_diff < 0 {
					seconds_diff -= 1;
					nanos_diff += NANOS_PER_SECOND;
				} else if nanos_diff >= NANOS_PER_SECOND {
					seconds_diff += 1;
					nanos_diff -= NANOS_PER_SECOND;
				}

				Self {
					seconds: seconds_diff,
					nanos: nanos_diff,
				}
			});

		result.ok_or(IntervalError::ConversionError(
			"Cannot convert to Duration due to missing start or end time".to_string(),
		))
	}
}

impl PartialOrd for Interval {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if !self.is_valid() || !other.is_valid() {
			return None;
		}

		// 1. Check Empty (Zero Duration)
		if self.is_empty() {
			return if other.is_empty() {
				Some(Ordering::Equal)
			} else {
				Some(Ordering::Less)
			};
		}
		if other.is_empty() {
			return Some(Ordering::Greater);
		}

		// We utilize the fact that TryFrom fails for infinite/open-ended intervals
		// and succeeds for bounded ones.
		let self_dur = Duration::try_from(*self);
		let other_dur = Duration::try_from(*other);

		match (self_dur, other_dur) {
			// Both Finite: Compare the actual time span
			(Ok(d1), Ok(d2)) => d1.partial_cmp(&d2),

			// Finite < Infinite
			(Ok(_), Err(_)) => Some(Ordering::Less),

			// Infinite > Finite
			(Err(_), Ok(_)) => Some(Ordering::Greater),

			// Infinite == Infinite
			// (Treat all infinite intervals as equal for sorting stability)
			(Err(_), Err(_)) => Some(Ordering::Equal),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn ts(s: i64) -> Timestamp {
		Timestamp {
			seconds: s,
			nanos: 0,
		}
	}

	#[test]
	fn test_constructor() {
		let t = ts(100);

		// Empty
		let empty = Interval::new(Some(t), Some(t)).unwrap();
		assert!(empty.is_empty());
		assert!(!empty.is_unspecified());

		// Unspecified
		let unspec = Interval::new(None, None).unwrap();
		assert!(unspec.is_unspecified());

		// Open Ended
		let open = Interval::new(Some(t), None).unwrap();
		assert!(!open.is_empty());
	}

	#[test]
	fn test_partial_ord_ranking() {
		let t0 = ts(0);
		let t10 = ts(10);
		let t20 = ts(20);

		let empty = Interval::new(Some(t0), Some(t0)).unwrap(); // Duration 0
		let finite_small = Interval::new(Some(t0), Some(t10)).unwrap(); // Duration 10
		let finite_large = Interval::new(Some(t0), Some(t20)).unwrap(); // Duration 20
		let infinite_end = Interval::new(Some(t0), None).unwrap(); // Duration Infinity
		let infinite_start = Interval::new(None, Some(t20)).unwrap(); // Duration Infinity
		let infinite_all = Interval::new(None, None).unwrap(); // Duration Infinity

		// 1. Empty < Finite
		assert!(empty < finite_small);

		// 2. Finite < Finite (Duration comparison)
		assert!(finite_small < finite_large);

		// 3. Finite < Infinite
		assert!(finite_large < infinite_end);
		assert!(finite_large < infinite_start);

		// 4. Infinite == Infinite (Stability)
		// Note: Even though "All Time" is conceptually larger than "From 2024",
		// without finite bounds we can't mathematically compare them, so Equality is safest.
		assert!(infinite_end.partial_cmp(&infinite_start) == Some(Ordering::Equal));
		assert!(infinite_end.partial_cmp(&infinite_all) == Some(Ordering::Equal));
	}
}
