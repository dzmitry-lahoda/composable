use crate::{types, Config, Pallet};
use codec::Encode;
use composable_support::validation::TryIntoValidated;
use composable_traits::undercollateralized_loans::{
	LoanConfig, LoanInput, MarketInput, UndercollateralizedLoans,
};
use frame_support::traits::Get;
use sp_runtime::{traits::AccountIdConversion, DispatchError};
use types::{LoanConfigOf, MarketInfoOf, Timestamp};

impl<T: Config> UndercollateralizedLoans for Pallet<T> {
	type BlockNumber = T::BlockNumber;
	type LiquidationStrategyId = T::LiquidationStrategyId;
	type VaultId = T::VaultId;
	type Timestamp = Timestamp;
	type WhiteListBound = T::WhiteListBound;
	type ScheduleBound = T::ScheduleBound;
	fn create_market(
		manager: Self::AccountId,
		input: MarketInput<
			Self::AccountId,
			Self::MayBeAssetId,
			Self::BlockNumber,
			Self::LiquidationStrategyId,
		>,
		keep_alive: bool,
	) -> Result<MarketInfoOf<T>, DispatchError> {
		Self::do_create_market(manager, input.try_into_validated()?, keep_alive)
	}

	fn create_loan(
		input: LoanInput<Self::AccountId, Self::Balance, Self::Timestamp>,
	) -> Result<LoanConfigOf<T>, DispatchError> {
		Self::do_create_loan(input.try_into_validated()?)
	}

	fn borrow(
		borrower_account_id: Self::AccountId,
		loan_account_id: Self::AccountId,
		keep_alive: bool,
	) -> Result<
		LoanConfig<Self::AccountId, Self::MayBeAssetId, Self::Balance, Self::Timestamp>,
		DispatchError,
	> {
		Self::do_borrow(borrower_account_id, loan_account_id, keep_alive)
	}

	fn market_account_id<S: Encode>(postfix: S) -> Self::AccountId {
		T::PalletId::get().into_sub_account_truncating(postfix)
	}

	fn loan_account_id<S: Encode>(postfix: S) -> Self::AccountId {
		T::LoanId::get().into_sub_account_truncating(postfix)
	}

	fn is_borrower_account_whitelisted(
		borrower_account_id_ref: &Self::AccountId,
		market_id_ref: &Self::AccountId,
	) -> Result<bool, DispatchError> {
		Self::is_borrower_account_whitelisted(borrower_account_id_ref, market_id_ref)
	}

	fn repay(
		payer_account_id: Self::AccountId,
		loan_account_id: Self::AccountId,
		repay_amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError> {
		Self::do_repay(payer_account_id, loan_account_id, repay_amount, keep_alive)
	}
}