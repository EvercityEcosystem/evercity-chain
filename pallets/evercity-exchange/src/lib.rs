#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

mod everusd_trade_request;
mod approve_mask;
mod cc_package_lot;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_std::{prelude::*};
pub use pallet::*;

pub type TradeRequestId = u128;
pub type CarbonCreditsId<T> = pallet_evercity_carbon_credits::AssetId<T>;
pub type CarbonCreditsBalance<T> = pallet_evercity_carbon_credits::Balance<T>;
type Timestamp<T> = pallet_timestamp::Module<T>;
pub type LotId = u128;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::{DispatchResultWithPostInfo},
		pallet_prelude::{*, OptionQuery, ValueQuery}, Blake2_128Concat, ensure,
	};
	use frame_system::pallet_prelude::*;
	use pallet_evercity_bonds::{EverUSDBalance, Expired};
	use crate::{
		everusd_trade_request::{EverUSDTradeRequest, EverUSDTradeHolderType}, 
		approve_mask::{CARBON_CREDITS_HOLDER_APPROVED, EVERUSD_HOLDER_APPROVED},
		cc_package_lot::{CarbonCreditsPackageLotOf},
	};
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: 
		frame_system::Config +
		pallet_evercity_carbon_credits::Config + 
		pallet_evercity_bonds::Config +
		pallet_timestamp::Config
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// pallet storages:
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
	/// Id of last trade everud request
	pub(super) type LastEverUSDTradeRequestId<T: Config> = StorageValue<_, TradeRequestId, ValueQuery>;

	/// Carbon Credits Lots registry - for every Account
	#[pallet::storage]
	pub(super) type CarbonCreditLotRegistry<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, LotId,
		CarbonCreditsPackageLotOf<T>,
		OptionQuery
	>;

	/// Id of last Carbon Credit Lot
	#[pallet::storage]
	pub(super) type LastLotId<T: Config> = StorageValue<_, LotId, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// No carbon credits balance
        InsufficientCarbonCreditsBalance,
		/// Not enough everusd
		InsufficientEverUSDBalance,
		/// If trade request id owerflowd
		TradeRequestIdOwerflow,
		/// Trade request state is invalid
		InvalidTradeRequestState,
		/// invalid holder
		BadHolder,
		/// No trade request found
		TradeRequestNotFound,
		/// Invalid approve state
		BadApprove,
		/// Lot reached time deadline 
		LotExpired,
		/// Lot details are invalid
		InvalidLotDetails,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", T::AssetId = "AssetId")]
	pub enum Event<T: Config> {
		/// \[TradeRequestId, EverUsdHolder, CarbonCreditsHolder\]
		EverUSDTradeRequestCreated(TradeRequestId, T::AccountId, T::AccountId),
		/// \[TradeRequestId\]
		EverUSDTradeRequestAccepted(TradeRequestId),
	}
	
	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

	/// Calls:
	#[pallet::call]
	impl<T: Config> Pallet<T> where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128>  {

		#[pallet::weight(10_000)]
		pub fn create_carbon_credit_lot(
			origin: OriginFor<T>,
			lot: CarbonCreditsPackageLotOf<T>
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			// check if lot doesn't have errors 
			let now = Timestamp::<T>::get();
			ensure!(!lot.is_expired(now), Error::<T>::LotExpired);
			if let Some(target) = lot.target_bearer {
				ensure!(&caller != &target, Error::<T>::InvalidLotDetails);
			}

			// purge expired lots
			let ids_to_delete = CarbonCreditLotRegistry::<T>::iter_prefix(&caller)
				.filter(|(id, lot)| lot.is_expired(now))
				.map(|(id, lot)| id);
			for id in ids_to_delete {
				CarbonCreditLotRegistry::<T>::remove(&caller, id);
			}

			// check if caller has enough Carbon Credits

			// add new lot

			Ok(().into())
		}

		/// <pre>
        /// Method: create_everusd_trade_request(
		/// 			origin: OriginFor<T>, 
		/// 			partner_trader: T::AccountId,
		/// 			ever_usd_count: EverUSDBalance,
		/// 			carbon_credits_asset_id: CarbonCreditsId<T>,
		/// 			carbon_credits_count: CarbonCreditsBalance<T>,
		/// 			holder_type: EverUSDTradeHolderType,)
		/// 
        /// Arguments: origin: OriginFor<T> - Transaction caller
        ///            partner_trader: AccountId - account to trade with
        ///            ever_usd_count: EverUSDBalance - amount of everusd to trade
		/// 		   carbon_credits_asset_id: CarbonCreditsId<T> - asset id of carbon credits
		/// 		   carbon_credits_count: CarbonCreditsBalance<T> - amount of carbon credits to trade
		/// 		   holder_type: EverUSDTradeHolderType - select holder type for transaction caller
		/// 
        /// Access: Any person holding everusd or carbon credits
        ///
        /// Creates trade request to trade carbon credits to everusd of everusd to carbon credits
        /// </pre>
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2))]
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

			let current_everusd_balance = pallet_evercity_bonds::Module::<T>::get_balance(&ever_usd_holder);
			if ever_usd_count > current_everusd_balance {
				return Err(Error::<T>::InsufficientEverUSDBalance.into());
			}
			
			let current_carbon_credits_balace = 
				pallet_evercity_assets::Module::<T>::balance(carbon_credits_asset_id, carbon_credits_holder.clone());
			if carbon_credits_count > current_carbon_credits_balace {
				return Err(Error::<T>::InsufficientCarbonCreditsBalance.into());
			}

			let trade_request = 
				EverUSDTradeRequest::new(
					ever_usd_holder.clone(), 
					carbon_credits_holder.clone(), 
					ever_usd_count, 
					carbon_credits_asset_id, 
					carbon_credits_count, 
					approve_mask
				);
			
			let new_id = match LastEverUSDTradeRequestId::<T>::get().checked_add(1) {
				Some(id) => id,
				None => return Err(Error::<T>::TradeRequestIdOwerflow.into()),
			};
			EverUSDTradeRequestById::<T>::insert(new_id, trade_request);
			LastEverUSDTradeRequestId::<T>::mutate(|x| *x = new_id);

			Self::deposit_event(Event::EverUSDTradeRequestCreated(new_id, ever_usd_holder, carbon_credits_holder));
			Ok(().into())
		}

		/// <pre>
        /// Method: accept_everusd_trade_request
		/// 
        /// Arguments: origin: OriginFor<T> - Transaction caller
        ///            trade_request_id: TradeRequestId - trade request id
        ///            holder_type: EverUSDTradeHolderType - select holder type for transaction caller
		/// 
        /// Access: Any person holding everusd or carbon credits depending on trade request approve mask
        ///
        /// Accepts trade request to trade carbon credits to everusd of everusd to carbon credits
        /// </pre>
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 3))]
		pub fn accept_everusd_trade_request(
			origin: OriginFor<T>, 
			trade_request_id: TradeRequestId, 
			holder_type: EverUSDTradeHolderType
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			EverUSDTradeRequestById::<T>::try_mutate(
				trade_request_id, |trade_request_opt| -> DispatchResultWithPostInfo {
                    match trade_request_opt  {
                        None => return Err(Error::<T>::TradeRequestNotFound.into()),
                        Some(trade_request) => {
                            match holder_type {
                                EverUSDTradeHolderType::EverUSDHolder => {
                                    ensure!(trade_request.approved == CARBON_CREDITS_HOLDER_APPROVED, Error::<T>::BadApprove);
                                    ensure!(caller == trade_request.ever_usd_holder, Error::<T>::BadHolder);

                                },
                                EverUSDTradeHolderType::CarbonCreditsHolder => {
                                    ensure!(trade_request.approved == EVERUSD_HOLDER_APPROVED, Error::<T>::BadApprove);
                                    ensure!(caller == trade_request.carbon_credits_holder, Error::<T>::BadHolder);
                                },
                            }

                            let current_everusd_balance = pallet_evercity_bonds::Module::<T>::get_balance(&trade_request.ever_usd_holder);
                            let carbon_credits_balance = pallet_evercity_assets::Module::<T>::balance(trade_request.carbon_credits_asset_id, trade_request.carbon_credits_holder.clone());

                            if trade_request.ever_usd_count > current_everusd_balance {
                                return Err(Error::<T>::InsufficientEverUSDBalance.into());
                            }
                            if trade_request.carbon_credits_count > carbon_credits_balance  {
                                return Err(Error::<T>::InsufficientCarbonCreditsBalance.into());
                            }

                            let cc_holder_origin = frame_system::RawOrigin::Signed(trade_request.carbon_credits_holder.clone()).into();
                            pallet_evercity_carbon_credits::Pallet::<T>::transfer_carbon_credits(
                                    cc_holder_origin, 
                                    trade_request.carbon_credits_asset_id, 
                                    trade_request.ever_usd_holder.clone(), 
                                    trade_request.carbon_credits_count
                            )?;

                            // transfer everusd then
							pallet_evercity_bonds::Module::<T>::transfer_everusd(
								&trade_request.ever_usd_holder, 
								&trade_request.carbon_credits_holder, 
								trade_request.ever_usd_count
							)?;

                        }
                    }
                    Ok(().into())
                })?;
			Self::deposit_event(Event::EverUSDTradeRequestAccepted(trade_request_id));
			Ok(().into())
		}
    }
}