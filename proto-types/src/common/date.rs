use core::{
	cmp::{Ord, Ordering, PartialOrd},
	fmt::Display,
};

use thiserror::Error;

use crate::{String, ToString, common::Date};

/// Errors that can occur during the creation, conversion or validation of a [`Date`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum DateError {
	#[error("{0}")]
	InvalidYear(String),
	#[error("{0}")]
	InvalidMonth(String),
	#[error("{0}")]
	InvalidDay(String),
	#[error("Date conversion error: {0}")]
	ConversionError(String),
}

impl Display for Date {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.kind() {
			DateKind::Full => write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day),
			DateKind::YearAndMonth => write!(f, "{:04}-{:02}", self.year, self.month),
			DateKind::YearOnly => write!(f, "{:04}", self.year),
			DateKind::MonthAndDay => write!(f, "{:02}-{:02}", self.month, self.day),
		}
	}
}

/// The kind of combinations that a [`Date`] can contain.
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum DateKind {
	/// A full date, with non-zero year, month, and day values
	Full,
	/// A year on its own, with zero month and day values
	YearOnly,
	/// A year and month value, with a zero day, such as a credit card expiration
	YearAndMonth,
	/// A month and day value, with a zero year, such as an anniversary
	MonthAndDay,
}

impl Date {
	/// Creates a new [`Date`] instance with validation.
	/// Allows `year: 0`, `month: 0`, `day: 0` as special cases described in the proto spec.
	/// Returns an error if any component is out of range or date is invalid (e.g., February 30th).
	pub fn new(year: i32, month: i32, day: i32) -> Result<Self, DateError> {
		validate_date(year, month, day)?;

		Ok(Self { year, month, day })
	}

	/// Returns the kind of values combination for this [`Date`]
	#[must_use]
	#[inline]
	pub const fn kind(&self) -> DateKind {
		if self.year != 0 && self.month == 0 && self.day == 0 {
			DateKind::YearOnly
		} else if self.year != 0 && self.month != 0 && self.day == 0 {
			DateKind::YearAndMonth
		} else if self.year == 0 && self.month != 0 && self.day != 0 {
			DateKind::MonthAndDay
		} else {
			DateKind::Full
		}
	}

	/// Checks if this [`Date`] instance represents a valid date according to its constraints.
	#[must_use]
	pub fn is_valid(&self) -> bool {
		validate_date(self.year, self.month, self.day).is_ok()
	}

	#[must_use]
	#[inline]
	pub const fn has_year(&self) -> bool {
		self.year != 0
	}

	/// Returns `true` if this [`Date`] only indicates a year.
	#[must_use]
	#[inline]
	pub const fn is_year_only(&self) -> bool {
		self.year != 0 && (self.month == 0 && self.day == 0)
	}

	/// Returns `true` if this [`Date`] only indicates a year and a month (i.e. for a credit card expiration date).
	#[must_use]
	#[inline]
	pub const fn is_year_and_month(&self) -> bool {
		self.year != 0 && self.month != 0 && self.day == 0
	}

	/// Returns `true` if this [`Date`] only indicates a month and a day, with no specific year.
	#[must_use]
	#[inline]
	pub const fn is_month_and_day(&self) -> bool {
		self.year == 0 && self.month != 0 && self.day != 0
	}
}

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if !(self.is_valid() && other.is_valid()) {
			return None;
		}

		let self_kind = self.kind();
		let other_kind = other.kind();

		if self_kind != other_kind {
			return None;
		}

		Some(
			self.year
				.cmp(&other.year)
				.then_with(|| self.month.cmp(&other.month))
				.then_with(|| self.day.cmp(&other.day)),
		)
	}
}

#[cfg(feature = "chrono")]
mod chrono_impls {
	use chrono::Utc;

	use super::validate_date;
	use crate::{Date, ToString, date::DateError, format};

	impl Date {
		/// Converts this [`Date`] to [`chrono::NaiveDate`]. It fails if the year, month or day are set to zero.
		pub fn to_naive_date(self) -> Result<::chrono::NaiveDate, DateError> {
			self.try_into()
		}

		#[cfg(any(feature = "std", feature = "chrono-wasm"))]
		/// Returns the current date.
		#[must_use]
		#[inline]
		pub fn today() -> Self {
			Utc::now().naive_utc().date().into()
		}
	}

	impl TryFrom<crate::Date> for chrono::NaiveDate {
		type Error = DateError;

		fn try_from(date: Date) -> Result<Self, Self::Error> {
			if date.year == 0 || date.month == 0 || date.day == 0 {
				return Err(DateError::ConversionError(
					"Cannot convert Date with year=0, month=0, or day=0 to NaiveDate".to_string(),
				));
			}

			validate_date(date.year, date.month, date.day)?;

			// Safe castings after validation
			Self::from_ymd_opt(
				date.year,
				date.month.cast_unsigned(),
				date.day.cast_unsigned(),
			)
			.ok_or_else(|| {
				DateError::ConversionError(format!(
					"Invalid date components for NaiveDate: Y:{}, M:{}, D:{}",
					date.year, date.month, date.day
				))
			})
		}
	}

	impl From<chrono::NaiveDate> for Date {
		#[inline]
		fn from(naive_date: chrono::NaiveDate) -> Self {
			use chrono::Datelike;
			// Casting is safe due to chrono's costructor API
			Self {
				year: naive_date.year(),
				month: naive_date.month().cast_signed(),
				day: naive_date.day().cast_signed(),
			}
		}
	}
}

const fn is_leap_year(year: i32) -> bool {
	(year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

pub(crate) const fn days_in_month(month: i32, year: i32) -> i32 {
	match month {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		4 | 6 | 9 | 11 => 30,
		2 => {
			// If year is 0, we assume it's a recurring date (like a birthday),
			// so we must allow Feb 29th.
			if year == 0 || is_leap_year(year) {
				29
			} else {
				28
			}
		}
		_ => 0,
	}
}

fn validate_date(year: i32, month: i32, day: i32) -> Result<(), DateError> {
	if !(0..=9999).contains(&year) {
		return Err(DateError::InvalidYear(
			"Invalid year value (must be within 0 and 9999)".to_string(),
		));
	}

	if !(0..=12).contains(&month) {
		return Err(DateError::InvalidMonth(
			"Invalid month value (must be within 0 and 12)".to_string(),
		));
	}

	if !(0..=31).contains(&day) {
		return Err(DateError::InvalidDay(
			"Invalid day value (must be within 0 and 31)".to_string(),
		));
	}

	if year == 0 {
		if month == 0 {
			return Err(DateError::InvalidMonth(
				"The month cannot be set to 0 if the year is also set to 0".to_string(),
			));
		}
		if day == 0 {
			return Err(DateError::InvalidDay(
				"The day cannot be set to 0 if the year is also set to 0".to_string(),
			));
		}
	} else if month == 0 {
		if day != 0 {
			return Err(DateError::InvalidMonth(
				"The month cannot be 0 if the day is set".to_string(),
			));
		}
		return Ok(());
	}

	if day != 0 {
		let max_days = days_in_month(month, year);
		if day > max_days {
			return Err(DateError::InvalidDay(alloc::format!(
				"Invalid day {day} for month {month} (max is {max_days} for year {year})"
			)));
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	fn date(y: i32, m: i32, d: i32) -> Result<Date, DateError> {
		Date::new(y, m, d)
	}

	#[test]
	fn test_date_kinds_creation() {
		// 1. Full Date
		let full = date(2024, 1, 15).unwrap();
		assert_eq!(full.kind(), DateKind::Full);
		assert_eq!(full.to_string(), "2024-01-15");
		assert!(full.is_valid());

		// 2. Year Only
		let year = date(2024, 0, 0).unwrap();
		assert_eq!(year.kind(), DateKind::YearOnly);
		assert_eq!(year.to_string(), "2024");
		assert!(year.is_year_only());

		// 3. Year and Month (Credit Card style)
		let ym = date(2025, 12, 0).unwrap();
		assert_eq!(ym.kind(), DateKind::YearAndMonth);
		assert_eq!(ym.to_string(), "2025-12");
		assert!(ym.is_year_and_month());

		// 4. Month and Day (Birthday style)
		let md = date(0, 5, 20).unwrap();
		assert_eq!(md.kind(), DateKind::MonthAndDay);
		assert_eq!(md.to_string(), "05-20");
		assert!(md.is_month_and_day());
	}

	#[test]
	fn test_validation_failures() {
		// Bounds checks
		assert!(matches!(date(-1, 1, 1), Err(DateError::InvalidYear(_))));
		assert!(matches!(date(10000, 1, 1), Err(DateError::InvalidYear(_))));
		assert!(matches!(date(2024, 13, 1), Err(DateError::InvalidMonth(_))));
		assert!(matches!(date(2024, 1, 32), Err(DateError::InvalidDay(_))));

		// Year=0, Month=0 -> Invalid
		assert!(matches!(date(0, 0, 5), Err(DateError::InvalidMonth(_))));

		// Year=0, Day=0 -> Invalid
		assert!(matches!(date(0, 5, 0), Err(DateError::InvalidDay(_))));

		// Year set, Month=0, Day set -> Invalid (Cannot have Day without Month)
		assert!(matches!(date(2024, 0, 5), Err(DateError::InvalidMonth(_))));
	}

	#[test]
	fn test_ordering() {
		// Same Kind Comparison
		let d1 = date(2024, 5, 10).unwrap();
		let d2 = date(2024, 5, 11).unwrap();
		let d3 = date(2025, 1, 1).unwrap();

		assert!(d1 < d2);
		assert!(d2 < d3);
		assert!(d1 < d3);

		// Year Only Comparison
		let y1 = date(2023, 0, 0).unwrap();
		let y2 = date(2024, 0, 0).unwrap();
		assert!(y1 < y2);

		// Different Kinds should return None (not comparable)
		let full = date(2024, 1, 1).unwrap();
		let year_only = date(2024, 0, 0).unwrap();
		assert_eq!(full.partial_cmp(&year_only), None);

		// Month-Day Comparison
		let md1 = date(0, 2, 1).unwrap();
		let md2 = date(0, 2, 2).unwrap();
		assert!(md1 < md2);
	}

	#[test]
	fn test_calendar_validation() {
		// Standard Months
		assert!(Date::new(2023, 1, 31).is_ok());
		assert!(Date::new(2023, 4, 30).is_ok());
		assert!(Date::new(2023, 4, 31).is_err()); // April has 30 days

		// Non-Leap Year
		assert!(Date::new(2023, 2, 28).is_ok());
		assert!(Date::new(2023, 2, 29).is_err()); // 2023 is not leap

		// Leap Year
		assert!(Date::new(2024, 2, 29).is_ok());
		assert!(Date::new(2024, 2, 30).is_err());

		// Century Leap Year rules
		assert!(Date::new(1900, 2, 29).is_err()); // 1900 not leap (div by 100)
		assert!(Date::new(2000, 2, 29).is_ok()); // 2000 is leap (div by 400)
	}

	#[test]
	fn test_special_zero_cases() {
		// YearOnly (Year set, Month 0, Day 0) - Should be OK now
		assert!(Date::new(2024, 0, 0).is_ok());

		// YearAndMonth (Year set, Month set, Day 0) - OK
		assert!(Date::new(2024, 2, 0).is_ok());

		// Invalid: Year set, Month 0, Day set
		assert!(Date::new(2024, 0, 5).is_err());

		// Recurrent Date (Year 0) - Leap Day
		// "Feb 29" without a year is a valid concept (e.g. "My birthday is Feb 29")
		assert!(Date::new(0, 2, 29).is_ok());
		assert!(Date::new(0, 2, 30).is_err());
	}

	#[cfg(feature = "chrono")]
	mod chrono_tests {
		use super::*;
		use chrono::NaiveDate;

		#[test]
		fn test_to_naive_date() {
			let d = date(2024, 2, 29).unwrap(); // Leap year
			let naive = d.to_naive_date().unwrap();
			assert_eq!(naive, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
		}

		#[test]
		fn test_from_naive_date() {
			let naive = NaiveDate::from_ymd_opt(2023, 10, 25).unwrap();
			let d: Date = naive.into();
			assert_eq!(d.year, 2023);
			assert_eq!(d.month, 10);
			assert_eq!(d.day, 25);
			assert_eq!(d.kind(), DateKind::Full);
		}
	}
}
