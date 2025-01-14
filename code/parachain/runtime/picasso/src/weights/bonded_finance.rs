
//! Autogenerated weights for `bonded_finance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-14, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `657e6acf5e95`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/7as5b27dws6pfhhpjrs68qfvfx2ldcli-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `bonded_finance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> bonded_finance::WeightInfo for WeightInfo<T> {
	// Storage: BondedFinance BondOfferCount (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: BondedFinance BondOffers (r:0 w:1)
	fn offer() -> Weight {
		// Minimum execution time: 115_882 nanoseconds.
		Weight::from_ref_time(118_779_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: BondedFinance BondOffers (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:2)
	// Storage: Vesting VestingScheduleNonce (r:1 w:1)
	// Storage: Vesting VestingSchedules (r:2 w:2)
	// Storage: Tokens Locks (r:2 w:2)
	fn bond() -> Weight {
		// Minimum execution time: 257_042 nanoseconds.
		Weight::from_ref_time(272_336_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: BondedFinance BondOffers (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn cancel() -> Weight {
		// Minimum execution time: 70_734 nanoseconds.
		Weight::from_ref_time(73_422_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
