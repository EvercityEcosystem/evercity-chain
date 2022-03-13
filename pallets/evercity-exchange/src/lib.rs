#![cfg_attr(not(feature = "std"), no_std)]

mod asset_trade_request;
mod everusd_trade_request;
mod approve_mask;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_std::{prelude::*};
use sp_runtime::{traits::{StaticLookup}
};

pub use pallet::*;

pub type TradeRequestId = u128;
pub type AssetId<T> = <T as pallet_assets::Config>::AssetId;
pub type CarbonCreditsId<T> = pallet_evercity_carbon_credits::AssetId<T>;
pub type CarbonCreditsBalance<T> = pallet_evercity_carbon_credits::Balance<T>;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*, traits::UnfilteredDispatchable,
	};
	use frame_system::pallet_prelude::*;
	use pallet_evercity_bonds::EverUSDBalance;
	use crate::{asset_trade_request::{
		AssetTradeRequest, AssetTradeHolderType}, 
		everusd_trade_request::{EverUSDTradeRequest, EverUSDTradeHolderType}, approve_mask::{CARBON_CREDITS_HOLDER_APPROVED, EVERUSD_HOLDER_APPROVED, ASSET_HOLDER_APPROVED}
	};
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


    #[pallet::config]
	/// The module configuration trait.
	pub trait Config: 
		frame_system::Config +
		pallet_assets::Config + 
		// pallet_evercity_assets::Config + 
		pallet_evercity_carbon_credits::Config + 
		pallet_evercity_bonds::Config +
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// pallet storages:

	#[pallet::storage]
	/// Details of a asset-carbon crdits trade request
	pub(super) type AssetTradeRequestById<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		TradeRequestId,
		AssetTradeRequest<T::AccountId, AssetId<T>, CarbonCreditsId<T>, <T as pallet_assets::Config>::Balance, CarbonCreditsBalance<T>>, 
		OptionQuery
	>;

	#[pallet::storage]
	/// Details of a evrusd-carbon crdits trade request
	pub(super) type EverUSDTradeRequestById<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		TradeRequestId,
		EverUSDTradeRequest<T::AccountId, CarbonCreditsId<T>, CarbonCreditsBalance<T>, EverUSDBalance>, 
		OptionQuery
	>;

	#[pallet::storage]
	/// Id of last trade asset request
	pub(super) type LastAssetTradeRequestId<T: Config> = StorageValue<_, TradeRequestId, ValueQuery>;

	#[pallet::storage]
	/// Id of last trade everud request
	pub(super) type LastEverUSDTradeRequestId<T: Config> = StorageValue<_, TradeRequestId, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
        InsufficientAssetBalance,
        InsufficientCarbonCreditsBalance,
		ExchangeIdOwerflow,
		InvalidTradeRequestState,
		BadHolder
    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", T::AssetId = "AssetId")]
	pub enum Event<T: Config> {
        /// \[TradeRequestId, AssetHolder, CarbonCreditsHolder, AssetId, CarbonCreditsId\]
        AssetCarbonCreditsTradeRequestCreated(TradeRequestId, T::AccountId, T::AccountId, <T as pallet_assets::Config>::AssetId, CarbonCreditsId<T>),
		/// \[TradeRequestId\]
        AssetCarbonCreditsTradeRequestAccepted(TradeRequestId),
    }
	
	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

    #[pallet::call]
	impl<T: Config> Pallet<T> where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128>  {
		#[pallet::weight(10_000)]
		pub fn create_everusd_trade_request(
			origin: OriginFor<T>, 
            partner_trader: T::AccountId,
            ever_usd_count: EverUSDBalance,
            carbon_credits_asset_id: CarbonCreditsId<T>,
            carbon_credits_count: CarbonCreditsBalance<T>,
            holder_type: EverUSDTradeHolderType,
		) -> DispatchResultWithPostInfo {
			let (ever_usd_holder, carbon_credits_holder, approve_mask) = match holder_type {
				EverUSDTradeHolderType::CarbonCreditsHolder => {
					(partner_trader, ensure_signed(origin)?, CARBON_CREDITS_HOLDER_APPROVED)
				},
				EverUSDTradeHolderType::EverUSDHolder => {
					(ensure_signed(origin)?, partner_trader, EVERUSD_HOLDER_APPROVED)
				}
			};

			

            Ok(().into())
		}


		#[pallet::weight(10_000)]
		pub fn create_asset_trade_request(
			origin: OriginFor<T>,
			partner_trader: T::AccountId, 
			asset_id: AssetId<T>,
			carbon_credits_id: CarbonCreditsId<T>,
			asset_count: <T as pallet_assets::Config>::Balance,
			carbon_credits_count: CarbonCreditsBalance<T>,
			holder_type: AssetTradeHolderType,
		) -> DispatchResultWithPostInfo {
			let (asset_holder, carbon_credits_holder, approve_mask) = match holder_type {
				AssetTradeHolderType::CarbonCreditsHolder => {
					(partner_trader, ensure_signed(origin)?, CARBON_CREDITS_HOLDER_APPROVED)
				},
				AssetTradeHolderType::AssetHolder => {
					(ensure_signed(origin)?, partner_trader, ASSET_HOLDER_APPROVED)
				}
			};

            let asset_balance = pallet_assets::Module::<T>::balance(asset_id, asset_holder.clone());
			ensure!(asset_balance >= asset_count, Error::<T>::InsufficientAssetBalance);
			let carbon_credits_balance = pallet_evercity_carbon_credits::Module::<T>::balance(carbon_credits_id, carbon_credits_holder.clone());
			ensure!(carbon_credits_balance >= carbon_credits_count, Error::<T>::InsufficientCarbonCreditsBalance);

			let trate_request = 
				AssetTradeRequest::new(
					asset_holder.clone(), 
					carbon_credits_holder.clone(), 
					asset_count, 
					asset_id, 
					carbon_credits_count, 
					carbon_credits_id,
					approve_mask
			);
			let new_id = match LastAssetTradeRequestId::<T>::get().checked_add(1) {
                Some(id) => id,
                None => return Err(Error::<T>::ExchangeIdOwerflow.into()),
            };
			AssetTradeRequestById::<T>::insert(new_id, trate_request);
			Self::deposit_event(Event::AssetCarbonCreditsTradeRequestCreated(new_id, asset_holder, carbon_credits_holder, asset_id, carbon_credits_id));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn accept_asset_trade_request(
			origin: OriginFor<T>, 
			trade_request_id: TradeRequestId, 
			holder_type: AssetTradeHolderType
		) -> DispatchResultWithPostInfo { 
			let caller = ensure_signed(origin)?;
			AssetTradeRequestById::<T>::try_mutate(trade_request_id, |trade_request_opt| -> DispatchResultWithPostInfo {
				match trade_request_opt {
					None => todo!(),
					Some(trade_request) => {
						match holder_type {
							AssetTradeHolderType::AssetHolder => {
								ensure!(trade_request.approved == CARBON_CREDITS_HOLDER_APPROVED, Error::<T>::InvalidTradeRequestState);
								ensure!(caller == trade_request.asset_holder, Error::<T>::BadHolder);

							},
							AssetTradeHolderType::CarbonCreditsHolder => {
								ensure!(trade_request.approved == ASSET_HOLDER_APPROVED, Error::<T>::InvalidTradeRequestState);
								ensure!(caller == trade_request.carbon_credits_holder, Error::<T>::BadHolder);
							},
						}

						let current_asset_balance = pallet_assets::Module::<T>::balance(trade_request.asset_id, trade_request.asset_holder.clone());
						let carbon_credits_balance = 
							pallet_evercity_carbon_credits::Module::<T>::balance(trade_request.carbon_credits_id, trade_request.carbon_credits_holder.clone());

						if trade_request.asset_count > current_asset_balance {
							return Err(Error::<T>::InsufficientAssetBalance.into());
						}
						if trade_request.carbon_credits_count > carbon_credits_balance  {
							return Err(Error::<T>::InsufficientCarbonCreditsBalance.into());
						}

						// transfer carbon credits
						let cc_holder_origin = frame_system::RawOrigin::Signed(trade_request.carbon_credits_holder.clone()).into();
						pallet_evercity_carbon_credits::Module::<T>::transfer_carbon_credits(
								cc_holder_origin, 
								trade_request.carbon_credits_id, 
								trade_request.asset_holder.clone(), 
								trade_request.carbon_credits_count
						)?;
						let carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(trade_request.carbon_credits_holder.clone());
						let asset_transfer_call = pallet_assets::Call::<T>::transfer(trade_request.asset_id, carbon_credits_holder_source, trade_request.asset_count);
						let asset_holder_origin = frame_system::RawOrigin::Signed(trade_request.asset_holder.clone()).into();
						asset_transfer_call.dispatch_bypass_filter(asset_holder_origin)?;
						Self::deposit_event(Event::AssetCarbonCreditsTradeRequestAccepted(trade_request_id));
					}
				}
				Ok(().into())
			})?;
			Ok(().into())
		}
    }

	impl<T: Config> Pallet<T> {
		#[cfg(test)]
		pub fn create_and_mint_test_asset(
			account_id: T::AccountId, 
			asset_id: AssetId<T>, 
			min_balance: <T as pallet_assets::Config>::Balance, 
			balance: <T as pallet_assets::Config>::Balance
		) {
			let cc_holder_origin = frame_system::RawOrigin::Signed(account_id.clone()).into();
			let carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(account_id.clone());
			let create_call = pallet_assets::Call::<T>::create(asset_id, carbon_credits_holder_source.clone(), 0, min_balance);
			let _ = create_call.dispatch_bypass_filter(cc_holder_origin);
			let cc_holder_origin = frame_system::RawOrigin::Signed(account_id.clone()).into();
			let mint_call = pallet_assets::Call::<T>::mint(asset_id, carbon_credits_holder_source, balance);
			let _ = mint_call.dispatch_bypass_filter(cc_holder_origin);
		}
	}
}