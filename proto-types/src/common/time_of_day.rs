use core::{cmp::Ordering, fmt::Display};

use thiserror::Error;

use crate::{common::TimeOfDay, constants::NANOS_PER_SECOND};

const NANOS_PER_MINUTE: i64 = NANOS_PER_SECOND as i64 * 60;
const NANOS_PER_HOUR: i64 = NANOS_PER_MINUTE * 60;

impl Display for TimeOfDay {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{:02}:{:02}:{:02}",
      self.hours, self.minutes, self.seconds
    )?;

    if self.nanos > 0 {
      write!(f, ".{:09}", self.nanos)?;
    }
    Ok(())
  }
}

/// Errors that can occur during the creation, conversion or validation of a [`TimeOfDay`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum TimeOfDayError {
  #[error("Hours out of valid range (0-23)")]
  InvalidHours,
  #[error("Minutes out of valid range (0-59)")]
  InvalidMinutes,
  #[error("Seconds out of valid range (0-59)")]
  InvalidSeconds,
  #[error("Nanoseconds out of valid range (0-999,999,999)")]
  InvalidNanos,
  #[error("The values for this TimeOfDay are outside of the allowed range")]
  ConversionError,
}

#[cfg(feature = "chrono")]
impl From<chrono::NaiveTime> for TimeOfDay {
  #[inline]
  fn from(value: chrono::NaiveTime) -> Self {
    use chrono::Timelike;

    // SAFETY: castings well within the safe range
    Self {
      hours: value.hour().cast_signed(),
      minutes: value.minute().cast_signed(),
      seconds: value.second().cast_signed(),
      nanos: value.nanosecond().cast_signed(),
    }
  }
}

#[cfg(feature = "chrono")]
impl TryFrom<TimeOfDay> for chrono::NaiveTime {
  type Error = TimeOfDayError;
  #[inline]
  fn try_from(value: TimeOfDay) -> Result<Self, Self::Error> {
    let hours_u32: u32 = value
      .hours
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidHours)?;
    let minutes_u32: u32 = value
      .minutes
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidMinutes)?;
    let seconds_u32: u32 = value
      .seconds
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidSeconds)?;
    let nanos_u32: u32 = value
      .nanos
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidNanos)?;

    Self::from_hms_nano_opt(hours_u32, minutes_u32, seconds_u32, nanos_u32)
      .ok_or(TimeOfDayError::ConversionError)
  }
}

impl PartialOrd for TimeOfDay {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for TimeOfDay {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    // Directly use the i64 comparison.
    self
      .nanos_since_midnight()
      .cmp(&other.nanos_since_midnight())
  }
}

fn validate_time_of_day(
  hours: i32,
  minutes: i32,
  seconds: i32,
  nanos: i32,
) -> Result<(), TimeOfDayError> {
  if !((0..=23).contains(&hours)) {
    return Err(TimeOfDayError::InvalidHours);
  }
  if !((0..=59).contains(&minutes)) {
    return Err(TimeOfDayError::InvalidMinutes);
  }
  if !((0..=59).contains(&seconds)) {
    return Err(TimeOfDayError::InvalidSeconds);
  }
  if !((0..=999_999_999).contains(&nanos)) {
    return Err(TimeOfDayError::InvalidNanos);
  }

  Ok(())
}

impl TimeOfDay {
  /// Returns the total amount of nanoseconds since midnight for this instance.
  #[must_use]
  #[inline]
  pub const fn nanos_since_midnight(&self) -> i64 {
    self.hours as i64 * NANOS_PER_HOUR
      + self.minutes as i64 * NANOS_PER_MINUTE
      + self.seconds as i64 * NANOS_PER_SECOND as i64
      + self.nanos as i64
  }

  #[inline]
  /// Creates a new [`TimeOfDay`] instance with validation.
  pub fn new(hours: i32, minutes: i32, seconds: i32, nanos: i32) -> Result<Self, TimeOfDayError> {
    validate_time_of_day(hours, minutes, seconds, nanos)?;

    Ok(Self {
      hours,
      minutes,
      seconds,
      nanos,
    })
  }

  /// Checks if this [`TimeOfDay`] instance represents a valid time.
  #[must_use]
  #[inline]
  pub fn is_valid(&self) -> bool {
    validate_time_of_day(self.hours, self.minutes, self.seconds, self.nanos).is_ok()
  }

  pub const MIDNIGHT: Self = Self {
    hours: 0,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const ONE_AM: Self = Self {
    hours: 1,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const TWO_AM: Self = Self {
    hours: 2,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const THREE_AM: Self = Self {
    hours: 3,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const FOUR_AM: Self = Self {
    hours: 4,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const FIVE_AM: Self = Self {
    hours: 5,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const SIX_AM: Self = Self {
    hours: 6,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const SEVEN_AM: Self = Self {
    hours: 7,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const EIGHT_AM: Self = Self {
    hours: 8,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const NINE_AM: Self = Self {
    hours: 9,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const TEN_AM: Self = Self {
    hours: 10,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const ELEVEN_AM: Self = Self {
    hours: 11,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const NOON: Self = Self {
    hours: 12,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const ONE_PM: Self = Self {
    hours: 13,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const TWO_PM: Self = Self {
    hours: 14,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const THREE_PM: Self = Self {
    hours: 15,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const FOUR_PM: Self = Self {
    hours: 16,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const FIVE_PM: Self = Self {
    hours: 17,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const SIX_PM: Self = Self {
    hours: 18,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const SEVEN_PM: Self = Self {
    hours: 19,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const EIGHT_PM: Self = Self {
    hours: 20,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const NINE_PM: Self = Self {
    hours: 21,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const TEN_PM: Self = Self {
    hours: 22,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
  pub const ELEVEN_PM: Self = Self {
    hours: 23,
    minutes: 0,
    seconds: 0,
    nanos: 0,
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use alloc::string::ToString;

  fn t(h: i32, m: i32, s: i32, n: i32) -> Result<TimeOfDay, TimeOfDayError> {
    TimeOfDay::new(h, m, s, n)
  }

  #[test]
  fn test_validation() {
    // Valid boundaries
    assert!(t(0, 0, 0, 0).is_ok());
    assert!(t(23, 59, 59, 999_999_999).is_ok());

    // Invalid Hours
    assert_eq!(t(24, 0, 0, 0), Err(TimeOfDayError::InvalidHours));
    assert_eq!(t(-1, 0, 0, 0), Err(TimeOfDayError::InvalidHours));

    // Invalid Minutes
    assert_eq!(t(12, 60, 0, 0), Err(TimeOfDayError::InvalidMinutes));

    // Invalid Seconds
    assert_eq!(t(12, 0, 60, 0), Err(TimeOfDayError::InvalidSeconds));

    // Invalid Nanos
    assert_eq!(
      t(12, 0, 0, 1_000_000_000),
      Err(TimeOfDayError::InvalidNanos)
    );
  }

  #[test]
  fn test_constants() {
    let mid = TimeOfDay::MIDNIGHT;
    assert_eq!(mid.hours, 0);
    assert_eq!(mid.nanos_since_midnight(), 0);

    let noon = TimeOfDay::NOON;
    assert_eq!(noon.hours, 12);
    // 12 * 60 * 60 * 1e9
    assert_eq!(noon.nanos_since_midnight(), 12 * 3600 * 1_000_000_000);
  }

  #[test]
  fn test_ordering() {
    let t1 = t(10, 0, 0, 0).unwrap();
    let t2 = t(11, 0, 0, 0).unwrap();
    let t3 = t(10, 0, 0, 1).unwrap(); // 1ns later

    assert!(t1 < t2);
    assert!(t1 < t3); // Check nanos precision
    assert!(t3 < t2);
  }

  #[test]
  fn test_display() {
    // Standard
    let time = t(12, 30, 45, 0).unwrap();
    assert_eq!(time.to_string(), "12:30:45");

    // With Nanos
    let precise = t(12, 30, 45, 123).unwrap();
    // :09 formatting pads with zeros
    assert_eq!(precise.to_string(), "12:30:45.000000123");
  }

  #[cfg(feature = "chrono")]
  mod chrono_tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_conversion() {
      let time = t(15, 30, 0, 0).unwrap();
      let naive: NaiveTime = time.try_into().unwrap();
      assert_eq!(naive, NaiveTime::from_hms_opt(15, 30, 0).unwrap());

      let back: TimeOfDay = naive.into();
      assert_eq!(back, time);
    }
  }
}
