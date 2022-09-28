use core::ops::{Add, Sub};

pub use crate::prelude::*;
use crate::{
	claim_of_stake,
	test::{
		runtime::{StakingRewards, System},
		Test,
	},
	validation::ValidSplitRatio,
	AccountIdOf, AssetIdOf, FinancialNftInstanceIdOf, Pallet, RewardPoolConfigurationOf,
	RewardPools, Stakes,
};
use composable_support::validation::Validated;
use composable_tests_helpers::test::{
	block::MILLISECS_PER_BLOCK,
	helper::{
		assert_event, assert_event_with, assert_extrinsic_event, assert_extrinsic_event_with,
		pallet_events,
	},
};
use frame_support::{
	assert_ok,
	traits::{fungibles::Inspect, OriginTrait},
};
use frame_system::pallet_prelude::OriginFor;
pub use sp_core::{
	sr25519::{Public, Signature},
	H256,
};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	PerThing, Permill,
};
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[cfg(test)]
pub use composable_tests_helpers::test::currency::*;

use super::runtime;

pub(crate) const fn block_seconds(amount_of_blocks: u64) -> u128 {
	// would use `.into()` instead of `as` but `.into()` is not const
	((MILLISECS_PER_BLOCK / 1_000) * amount_of_blocks) as u128
}

pub(crate) const ONE_YEAR_OF_BLOCKS: u64 = 60 * 60 * 24 * 365 / (block_seconds(1) as u64);

/// Mock ID for staking fNFT collection
pub(crate) const STAKING_FNFT_COLLECTION_ID: CurrencyId = 1;

// helpers

// TODO: Make generic over runtime
pub(crate) fn add_to_rewards_pot_and_assert(
	who: <Test as frame_system::Config>::AccountId,
	pool_id: <Test as crate::Config>::AssetId,
	asset_id: <Test as crate::Config>::AssetId,
	amount: <Test as crate::Config>::Balance,
) {
	assert_extrinsic_event::<Test, _, _, _>(
		StakingRewards::add_to_rewards_pot(
			runtime::Origin::signed(who),
			pool_id,
			asset_id,
			amount,
			false,
		),
		crate::Event::<Test>::RewardsPotIncreased { pool_id, asset_id, amount },
	)
}

pub fn stake_and_assert<Runtime, RuntimeEvent>(
	staker: AccountIdOf<Runtime>,
	pool_id: <Runtime as crate::Config>::AssetId,
	amount: <Runtime as crate::Config>::Balance,
	duration_preset: u64,
) -> <Runtime as crate::Config>::FinancialNftInstanceId
where
	Runtime: crate::Config<Event = RuntimeEvent> + frame_system::Config<Event = RuntimeEvent>,
	RuntimeEvent: Parameter + Member + core::fmt::Debug + Clone,
	RuntimeEvent: TryInto<crate::Event<Runtime>>,
	<RuntimeEvent as TryInto<crate::Event<Runtime>>>::Error: core::fmt::Debug,
	<Runtime as frame_system::Config>::Origin: OriginTrait<AccountId = AccountIdOf<Runtime>>,
{
	assert_extrinsic_event_with::<Runtime, RuntimeEvent, crate::Event<Runtime>, _, _, _>(
		Pallet::<Runtime>::stake(
			OriginFor::<Runtime>::signed(staker.clone()),
			pool_id,
			amount,
			duration_preset,
		),
		|event| match event {
			crate::Event::Staked {
				pool_id: event_pool_id,
				owner: event_owner,
				amount: event_amount,
				duration_preset: event_duration_preset,
				fnft_collection_id: event_fnft_collection_id,
				fnft_instance_id,
				keep_alive,
			} => {
				assert_eq!(pool_id, event_pool_id);
				assert_eq!(staker, event_owner);
				assert_eq!(amount, event_amount);
				assert_eq!(duration_preset, event_duration_preset);

				let pool = RewardPools::<Runtime>::get(pool_id).unwrap();
				assert_eq!(pool.financial_nft_asset_id, event_fnft_collection_id);

				Some(fnft_instance_id)
			},
			_ => None,
		},
	)
}

pub fn unstake_and_assert<Runtime, RuntimeEvent>(
	owner: AccountIdOf<Runtime>,
	fnft_collection_id: AssetIdOf<Runtime>,
	fnft_instance_id: FinancialNftInstanceIdOf<Runtime>,
	should_be_early_unstake: bool,
) where
	Runtime: crate::Config<Event = RuntimeEvent> + frame_system::Config<Event = RuntimeEvent>,
	RuntimeEvent: Parameter + Member + core::fmt::Debug + Clone,
	RuntimeEvent: TryInto<crate::Event<Runtime>>,
	<RuntimeEvent as TryInto<crate::Event<Runtime>>>::Error: core::fmt::Debug,
	<Runtime as frame_system::Config>::Origin: OriginTrait<AccountId = AccountIdOf<Runtime>>,
{
	let position_before_unstake =
		Stakes::<Runtime>::get(fnft_collection_id, fnft_instance_id).unwrap();

	let slashed_amount_of =
		|amount: Runtime::Balance| position_before_unstake.lock.unlock_penalty.mul_floor(amount);

	let staker_staked_asset_balance_before_unstake =
		Runtime::Assets::balance(position_before_unstake.reward_pool_id, &owner);

	let rewards_pool = Pallet::<Runtime>::pools(position_before_unstake.reward_pool_id)
		.expect("rewards_pool expected");

	let mut rewards_balances_in_pool_account_before_unstake = rewards_pool
		.rewards
		.clone()
		.into_iter()
		.map(|(reward_asset_id, _)| {
			(
				reward_asset_id,
				Runtime::Assets::balance(
					reward_asset_id,
					&Pallet::<Runtime>::pool_account_id(&position_before_unstake.reward_pool_id),
				),
			)
		})
		.collect::<BTreeMap<_, _>>();

	let mut rewards_balances_in_stakers_account_before_unstake = rewards_pool
		.rewards
		.clone()
		.into_iter()
		.map(|(reward_asset_id, _)| {
			(reward_asset_id, Runtime::Assets::balance(reward_asset_id, &owner))
		})
		.collect::<BTreeMap<_, _>>();

	let mut rewards_balances_in_treasury_account_before_unstake = rewards_pool
		.rewards
		.clone()
		.into_iter()
		.map(|(reward_asset_id, _)| {
			(
				reward_asset_id,
				Runtime::Assets::balance(reward_asset_id, &Runtime::TreasuryAccount::get()),
			)
		})
		.collect::<BTreeMap<_, _>>();

	assert_extrinsic_event_with::<Runtime, RuntimeEvent, crate::Event<Runtime>, _, _, _>(
		Pallet::<Runtime>::unstake(
			OriginFor::<Runtime>::signed(owner.clone()),
			fnft_collection_id,
			fnft_instance_id,
		),
		|event| match event {
			crate::Event::Unstaked {
				owner: event_owner,
				fnft_collection_id: event_fnft_collection_id,
				fnft_instance_id: event_fnft_instance_id,
				slash,
			} => {
				if should_be_early_unstake {
					assert!(slash.is_some(), "unstake was expected to be slashed but it was not");
					assert_eq!(
						slash.unwrap(),
						slashed_amount_of(position_before_unstake.stake),
						"slash was not the expected amount"
					);
				} else {
					assert_eq!(slash, None, "unstake was not expected to be slashed")
				}

				assert_eq!(
					fnft_collection_id, event_fnft_collection_id,
					"event should emit the provided fnft collection id"
				);
				assert_eq!(
					fnft_instance_id, event_fnft_instance_id,
					"event should emit the provided fnft instance id"
				);

				assert_eq!(
					owner, event_owner,
					"event owner should be the owner of the position that was unstaked"
				);

				Some(())
			},
			_ => None,
		},
	);

	assert!(
		Stakes::<Runtime>::get(fnft_collection_id, fnft_instance_id).is_none(),
		"staked position should not exist after successfully unstaking"
	);

	// consistency check
	assert_eq!(
		position_before_unstake.reductions.keys().collect::<BTreeSet<_>>(),
		rewards_pool.rewards.keys().collect::<BTreeSet<_>>()
	);

	if should_be_early_unstake {
		let expected_slashed_stake_amount = slashed_amount_of(position_before_unstake.stake);

		let owner_expected_staked_asset_balance_after_early_unstake =
			staker_staked_asset_balance_before_unstake.add(expected_slashed_stake_amount);

		// if the staked asset is the same as one of the reward assets, it needs to be
		// accounted for
		if let Some(reward) = rewards_pool.rewards.get(&position_before_unstake.reward_pool_id) {
			let expected_claim = claim_of_stake::<Runtime>(
				&position_before_unstake,
				&rewards_pool.total_shares,
				reward,
				&position_before_unstake.reward_pool_id,
			)
			.expect("should not fail");

			let expected_slashed_claim_amount = slashed_amount_of(expected_claim);

			assert_event::<Runtime, RuntimeEvent, crate::Event<Runtime>>(
				crate::Event::UnstakeRewardSlashed {
					pool_id: position_before_unstake.reward_pool_id,
					owner,
					fnft_instance_id,
					reward_asset_id: position_before_unstake.reward_pool_id,
					amount_slashed: expected_slashed_claim_amount,
				},
			);

			// Check owner's balance
			assert_eq!(
				Runtime::Assets::balance(position_before_unstake.reward_pool_id, &owner),
				owner_expected_staked_asset_balance_after_early_unstake
					.add(expected_slashed_stake_amount)
					.add(expected_slashed_claim_amount),
				r#"owner's staked asset balance after an early unstake was not as expected.
staked asset id: {staked_asset:?} (was also a reward asset).

expected claim: {expected_claim:?}
expected slashed claim amount: {expected_slashed_claim_amount:?}
staked amount: {staked_amount:?}
expected slashed stake amount: {expected_slashed_stake_amount:?}"#,
				staked_asset = position_before_unstake.reward_pool_id,
				staked_amount = position_before_unstake.stake
			);

			// Check treasury account's balance
			assert_eq!(
				Runtime::Assets::balance(
					position_before_unstake.reward_pool_id,
					&Runtime::TreasuryAccount::get()
				),
				rewards_balances_in_treasury_account_before_unstake
					[&position_before_unstake.reward_pool_id]
					.add(expected_claim.sub(expected_slashed_claim_amount))
					.add(position_before_unstake.stake.sub(expected_slashed_stake_amount)),
				r#"treasury account's staked asset balance after an early unstake was not as expected.
staked asset id: {staked_asset:?} (was also a reward asset).

expected claim: {expected_claim:?}
expected slashed claim amount: {expected_slashed_claim_amount:?}
staked amount: {staked_amount:?}
expected slashed stake amount: {expected_slashed_stake_amount:?}"#,
				staked_asset = position_before_unstake.reward_pool_id,
				staked_amount = position_before_unstake.stake
			);
		} else {
			// Check owner's balance
			assert_eq!(
				Runtime::Assets::balance(position_before_unstake.reward_pool_id, &owner),
				owner_expected_staked_asset_balance_after_early_unstake
					.add(expected_slashed_stake_amount),
				r#"owner's staked asset balance after an early unstake was not as expected.
staked asset id: {staked_asset:?}

staked amount: {staked_amount:?}
expected slashed stake amount: {expected_slashed_stake_amount:?}"#,
				staked_asset = position_before_unstake.reward_pool_id,
				staked_amount = position_before_unstake.stake
			);

			// Check treasury account's balance
			assert_eq!(
				Runtime::Assets::balance(
					position_before_unstake.reward_pool_id,
					&Runtime::TreasuryAccount::get()
				),
				rewards_balances_in_treasury_account_before_unstake
					[&position_before_unstake.reward_pool_id]
					.add(position_before_unstake.stake.sub(expected_slashed_stake_amount)),
				r#"treasury account's staked asset balance after an early unstake was not as expected.
staked asset id: ({staked_asset:?})

staked amount: {staked_amount:?}
expected slashed stake amount: {expected_slashed_stake_amount:?}"#,
				staked_asset = position_before_unstake.reward_pool_id,
				staked_amount = position_before_unstake.stake
			);
		}
	} else {
		let owner_expected_staked_asset_balance_after_unstake =
			staker_staked_asset_balance_before_unstake
				// see above comment
				.add(
					rewards_balances_in_treasury_account_before_unstake
						.get(&position_before_unstake.reward_pool_id)
						.copied()
						.unwrap_or_else(Zero::zero),
				);

		assert_eq!(
			Runtime::Assets::balance(position_before_unstake.reward_pool_id, &owner),
			owner_expected_staked_asset_balance_after_unstake
		);
	}

	for (reward_asset_id, reward) in &rewards_pool.rewards {
		let expected_claim = claim_of_stake::<Runtime>(
			&position_before_unstake,
			&rewards_pool.total_shares,
			reward,
			reward_asset_id,
		)
		.expect("should not fail");

		// Check pool account's balance
		assert_eq!(
			Runtime::Assets::balance(
				position_before_unstake.reward_pool_id,
				&Pallet::<Runtime>::pool_account_id(&position_before_unstake.reward_pool_id)
			),
			rewards_balances_in_pool_account_before_unstake[reward_asset_id].sub(expected_claim),
			r#"pool account's staked asset balance after unstaking was not as expected.
staked asset id: {staked_asset:?}

reward asset id: {reward_asset_id:?}
expected claim: {expected_claim:?}"#,
			staked_asset = position_before_unstake.reward_pool_id,
		);

		// everything else where the staked asset is also a reward asset is accounted for when
		// checking the staked asset
		if reward_asset_id == &position_before_unstake.reward_pool_id {
			continue
		}

		if should_be_early_unstake {
			let expected_slashed_amount = slashed_amount_of(expected_claim);

			assert_eq!(
				Runtime::Assets::balance(*reward_asset_id, &owner),
				rewards_balances_in_treasury_account_before_unstake
					.get(&reward_asset_id)
					.copied()
					.map(slashed_amount_of)
					.unwrap_or_else(Zero::zero),
				""
			);

			assert_eq!(
				Runtime::Assets::balance(*reward_asset_id, &Runtime::TreasuryAccount::get()),
				rewards_balances_in_pool_account_before_unstake[&reward_asset_id]
					.add(expected_claim.sub(expected_slashed_amount))
			);

			assert_event::<Runtime, RuntimeEvent, crate::Event<Runtime>>(
				crate::Event::UnstakeRewardSlashed {
					pool_id: position_before_unstake.reward_pool_id,
					owner,
					fnft_instance_id,
					reward_asset_id: *reward_asset_id,
					amount_slashed: expected_slashed_amount,
				},
			);
		} else {
			assert_eq!(
				Runtime::Assets::balance(*reward_asset_id, &owner),
				rewards_balances_in_treasury_account_before_unstake[&reward_asset_id]
					.add(expected_claim)
			);
		}

		assert_eq!(
			Runtime::Assets::balance(
				*reward_asset_id,
				&Pallet::<Runtime>::pool_account_id(&position_before_unstake.reward_pool_id)
			),
			rewards_balances_in_pool_account_before_unstake[&reward_asset_id].sub(expected_claim)
		);
	}

	// assert_eq!(balance(staked_asset_id, &staker), amount * 2);
	// assert_eq!(
	// 	balance(staked_asset_id, &crate::Pallet::<Runtime>::pool_account_id(&pool_id)),
	// 	amount * 2
	// );
	// for (rewarded_asset_id, _) in rewards_pool.rewards.iter() {
	// 	assert_eq!(balance(*rewarded_asset_id, &staker), amount * 2 + claim);
	// 	assert_eq!(
	// 		balance(*rewarded_asset_id, &crate::Pallet::<Runtime>::pool_account_id(&pool_id)),
	// 		amount * 2 - claim
	// 	);
	// }
}

pub fn split_and_assert<Runtime: Clone, RuntimeEvent>(
	staker: AccountIdOf<Runtime>,
	fnft_collection_id: AssetIdOf<Runtime>,
	fnft_instance_id: FinancialNftInstanceIdOf<Runtime>,
	ratio: Validated<Permill, ValidSplitRatio>,
) -> FinancialNftInstanceIdOf<Runtime>
where
	Runtime: crate::Config<Event = RuntimeEvent> + frame_system::Config<Event = RuntimeEvent>,
	RuntimeEvent: Parameter + Member + core::fmt::Debug + Clone,
	RuntimeEvent: TryInto<crate::Event<Runtime>>,
	<RuntimeEvent as TryInto<crate::Event<Runtime>>>::Error: core::fmt::Debug,
	<Runtime as frame_system::Config>::Origin: OriginTrait<AccountId = AccountIdOf<Runtime>>,
{
	let existing_stake_before_split =
		Stakes::<Runtime>::get(fnft_collection_id, fnft_instance_id).unwrap();

	let [(
		event_existing_fnft_collection_id,
		event_existing_fnft_instance_id,
		existing_position_staked_amount,
	), (event_new_fnft_collection_id, event_new_fnft_instance_id, new_position_staked_amount)] =
		assert_extrinsic_event_with::<Runtime, RuntimeEvent, crate::Event<Runtime>, _, _, _>(
			Pallet::<Runtime>::split(
				OriginFor::<Runtime>::signed(staker),
				fnft_collection_id,
				fnft_instance_id,
				ratio,
			),
			|event| match event {
				crate::Event::SplitPosition { positions } =>
					if let [existing, new] = positions[..] {
						Some([existing, new])
					} else {
						panic!("expected 2 positions in event, found {positions:#?}")
					},
				_ => None,
			},
		);

	let pool = RewardPools::<Runtime>::get(existing_stake_before_split.reward_pool_id).unwrap();

	assert_eq!(
		event_existing_fnft_collection_id, event_new_fnft_collection_id,
		"positions emitted in event should have the same fnft collection"
	);
	assert_eq!(
		pool.financial_nft_asset_id, event_new_fnft_collection_id,
		"positions emitted in event should have the same fnft collection id as the pool"
	);

	assert_eq!(
		fnft_instance_id, event_existing_fnft_instance_id,
		"event should emit the existing fnft instance id"
	);
	assert_ne!(
		event_new_fnft_instance_id, event_existing_fnft_instance_id,
		"new fnft instance id should be different than the existing fnft instance id"
	);

	let new_position =
		Stakes::<Runtime>::get(fnft_collection_id, event_new_fnft_instance_id).unwrap();
	let existing_position_after_split =
		Stakes::<Runtime>::get(fnft_collection_id, fnft_instance_id).unwrap();

	assert_eq!(
		new_position_staked_amount, new_position.stake,
		"event should emit the amount in the new stake"
	);
	assert_eq!(
		existing_position_staked_amount, existing_position_after_split.stake,
		"event should emit the new amount in the existing stake"
	);

	// consistency checks
	assert_eq!(
		existing_stake_before_split.reward_pool_id, existing_position_after_split.reward_pool_id,
		r#"
reward_pool_id of original staked position should not change
stake id: {fnft_collection_id:?}, {fnft_instance_id:?}
"#
	);
	assert_eq!(
		existing_stake_before_split.reward_pool_id, new_position.reward_pool_id,
		r#"
reward_pool_id of new staked position should be the same as the original position
new stake id: {fnft_collection_id:?}, {event_new_fnft_instance_id:?}
"#
	);

	assert_eq!(
		existing_stake_before_split.lock, existing_position_after_split.lock,
		r#"
lock of original staked position changed when it should not have
original stake id: {fnft_collection_id:?}, {fnft_instance_id:?}
"#
	);
	assert_eq!(
		existing_stake_before_split.lock, new_position.lock,
		r#"
lock of new staked position should be the same as the original position
new stake id: {fnft_collection_id:?}, {event_new_fnft_instance_id:?}
"#
	);

	// stake & share ratio checks
	assert_eq!(
		existing_position_after_split.stake,
		ratio.mul_floor(existing_stake_before_split.stake),
		r#"
stake of the original staked position should be {:?} of what it was before the split
original stake id: {fnft_collection_id:?}, {fnft_instance_id:?}
"#,
		*ratio
	);
	assert_eq!(
		new_position.stake,
		ratio.left_from_one().mul_ceil(existing_stake_before_split.stake),
		r#"
stake of the original staked position should be 1 - {:?} ({left_from_one:?}) of what it was before the split
new stake id: {fnft_collection_id:?}, {event_new_fnft_instance_id:?}
"#,
		*ratio,
		left_from_one = ratio.left_from_one()
	);

	assert_eq!(
		existing_position_after_split.share,
		ratio.mul_floor(existing_stake_before_split.share),
		r#"
share of the original staked position should be {:?} of what it was before the split
original stake id: {fnft_collection_id:?}, {fnft_instance_id:?}
"#,
		*ratio
	);
	assert_eq!(
		new_position.share,
		ratio.left_from_one().mul_ceil(existing_stake_before_split.share),
		r#"
share of the original staked position should be 1 - {:?} ({left_from_one:?}) of what it was before the split
new stake id: {fnft_collection_id:?}, {event_new_fnft_instance_id:?}
"#,
		*ratio,
		left_from_one = ratio.left_from_one()
	);

	// assert that there is no loss in assets when splitting
	assert_eq!(
		existing_stake_before_split.stake,
		existing_position_after_split.stake + new_position.stake,
		"split should not cause any loss or gain of assets"
	);
	assert_eq!(
		existing_stake_before_split.share,
		existing_position_after_split.share + new_position.share,
		"split should not cause any loss or gain of assets"
	);

	// reductions checks
	let mut original_stake_after_split_reductions =
		existing_position_after_split.reductions.clone();
	let mut new_stake_reductions = new_position.reductions.clone();

	for (reward_asset_id, original_stake_reduction_before_split) in
		existing_stake_before_split.reductions
	{
		let original_stake_after_split_reduction =
			original_stake_after_split_reductions.remove(&reward_asset_id).unwrap();
		let new_stake_reduction = new_stake_reductions.remove(&reward_asset_id).unwrap();

		assert_eq!(
			original_stake_after_split_reduction,
			ratio.mul_floor(original_stake_reduction_before_split),
			r#"
reductions of the original staked position should be {:?} of what it was before the split
original stake id: {fnft_collection_id:?}, {fnft_instance_id:?}
asset id: {reward_asset_id:?}
"#,
			*ratio
		);
		assert_eq!(
			new_stake_reduction,
			ratio.left_from_one().mul_ceil(original_stake_reduction_before_split),
			r#"
reductions of the original staked position should be 1 - {:?} ({left_from_one:?}) of what it was before the split
new stake id: {fnft_collection_id:?}, {event_new_fnft_instance_id:?}
asset id: {reward_asset_id:?}
"#,
			*ratio,
			left_from_one = ratio.left_from_one()
		);

		// assert that there is no loss in assets when splitting
		assert_eq!(
			original_stake_reduction_before_split,
			original_stake_after_split_reduction + new_stake_reduction,
			"split should not cause any loss or gain of assets"
		);
	}

	assert!(
		new_stake_reductions.is_empty(),
		"new staked position contains extra reward assets: {:#?}",
		new_stake_reductions
	);

	assert!(
		original_stake_after_split_reductions.is_empty(),
		"new staked position contains extra reward assets: {:#?}",
		original_stake_after_split_reductions
	);

	event_new_fnft_instance_id
}

pub(crate) fn create_rewards_pool_and_assert(
	reward_config: RewardPoolConfigurationOf<Test>,
) -> <Test as crate::Config>::AssetId {
	assert_ok!(StakingRewards::create_reward_pool(Origin::root(), reward_config.clone()));

	match System::events().last().expect("no events present").event {
		runtime::Event::StakingRewards(crate::Event::<Test>::RewardPoolCreated {
			pool_id,
			owner: event_owner,
			end_block: event_end_block,
		}) => {
			match reward_config {
				RewardPoolConfiguration::RewardRateBasedIncentive {
					end_block,
					owner,
					asset_id: _,
					..
				} => {
					assert_eq!(end_block, event_end_block);
					assert_eq!(owner, event_owner);
				},
				_ => unimplemented!(),
			}

			pool_id
		},
		_ => panic!("RewardPoolCreated event not emitted"),
	}
}
