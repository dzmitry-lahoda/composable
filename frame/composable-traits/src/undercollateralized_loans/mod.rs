use crate::defi::DeFiEngine;
pub use configs::*;
use frame_support::pallet_prelude::*;

pub mod configs;

pub trait UndercollateralizedLoans: DeFiEngine {
	type BlockNumber: Clone + Eq + PartialEq;
	type LiquidationStrategyId: Clone + Eq + PartialEq;
	type VaultId: Clone + Eq + PartialEq;
	type Timestamp: Clone + Eq + PartialEq;
	// Bounds are used to validate inputs sensitive to them length.
	type WhiteListBound: Get<u32>;
	type ScheduleBound: Get<u32>;

	fn create_market(
		manager: Self::AccountId,
		input: MarketInput<
			Self::AccountId,
			Self::MayBeAssetId,
			Self::BlockNumber,
			Self::LiquidationStrategyId,
		>,
		keep_alive: bool,
	) -> Result<
		MarketInfo<
			Self::AccountId,
			Self::MayBeAssetId,
			Self::BlockNumber,
			Self::LiquidationStrategyId,
			Self::VaultId,
		>,
		DispatchError,
	>;

	fn create_loan(
		input: LoanInput<Self::AccountId, Self::Balance, Self::Timestamp>,
	) -> Result<
		LoanConfig<Self::AccountId, Self::MayBeAssetId, Self::Balance, Self::Timestamp>,
		DispatchError,
	>;

	fn borrow(
		borrower_account_id: Self::AccountId,
		loan_account_id: Self::AccountId,
		keep_alive: bool,
	) -> Result<
		LoanConfig<Self::AccountId, Self::MayBeAssetId, Self::Balance, Self::Timestamp>,
		DispatchError,
	>;

	fn repay(
		payer_account_id: Self::AccountId,
		loan_account_id: Self::AccountId,
		repay_amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	fn market_account_id<S: Encode>(postfix: S) -> Self::AccountId;

	fn loan_account_id<S: Encode>(postfix: S) -> Self::AccountId;

	fn is_borrower_account_whitelisted(
		borrower_account_id_ref: &Self::AccountId,
		market_id_ref: &Self::AccountId,
	) -> Result<bool, DispatchError>;
}