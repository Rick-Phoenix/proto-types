#![allow(clippy::option_map_unit_fn)]
use alloc::string::String;
use core::fmt::Write;

use super::data::DurationData;
use crate::{Duration, Vec};

impl core::fmt::Display for Duration {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let normalized = self.normalized();

		// 1. Handle Negative case
		// Note: If seconds is -0 and nanos is -500, we need to print "-0.000000500s"
		let is_negative = normalized.seconds < 0 || normalized.nanos < 0;

		if is_negative {
			write!(f, "-")?;
		}

		// Work with absolute values for printing
		let abs_seconds = normalized.seconds.abs();
		let mut abs_nanos = normalized.nanos.abs();

		write!(f, "{abs_seconds}")?;

		if abs_nanos > 0 {
			let mut width = 9;

			// Strip trailing zeros mathematically
			// e.g. 500_000_000 (width 9) -> 5 (width 1) -> prints ".5"
			// e.g. 000_000_500 (width 9) -> 5 (width 7) -> prints ".0000005"
			while abs_nanos % 10 == 0 {
				abs_nanos /= 10;
				width -= 1;
			}

			write!(f, ".{abs_nanos:0width$}")?;
		}

		write!(f, "s")
	}
}

impl Duration {
	/// Formats a duration in human readable form. (e.g. "2 days 15 hours 12 minutes and 15 seconds")
	#[must_use]
	pub fn to_human_readable_string(&self) -> String {
		let DurationData {
			months,
			days,
			hours,
			minutes,
			seconds,
			is_negative,
			..
		} = self.get_data();

		let mut str = String::new();

		let mut parts = Vec::new();

		months.format_if_nonzero().map(|p| parts.push(p));
		days.format_if_nonzero().map(|p| parts.push(p));
		hours.format_if_nonzero().map(|p| parts.push(p));
		minutes.format_if_nonzero().map(|p| parts.push(p));
		seconds.format_if_nonzero().map(|p| parts.push(p));

		if parts.is_empty() {
			str.push_str("0 seconds");
		} else {
			let sign = if is_negative { "- " } else { "" };

			match parts.len() {
				1 => str.push_str(&parts.remove(0)),
				2 => {
					let _ = write!(str, "{}{} and {}", sign, parts[0], parts[1]);
				}
				_ => {
					let last = parts.pop().unwrap();
					let _ = write!(str, "{}{} and {}", sign, parts.join(" "), last);
				}
			};
		}

		str
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::duration::duration_units::*;
	use alloc::string::ToString;

	fn dur(s: i64, n: i32) -> Duration {
		Duration {
			seconds: s,
			nanos: n,
		}
	}

	#[test]
	fn test_canonical_display() {
		// Simple
		let d = dur(10, 0);
		assert_eq!(d.to_string(), "10s");

		// Fractional
		let d = dur(10, 500_000_000);
		assert_eq!(d.to_string(), "10.5s");

		// Small Fractional
		let d = dur(0, 1_000);
		assert_eq!(d.to_string(), "0.000001s");

		// Negative
		let d = dur(-10, -500_000_000);
		assert_eq!(d.to_string(), "-10.5s");

		// Negative Zero
		let d = dur(0, -500_000_000);
		assert_eq!(d.to_string(), "-0.5s");
	}

	// --- 1. Unit Formatter Tests ---

	#[test]
	fn test_unit_display_formatting() {
		// Singular
		assert_eq!(Seconds { value: 1 }.to_string(), "1 second");
		assert_eq!(Minutes { value: 1 }.to_string(), "1 minute");
		assert_eq!(Hours { value: 1 }.to_string(), "1 hour");
		assert_eq!(Days { value: 1 }.to_string(), "1 day");
		assert_eq!(Weeks { value: 1 }.to_string(), "1 week");
		assert_eq!(Months { value: 1 }.to_string(), "1 month");
		assert_eq!(Years { value: 1 }.to_string(), "1 year");

		// Plural
		assert_eq!(Seconds { value: 2 }.to_string(), "2 seconds");
		assert_eq!(Seconds { value: 0 }.to_string(), "0 seconds");
		assert_eq!(Years { value: 10 }.to_string(), "10 years");
	}

	#[test]
	fn test_format_if_nonzero() {
		let s_zero = Seconds { value: 0 };
		let s_one = Seconds { value: 1 };

		assert_eq!(s_zero.format_if_nonzero(), None);
		assert_eq!(s_one.format_if_nonzero(), Some("1 second".to_string()));
	}

	// --- 2. get_data Decomposition Tests ---

	#[test]
	fn test_get_data_basic_units() {
		// 1 Minute
		let d = Duration {
			seconds: 60,
			nanos: 0,
		};
		let data = d.get_data();
		assert_eq!(data.minutes.value, 1);
		assert_eq!(data.seconds.value, 0);

		// 1 Hour
		let d = Duration {
			seconds: 3600,
			nanos: 0,
		};
		let data = d.get_data();
		assert_eq!(data.hours.value, 1);
		assert_eq!(data.minutes.value, 0);

		// 1 Day
		let d = Duration {
			seconds: 86400,
			nanos: 0,
		};
		let data = d.get_data();
		assert_eq!(data.days.value, 1);
		assert_eq!(data.hours.value, 0);
	}

	#[test]
	fn test_get_data_greedy_decomposition() {
		// 1 Day, 1 Hour, 1 Minute, 1 Second
		let total_seconds = 86400 + 3600 + 60 + 1;
		let d = Duration {
			seconds: total_seconds,
			nanos: 0,
		};

		let data = d.get_data();
		assert_eq!(data.days.value, 1);
		assert_eq!(data.hours.value, 1);
		assert_eq!(data.minutes.value, 1);
		assert_eq!(data.seconds.value, 1);
	}

	#[test]
	fn test_get_data_negative() {
		// -65 seconds -> 1 minute, 5 seconds (negative flag set)
		let d = Duration {
			seconds: -65,
			nanos: 0,
		};

		let data = d.get_data();
		assert!(data.is_negative);
		assert_eq!(data.minutes.value, 1);
		assert_eq!(data.seconds.value, 5);
	}

	// --- 3. Duration Display Tests ---

	#[test]
	fn test_duration_display_cases() {
		// Case 0: Zero
		let d = Duration {
			seconds: 0,
			nanos: 0,
		};
		assert_eq!(d.to_human_readable_string(), "0 seconds");

		// Case 1: Single unit
		let d = Duration {
			seconds: 10,
			nanos: 0,
		};
		assert_eq!(d.to_human_readable_string(), "10 seconds");

		// Case 2: Two units (use "and")
		// 1 minute (60) + 30 seconds
		let d = Duration {
			seconds: 90,
			nanos: 0,
		};
		assert_eq!(d.to_human_readable_string(), "1 minute and 30 seconds");

		// Case 3: Three+ units (spaces and "and" at the end)
		// 1 hour (3600) + 1 minute (60) + 1 second
		let d = Duration {
			seconds: 3661,
			nanos: 0,
		};
		assert_eq!(d.to_human_readable_string(), "1 hour 1 minute and 1 second");

		// Case 4: Skipping zero units
		// 1 hour (3600) + 5 seconds (no minutes)
		let d = Duration {
			seconds: 3605,
			nanos: 0,
		};
		assert_eq!(d.to_human_readable_string(), "1 hour and 5 seconds");
	}

	#[test]
	fn test_duration_display_negative() {
		// - 1 minute and 30 seconds
		let d = Duration {
			seconds: -90,
			nanos: 0,
		};
		assert_eq!(d.to_human_readable_string(), "- 1 minute and 30 seconds");
	}
}
