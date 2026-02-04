use core::{cmp::Ordering, fmt::Display};

use thiserror::Error;

use crate::common::Fraction;

impl Display for Fraction {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}

/// Errors that can occur during the creation, conversion or validation of a [`Fraction`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum FractionError {
	#[error("Denominator cannot be zero")]
	ZeroDenominator,
	#[error("Fraction arithmetic operation resulted in an overflow")]
	Overflow,
	#[error("Fraction arithmetic operation resulted in an undefined state")]
	Undefined,
}

impl Fraction {
	/// Helper to calculate Greatest Common Divisor (GCD).
	///
	/// Returns `u64` to correctly handle the edge case where the GCD is `i64::MIN.abs()` (2^63),
	/// which cannot be represented in a positive `i64`.
	#[must_use]
	#[inline]
	pub const fn gcd(a: i64, b: i64) -> u64 {
		let mut ua = a.unsigned_abs();
		let mut ub = b.unsigned_abs();

		while ub != 0 {
			let temp = ub;
			ub = ua % ub;
			ua = temp;
		}
		ua
	}

	/// Helper to calculate Least Common Multiple (LCM)
	#[inline]
	pub fn lcm(a: i64, b: i64) -> Result<i128, FractionError> {
		if a == 0 || b == 0 {
			return Err(FractionError::ZeroDenominator);
		}
		let common_divisor = i128::from(Self::gcd(a, b));
		let val_a = i128::from(a);
		let val_b = i128::from(b);

		let term1 = val_a
			.checked_div(common_divisor)
			.ok_or(FractionError::Overflow)?;
		term1
			.checked_mul(val_b)
			.ok_or(FractionError::Overflow)
	}

	/// Creates a new Fraction, ensuring the denominator is positive
	/// and the fraction is reduced to its simplest form.
	#[inline]
	pub const fn new(numerator: i64, denominator: i64) -> Result<Self, FractionError> {
		if denominator == 0 {
			return Err(FractionError::ZeroDenominator);
		}

		// Safety Check: i64::MIN cannot be negated to make denominator positive
		if denominator == i64::MIN {
			return Err(FractionError::Overflow);
		}
		// Safety Check: If denominator is negative, we must flip numerator.
		// If numerator is i64::MIN, it cannot be flipped.
		if denominator < 0 && numerator == i64::MIN {
			return Err(FractionError::Overflow);
		}

		let (mut num, mut den) = (numerator, denominator);

		// Ensure denominator is positive
		if den < 0 {
			num = -num;
			den = -den;
		}

		let common_divisor = Self::gcd(num, den);

		// We cast to i64 here.
		// Since 'den' is positive i64 (checked above), the GCD cannot exceed 'den'.
		// Therefore common_divisor fits in i64.
		let cd = common_divisor.cast_signed();

		Ok(Self {
			numerator: num / cd,
			denominator: den / cd,
		})
	}

	/// Reduces the fraction to its simplest form.
	///
	/// # Safety behavior
	/// If the fraction contains `i64::MIN` in the denominator (which is invalid state),
	/// or `i64::MIN` in the numerator with a negative denominator, this function
	/// will **silently return** without modifying the struct, to avoid panicking.
	#[inline]
	pub const fn reduce(&mut self) {
		if self.denominator == 0 {
			return;
		}

		// Edge Case Protection:
		// We cannot normalize signs if values are i64::MIN
		if self.denominator == i64::MIN {
			return;
		}
		if self.denominator < 0 && self.numerator == i64::MIN {
			return;
		}

		// Ensure denominator is positive
		if self.denominator < 0 {
			self.numerator = -self.numerator;
			self.denominator = -self.denominator;
		}

		let common_divisor = Self::gcd(self.numerator, self.denominator);

		// Since self.denominator is a valid positive i64, the GCD fits in i64.
		let cd = common_divisor.cast_signed();

		self.numerator /= cd;
		self.denominator /= cd;
	}

	/// Returns a new, reduced Fraction.
	#[must_use]
	#[inline]
	pub const fn reduced(mut self) -> Self {
		self.reduce();
		self
	}

	/// Checked addition for [`Fraction`]s.
	#[inline]
	pub fn checked_add(self, other: Self) -> Result<Self, FractionError> {
		let common_denominator_i128 = Self::lcm(self.denominator, other.denominator)?;

		let factor_self = common_denominator_i128
			.checked_div(i128::from(self.denominator))
			.ok_or(FractionError::Overflow)?;

		let factor_other = common_denominator_i128
			.checked_div(i128::from(other.denominator))
			.ok_or(FractionError::Overflow)?;

		let new_numerator_left = i128::from(self.numerator)
			.checked_mul(factor_self)
			.ok_or(FractionError::Overflow)?;

		let new_numerator_right = i128::from(other.numerator)
			.checked_mul(factor_other)
			.ok_or(FractionError::Overflow)?;

		let new_numerator = new_numerator_left
			.checked_add(new_numerator_right)
			.ok_or(FractionError::Overflow)?;

		let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
		let den_i64 =
			i64::try_from(common_denominator_i128).map_err(|_| FractionError::Overflow)?;

		Self::new(num_i64, den_i64)
	}

	/// Checked subtraction for [`Fraction`]s.
	#[inline]
	pub fn checked_sub(self, other: Self) -> Result<Self, FractionError> {
		let common_denominator_i128 = Self::lcm(self.denominator, other.denominator)?;

		let factor_self = common_denominator_i128
			.checked_div(i128::from(self.denominator))
			.ok_or(FractionError::Overflow)?;

		let factor_other = common_denominator_i128
			.checked_div(i128::from(other.denominator))
			.ok_or(FractionError::Overflow)?;

		let new_numerator_left = i128::from(self.numerator)
			.checked_mul(factor_self)
			.ok_or(FractionError::Overflow)?;

		let new_numerator_right = i128::from(other.numerator)
			.checked_mul(factor_other)
			.ok_or(FractionError::Overflow)?;

		let new_numerator = new_numerator_left
			.checked_sub(new_numerator_right)
			.ok_or(FractionError::Overflow)?;

		let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
		let den_i64 =
			i64::try_from(common_denominator_i128).map_err(|_| FractionError::Overflow)?;

		Self::new(num_i64, den_i64)
	}

	/// Checked multiplication for [`Fraction`]s.
	#[inline]
	pub fn checked_mul(self, other: Self) -> Result<Self, FractionError> {
		let new_numerator = i128::from(self.numerator)
			.checked_mul(i128::from(other.numerator))
			.ok_or(FractionError::Overflow)?;

		let new_denominator = i128::from(self.denominator)
			.checked_mul(i128::from(other.denominator))
			.ok_or(FractionError::Overflow)?;

		let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
		let den_i64 = i64::try_from(new_denominator).map_err(|_| FractionError::Overflow)?;

		Self::new(num_i64, den_i64)
	}

	/// Checked division for [`Fraction`]s.
	#[inline]
	pub fn checked_div(self, other: Self) -> Result<Self, FractionError> {
		if other.numerator == 0 {
			return Err(FractionError::Undefined);
		}

		let new_numerator = i128::from(self.numerator)
			.checked_mul(i128::from(other.denominator))
			.ok_or(FractionError::Overflow)?;

		let new_denominator = i128::from(self.denominator)
			.checked_mul(i128::from(other.numerator))
			.ok_or(FractionError::Overflow)?;

		let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
		let den_i64 = i64::try_from(new_denominator).map_err(|_| FractionError::Overflow)?;

		Self::new(num_i64, den_i64)
	}

	/// Converts the fraction to an `f64`.
	///
	/// # Panics
	/// Panics if the denominator is zero. This should not happen for [`Fraction`]
	/// instances created via [`Fraction::new()`] or other checked arithmetic,
	/// but can occur if a [`Fraction`] is constructed directly in an invalid state.
	///
	/// For a fallible conversion that returns a `Result`, use `TryFrom<Fraction> for f64`.
	#[must_use]
	#[inline]
	pub fn to_f64_unchecked(self) -> f64 {
		self.try_into().unwrap()
	}
}

impl PartialOrd for Fraction {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.denominator <= 0 || other.denominator <= 0 {
			return None;
		}
		let self_val = i128::from(self.numerator) * i128::from(other.denominator);
		let other_val = i128::from(other.numerator) * i128::from(self.denominator);

		Some(self_val.cmp(&other_val))
	}
}

impl TryFrom<Fraction> for f64 {
	type Error = FractionError;
	#[inline]
	fn try_from(fraction: Fraction) -> Result<Self, Self::Error> {
		if fraction.denominator == 0 {
			return Err(FractionError::ZeroDenominator);
		}

		let num_f64 = fraction.numerator as Self;
		let den_f64 = fraction.denominator as Self;

		Ok(num_f64 / den_f64)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn frac(n: i64, d: i64) -> Result<Fraction, FractionError> {
		Fraction::new(n, d)
	}

	#[test]
	fn test_creation_and_reduction() {
		// Standard reduction
		let f = frac(2, 4).unwrap();
		assert_eq!(f.numerator, 1);
		assert_eq!(f.denominator, 2);

		// Negative denominator normalization
		let f = frac(1, -2).unwrap();
		assert_eq!(f.numerator, -1);
		assert_eq!(f.denominator, 2);

		// Double negative
		let f = frac(-2, -4).unwrap();
		assert_eq!(f.numerator, 1);
		assert_eq!(f.denominator, 2);

		// Zero
		let f = frac(0, 5).unwrap();
		assert_eq!(f.numerator, 0);
		assert_eq!(f.denominator, 1); // 0/5 reduces to 0/1
	}

	#[test]
	fn test_creation_edge_cases() {
		// Zero denom
		assert_eq!(frac(1, 0), Err(FractionError::ZeroDenominator));

		// i64::MIN Numerator (valid if denom positive)
		let f = frac(i64::MIN, 1).unwrap();
		assert_eq!(f.numerator, i64::MIN);
		assert_eq!(f.denominator, 1);

		// i64::MIN Denominator (Invalid, cannot be positive)
		assert_eq!(frac(1, i64::MIN), Err(FractionError::Overflow));

		// i64::MIN Numerator with Negative Denom (Invalid, needs to flip num sign)
		assert_eq!(frac(i64::MIN, -1), Err(FractionError::Overflow));
	}

	#[test]
	fn test_arithmetic_add() {
		// 1/2 + 1/3 = 5/6
		let f1 = frac(1, 2).unwrap();
		let f2 = frac(1, 3).unwrap();
		let res = f1.checked_add(f2).unwrap();
		assert_eq!(res.numerator, 5);
		assert_eq!(res.denominator, 6);

		// 1/2 + 1/-2 = 0
		let f1 = frac(1, 2).unwrap();
		let f2 = frac(1, -2).unwrap();
		let res = f1.checked_add(f2).unwrap();
		assert_eq!(res.numerator, 0);
		assert_eq!(res.denominator, 1);
	}

	#[test]
	fn test_arithmetic_sub() {
		// 1/2 - 1/3 = 1/6
		let f1 = frac(1, 2).unwrap();
		let f2 = frac(1, 3).unwrap();
		let res = f1.checked_sub(f2).unwrap();
		assert_eq!(res.numerator, 1);
		assert_eq!(res.denominator, 6);
	}

	#[test]
	fn test_arithmetic_mul() {
		// 2/3 * 3/4 = 6/12 = 1/2
		let f1 = frac(2, 3).unwrap();
		let f2 = frac(3, 4).unwrap();
		let res = f1.checked_mul(f2).unwrap();
		assert_eq!(res.numerator, 1);
		assert_eq!(res.denominator, 2);
	}

	#[test]
	fn test_arithmetic_div() {
		// (1/2) / (1/2) = 1
		let f1 = frac(1, 2).unwrap();
		let f2 = frac(1, 2).unwrap();
		let res = f1.checked_div(f2).unwrap();
		assert_eq!(res.numerator, 1);
		assert_eq!(res.denominator, 1);

		// Division by zero fraction
		let f1 = frac(1, 2).unwrap();
		let f2 = frac(0, 1).unwrap();
		assert_eq!(f1.checked_div(f2), Err(FractionError::Undefined));
	}

	#[test]
	fn test_ordering() {
		let f1 = frac(1, 2).unwrap();
		let f2 = frac(1, 3).unwrap();
		let f3 = frac(2, 4).unwrap(); // Same as 1/2

		assert!(f1 > f2); // 1/2 > 1/3
		assert!(f2 < f1);
		assert_eq!(f1.partial_cmp(&f3), Some(Ordering::Equal));

		// Test with negative
		let neg = frac(-1, 2).unwrap();
		assert!(neg < f1);
	}

	#[test]
	fn test_f64_conversion() {
		let f = frac(1, 2).unwrap();
		let val: f64 = f.try_into().unwrap();
		assert!((val - 0.5).abs() < f64::EPSILON);

		// Test panic/error on raw struct with 0 denom
		let bad_frac = Fraction {
			numerator: 1,
			denominator: 0,
		};
		assert_eq!(f64::try_from(bad_frac), Err(FractionError::ZeroDenominator));
	}

	#[test]
	fn test_overflow_checks() {
		// Adding two huge fractions that overflow i64 but fit in i128 during calc,
		// but the result numerator overflows i64.
		let f1 = frac(i64::MAX - 1, 1).unwrap();
		let f2 = frac(2, 1).unwrap();

		// (MAX-1) + 2 = MAX + 1 -> Overflow i64
		assert_eq!(f1.checked_add(f2), Err(FractionError::Overflow));
	}

	#[test]
	fn test_gcd_edge_cases() {
		// Standard
		assert_eq!(Fraction::gcd(10, 5), 5);
		assert_eq!(Fraction::gcd(-10, 5), 5);

		// The "Impossible" GCD
		// gcd(i64::MIN, 0) = 2^63. This fits in u64, but not i64.
		assert_eq!(Fraction::gcd(i64::MIN, 0), 9_223_372_036_854_775_808);
	}

	#[test]
	fn test_manual_corruption_resilience() {
		// 1. Manually set Denominator to i64::MIN
		let mut f = Fraction {
			numerator: 1,
			denominator: i64::MIN,
		};
		// Should not panic
		f.reduce();
		// Should remain unchanged (because it returned early)
		assert_eq!(f.denominator, i64::MIN);

		// 2. Manually set Num=MIN, Denom=-1
		let mut f2 = Fraction {
			numerator: i64::MIN,
			denominator: -1,
		};
		// Normalizing would require flipping Num to +MIN (impossible).
		// Should not panic.
		f2.reduce();
		assert_eq!(f2.denominator, -1);
	}

	#[test]
	fn test_reduce_works_normally() {
		let mut f = Fraction {
			numerator: 2,
			denominator: 4,
		};
		f.reduce();
		assert_eq!(f.numerator, 1);
		assert_eq!(f.denominator, 2);

		let mut f2 = Fraction {
			numerator: 2,
			denominator: -4,
		};
		f2.reduce();
		assert_eq!(f2.numerator, -1);
		assert_eq!(f2.denominator, 2);
	}
}
