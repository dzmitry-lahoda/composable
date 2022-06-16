#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// -------------------------------------------------------------------------------------------
	//                                   Imports and Dependencies
	// -------------------------------------------------------------------------------------------
	use codec::{Codec, FullCodec};
	use composable_traits::{
		dex::Amm,
		instrumental::InstrumentalProtocolStrategy,
		vault::{FundsAvailability, StrategicVault, Vault},
	};
	use frame_support::{
		dispatch::{DispatchError, DispatchResult},
		pallet_prelude::*,
		storage::bounded_btree_set::BoundedBTreeSet,
		traits::fungibles::{Inspect, Mutate, MutateHold, Transfer},
		transactional, Blake2_128Concat, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Zero,
	};
	use sp_std::fmt::Debug;

	use crate::weights::WeightInfo;

	// -------------------------------------------------------------------------------------------
	//                                Declaration Of The Pallet Type
	// -------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// -------------------------------------------------------------------------------------------
	//                                         Config Trait
	// -------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		/// The [`Balance`](Config::Balance) type used by the pallet for bookkeeping.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ Zero;

		/// The [`AssetId`](Config::AssetId) used by the pallet. Corresponds to the Ids used by the
		/// Currency pallet.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		/// The [`VaultId`](Config::VaultId) used by the pallet. Corresponds to the Ids used by the
		/// Vault pallet.
		type VaultId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ Ord
			+ TypeInfo
			+ Into<u128>;

		type Vault: StrategicVault<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			VaultId = Self::VaultId,
		>;

		/// The [`Currency`](Config::Currency).
		///
		/// Currency is used for the assets managed by the vaults.
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type Pablo: Amm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolId = Self::PoolId,
		>;

		/// Type representing the unique ID of a pool.
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ Debug
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy;

		/// The maximum number of vaults that can be associated with this strategy.
		#[pallet::constant]
		type MaxAssociatedVaults: Get<u32>;

		/// The id used as the
		/// [`AccountId`](composable_traits::instrumental::Instrumental::AccountId) of the vault.
		/// This should be unique across all pallets to avoid name collisions with other pallets and
		/// vaults.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// -------------------------------------------------------------------------------------------
	//                                         Pallet Types
	// -------------------------------------------------------------------------------------------

	// -------------------------------------------------------------------------------------------
	//                                       Runtime  Storage
	// -------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn associated_vaults)]
	pub type AssociatedVaults<T: Config> =
		StorageValue<_, BoundedBTreeSet<T::VaultId, T::MaxAssociatedVaults>, ValueQuery>;

	/// An asset whitelisted by Instrumental.
	///
	/// The corresponding Pool to invest the whitelisted asset into.
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::PoolId>;

	// -------------------------------------------------------------------------------------------
	//                                        Runtime Events
	// -------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssociatedVault { vault_id: T::VaultId },

		RebalancedVault { vault_id: T::VaultId },

		UnableToRebalanceVault { vault_id: T::VaultId },

		Setted { asset_id: T::AssetId, pool_id: T::PoolId },
	}

	// -------------------------------------------------------------------------------------------
	//                                        Runtime Errors
	// -------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		VaultAlreadyAssociated,

		TooManyAssociatedStrategies,

		// TODO(belousm): only for MVP version we can assume the `pool_id` is already known and
		// exist. We should remove it in V1.
		PoolNotFound,
	}

	// -------------------------------------------------------------------------------------------
	//                                            Hooks
	// -------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// -------------------------------------------------------------------------------------------
	//                                          Extrinsics
	// -------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Store a mapping of asset_id -> pool_id in the pools runtime storage object
		///
		/// Emits `Setted` event when successful.
		#[pallet::weight(T::WeightInfo::set_pool_id_for_asset())]
		pub fn set_pool_id_for_asset(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			pool_id: T::PoolId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ =
				<Self as InstrumentalProtocolStrategy>::set_pool_id_for_asset(asset_id, pool_id)?;
			Ok(())
		}
	}

	// -------------------------------------------------------------------------------------------
	//                                      Protocol Strategy
	// -------------------------------------------------------------------------------------------

	impl<T: Config> InstrumentalProtocolStrategy for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type VaultId = T::VaultId;
		type PoolId = T::PoolId;

		fn account_id() -> Self::AccountId {
			T::PalletId::get().into_account()
		}

		#[transactional]
		fn associate_vault(vault_id: &Self::VaultId) -> DispatchResult {
			AssociatedVaults::<T>::try_mutate(|vaults| -> DispatchResult {
				ensure!(!vaults.contains(vault_id), Error::<T>::VaultAlreadyAssociated);

				vaults
					.try_insert(*vault_id)
					.map_err(|_| Error::<T>::TooManyAssociatedStrategies)?;

				Self::deposit_event(Event::AssociatedVault { vault_id: *vault_id });

				Ok(())
			})
		}

		#[transactional]
		fn set_pool_id_for_asset(
			asset_id: T::AssetId,
			pool_id: T::PoolId,
		) -> Result<(), DispatchError> {
			if Pools::<T>::contains_key(asset_id) {
				Pools::<T>::mutate(asset_id, |_| pool_id);
			} else {
				Pools::<T>::insert(asset_id, pool_id);
			}
			Ok(())
		}

		fn rebalance() -> DispatchResult {
			AssociatedVaults::<T>::try_mutate(|vaults| -> DispatchResult {
				vaults.iter().for_each(|vault_id| {
					if Self::do_rebalance(vault_id).is_ok() {
						Self::deposit_event(Event::RebalancedVault { vault_id: *vault_id });
					} else {
						Self::deposit_event(Event::UnableToRebalanceVault { vault_id: *vault_id });
					}
				});

				Ok(())
			})
		}

		fn get_apy(_asset: Self::AssetId) -> Result<u128, DispatchError> {
			Ok(0)
		}
	}

	// -------------------------------------------------------------------------------------------
	//                                   Low Level Functionality
	// -------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		#[transactional]
		fn do_rebalance(vault_id: &T::VaultId) -> DispatchResult {
			let asset_id = T::Vault::asset_id(vault_id)?;
			let account_id = T::Vault::account_id(vault_id);
			let pool_id = Self::pools(asset_id).ok_or(Error::<T>::PoolNotFound)?;
			let task = T::Vault::available_funds(vault_id, &Self::account_id())?;
			match task {
				FundsAvailability::Withdrawable(balance) => {
					Self::withdraw(vault_id, pool_id, balance)?;
				},
				FundsAvailability::Depositable(balance) => {
					Self::deposit(vault_id, pool_id, balance)?;
				},
				FundsAvailability::MustLiquidate => {
					Self::liquidate(vault_id, pool_id, &account_id)?;
				},
				FundsAvailability::None => {},
			};
			Ok(())
		}

		#[transactional]
		fn withdraw(
			vault_id: &T::VaultId,
			pool_id: T::PoolId,
			balance: T::Balance,
		) -> DispatchResult {
			let vault_account = T::Vault::account_id(vault_id);
			let lp_token_amount = T::Pablo::amount_of_lp_token_for_added_liquidity(
				pool_id,
				T::Balance::zero(),
				balance,
			)?;
			T::Pablo::add_liquidity(
				&vault_account,
				pool_id,
				T::Balance::zero(),
				balance,
				lp_token_amount,
				bool::default(),
			)
		}

		#[transactional]
		fn deposit(
			vault_id: &T::VaultId,
			pool_id: T::PoolId,
			balance: T::Balance,
		) -> DispatchResult {
			let vault_account = T::Vault::account_id(vault_id);
			let lp_token_amount = T::Pablo::amount_of_lp_token_for_added_liquidity(
				pool_id,
				T::Balance::zero(),
				balance,
			)?;
			T::Pablo::remove_liquidity(
				&vault_account,
				pool_id,
				lp_token_amount,
				T::Balance::zero(),
				T::Balance::zero(),
			)
		}

		#[transactional]
		fn liquidate(
			vault_id: &T::VaultId,
			pool_id: T::PoolId,
			account_id: &T::AccountId,
		) -> DispatchResult {
			let vault_account = T::Vault::account_id(vault_id);
			let lp_token_id = T::Pablo::lp_token(pool_id)?;
			let balance_of_lp_token = T::Currency::balance(lp_token_id, account_id);
			T::Pablo::remove_liquidity(
				&vault_account,
				pool_id,
				balance_of_lp_token,
				T::Balance::zero(),
				T::Balance::zero(),
			)
		}
	}
}

// -----------------------------------------------------------------------------------------------
//                                             Unit Tests
// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}
