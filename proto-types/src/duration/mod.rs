mod base;
pub use base::DurationError;

use crate::Duration;

mod duration_impls;

mod formatting;

/// Structs for duration units such as Seconds and Minutes.
pub mod data {
  pub use super::{duration_data::*, duration_units::*};
}

mod duration_data;
mod duration_operations;
mod duration_units;

impl Duration {
  /// Whether the duration is negative or not.
  #[must_use]
  pub const fn is_negative(&self) -> bool {
    self.seconds < 0 || (self.seconds == 0 && self.nanos < 0)
  }

  #[must_use]
  pub fn new(seconds: i64, nanos: i32) -> Self {
    let mut instance = Self { seconds, nanos };
    instance.normalize();
    instance
  }
}

#[cfg(test)]
mod test {
  use crate::Duration;

  #[test]
  fn basic_tests() {
    let positive = Duration {
      seconds: 1,
      nanos: 0,
    };
    assert!(!positive.is_negative());

    let zero = Duration::default();
    assert!(!zero.is_negative());

    let negative = Duration::new(-1, 0);
    assert!(negative.is_negative());
  }
}
