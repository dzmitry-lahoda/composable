
//! Autogenerated weights for `pallet_staking_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_staking_rewards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_staking_rewards::WeightInfo for WeightInfo<T> {
	// Storage: StakingRewards RewardPoolCount (r:1 w:1)
	// Storage: StakingRewards RewardPools (r:0 w:1)
	fn create_reward_pool() -> Weight {
		(39_163_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: StakingRewards StakeCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: StakingRewards Stakes (r:0 w:1)
	fn stake() -> Weight {
		(119_786_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: StakingRewards Stakes (r:1 w:1)
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	fn extend() -> Weight {
		(96_421_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: StakingRewards Stakes (r:1 w:2)
	// Storage: StakingRewards StakeCount (r:1 w:1)
	fn split() -> Weight {
		(47_533_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
