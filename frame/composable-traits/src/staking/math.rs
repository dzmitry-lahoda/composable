
use crate::{self as composable_traits};

use composable_support::math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub};
use composable_traits::time::DurationSeconds;
use sp_runtime::{ArithmeticError, Perbill};
use sp_std::convert::TryInto;
type Balance = u128;

// keep in sync with python math
pub fn honest_locked_stake_increase(
	after_penalized: Perbill,
	original_amount: Balance,
	new_amount: Balance,
	duration: DurationSeconds,
	passed: DurationSeconds,
) -> Result<DurationSeconds, ArithmeticError> {
	let penalized_amount = after_penalized.mul_ceil(original_amount); // really we want to consider more original staking to reduce error in time
	let total = penalized_amount.safe_add(&new_amount)?;
	let remaining = duration.safe_sub(&passed)?;
	let bonus_time = penalized_amount.safe_mul(&(remaining as u128))?;
	let new_time = new_amount.safe_mul(&(duration as u128))?;
	let new_remaining_time =
		(duration as u128).min(bonus_time.safe_add(&new_time)?.safe_div(&total)?.safe_add(&1)?);
	new_remaining_time.try_into().ok().ok_or(ArithmeticError::Overflow)
}

#[cfg(test)]
mod tests {
	use composable_support::math::safe::SafeSub;
use sp_runtime::{Perbill, ArithmeticError};

	use super::honest_locked_stake_increase;

	#[test]
	fn with_zero_time_passed_staking_gives_same_time() {
		let original_amount = 1_000;
		let new_amount = 1_000;
		let duration = 1_000;
		let passed = 0;
		let after_penalized = Perbill::from_rational(50_u32, 100_u32);
		let remaining = honest_locked_stake_increase(
			after_penalized,
			original_amount,
			new_amount,
			duration,
			passed,
		)
		.expect("valid parameters");
		assert_eq!(remaining, 1000, "does not allows to reduce duration doing staking");
	}

	fn honest_lock_extensions(
		now: u64,
		lock_date: u64,
		new_lock: u64,
		previous_lock: u64,
	) -> Result<u64, ArithmeticError> {
		let passed_time = now - lock_date;
		let rolling = passed_time.min(new_lock.safe_sub(&previous_lock)?);
		Ok(rolling)
	}


	#[test]
	fn with_some_time_passed_rounds_to_bigger() {
		let original_amount = 1_000;
		let new_amount = 100_000;
		let duration = 1_000;
		let passed = 500;
		let after_penalized = Perbill::from_rational(50_u32, 100_u32);
		let remaining = honest_locked_stake_increase(
			after_penalized,
			original_amount,
			new_amount,
			duration,
			passed,
		)
		.expect("valid parameters");
		assert_eq!(remaining, 998, "rounded up from 997.5124378109452");
	}
}
