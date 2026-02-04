use alloc::string::ToString;

use crate::String;

macro_rules! impl_display {
  ($($name:ident),*) => {
    paste::paste! {
      $(
        impl core::fmt::Display for [< $name s >] {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {}{}", self.value, stringify!([< $name:lower >]), if self.value != 1 { "s" } else { "" })
          }
        }
      )*
    }
  };
}

impl_display!(Second, Minute, Hour, Day, Week, Month, Year);

/// A struct representing seconds. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seconds {
	pub value: u64,
}

impl Seconds {
	/// Returns a string with the amount of seconds, but only if the amount is more than 0.
	#[must_use]
	pub fn format_if_nonzero(&self) -> Option<String> {
		if self.is_zero() {
			return None;
		}
		Some(self.to_string())
	}

	const fn is_zero(&self) -> bool {
		self.value == 0
	}
}

/// A struct representing minutes. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Minutes {
	pub value: u64,
}

/// Returns a string displaying the amount of minutes (e.g. "1 minute", "2 minutes")
impl Minutes {
	/// Returns a string with the amount of minutes, but only if the amount is more than 0.
	#[must_use]
	pub fn format_if_nonzero(&self) -> Option<String> {
		if self.is_zero() {
			return None;
		}
		Some(self.to_string())
	}

	const fn is_zero(&self) -> bool {
		self.value == 0
	}
}

/// A struct representing hours. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hours {
	pub value: u64,
}

/// Returns a string displaying the amount of hours (e.g. "1 hour", "2 hours")
impl Hours {
	/// Returns a string with the amount of hours, but only if the amount is more than 0.
	#[must_use]
	pub fn format_if_nonzero(&self) -> Option<String> {
		if self.is_zero() {
			return None;
		}
		Some(self.to_string())
	}

	const fn is_zero(&self) -> bool {
		self.value == 0
	}
}

/// A struct representing days. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Days {
	pub value: u64,
}

/// Returns a string displaying the amount of days (e.g. "1 day", "2 days")
impl Days {
	/// Returns a string with the amount of days, but only if the amount is more than 0.
	#[must_use]
	pub fn format_if_nonzero(&self) -> Option<String> {
		if self.is_zero() {
			return None;
		}
		Some(self.to_string())
	}

	const fn is_zero(&self) -> bool {
		self.value == 0
	}
}

/// A struct representing weeks. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weeks {
	pub value: u64,
}

impl Weeks {
	/// Returns a string with the amount of weeks, but only if the amount is more than 0.
	#[must_use]
	pub fn format_if_nonzero(&self) -> Option<String> {
		if self.is_zero() {
			return None;
		}
		Some(self.to_string())
	}

	/// Returns `true` if the value is zero.
	const fn is_zero(&self) -> bool {
		self.value == 0
	}
}

/// A struct representing months. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Months {
	pub value: u64,
}

impl Months {
	/// Returns a string with the amount of months, but only if the amount is more than 0.
	#[must_use]
	pub fn format_if_nonzero(&self) -> Option<String> {
		if self.is_zero() {
			return None;
		}
		Some(self.to_string())
	}

	const fn is_zero(&self) -> bool {
		self.value == 0
	}
}

/// A struct representing years. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Years {
	pub value: u64,
}

impl Years {
	/// Returns a string with the amount of years, but only if the amount is more than 0.
	#[must_use]
	pub fn format_if_nonzero(&self) -> Option<String> {
		if self.is_zero() {
			return None;
		}
		Some(self.to_string())
	}

	const fn is_zero(&self) -> bool {
		self.value == 0
	}
}
