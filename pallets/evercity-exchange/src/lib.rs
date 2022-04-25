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

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::{DispatchResultWithPostInfo},
		pallet_prelude::{*, OptionQuery, ValueQuery}, Blake2_128Concat, ensure,
	};
	use frame_system::pallet_prelude::*;
	use pallet_evercity_bonds::{EverUSDBalance, Expired};
	use sp_runtime::traits::{CheckedAdd};
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

	/// Carbon Credits Lots registry - for every AccountId and AssetId
	#[pallet::storage]
	pub(super) type CarbonCreditLotRegistry<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, CarbonCreditsId<T>,
		Vec<CarbonCreditsPackageLotOf<T>>,
		ValueQuery
	>;

	#[pallet::error]
	pub enum Error<T> {
		/// Not enough carbon credits balance
        InsufficientCarbonCreditsBalance,
		/// Not enough everUSD
		InsufficientEverUSDBalance,
		/// If trade request id owerflow
		TradeRequestIdOwerflow,
		/// Trade request state is invalid
		InvalidTradeRequestState,
		/// Invalid holder
		BadHolder,
		/// No trade request found
		TradeRequestNotFound,
		/// Invalid approve state
		BadApprove,
		/// Lot reached time deadline 
		LotExpired,
		/// Lot details are invalid
		InvalidLotDetails,
		/// Attempt to buy more CC than exist in lot
		NotEnoughCarbonCreditsInLot,
		/// Lot not found
		LotNotFound,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", T::AssetId = "AssetId")]
	pub enum Event<T: Config> {
		/// \[TradeRequestId, EverUsdHolder, CarbonCreditsHolder\]
		EverUSDTradeRequestCreated(TradeRequestId, T::AccountId, T::AccountId),
		/// \[TradeRequestId\]
		EverUSDTradeRequestAccepted(TradeRequestId),
		/// \[CarbonCreditsSeller, AssetId, CarbonCreditsLot\]
		CarbonCreditsLotCreated(T::AccountId, CarbonCreditsId::<T>, CarbonCreditsPackageLotOf::<T>),
		/// \[Buyer, Seller, Amount\]
		CarbonCreditsBought(T::AccountId, T::AccountId, CarbonCreditsBalance::<T>),
	}
	
	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

	/// Calls:
	#[pallet::call]
	impl<T: Config> Pallet<T> where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u64> + Into<u64>  {

		/// <pre>
		/// Method: create_carbon_credit_lot
		/// Arguments: origin: OriginFor<T> - transaction caller
		///			   asset_id: CarbonCreditsId<T> - Carbon Credit asset id
		///			   new_lot: CarbonCreditsPackageLotOf<T> - new lot of Carbon Credits to auction off
		/// Access: for Carbon Credits holder
		/// 
		/// Creates new Carbon Credits Lot of given asset_id. 
		/// CarbonCreditsPackageLotOf new_lot contains:
		/// 			"target_bearer" - optional, if set - lot is private
		/// 			"deadline" - lot can be sold only before deadline
		/// 			"amount" - amount of Carbon Credits for sell
		/// 			"price_per_item" - price per one Carbon Credit
		/// Function checks if deadline is correct, if caller has enough Carbon Credits.
		/// Function purges another expired lots for this caller.
		/// </pre>
		#[pallet::weight(10_000)]
		pub fn create_carbon_credit_lot(
			origin: OriginFor<T>,
			asset_id: CarbonCreditsId<T>,
			new_lot: CarbonCreditsPackageLotOf<T>
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			// check if lot doesn't have errors 
			let now = Timestamp::<T>::get();
			ensure!(!new_lot.is_expired(now), Error::<T>::LotExpired);
			if let Some(target) = &new_lot.target_bearer {
				ensure!(&caller != target, Error::<T>::InvalidLotDetails);
			}

			// purge expired lots
			CarbonCreditLotRegistry::<T>::mutate(&caller, asset_id, 
				|lots| lots.retain(|lot| !lot.is_expired(now)));

			// check if caller has enough Carbon Credits
			let lots_sum = CarbonCreditLotRegistry::<T>::get(&caller, asset_id)
				.iter().map(|lot| lot.amount).sum();
			let total_sum = new_lot.amount.checked_add(&lots_sum);
			if let Some(sum) = total_sum {
				ensure!(sum <= pallet_evercity_assets::Module::<T>::balance(asset_id, caller.clone()), 
					Error::<T>::InsufficientCarbonCreditsBalance);
			}

			// add new lot
			CarbonCreditLotRegistry::<T>::mutate(&caller, asset_id, |lots| lots.push(new_lot.clone()));
			Self::deposit_event(Event::CarbonCreditsLotCreated(caller, asset_id, new_lot));

			Ok(().into())
		}

		/// <pre>
		/// Method: buy_carbon_credit_lot_units
		/// Arguments: origin: OriginFor<T> - transaction caller
		///				seller: T::AccountId - lot seller
		///				asset_id: CarbonCreditsId<T> - Carbon Credit asset id
		///				lot: CarbonCreditsPackageLotOf<T> - from whitch Carbon Credits are bought 
		///				amount: CarbonCreditsBalance<T> - amount of Carbon Credits to buy
		/// Access: any account having enough EverUSD,
		/// 		for private lot - only account in that lot
		/// 
		/// Buys a specified amount of Carbon Credits from specified lot created by 
		/// create_carbon_credit_lot(..) call. Lot should not be expired. 
		/// Buyer should have enough EverUSD balance. If lot is private 
		/// (lot.targer_bearer are set) - only target_bearer can buy from that lot. 
		/// After selling other expired seller's lots are purged.
		/// </pre>
		#[pallet::weight(10_000)]
		pub fn buy_carbon_credit_lot_units(
			origin: OriginFor<T>,
			seller: T::AccountId,
			asset_id: CarbonCreditsId<T>,
			mut lot: CarbonCreditsPackageLotOf<T>,
			amount: CarbonCreditsBalance<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			// check if buy attempt is correct
			let now = Timestamp::<T>::get();
			ensure!(!lot.is_expired(now), Error::<T>::LotExpired);
			ensure!(amount <= lot.amount, Error::<T>::NotEnoughCarbonCreditsInLot);
			let total_price = lot.price_per_item*pallet_evercity_carbon_credits::Module::<T>::balance_to_u64(amount);
			ensure!(total_price < pallet_evercity_bonds::Module::<T>::get_balance(&caller),
				Error::<T>::InsufficientEverUSDBalance);
			// check that target bearer is the same as caller if lot is private 
			if let Some(account) = lot.target_bearer.clone() {
				ensure!(account == caller, Error::<T>::LotNotFound)
			}
			
			// change or remove lot if all ok
			CarbonCreditLotRegistry::<T>::try_mutate(&seller, asset_id, 
				|lots| -> DispatchResultWithPostInfo {
					if let Some(index) = lots.iter().position(|item| item==&lot ){
						// remove or change lot
						if lot.amount == amount {
							lots.remove( index );
						} else if lot.amount > amount {
							lot.amount = lot.amount - amount;
							lots[index] = lot;
						}
						// transfer CC
						let cc_holder_origin = frame_system::RawOrigin::Signed(seller.clone()).into();
						pallet_evercity_carbon_credits::Pallet::<T>::transfer_carbon_credits(
								cc_holder_origin, 
								asset_id, 
								caller.clone(), 
								amount
						)?;
						// transfer everUSD then
						pallet_evercity_bonds::Module::<T>::transfer_everusd(
							&caller, 
							&seller, 
							total_price
						)?;
						
						// purge expired lots
						if !lots.is_empty() {
							lots.retain(|item| !item.is_expired(now));
						}
						
						Self::deposit_event(Event::CarbonCreditsBought(caller, seller.clone(), amount));

						Ok(().into())
					}else{
						Err(Error::<T>::InvalidLotDetails.into())
					}
			})?;
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