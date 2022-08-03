use crate::defi::CurrencyPair;
use composable_support::math::safe::SafeAdd;
use frame_support::pallet_prelude::*;
use sp_runtime::{traits::Zero, ArithmeticError, Perquintill};
use sp_std::collections::btree_set::BTreeSet;
use sp_std::collections::btree_map::BTreeMap;

#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct MarketConfig<AccountId, AssetId, BlockNumber, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	/// The account id of this market
	account_id: AccountId,
	/// The owner of this market.
	manager: AccountId,
	/// The vault containing the borrow asset.
	borrow_asset_vault: VaultId,
	/// The asset being used as collateral.
	collateral_asset: AssetId,
	/// The asset being used as borrow asset.
	borrow_asset: AssetId,
	/// Number of blocks until invalidate oracle's price.
	max_price_age: BlockNumber,
	/// Borrowers which are allowed to use the service.
	whitelist: BTreeSet<AccountId>,
}

impl<AccountId, AssetId, BlockNumber, VaultId>
	MarketConfig<AccountId, AssetId, BlockNumber, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	pub fn new(
		account_id: AccountId,
		manager: AccountId,
		borrow_asset_vault: VaultId,
		borrow_asset: AssetId,
		collateral_asset: AssetId,
		max_price_age: BlockNumber,
		whitelist: BTreeSet<AccountId>,
	) -> Self {
		Self {
			account_id,
			manager,
			borrow_asset_vault,
			borrow_asset,
			collateral_asset,
			max_price_age,
			whitelist,
		}
	}

	/// Get a reference to the market config's account id.
	pub fn account_id(&self) -> &AccountId {
		&self.account_id
	}

	/// Get a reference to the market config's manager.
	pub fn manager(&self) -> &AccountId {
		&self.manager
	}
	/// Get a reference to the market config's borrow asset vault.
	pub fn borrow_asset_vault(&self) -> &VaultId {
		&self.borrow_asset_vault
	}

	/// Get a reference to the market config's borrow asset.
	pub fn borrow_asset(&self) -> &AssetId {
		&self.borrow_asset
	}

	/// Get a reference to the market config's collateral asset.
	pub fn collateral_asset(&self) -> &AssetId {
		&self.collateral_asset
	}

	/// Get a reference to the market config's max price age.
	pub fn max_price_age(&self) -> &BlockNumber {
		&self.max_price_age
	}

	/// Get a reference to the market config's whitelist.
	#[must_use]
	pub fn whitelist(&self) -> &BTreeSet<AccountId> {
		&self.whitelist
	}
}

#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct LoanConfig<AccountId, Balance, Percent, RepaymentStrategy, TimeMeasure>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	TimeMeasure: Clone + Eq + PartialEq,
	Percent: Clone + Eq + PartialEq,
	RepaymentStrategy: Clone + Eq + PartialEq,
{
	/// Loan account id.
	account_id: AccountId,
	/// Market account id.
	market_account_id: AccountId,
	/// Borrower account id.
	/// Should be whitelisted.
	borrower_account_id: AccountId,
	/// Amount of borrowed money.  
	principal: Balance,
	/// Amount of assets which should be putted as collateral.
	collateral: Balance,
    /// Schedule of payments 
    schedule: BTreeMap<TimeMeasure, Percent>,	
    /// The moment of the first interest payment. 
    first_payment_moment: TimeMeasure,
    /// The moment of the last interest payment and principal repayment. 
    last_payment_moment: TimeMeasure,
    /// Payment strategy which should be applyed.
	/// For instance borrower have to pay principal when loan is mature (one strategy),
	/// or he may pay principal partially, simultaneously with interest payments.   
	repayment_strategy: RepaymentStrategy,
}

impl<AccountId, Balance, Percent, RepaymentStrategy, TimeMeasure>
	LoanConfig<AccountId, Balance, Percent, RepaymentStrategy, TimeMeasure>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	TimeMeasure: Clone + Eq + PartialEq + Ord,
	Percent: Clone + Eq + PartialEq,
	RepaymentStrategy: Clone + Eq + PartialEq,
{
	pub fn new(
		account_id: AccountId,
		market_account_id: AccountId,
		borrower_account_id: AccountId,
		principal: Balance,
		collateral: Balance,
        schedule: Vec<(TimeMeasure, Percent)>,	
		repayment_strategy: RepaymentStrategy,
	) -> Self {
	    let schedule: BTreeMap<TimeMeasure, Percent> = schedule.into_iter().collect();
        // We are sure thate BTreeMap is not empty
        // TODO: @mikolaichuk: May be it would be better to use BiBoundedVec as input here.
        let first_payment_moment = schedule.keys().min().unwrap().clone();
        let last_payment_moment = schedule.keys().max().unwrap().clone();
        Self {
			account_id,
			market_account_id,
			borrower_account_id,
			principal,
			collateral,
		    schedule,
            first_payment_moment,
            last_payment_moment,
            repayment_strategy,
		}
	}

	/// Get a reference to the loan config's account id.
	pub fn account_id(&self) -> &AccountId {
		&self.account_id
	}

	/// Get a reference to the loan config's market account id.
	pub fn market_account_id(&self) -> &AccountId {
		&self.market_account_id
	}

	/// Get a reference to the loan config's borrower account id.
	pub fn borrower_account_id(&self) -> &AccountId {
		&self.borrower_account_id
	}

	/// Get a reference to the loan config's principal.
	pub fn principal(&self) -> &Balance {
		&self.principal
	}

	/// Get a reference to the loan config's collateral.
	pub fn collateral(&self) -> &Balance {
		&self.collateral
	}
    
	/// Get a reference to the loan payment schedule.
    pub fn schedule(&self) -> &BTreeMap<TimeMeasure, Percent> {
        &self.schedule
    }
    
	/// Get a reference to the loan first payment moment. 
    pub fn first_payment_moment(&self) -> &TimeMeasure {
        &self.first_payment_moment
    }

	/// Get a reference to the loan last payment moment. 
    pub fn last_payment_moment(&self) -> &TimeMeasure {
        &self.last_payment_moment
    }

	/// Get a reference to the loan config's payment strategy.
	pub fn repayment_strategy(&self) -> &RepaymentStrategy {
		&self.repayment_strategy
	}

   }

// some fields are hiden since they should be immutable
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct MarketInfo<AccountId, AssetId, BlockNumber, LiquidationStrategyId, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	LiquidationStrategyId: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	config: MarketConfig<AccountId, AssetId, BlockNumber, VaultId>,
	pub liquidation_strategies: Vec<LiquidationStrategyId>,
}

impl<AccountId, AssetId, BlockNumber, LiquidationStrategyId, VaultId>
	MarketInfo<AccountId, AssetId, BlockNumber, LiquidationStrategyId, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	LiquidationStrategyId: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	pub fn new(
		config: MarketConfig<AccountId, AssetId, BlockNumber, VaultId>,
		liquidation_strategies: Vec<LiquidationStrategyId>,
	) -> Self {
		Self { config, liquidation_strategies }
	}

	/// Get a reference to the market info's config.
	pub fn config(&self) -> &MarketConfig<AccountId, AssetId, BlockNumber, VaultId> {
		&self.config
	}

	/// Get a reference to the market info's liquidation strategies.
	pub fn liquidation_strategies(&self) -> &Vec<LiquidationStrategyId> {
		&self.liquidation_strategies
	}
}

// This entity can be usde if we need add mutable field to the loan's configuration. 
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct LoanInfo<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	Percent: Clone + Eq + PartialEq,
	RepaymentStrategy: Clone + Eq + PartialEq,
{
	/// Loan configuration defines loan terms
	config: LoanConfig<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>,
}

impl<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>
	LoanInfo<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq + Zero,
	BlockNumber: SafeAdd + Clone + Eq + PartialEq,
	Percent: Clone + Eq + PartialEq,
	RepaymentStrategy: Clone + Eq + PartialEq,

{
	pub fn new(
		config: LoanConfig<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>,
	) -> Result<Self, ArithmeticError> {
		Ok(Self { config })
	}

	/// Get a reference to the loan info's config.
	pub fn config(
		&self,
	) -> &LoanConfig<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy> {
		&self.config
	}
}

/// input to create market extrinsic
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub struct MarketInput<AccountId, AssetId, BlockNumber, LiquidationStrategyId> {
	/// Collateral currency and borrow currency.
	pub currency_pair: CurrencyPair<AssetId>,
	/// Reserve factor of market borrow vault.
	pub reserved_factor: Perquintill,
    /// List of trusted borrowers	
    pub whitelist: BTreeSet<AccountId>,
	/// Liquidation engine id.
	pub liquidation_strategies: Vec<LiquidationStrategyId>,
	/// Count of blocks until throw error PriceIsTooOld.
	pub max_price_age: BlockNumber,
}

impl<AccountId, AssetId: Copy, BlockNumber, LiquidationStrategyId>
	MarketInput<AccountId, AssetId, BlockNumber, LiquidationStrategyId>
{
	pub fn borrow_asset(&self) -> AssetId {
		self.currency_pair.quote
	}
	pub fn collateral_asset(&self) -> AssetId {
		self.currency_pair.base
	}
	pub fn reserved_factor(&self) -> Perquintill {
		self.reserved_factor
	}
}

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, RuntimeDebug)]
pub struct LoanInput<AccountId, Balance, Percent, RepaymentStrategy, TimeMeasure> {
	/// Loan belongs to this market.
	pub market_account_id: AccountId,
	/// This account id have to be whitelisted.
	pub borrower_account_id: AccountId,
	/// Amount of borrowed money.  
	pub principal: Balance,
	/// Amount of assets which should be deposited as collateral.
	pub collateral: Balance,
	/// How often borrowers have to pay interest.
    pub payment_schedule: Vec<(TimeMeasure, Percent)>,
	/// Payment strategie which should be applyed.
	/// For instance borrower have to pay principal when loan is mature (one strategy),
	/// or he may pay principal partially, simultaneously with interest payments.   
	pub repayment_strategy: RepaymentStrategy,
}



