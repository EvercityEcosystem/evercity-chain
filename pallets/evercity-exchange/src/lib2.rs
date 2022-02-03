#![allow(clippy::unused_unit)]
#![cfg_attr(not(feature = "std"), no_std)]

// mod exchange;
mod trade_request;

// #[cfg(test)]
// mod tests;
// #[cfg(test)]
// mod mock;

use frame_system::RawOrigin;
// use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    decl_event,
    dispatch::{
        DispatchResult,
        Vec,
    },
    traits::UnfilteredDispatchable,
};
use frame_system::{
    ensure_signed,
};
use sp_runtime::traits::StaticLookup;
use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};
use pallet_evercity_assets;
use pallet_assets;

use crate::trade_request::{TradeRequest, HolderType, CARBON_CREDITS_HOLDER_APPROVED, ASSET_HOLDER_APPROVED};

pub type TradeRequestId = u128;
pub type AssetId<T> = <T as pallet_assets::Config>::AssetId;
pub type CarbonCreditsId<T> = <T as pallet_evercity_assets::Config>::AssetId;


pub trait Config: 
    frame_system::Config + 
    // pallet_timestamp::Config + 
    pallet_assets::Config + 
    pallet_evercity_assets::Config + 
    pallet_evercity_carbon_credits::Config + 
    // pallet_evercity::Config + 
{
        type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

type AssBalance<T> =  <T as pallet_assets::Config>::Balance;
type EverAssBalance<T> =  <T as pallet_evercity_assets::Config>::Balance;

type TradeRequestOpt<T: Config> = Option<TradeRequest<T::AccountId, AssetId<T>, CarbonCreditsId<T>, AssBalance<T>, EverAssBalance<T>>> ;

decl_storage! {
    trait Store for Module<T: Config> as EvercityExchange {
        /// Main storage for exchanges
        // TradeRequestById
        //     get(fn trade_request_by_id):
        //     map hasher(blake2_128_concat) TradeRequestId => Option<TradeRequest<T::AccountId, AssetId<T>, CarbonCreditsId<T>, <T as pallet_assets::Config>::Balance, <T as pallet_evercity_assets::Config>::Balance>>;    
        TradeRequestById
            get(fn trade_request_by_id):
            map hasher(blake2_128_concat) TradeRequestId => Option<TradeRequest<T::AccountId, AssetId<T>, CarbonCreditsId<T>, AssBalance<T>, EverAssBalance<T>>>;
        LastTradeRequestId: TradeRequestId;
        
    }
}

// Pallet events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        AssetId = <T as pallet_assets::Config>::AssetId,
        Balance = <T as pallet_assets::Config>::Balance,
    {
        /// \[EverUSDHolder, CarbonCreditsHolder, AssetId, Balance\]
        // CarbonCreditsTradeRequestCreated(TradeRequestId, AccountId, AccountId, <T as pallet_assets::Config>::AssetId, <T as pallet_evercity_assets::Config>::AssetId),
        Lol(AccountId, AssetId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        /// Account does not have an auditor role in Accounts Pallet
        InsufficientAssetBalance,
        InsufficientCarbonCreditsBalance,
		ExchangeIdOwerflow,
		InvalidTradeRequestState,
		BadHolder
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        // fn deposit_event() = default;
        #[weight = 10_000]
        pub fn create_trade_request(origin, 
			partner_trader: T::AccountId, 
			asset_id: AssetId<T>,
			carbon_credits_id: CarbonCreditsId<T>,
			asset_count: <T as pallet_assets::Config>::Balance,
			carbon_credits_count: <T as pallet_evercity_assets::Config>::Balance,
			holder_type: HolderType,
        ) -> DispatchResult {
			let (asset_holder, carbon_credits_holder, approve_mask) = match holder_type {
				HolderType::CarbonCreditsHolder => {
					(partner_trader, ensure_signed(origin)?, CARBON_CREDITS_HOLDER_APPROVED)
				},
				HolderType::AssetHolder => {
					(ensure_signed(origin)?, partner_trader, ASSET_HOLDER_APPROVED)
				}
			};

            let asset_balance = pallet_assets::Module::<T>::balance(asset_id, asset_holder.clone());
			ensure!(asset_balance >= asset_count, Error::<T>::InsufficientAssetBalance);
			let current_carbon_credits_balace = pallet_evercity_assets::Module::<T>::balance(carbon_credits_id, carbon_credits_holder.clone());
			ensure!(current_carbon_credits_balace >= carbon_credits_count, Error::<T>::InsufficientCarbonCreditsBalance);

			let trate_request = 
				TradeRequest::new(
					asset_holder.clone(), 
					carbon_credits_holder.clone(), 
					asset_count, 
					asset_id, 
					carbon_credits_count, 
					carbon_credits_id,
					approve_mask
			);
			let new_id = match LastTradeRequestId::get().checked_add(1) {
                Some(id) => id,
                None => return Err(Error::<T>::ExchangeIdOwerflow.into()),
            };
			// TradeRequestById::<T>::insert(new_id, trate_request);
			// Self::deposit_event(Event::CarbonCreditsTradeRequestCreated(new_id, asset_holder, carbon_credits_holder, asset_id, carbon_credits_id));

            Ok(())
        }

        #[weight = 10_000]
        pub fn accept_trade_request(origin, trade_request_id: TradeRequestId, holder_type: HolderType) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			TradeRequestById::<T>::try_mutate(trade_request_id, |trade_request_opt| -> DispatchResult {
				match trade_request_opt {
					None => todo!(),
					Some(trade_request) => {
						match holder_type {
							HolderType::AssetHolder => {
								ensure!(trade_request.approved == trade_request::CARBON_CREDITS_HOLDER_APPROVED, Error::<T>::InvalidTradeRequestState);
								ensure!(caller == trade_request.asset_holder, Error::<T>::BadHolder);

							},
							HolderType::CarbonCreditsHolder => {
								ensure!(trade_request.approved == trade_request::ASSET_HOLDER_APPROVED, Error::<T>::InvalidTradeRequestState);
								ensure!(caller == trade_request.carbon_credits_holder, Error::<T>::BadHolder);
							},
						}

						let current_asset_balance = pallet_assets::Module::<T>::balance(trade_request.asset_id, trade_request.asset_holder.clone());
						let carbon_credits_balance = 
							pallet_evercity_assets::Module::<T>::balance(trade_request.carbon_credits_id, trade_request.carbon_credits_holder.clone());

						if trade_request.asset_count > current_asset_balance {
							return Err(Error::<T>::InsufficientAssetBalance.into());
						}
						if trade_request.carbon_credits_count > carbon_credits_balance  {
							return Err(Error::<T>::InsufficientCarbonCreditsBalance.into());
						}

						// transfer carbon credits
						let cc_holder_origin = frame_system::RawOrigin::Signed(trade_request.carbon_credits_holder.clone()).into();

						let lol = pallet_evercity_carbon_credits::Module::<T>::is_passport_correct(trade_request.carbon_credits_id);

                            // pallet_evercity_carbon_credits::Module::<T>::transfer_carbon_credits(
                            //         cc_holder_origin, 
                            //         trade_request.carbon_credits_id, 
                            //         trade_request.asset_holder.clone(), 
                            //         trade_request.carbon_credits_count
                            // )?;
							// let asset_holder_source = 
							// 	<T::Lookup as StaticLookup>::unlookup(trade_request.asset_holder.clone());
							// let call = 
							// 	pallet_evercity_assets::Call::<T>::transfer(trade_request.carbon_credits_id, asset_holder_source, trade_request.carbon_credits_count);
							// let result = transfer_call.dispatch_bypass_filter(origin);

                            // transfer everusd then
                            // pallet_evercity::Module::<T>::transfer_everusd(&exchange.ever_usd_holder, &exchange.carbon_credits_holder, exchange.ever_usd_count)?;

					}
				}
				Ok(())
			})?;
			Ok(())
        }


    }
}