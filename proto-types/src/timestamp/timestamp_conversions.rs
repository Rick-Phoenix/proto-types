#[cfg(feature = "chrono")]
mod chrono_impls {
	use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

	use crate::{Timestamp, timestamp::TimestampError};

	impl From<DateTime<Utc>> for Timestamp {
		#[inline]
		fn from(datetime: DateTime<Utc>) -> Self {
			let mut ts = Self {
				seconds: datetime.timestamp(),
				// Safe casting as this value is limited by chrono
				nanos: datetime.timestamp_subsec_nanos().cast_signed(),
			};
			ts.normalize();
			ts
		}
	}

	impl From<NaiveDateTime> for Timestamp {
		#[inline]
		fn from(datetime: NaiveDateTime) -> Self {
			let mut ts = Self {
				seconds: datetime.and_utc().timestamp(),
				// Safe casting as this value is limited by chrono
				nanos: datetime
					.and_utc()
					.timestamp_subsec_nanos()
					.cast_signed(),
			};
			ts.normalize();
			ts
		}
	}

	impl TryFrom<Timestamp> for DateTime<Utc> {
		type Error = TimestampError;

		#[inline]
		fn try_from(mut timestamp: Timestamp) -> Result<Self, Self::Error> {
			timestamp.normalize();

			u32::try_from(timestamp.nanos)
				.ok()
				.and_then(|nanos| Self::from_timestamp(timestamp.seconds, nanos))
				.ok_or(TimestampError::OutOfSystemRange(timestamp))
		}
	}

	impl TryFrom<Timestamp> for NaiveDateTime {
		type Error = TimestampError;

		#[inline]
		fn try_from(mut timestamp: Timestamp) -> Result<Self, Self::Error> {
			timestamp.normalize();

			u32::try_from(timestamp.nanos)
				.ok()
				.and_then(|nanos| DateTime::<Utc>::from_timestamp(timestamp.seconds, nanos))
				.map(|d| d.naive_local())
				.ok_or(TimestampError::OutOfSystemRange(timestamp))
		}
	}

	impl TryFrom<Timestamp> for DateTime<FixedOffset> {
		type Error = TimestampError;

		#[inline]
		fn try_from(mut timestamp: Timestamp) -> Result<Self, Self::Error> {
			timestamp.normalize();

			let chrono_utc: DateTime<Utc> = timestamp.try_into()?;

			Ok(chrono_utc.into())
		}
	}

	impl TryFrom<chrono::DateTime<chrono::FixedOffset>> for Timestamp {
		type Error = TimestampError;

		#[inline]
		fn try_from(dt: chrono::DateTime<chrono::FixedOffset>) -> Result<Self, Self::Error> {
			let seconds = dt.timestamp();
			let nanos = dt
				.timestamp_subsec_nanos()
				.try_into()
				.map_err(|_| TimestampError::InvalidDateTime)?;

			Ok(Self { seconds, nanos })
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::Timestamp;

	use alloc::string::ToString;
	use std::time::{SystemTime, UNIX_EPOCH};

	fn ts(s: i64, n: i32) -> Timestamp {
		Timestamp {
			seconds: s,
			nanos: n,
		}
	}

	// --- 1. SystemTime Conversions ---

	#[test]
	fn test_system_time_epoch() {
		// 1. Exact Epoch
		let t: Timestamp = UNIX_EPOCH.into();
		assert_eq!(t, ts(0, 0));

		// 2. Roundtrip Epoch
		let sys: SystemTime = ts(0, 0).try_into().unwrap();
		assert_eq!(sys, UNIX_EPOCH);
	}

	#[test]
	fn test_system_time_roundtrip() {
		let now = SystemTime::now();

		// SystemTime -> Timestamp
		let t: Timestamp = now.into();

		// Timestamp -> SystemTime
		let back: SystemTime = t
			.try_into()
			.expect("Timestamp should fit in SystemTime");

		let diff = now
			.duration_since(back)
			.unwrap_or_else(|e| e.duration());
		assert!(diff.as_nanos() < 1, "Roundtrip drifted significantly");
	}

	#[test]
	fn test_system_time_pre_epoch() {
		// 1969-12-31 23:59:59
		let pre_epoch = UNIX_EPOCH - core::time::Duration::from_secs(1);

		let t: Timestamp = pre_epoch.into();
		assert_eq!(t.seconds, -1);

		let back: SystemTime = t.try_into().unwrap();
		assert_eq!(back, pre_epoch);
	}

	// --- 2. String Parsing & Formatting (RFC 3339) ---

	#[test]
	fn test_display_rfc3339() {
		// 1. Epoch
		assert_eq!(ts(0, 0).to_string(), "1970-01-01T00:00:00Z");

		// 2. Nanos (Standard trimming logic expected)
		// 0.5s
		assert_eq!(ts(0, 500_000_000).to_string(), "1970-01-01T00:00:00.5Z");

		// 3. Pre-Epoch
		// 1969-12-31T23:59:59Z
		assert_eq!(ts(-1, 0).to_string(), "1969-12-31T23:59:59Z");
	}

	#[test]
	fn test_from_str_rfc3339() {
		use core::str::FromStr;

		// 1. Basic
		let t = Timestamp::from_str("1970-01-01T00:00:00Z").unwrap();
		assert_eq!(t, ts(0, 0));

		// 2. With Nanos
		let t = Timestamp::from_str("1970-01-01T00:00:00.123456789Z").unwrap();
		assert_eq!(t, ts(0, 123_456_789));
	}

	#[test]
	fn test_string_roundtrip() {
		let t = ts(1_600_000_000, 123_000_000);
		let s = t.to_string();
		let back: Timestamp = s.parse().unwrap();
		assert_eq!(t, back);
	}

	// --- 3. Chrono Integrations ---

	#[cfg(feature = "chrono")]
	mod chrono_tests {
		use super::*;
		use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};

		#[test]
		fn test_chrono_utc_roundtrip() {
			// 2024-01-01 12:00:00 UTC
			let dt = Utc
				.with_ymd_and_hms(2024, 1, 1, 12, 0, 0)
				.unwrap();

			// Into Timestamp
			let t: Timestamp = dt.into();
			assert_eq!(t.seconds, 1_704_110_400);
			assert_eq!(t.nanos, 0);

			// Back to Chrono
			let back: chrono::DateTime<Utc> = t.try_into().unwrap();
			assert_eq!(dt, back);
		}

		#[test]
		fn test_chrono_naive_utc_assumption() {
			// NaiveDateTime is assumed to be UTC when converting to Timestamp
			let naive = NaiveDate::from_ymd_opt(2024, 1, 1)
				.unwrap()
				.and_hms_opt(12, 0, 0)
				.unwrap();

			let t: Timestamp = naive.into();

			// Should match the UTC seconds from above
			assert_eq!(t.seconds, 1_704_110_400);

			// Roundtrip back
			let back_naive: NaiveDateTime = t.try_into().unwrap();
			assert_eq!(naive, back_naive);
		}

		#[test]
		fn test_fixed_offset_conversions() {
			use chrono::{FixedOffset, TimeZone};

			// 1. Success: Offset +05:00
			// 2024-01-01 12:00:00 +05:00 == 07:00:00 UTC
			let dt_offset = FixedOffset::east_opt(5 * 3600)
				.unwrap()
				.with_ymd_and_hms(2024, 1, 1, 12, 0, 0)
				.unwrap();

			let t: Timestamp = dt_offset
				.try_into()
				.expect("Should convert with normalization");

			// 12:00 local - 5h offset = 07:00 UTC.
			// Timestamp for 2024-01-01 07:00:00 UTC is 1704092400.
			// (1704110400 is 12:00 UTC)
			assert_eq!(t.seconds, 1_704_092_400);
		}
	}
}
