use core::{fmt::Debug, num::NonZeroU64};

use crate::{
	staking::lock::{Lock, LockConfig},
	time::DurationSeconds,
};

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, BoundedBTreeMap};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Zero;
use sp_runtime::{DispatchError, Permill};

pub mod lock;
pub mod math;

/// Defines staking duration, rewards and early unstake penalty for a given asset type.
/// TODO refer to the relevant section in the design doc.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
// TODO: Rename to RateBasedReward
pub struct RateBasedReward<Balance> {
	/// Total rewards including inflation for adjusting for new stakers joining the pool. All
	/// stakers in a pool are eligible to receive a part of this value based on their share of the
	/// pool.
	pub total_rewards: Balance,

	/// Already claimed rewards by stakers by unstaking.
	pub claimed_rewards: Balance,

	/// A book keeping field to track the actual total reward without the
	/// reward dilution adjustment caused by new stakers joining the pool.
	pub total_dilution_adjustment: Balance,

	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,

	/// The last time the reward was updated, in seconds.
	pub last_updated_timestamp: u64,
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub enum RewardType<Balance> {
	Earnings(),
	RateBased(RateBasedReward<Balance>),
}

impl<Balance> RewardType<Balance> {
	pub fn as_ref_mut_rate_based(&mut self) -> Option<&mut RateBasedReward<Balance>> {
		match self {
			RewardType::Earnings() => None,
			RewardType::RateBased(ref mut rate_based_reward) => Some(rate_based_reward),
		}
	}
}

impl<Balance: Zero> RewardType<Balance> {
	pub fn from_config(reward_config: RewardConfigType<Balance>, now_seconds: u64) -> Self {
		match reward_config {
			RewardConfigType::Earnings() => RewardType::Earnings(),
			RewardConfigType::RateBased(rate_based_config) =>
				RewardType::RateBased(RateBasedReward::from_config(rate_based_config, now_seconds)),
		}
	}
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub enum RewardConfigType<Balance> {
	// REVIEW(benluelo): Consider renaming. Perhaps `ExternallyFunded` or something similar?
	Earnings(),
	RateBased(RateBasedConfig<Balance>),
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RewardRate<Balance> {
	/// The period that the rewards are handed out in.
	pub period: RewardRatePeriod,
	/// The amount that is rewarded each period.
	pub amount: Balance,
}

impl<Balance> RewardRate<Balance> {
	pub fn per_second<B: Into<Balance>>(amount: B) -> Self {
		Self { period: RewardRatePeriod::PerSecond, amount: amount.into() }
	}
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub enum RewardRatePeriod {
	PerSecond,
}

impl RewardRatePeriod {
	/// Returns the length of the period in seconds.
	pub fn as_secs(&self) -> NonZeroU64 {
		match self {
			RewardRatePeriod::PerSecond =>
				sp_std::num::NonZeroU64::new(1).expect("1 is non-zero; qed;"),
		}
	}
}

/// A reward update states the new reward and reward_rate for a given asset
// REVIEW(benluelo): Make this an enum with variants per reward type?
#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
pub struct RewardUpdate<Balance> {
	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,
}

impl<Balance: Zero> RateBasedReward<Balance> {
	pub fn from_config(
		reward_config: RateBasedConfig<Balance>,
		now_seconds: u64,
	) -> RateBasedReward<Balance> {
		RateBasedReward {
			total_rewards: Zero::zero(),
			claimed_rewards: Zero::zero(),
			total_dilution_adjustment: Zero::zero(),
			max_rewards: reward_config.max_rewards,
			reward_rate: reward_config.reward_rate,
			last_updated_timestamp: now_seconds,
		}
	}
}

/// A reward pool is a collection of rewards that are allocated to stakers to incentivize a
/// particular purpose. Eg: a pool of rewards for incentivizing adding liquidity to a pablo swap
/// pool. TODO refer to the relevant section in the design doc.
#[derive(
	RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, Encode, Decode, TypeInfo,
)]
#[scale_info(skip_type_params(MaxDurationPresets, MaxRewards))]
pub struct RewardPool<
	AccountId: Debug + PartialEq + Eq + Clone,
	AssetId: Debug + PartialEq + Eq + Clone,
	Balance: Debug + PartialEq + Eq + Clone,
	BlockNumber: Debug + PartialEq + Eq + Clone,
	MaxDurationPresets: Get<u32>,
	MaxRewards: Get<u32>,
> {
	pub owner: AccountId,

	/// rewards accumulated
	pub rewards: BoundedBTreeMap<AssetId, RewardType<Balance>, MaxRewards>,

	/// Total shares distributed among stakers
	pub total_shares: Balance,

	/// Already claimed shares by stakers by unstaking
	pub claimed_shares: Balance,

	/// Pool would stop adding rewards to pool at this block number.
	pub end_block: BlockNumber,

	// possible lock config for this pool
	pub lock: LockConfig<MaxDurationPresets>,

	// Asset ID issued as shares for staking in the pool. Eg: for PBLO -> xPBLO
	pub share_asset_id: AssetId,

	// Asset ID (collection ID) of the financial NFTs issued for staking positions of this pool
	pub fnft_collection_id: AssetId,
}

/// Default transfer limit on new asset added as rewards.
pub const DEFAULT_MAX_REWARDS: u128 = 1_000_000_000_000_000_000_u128;

/// Reward configurations for a given asset type.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RateBasedConfig<Balance> {
	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,
}

/// Categorize the reward pool by it's incentive characteristics and expose
/// initial configuration parameters.
/// TODO refer to the relevant section in the design doc.
#[derive(
	RuntimeDebugNoBound,
	Encode,
	Decode,
	MaxEncodedLen,
	CloneNoBound,
	PartialEqNoBound,
	EqNoBound,
	TypeInfo,
)]
#[scale_info(skip_type_params(MaxRewardConfigs, MaxDurationPresets))]
pub struct RewardPoolConfig<
	AccountId: Eq + PartialEq + Clone + Debug,
	AssetId: Eq + PartialEq + Clone + Debug,
	Balance: Eq + PartialEq + Clone + Debug,
	BlockNumber: Eq + PartialEq + Clone + Debug,
	MaxRewardConfigs: Get<u32>,
	MaxDurationPresets: Get<u32>,
> {
	/// Protocol or the user account that owns this pool
	pub owner: AccountId,

	/// The staked asset id of the reward pool.
	pub asset_id: AssetId,

	/// Pool would stop adding rewards to pool at this block number.
	pub end_block: BlockNumber,

	/// initial reward configuration for this pool
	pub reward_configs: BoundedBTreeMap<AssetId, RewardConfigType<Balance>, MaxRewardConfigs>,

	/// possible lock config for this reward
	pub lock: LockConfig<MaxDurationPresets>,

	/// Asset ID issued as shares for staking in the pool. Eg: for PBLO -> xPBLO
	pub share_asset_id: AssetId,

	/// Asset ID (collection ID) of the financial NFTs issued for staking positions of this pool
	pub financial_nft_asset_id: AssetId,
}

/// Staking typed fNFT, usually can be mapped to raw fNFT storage type. A position identifier
/// should exist for each position when stored in the runtime storage.
/// TODO refer to the relevant section in the design doc.
#[derive(DebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(MaxReductions))]
pub struct Stake<
	AssetId: Debug + PartialEq + Eq + Clone,
	RewardPoolId: Debug + PartialEq + Eq + Clone,
	Balance: Debug + PartialEq + Eq + Clone,
	MaxReductions: Get<u32>,
> {
	/// Reward Pool ID from which pool to allocate rewards for this
	pub staked_asset_id: RewardPoolId,

	/// The original stake this position was created for or updated position with any extended
	/// stake amount.
	pub amount: Balance,

	/// Pool share received for this position
	pub share: Balance,

	/// Reduced rewards by asset for the position (d_n)
	pub reductions: BoundedBTreeMap<AssetId, Balance, MaxReductions>,

	/// The lock period for the stake.
	pub lock: Lock,
}

/// Trait to provide interface to manage staking reward pool.
pub trait ManageStaking {
	type AccountId: Eq + Clone + PartialEq + Debug;
	type AssetId: Eq + Clone + PartialEq + Debug;
	type BlockNumber: Eq + Clone + PartialEq + Debug;
	type Balance: Eq + Clone + PartialEq + Debug;
	type RewardPoolId: Eq + Clone + PartialEq + Debug;

	type RewardConfigsLimit: Get<u32>;
	type StakingDurationPresetsLimit: Get<u32>;

	/// Create a staking reward pool from configurations passed as inputs.
	fn create_staking_pool(
		pool_config: RewardPoolConfig<
			Self::AccountId,
			Self::AssetId,
			Self::Balance,
			Self::BlockNumber,
			Self::RewardConfigsLimit,
			Self::StakingDurationPresetsLimit,
		>,
	) -> Result<Self::RewardPoolId, DispatchError>;
}

/// is unaware of concrete positions
pub trait ProtocolStaking {
	type AccountId;
	type AssetId;
	type Balance;
	type RewardPoolId;

	/// Transfers rewards `from` to pool.
	/// If may be bigger than total shares.
	fn transfer_earnings(
		from: &Self::AccountId,
		pool: &Self::RewardPoolId,
		reward_currency: Self::AssetId,
		reward_increment: Self::Balance,
		keep_alive: bool,
	) -> DispatchResult;
}

/// Interface for protocol staking.
pub trait Staking {
	type AccountId;
	type RewardPoolId;
	type Balance;
	type PositionId;

	/// Stake an amount of protocol asset.
	///
	/// Arguments
	///
	/// * `who` the account to transfer the stake from.
	/// * `amount` the amount to stake. the end trigger the unstake penalty.
	/// * `keep_alive` whether to keep the `from` account alive or not while transferring the stake.
	fn stake(
		who: &Self::AccountId,
		pool_id: &Self::RewardPoolId,
		amount: Self::Balance,
		duration_preset: DurationSeconds,
		keep_alive: bool,
	) -> Result<Self::PositionId, DispatchError>;

	/// Extend the stake of an existing position.
	fn extend(
		who: &Self::AccountId,
		position: Self::PositionId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::PositionId, DispatchError>;

	/// Unstake an actual staked position, represented by a NFT.
	///
	/// Arguments
	///
	/// * `instance_id` the ID uniquely identifying the NFT from which we will compute the available
	///   rewards.
	/// * `to` the account to transfer the final claimed rewards to.
	fn unstake(who: &Self::AccountId, position: &Self::PositionId) -> DispatchResult;

	/// `ratio` - how much of share to retain in the original position.
	fn split(
		who: &Self::AccountId,
		position: &Self::PositionId,
		ratio: Permill,
	) -> Result<Self::PositionId, DispatchError>;

	/// Claim remaining reward earned up to this point in time.
	///
	/// Arguments
	/// * `who` - the account to transfer the final claimed rewards to.
	/// * `position` - The uniquely identifying NFT from which we will compute the rewards.
	fn claim(who: &Self::AccountId, position: &Self::PositionId) -> DispatchResult;
}

/// Interface for managing staking through financial NFTs.
pub trait StakingFinancialNft {
	type AccountId;
	type CollectionId;
	type InstanceId;
	type Balance;

	/// Extend the stake of an existing position represented by a financial NFT.
	fn extend(
		who: &Self::AccountId,
		collection: Self::CollectionId,
		instance: Self::InstanceId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::InstanceId, DispatchError>;

	/// Unstake an actual staked position, represented by a financial NFT.
	fn burn(
		who: &Self::AccountId,
		collection: Self::CollectionId,
		instance: Self::InstanceId,
		remove_amount: Self::Balance,
	) -> DispatchResult;

	/// `ratio` - how much of share to retain in the original position.
	fn split(
		who: &Self::AccountId,
		collection: Self::CollectionId,
		instance: Self::InstanceId,
		ratio: Permill,
	) -> Result<[Self::InstanceId; 2], DispatchError>;
}