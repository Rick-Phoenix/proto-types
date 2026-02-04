//! Implementations for the google.type.Money message.
//!
//!
//! DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.

use core::cmp::Ordering;
use core::fmt::Write;

use thiserror::Error;

use crate::{String, ToString, common::Money};

const NANO_FACTOR: i32 = 1_000_000_000;

/// Errors that can occur during the creation, conversion or validation of [`Money`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum MoneyError {
	#[error("Currency mismatch: Expected '{expected}', found '{found}'")]
	CurrencyMismatch { expected: String, found: String },
	#[error("Money arithmetic operation failed (overflow, underflow, or invalid operand)")]
	OutOfRange,
}

fn normalize_money_fields_checked(
	mut units: i64,
	mut nanos: i32,
) -> Result<(i64, i32), MoneyError> {
	if nanos.abs() >= NANO_FACTOR {
		let units_carry = i64::from(nanos / (NANO_FACTOR));
		units = units
			.checked_add(units_carry)
			.ok_or(MoneyError::OutOfRange)?;
		nanos %= NANO_FACTOR;
	}

	if units > 0 && nanos < 0 {
		units = units
			.checked_sub(1)
			.ok_or(MoneyError::OutOfRange)?;
		nanos = nanos
			.checked_add(NANO_FACTOR)
			.ok_or(MoneyError::OutOfRange)?;
	} else if units < 0 && nanos > 0 {
		units = units
			.checked_add(1)
			.ok_or(MoneyError::OutOfRange)?;
		nanos = nanos
			.checked_sub(NANO_FACTOR)
			.ok_or(MoneyError::OutOfRange)?;
	}

	Ok((units, nanos))
}

impl PartialOrd for Money {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.currency_code != other.currency_code {
			return None;
		}

		self.total_nanos()
			.partial_cmp(&other.total_nanos())
	}
}

fn fields_from_total_nanos(total: i128) -> Result<(i64, i32), MoneyError> {
	let factor = i128::from(NANO_FACTOR);

	let units_val = total / factor;
	let units = i64::try_from(units_val).map_err(|_| MoneyError::OutOfRange)?;

	let remainder_val = total % factor;
	let nanos = i32::try_from(remainder_val).map_err(|_| MoneyError::OutOfRange)?;

	Ok((units, nanos))
}

impl Money {
	/// Returns the total amount in `nano` units.
	#[inline]
	#[must_use]
	pub const fn total_nanos(&self) -> i128 {
		(self.units as i128) * (NANO_FACTOR as i128) + (self.nanos as i128)
	}

	/// Creates a new instance from a total amount of nanos and a currency code.
	pub fn from_total_nanos(currency: impl Into<String>, total: i128) -> Result<Self, MoneyError> {
		let (units, nanos) = fields_from_total_nanos(total)?;

		Ok(Self {
			currency_code: currency.into(),
			units,
			nanos,
		})
	}

	/// Normalizes the [`Money`] amount and returns a string containing the currency symbol and the monetary amount with the specified amount of decimal places, while truncating the rest.
	#[must_use]
	pub fn to_formatted_string(&self, symbol: &str, decimal_places: u32) -> String {
		let decimal_places = u32::min(9, decimal_places);

		let mut current_units: i128 = i128::from(self.units);
		let mut current_nanos: i128 = i128::from(self.nanos);

		let ten_pow_9 = i128::from(NANO_FACTOR);
		if current_nanos >= ten_pow_9 || current_nanos <= -ten_pow_9 {
			current_units += current_nanos / ten_pow_9;
			current_nanos %= ten_pow_9;
		}

		if current_units > 0 && current_nanos < 0 {
			current_units -= 1;
			current_nanos += ten_pow_9;
		} else if current_units < 0 && current_nanos > 0 {
			current_units += 1;
			current_nanos -= ten_pow_9;
		}

		let mut rounded_nanos = 0;
		let mut units_carry = 0;

		if decimal_places > 0 {
			let power_of_10_for_display = 10_i128.pow(decimal_places);
			let rounding_power = 10_i128.pow(9 - decimal_places);

			let abs_nanos = current_nanos.abs();

			let remainder_for_rounding = abs_nanos % rounding_power;
			rounded_nanos = abs_nanos / rounding_power;

			// Only round if we are actually truncating precision (rounding_power > 1).
			// Otherwise, when decimal_places=9 (rounding_power=1),
			// remainder (0) >= 1/2 (0) triggers an unwanted increment.
			if rounding_power > 1 && remainder_for_rounding >= rounding_power / 2 {
				rounded_nanos += 1;
			}

			// Handle carry-over from nanos rounding to units
			if rounded_nanos >= power_of_10_for_display {
				units_carry = 1;
				rounded_nanos = 0;
			}
		}

		let is_negative = current_units < 0 || (current_units == 0 && current_nanos < 0);

		let final_units_abs = current_units.abs() + units_carry;

		let mut formatted_string = String::new();

		if is_negative {
			formatted_string.push('-');
		}
		formatted_string.push_str(symbol);
		formatted_string.push_str(&final_units_abs.to_string());

		if decimal_places > 0 {
			formatted_string.push('.');
			// Format rounded_nanos to the specified number of decimal places, zero-padded
			let _ = write!(
				formatted_string,
				"{:0width$}",
				rounded_nanos,
				width = decimal_places as usize
			);
		}

		formatted_string
	}

	/// Normalizes units and nanos. Fails in case of overflow.
	pub fn normalize(mut self) -> Result<Self, MoneyError> {
		let (normalized_units, normalized_nanos) =
			normalize_money_fields_checked(self.units, self.nanos)?;
		self.units = normalized_units;
		self.nanos = normalized_nanos;

		Ok(self)
	}

	/// Creates a new instance, if the normalization does not return errors like Overflow or Underflow.
	pub fn new(
		currency_code: impl Into<String>,
		units: i64,
		nanos: i32,
	) -> Result<Self, MoneyError> {
		let (normalized_units, normalized_nanos) = normalize_money_fields_checked(units, nanos)?;
		Ok(Self {
			currency_code: currency_code.into(),
			units: normalized_units,
			nanos: normalized_nanos,
		})
	}

	/// Converts the [`Money`] amount into a decimal (f64) representation,
	/// rounded to the specified number of decimal places.
	///
	/// `decimal_places` determines the precision of the rounding. For example:
	/// - `0` rounds to the nearest whole unit.
	/// - `2` rounds to two decimal places (e.g., for cents).
	///
	/// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
	pub fn to_rounded_imprecise_f64(&self, decimal_places: u32) -> Result<f64, MoneyError> {
		if decimal_places > i32::MAX as u32 {
			return Err(MoneyError::OutOfRange);
		}

		let full_amount = self.as_imprecise_f64();

		let factor_exponent: i32 = decimal_places
			.try_into()
			.map_err(|_| MoneyError::OutOfRange)?;
		let factor = 10.0f64.powi(factor_exponent);

		if !factor.is_finite() {
			return Err(MoneyError::OutOfRange);
		}

		let result = (full_amount * factor).round() / factor;

		if !result.is_finite() {
			return Err(MoneyError::OutOfRange);
		}

		Ok(result)
	}

	/// Converts the `Money` amount into a decimal (f64) representation.
	///
	/// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
	#[must_use]
	pub fn as_imprecise_f64(&self) -> f64 {
		self.units as f64 + (f64::from(self.nanos) / 1_000_000_000.0)
	}

	/// Creates a new `Money` instance with the given currency code and decimal amount.
	///
	/// This is a convenience constructor that handles splitting a decimal value
	/// into units and nanos.
	///
	/// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
	pub fn from_imprecise_f64(
		currency_code: impl Into<String>,
		amount: f64,
	) -> Result<Self, MoneyError> {
		if !amount.is_finite() {
			return Err(MoneyError::OutOfRange);
		}

		let truncated_amount = amount.trunc();

		if truncated_amount > i64::MAX as f64 || truncated_amount < i64::MIN as f64 {
			return Err(MoneyError::OutOfRange);
		}

		// SAFETY: We already truncateda, and this cast is safe because we checked the range
		#[allow(clippy::cast_possible_truncation)]
		let units = truncated_amount as i64;

		let raw_nanos_f64 = amount.fract().abs() * f64::from(NANO_FACTOR);
		// SAFETY: The range is guaranteed to be 0..1,000,000,000 by logic.
		#[allow(clippy::cast_possible_truncation)]
		let nanos: i32 = raw_nanos_f64.round() as i32;

		let final_nanos = if units < 0 && nanos > 0 {
			-nanos
		} else if units == 0 && amount < 0.0 && nanos > 0 {
			// For -0.5, ensure nanos is -500M
			-nanos
		} else {
			nanos
		};

		Self::new(currency_code, units, final_nanos)
	}

	/// Attempts to add another [`Money`] amount to this one, returning a new [`Money`] instance.
	/// Returns an error if currencies mismatch or if addition causes an overflow/underflow.
	pub fn try_add(&self, other: &Self) -> Result<Self, MoneyError> {
		if self.currency_code != other.currency_code {
			return Err(MoneyError::CurrencyMismatch {
				expected: self.currency_code.clone(),
				found: other.currency_code.clone(),
			});
		}

		let total = self
			.total_nanos()
			.checked_add(other.total_nanos())
			.ok_or(MoneyError::OutOfRange)?;
		Self::from_total_nanos(self.currency_code.clone(), total)
	}

	/// Attempts to add another [`Money`] amount to this one in place.
	/// Returns an error if currencies mismatch or if addition causes an overflow/underflow.
	pub fn try_add_assign(&mut self, other: &Self) -> Result<(), MoneyError> {
		if self.currency_code != other.currency_code {
			return Err(MoneyError::CurrencyMismatch {
				expected: self.currency_code.clone(),
				found: other.currency_code.clone(),
			});
		}

		let total = self
			.total_nanos()
			.checked_add(other.total_nanos())
			.ok_or(MoneyError::OutOfRange)?;
		let (new_units, new_nanos) = fields_from_total_nanos(total)?;

		self.units = new_units;
		self.nanos = new_nanos;
		Ok(())
	}

	/// Attempts to subtract another [`Money`] amount from this one, returning a new [`Money`] instance.
	/// Returns an error if currencies mismatch or if subtraction causes an overflow/underflow.
	pub fn try_sub(&self, other: &Self) -> Result<Self, MoneyError> {
		if self.currency_code != other.currency_code {
			return Err(MoneyError::CurrencyMismatch {
				expected: self.currency_code.clone(),
				found: other.currency_code.clone(),
			});
		}

		let total = self
			.total_nanos()
			.checked_sub(other.total_nanos())
			.ok_or(MoneyError::OutOfRange)?;
		Self::from_total_nanos(self.currency_code.clone(), total)
	}

	/// Attempts to subtract another [`Money`] amount from this one in place.
	/// Returns an error if currencies mismatch or if subtraction causes an overflow/underflow.
	pub fn try_sub_assign(&mut self, other: &Self) -> Result<(), MoneyError> {
		if self.currency_code != other.currency_code {
			return Err(MoneyError::CurrencyMismatch {
				expected: self.currency_code.clone(),
				found: other.currency_code.clone(),
			});
		}

		let total = self
			.total_nanos()
			.checked_sub(other.total_nanos())
			.ok_or(MoneyError::OutOfRange)?;
		let (new_units, new_nanos) = fields_from_total_nanos(total)?;

		self.units = new_units;
		self.nanos = new_nanos;
		Ok(())
	}

	/// Attempts to multiply this [`Money`] amount by an integer scalar, returning a new [`Money`] instance.
	/// Returns an error if multiplication causes an overflow/underflow.
	pub fn try_mul_i64(&self, rhs: i64) -> Result<Self, MoneyError> {
		let total = self
			.total_nanos()
			.checked_mul(i128::from(rhs))
			.ok_or(MoneyError::OutOfRange)?;
		Self::from_total_nanos(self.currency_code.clone(), total)
	}

	/// Attempts to multiply this [`Money`] amount by a float scalar, returning a new [`Money`] instance.
	/// Returns an error if the result is non-finite or causes an internal conversion error.
	/// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
	pub fn try_mul_f64(&self, rhs: f64) -> Result<Self, MoneyError> {
		if !rhs.is_finite() {
			return Err(MoneyError::OutOfRange);
		}

		let decimal_amount = self.as_imprecise_f64();
		let result_decimal = decimal_amount * rhs;

		if !result_decimal.is_finite() {
			return Err(MoneyError::OutOfRange);
		}

		// Pass the result to from_decimal_f64, which will normalize and validate.
		Self::from_imprecise_f64(self.currency_code.clone(), result_decimal)
	}

	/// Attempts to divide this [`Money`] amount by an integer scalar, returning a new [`Money`] instance.
	/// Returns an error if the divisor is zero, or if division causes an overflow/underflow.
	pub fn try_div_i64(&self, rhs: i64) -> Result<Self, MoneyError> {
		if rhs == 0 {
			return Err(MoneyError::OutOfRange);
		}

		let total = self
			.total_nanos()
			.checked_div(i128::from(rhs))
			.ok_or(MoneyError::OutOfRange)?;
		Self::from_total_nanos(self.currency_code.clone(), total)
	}

	/// Attempts to divide this [`Money`] amount by a float scalar, returning a new [`Money`] instance.
	/// Returns an error if the divisor is zero, non-finite, or if division causes an internal conversion error.
	/// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
	pub fn try_div_f64(&self, rhs: f64) -> Result<Self, MoneyError> {
		if rhs == 0.0 {
			return Err(MoneyError::OutOfRange);
		}
		if !rhs.is_finite() {
			return Err(MoneyError::OutOfRange);
		}

		let decimal_amount = self.as_imprecise_f64();
		let result_decimal = decimal_amount / rhs;

		if !result_decimal.is_finite() {
			return Err(MoneyError::OutOfRange);
		}

		Self::from_imprecise_f64(self.currency_code.clone(), result_decimal)
	}

	/// Attempts to negate this [`Money`] amount, returning a new [`Money`] instance.
	/// Returns an error if negation causes an overflow/underflow.
	pub fn try_neg(&self) -> Result<Self, MoneyError> {
		let neg_units = self
			.units
			.checked_neg()
			.ok_or(MoneyError::OutOfRange)?;
		let neg_nanos = self
			.nanos
			.checked_neg()
			.ok_or(MoneyError::OutOfRange)?;

		Self::new(self.currency_code.clone(), neg_units, neg_nanos)
	}

	/// Checks if the money's currency code matches the given `code`.
	/// The `code` should be a three-letter ISO 4217 currency code (e.g., "USD", "EUR").
	#[must_use]
	#[inline]
	pub fn is_currency(&self, code: &str) -> bool {
		self.currency_code == code
	}

	/// Checks if the money's currency is United States Dollar (USD).
	#[must_use]
	#[inline]
	pub fn is_usd(&self) -> bool {
		self.is_currency("USD")
	}

	/// Checks if the money's currency is Euro (EUR).
	#[must_use]
	#[inline]
	pub fn is_eur(&self) -> bool {
		self.is_currency("EUR")
	}

	/// Checks if the money's currency is British Pound Sterling (GBP).
	#[must_use]
	#[inline]
	pub fn is_gbp(&self) -> bool {
		self.is_currency("GBP")
	}

	/// Checks if the money's currency is Japanese Yen (JPY).
	#[must_use]
	#[inline]
	pub fn is_jpy(&self) -> bool {
		self.is_currency("JPY")
	}

	/// Checks if the money's currency is Canadian Dollar (CAD).
	#[must_use]
	#[inline]
	pub fn is_cad(&self) -> bool {
		self.is_currency("CAD")
	}

	/// Checks if the money's currency is Australian Dollar (AUD).
	#[must_use]
	#[inline]
	pub fn is_aud(&self) -> bool {
		self.is_currency("AUD")
	}

	/// Checks if the money amount is strictly positive (greater than zero).
	#[must_use]
	#[inline]
	pub const fn is_positive(&self) -> bool {
		self.units > 0 || (self.units == 0 && self.nanos > 0)
	}

	/// Checks if the money amount is strictly negative (less than zero).
	#[must_use]
	#[inline]
	pub const fn is_negative(&self) -> bool {
		self.units < 0 || (self.units == 0 && self.nanos < 0)
	}

	/// Checks if the money amount is exactly zero.
	#[must_use]
	#[inline]
	pub const fn is_zero(&self) -> bool {
		self.units == 0 && self.nanos == 0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn usd(u: i64, n: i32) -> Money {
		Money::new("USD", u, n).unwrap()
	}

	fn eur(u: i64, n: i32) -> Money {
		Money::new("EUR", u, n).unwrap()
	}

	#[test]
	fn test_normalization_carry() {
		// 1. Simple positive carry
		// 1 unit + 1.5B nanos -> 2 units + 500M nanos
		let m = usd(1, 1_500_000_000);
		assert_eq!(m.units, 2);
		assert_eq!(m.nanos, 500_000_000);

		// 2. Simple negative carry
		// -1 unit - 1.5B nanos -> -2 units - 500M nanos
		let m = usd(-1, -1_500_000_000);
		assert_eq!(m.units, -2);
		assert_eq!(m.nanos, -500_000_000);

		// 1 unit + 1000M nanos -> 2 units
		let m = usd(1, 1_000_000_000);
		assert_eq!(m.units, 2);
		assert_eq!(m.nanos, 0);
	}

	#[test]
	fn test_normalization_sign_correction() {
		// 1. Positive Units, Negative Nanos -> Reduce Unit
		// 1 unit - 100 nanos -> 0 units, 999,999,900 nanos
		let m = usd(1, -100);
		assert_eq!(m.units, 0);
		assert_eq!(m.nanos, 999_999_900);

		// 2. Negative Units, Positive Nanos -> Increase Unit (towards zero)
		// -1 unit + 100 nanos -> 0 units, -999,999,900 nanos
		let m = usd(-1, 100);
		assert_eq!(m.units, 0);
		assert_eq!(m.nanos, -999_999_900);

		// 3. Zero units, mixed nanos (allowed, sign is strictly determined by nanos)
		let m = usd(0, -500);
		assert!(m.is_negative());
	}

	// --- 2. Arithmetic Stress Tests ---

	#[test]
	fn test_add_sub() {
		// 1. Standard Addition
		let m1 = usd(10, 500_000_000);
		let m2 = usd(20, 500_000_000);
		let sum = m1.try_add(&m2).unwrap();
		assert_eq!(sum.units, 31);
		assert_eq!(sum.nanos, 0);

		// 2. Addition causing Overflow of Units
		let max = usd(i64::MAX, 0);
		let one = usd(1, 0);
		assert_eq!(max.try_add(&one), Err(MoneyError::OutOfRange));

		// 3. Subtraction crossing zero
		let m1 = usd(1, 0);
		let m2 = usd(2, 0);
		let diff = m1.try_sub(&m2).unwrap();
		assert_eq!(diff.units, -1);
		assert_eq!(diff.nanos, 0);

		// 4. Subtraction causing Underflow
		let min = usd(i64::MIN, 0);
		let one = usd(1, 0);
		assert_eq!(min.try_sub(&one), Err(MoneyError::OutOfRange));

		// 5. Currency Mismatch
		let u = usd(10, 0);
		let e = eur(10, 0);
		assert!(matches!(
			u.try_add(&e),
			Err(MoneyError::CurrencyMismatch { .. })
		));
	}

	#[test]
	fn test_assign_ops() {
		// Reuse logic from try_add but specifically testing the mutable reference implementations
		let mut m = usd(1, 500_000_000);
		m.try_add_assign(&usd(0, 600_000_000)).unwrap();
		// 1.5 + 0.6 = 2.1
		assert_eq!(m.units, 2);
		assert_eq!(m.nanos, 100_000_000);

		m.try_sub_assign(&usd(3, 0)).unwrap();
		// 2.1 - 3.0 = -0.9
		assert_eq!(m.units, 0);
		assert_eq!(m.nanos, -900_000_000);
	}

	#[test]
	fn test_mul() {
		// 1. Multiply positive
		let m = usd(10, 500_000_000); // 10.5
		let res = m.try_mul_i64(2).unwrap();
		assert_eq!(res.units, 21);

		// 2. Multiply negative
		let res = m.try_mul_i64(-2).unwrap();
		assert_eq!(res.units, -21);

		// 3. Multiply Overflow
		// (i64::MAX / 2) + 1  -> Multiplying by 2 should overflow
		let huge = usd((i64::MAX / 2) + 2, 0);
		assert_eq!(huge.try_mul_i64(2), Err(MoneyError::OutOfRange));

		// 4. Nanos Overflow triggering Unit Overflow
		// Units = i64::MAX - 1. Nanos = big enough that doubling adds 2 units.
		let edge = usd(i64::MAX - 1, 600_000_000);
		// Double nanos = 1.2B -> +1 unit carry, 200M rem.
		// Units = (MAX-1)*2 + 1 = Overflow.
		assert_eq!(edge.try_mul_i64(2), Err(MoneyError::OutOfRange));
	}

	#[test]
	fn test_div() {
		// 1. Clean Division
		let m = usd(10, 0);
		let res = m.try_div_i64(2).unwrap();
		assert_eq!(res.units, 5);

		// 2. Fractional Division (Penny splitting)
		// 1 unit / 3 = 0.333333333
		let m = usd(1, 0);
		let res = m.try_div_i64(3).unwrap();
		assert_eq!(res.units, 0);
		assert_eq!(res.nanos, 333_333_333);

		// 3. Division by Zero
		assert_eq!(m.try_div_i64(0), Err(MoneyError::OutOfRange));
	}

	// --- 3. Float Conversions & Math ---

	#[test]
	fn test_f64_math_robustness() {
		let m = usd(10, 0);

		// 1. Normal Mul
		let res = m.try_mul_f64(1.5).unwrap();
		assert_eq!(res.units, 15);

		// 2. Normal Div
		let res = m.try_div_f64(2.0).unwrap();
		assert_eq!(res.units, 5);

		// 3. Infinite Result
		assert_eq!(m.try_mul_f64(f64::INFINITY), Err(MoneyError::OutOfRange));
		assert_eq!(m.try_div_f64(0.0), Err(MoneyError::OutOfRange)); // Checked specifically

		// 4. Round trip precision check
		// Create 10.55
		let m = Money::from_imprecise_f64("USD", 10.55).unwrap();
		assert_eq!(m.nanos, 550_000_000);
		let f = m.as_imprecise_f64();
		assert!((f - 10.55).abs() < 1e-9);
	}

	#[test]
	fn test_f64_construction_edge_cases() {
		// 1. NaN
		assert_eq!(
			Money::from_imprecise_f64("USD", f64::NAN),
			Err(MoneyError::OutOfRange)
		);

		// 2. Overflow (Amount larger than i64::MAX)
		assert_eq!(
			Money::from_imprecise_f64("USD", 1e20),
			Err(MoneyError::OutOfRange)
		);

		// 3. Negative handling (-0.005) -> -5M nanos
		let m = Money::from_imprecise_f64("USD", -0.005).unwrap();
		assert_eq!(m.units, 0);
		assert_eq!(m.nanos, -5_000_000);
	}

	// --- 4. Comparison & Helpers ---

	#[test]
	fn test_comparison() {
		let m1 = usd(10, 0);
		let m2 = usd(10, 1);
		let m3 = usd(10, 0);

		assert!(m1 < m2);
		assert!(m1 == m3);

		let e = eur(10, 0);
		// Currency mismatch -> None
		assert_eq!(m1.partial_cmp(&e), None);
	}

	#[test]
	fn test_flags() {
		let zero = usd(0, 0);
		assert!(zero.is_zero());
		assert!(!zero.is_positive());
		assert!(!zero.is_negative());

		let pos = usd(0, 1);
		assert!(pos.is_positive());

		let neg = usd(0, -1);
		assert!(neg.is_negative());
	}

	// --- 5. Formatting ---

	#[test]
	fn test_formatting_precision() {
		// 1. Basic
		assert_eq!(usd(1, 0).to_formatted_string("$", 2), "$1.00");

		// 2. Rounding up
		// 1.005 -> 1.01
		assert_eq!(usd(1, 5_000_000).to_formatted_string("$", 2), "$1.01");

		// 3. Rounding down
		// 1.004 -> 1.00
		assert_eq!(usd(1, 4_000_000).to_formatted_string("$", 2), "$1.00");

		// 4. Large precision
		assert_eq!(
			usd(1, 123_456_789).to_formatted_string("$", 9),
			"$1.123456789"
		);

		// 5. Zero precision
		assert_eq!(usd(1, 900_000_000).to_formatted_string("$", 0), "$1"); // Truncates to $1

		// 6. Negative
		assert_eq!(usd(-5, -500_000_000).to_formatted_string("€", 2), "-€5.50");
	}

	#[test]
	fn test_arithmetic_rollover_bugs() {
		// This previously failed in `try_mul_i64` because 500M * 3 > i32::MAX
		let m = usd(0, 500_000_000);
		let res = m.try_mul_i64(10).unwrap(); // 5B nanos -> 5 units
		assert_eq!(res.units, 5);
		assert_eq!(res.nanos, 0);

		// Addition Rollover
		let m1 = usd(0, 600_000_000);
		let m2 = usd(0, 600_000_000);
		let res = m1.try_add(&m2).unwrap(); // 1.2B nanos -> 1 unit, 200M nanos
		assert_eq!(res.units, 1);
		assert_eq!(res.nanos, 200_000_000);
	}

	#[test]
	fn test_formatting() {
		let m = usd(10, 500_000_000); // 10.50
		assert_eq!(m.to_formatted_string("$", 2), "$10.50");

		// Rounding check
		let m = usd(10, 555_000_000); // 10.555
		assert_eq!(m.to_formatted_string("$", 2), "$10.56");

		// Negative formatting
		let m = usd(-5, -500_000_000); // -5.50
		assert_eq!(m.to_formatted_string("$", 2), "-$5.50");
	}

	#[test]
	fn test_f64_conversions() {
		// From f64
		let m = Money::from_imprecise_f64("USD", 10.50).unwrap();
		assert_eq!(m.units, 10);
		assert_eq!(m.nanos, 500_000_000);

		// To f64 (Rounded)
		// 10.555 -> round(2) -> 10.56
		let m = usd(10, 555_000_000);
		let f = m.to_rounded_imprecise_f64(2).unwrap();
		assert!((f - 10.56).abs() < f64::EPSILON);
	}
}
