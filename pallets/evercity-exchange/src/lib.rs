#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod trade_request;

use sp_std::{fmt::Debug, prelude::*};
use sp_runtime::{
	RuntimeDebug,
	traits::{
		AtLeast32BitUnsigned, Zero, StaticLookup, Saturating, CheckedSub, CheckedAdd,
	}
};
use codec::{Encode, Decode, HasCompact};
use frame_support::{
	ensure,
	traits::{Currency, ReservableCurrency, BalanceStatus::Reserved},
	dispatch::DispatchError,
};

pub use pallet::*;

pub type TradeRequestId = u128;
pub type AssetId<T> = <T as pallet_assets::Config>::AssetId;
pub type CarbonCreditsId<T> = pallet_evercity_carbon_credits::AssetId<T>;
pub type CarbonCreditsBalance<T> = pallet_evercity_carbon_credits::Balance<T>;
// pub type CarbonCreditsId<T> = <T as pallet_evercity_assets::Config>::AssetId;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use crate::trade_request::{TradeRequest, HolderType, CARBON_CREDITS_HOLDER_APPROVED, ASSET_HOLDER_APPROVED};
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


    #[pallet::config]
	/// The module configuration trait.
	pub trait Config: 
		frame_system::Config +
		pallet_assets::Config + 
		pallet_evercity_assets::Config + 
		pallet_evercity_carbon_credits::Config + 
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// pallet storages:

	#[pallet::storage]
	/// Details of a asset-carbon crdits trade request
	pub(super) type TradeRequestById<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		TradeRequestId,
		TradeRequest<T::AccountId, AssetId<T>, CarbonCreditsId<T>, <T as pallet_assets::Config>::Balance, CarbonCreditsBalance<T>>, 
		OptionQuery
	>;

	#[pallet::storage]
	/// Id of last trade request
	pub(super) type LastTradeRequestId<T: Config> = StorageValue<_, TradeRequestId, ValueQuery>;


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
        /// \[AssetHolder, CarbonCreditsHolder, AssetId, CarbonCreditsId\]
        CarbonCreditsTradeRequestCreated(TradeRequestId, T::AccountId, T::AccountId, <T as pallet_assets::Config>::AssetId, CarbonCreditsId<T>),
    }
	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;


    #[pallet::call]
	impl<T: Config> Pallet<T> where T: pallet_evercity_carbon_credits::Config + pallet_evercity_assets::Config {
		#[pallet::weight(10_000)]
		pub fn create_trade_request(
			origin: OriginFor<T>,
			partner_trader: T::AccountId, 
			asset_id: AssetId<T>,
			carbon_credits_id: CarbonCreditsId<T>,
			asset_count: <T as pallet_assets::Config>::Balance,
			carbon_credits_count: CarbonCreditsBalance<T>,
			holder_type: HolderType,
		) -> DispatchResultWithPostInfo {
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
			let current_carbon_credits_balace = pallet_evercity_carbon_credits::Module::<T>::balance(carbon_credits_id, carbon_credits_holder.clone());
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
			let new_id = match LastTradeRequestId::<T>::get().checked_add(1) {
                Some(id) => id,
                None => return Err(Error::<T>::ExchangeIdOwerflow.into()),
            };
			TradeRequestById::<T>::insert(new_id, trate_request);
			Self::deposit_event(Event::CarbonCreditsTradeRequestCreated(new_id, asset_holder, carbon_credits_holder, asset_id, carbon_credits_id));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn accept_trade_request(origin: OriginFor<T>, trade_request_id: TradeRequestId, holder_type: HolderType) -> DispatchResultWithPostInfo { 
			let caller = ensure_signed(origin)?;
			TradeRequestById::<T>::try_mutate(trade_request_id, |trade_request_opt| -> DispatchResultWithPostInfo {
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
							pallet_evercity_carbon_credits::Module::<T>::balance(trade_request.carbon_credits_id, trade_request.carbon_credits_holder.clone());

						if trade_request.asset_count > current_asset_balance {
							return Err(Error::<T>::InsufficientAssetBalance.into());
						}
						if trade_request.carbon_credits_count > carbon_credits_balance  {
							return Err(Error::<T>::InsufficientCarbonCreditsBalance.into());
						}

						// transfer carbon credits
						let cc_holder_origin = frame_system::RawOrigin::Signed(trade_request.carbon_credits_holder.clone()).into();

						// let lol = pallet_evercity_carbon_credits::Module::<T>::is_passport_correct(trade_request.carbon_credits_id);

						pallet_evercity_carbon_credits::Module::<T>::transfer_carbon_credits(
								cc_holder_origin, 
								trade_request.carbon_credits_id, 
								trade_request.asset_holder.clone(), 
								trade_request.carbon_credits_count
						)?;
						// let asset_holder_source = 
						// 	<T::Lookup as StaticLookup>::unlookup(trade_request.asset_holder.clone());
						// let call = 
						// 	pallet_evercity_assets::Call::<T>::transfer(trade_request.carbon_credits_id, asset_holder_source, trade_request.carbon_credits_count);
						// let result = transfer_call.dispatch_bypass_filter(origin);

						//transfer everusd then
						// pallet_evercity::Module::<T>::transfer_everusd(&exchange.ever_usd_holder, &exchange.carbon_credits_holder, exchange.ever_usd_count)?;

					}
				}
				Ok(().into())
			})?;
			Ok(().into())
		}
    }
}







// #![allow(clippy::unused_unit)]
// #![cfg_attr(not(feature = "std"), no_std)]

// mod exchange;
// mod everusdasset;

// #[cfg(test)]
// mod tests;
// #[cfg(test)]
// mod mock;

// use frame_system::RawOrigin;
// use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
// use frame_support::{
//     ensure,
//     decl_error, 
//     decl_module, 
//     decl_storage,
//     decl_event,
//     dispatch::{
//         DispatchResult,
//         Vec,
//     },
//     traits::UnfilteredDispatchable,
// };
// use frame_system::{
//     ensure_signed,
// };
// use sp_runtime::traits::StaticLookup;
// use frame_support::sp_std::{
//     cmp::{
//         Eq, 
//         PartialEq}, 
// };
// use pallet_evercity_assets;
// use pallet_assets;
// use exchange::{ExchangeStruct, HolderType};
// use pallet_evercity::{EverUSDBalance};
// use everusdasset::{EverUSDAssetMinRequest};

// type AssetId<T> = <T as pallet_assets::Config>::AssetId;
// type ExchangeId = u128;
// type EverUSDAssetMintRequestId = u128;

// pub trait Config: 
//     frame_system::Config + 
//     pallet_timestamp::Config + 
//     pallet_assets::Config + 
//     pallet_evercity_carbon_credits::Config + 
//     pallet_evercity::Config + 
// {
//         type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
// }

// decl_storage! {
//     trait Store for Module<T: Config> as CarbonCredits {
//         /// Main storage for exchanges
//         ExchangeById
//             get(fn exchange_by_id):
//             map hasher(blake2_128_concat) ExchangeId => Option<ExchangeStruct<T::AccountId, AssetId<T>, T::Balance, EverUSDBalance>>;    
//         LastID: ExchangeId;

//         /// EverAssetMintRequestById
//         EverUSDAssetMintRequestById
//             get(fn ever_asset_mint_request_by_id):
//             map hasher(blake2_128_concat) EverUSDAssetMintRequestId => Option<EverUSDAssetMinRequest<T::AccountId, AssetId<T>, T::Balance>>;
//         LastMintID: EverUSDAssetMintRequestId;
//     }
// }

// // Pallet events
// decl_event!(
//     pub enum Event<T>
//     where
//         AccountId = <T as frame_system::Config>::AccountId,
//         AssetId = <T as pallet_assets::Config>::AssetId,
//         Balance = <T as pallet_assets::Config>::Balance,
//     {
//         /// \[EverUSDHolder, CarbonCreditsHolder, AssetId, Balance\]
//         EchangeCreated(AccountId, AccountId, AssetId, Balance),
//     }
// );

// decl_error! {
//     pub enum Error for Module<T: Config> {
//         /// Account does not have an auditor role in Accounts Pallet
//         AccountNotTokenOwner,
//         BadHolder,
//         ExchangeIdOwerflow,
//         InsufficientEverUSDBalance,
//         InsufficientCarbonCreditsBalance,
//         ExchangeNotFound,
//         BadApprove,
//         AssetNotFound,
//         BadRole,
//         MintError,
//     }
// }

// decl_module! {
//     pub struct Module<T: Config> for enum Call where origin: T::Origin {
//         type Error = Error<T>;
//         // fn deposit_event() = default;
//         #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 2)]
//         pub fn create_exhange(origin, 
//             partner_holder: T::AccountId,
//             ever_usd_count: EverUSDBalance,
//             carbon_credits_asset_id: AssetId<T>,
//             carbon_credits_count: T::Balance,
//             holder_type: HolderType,
//         ) -> DispatchResult {
//             let caller = ensure_signed(origin)?;
//             // let mut new_exchange = ExchangeStruct::new(ever_usd_holder, carbon_credits_holder, ever_usd_count, carbon_credits_asset_id, carbon_credits_count);
//             // ensure!();
//             let asset_opt = pallet_assets::Module::<T>::get_asset_details(carbon_credits_asset_id);
//             ensure!(asset_opt.is_some(), Error::<T>::AssetNotFound);
            
//             let new_exchange = match holder_type {
//                 HolderType::EverUSDHolder => {

//                     // Check EverUSD balance HERE!!!!!
//                     let current_everusd_balance = pallet_evercity::Module::<T>::get_balance(&caller);
//                     if ever_usd_count > current_everusd_balance {
//                         return Err(Error::<T>::InsufficientEverUSDBalance.into());
//                     } 

//                     ExchangeStruct::new(caller, partner_holder, ever_usd_count, carbon_credits_asset_id, carbon_credits_count, exchange::EVERUSD_HOLDER_APPROVED)
//                 },
//                 HolderType::CarbonCreditsHolder => {
//                     let current_carbon_credits_balace = pallet_evercity_assets::Module::<T>::balance(carbon_credits_asset_id, caller.clone());
//                     if carbon_credits_count > current_carbon_credits_balace {
//                         return Err(Error::<T>::InsufficientCarbonCreditsBalance.into());
//                     }

//                     ExchangeStruct::new(partner_holder, caller, ever_usd_count, carbon_credits_asset_id, carbon_credits_count, exchange::CARBON_CREDITS_HOLDER_APPROVED)
//                 },
//             };

//             let new_id = match LastID::get().checked_add(1) {
//                 Some(id) => id,
//                 None => return Err(Error::<T>::ExchangeIdOwerflow.into()),
//             };
//             ExchangeById::<T>::insert(new_id, new_exchange);
//             LastID::mutate(|x| *x = new_id);

//             Ok(())
//         }

//         #[weight = 10_000 + T::DbWeight::get().reads_writes(5, 3)]
//         pub fn approve_exchange(origin, exchange_id: ExchangeId, holder_type: HolderType) -> DispatchResult {
//             let caller = ensure_signed(origin)?;
//             ExchangeById::<T>::try_mutate(
//                 exchange_id, |project_to_mutate| -> DispatchResult {
//                     match project_to_mutate  {
//                         None => return Err(Error::<T>::ExchangeNotFound.into()),
//                         Some(exchange) => {
//                             match holder_type {
//                                 HolderType::EverUSDHolder => {
//                                     ensure!(exchange.approved == exchange::CARBON_CREDITS_HOLDER_APPROVED, Error::<T>::BadApprove);
//                                     ensure!(caller == exchange.ever_usd_holder, Error::<T>::BadHolder);

//                                 },
//                                 HolderType::CarbonCreditsHolder => {
//                                     ensure!(exchange.approved == exchange::EVERUSD_HOLDER_APPROVED, Error::<T>::BadApprove);
//                                     ensure!(caller == exchange.carbon_credits_holder, Error::<T>::BadHolder);
//                                 },
//                             }

//                             let current_everusd_balance = pallet_evercity::Module::<T>::get_balance(&exchange.ever_usd_holder);
//                             let carbon_credits_balance = pallet_evercity_assets::Module::<T>::balance(exchange.carbon_credits_asset_id, exchange.carbon_credits_holder.clone());

//                             if exchange.ever_usd_count > current_everusd_balance {
//                                 return Err(Error::<T>::InsufficientEverUSDBalance.into());
//                             }
//                             if exchange.carbon_credits_count > carbon_credits_balance  {
//                                 return Err(Error::<T>::InsufficientCarbonCreditsBalance.into());
//                             }

//                             // transfer carbon credits
//                             let cc_holder_origin = frame_system::RawOrigin::Signed(exchange.carbon_credits_holder.clone()).into();
//                             pallet_evercity_carbon_credits::Module::<T>::transfer_carbon_credits(
//                                     cc_holder_origin, 
//                                     exchange.carbon_credits_asset_id, 
//                                     exchange.ever_usd_holder.clone(), 
//                                     exchange.carbon_credits_count
//                             )?;

//                             // transfer everusd then
//                             pallet_evercity::Module::<T>::transfer_everusd(&exchange.ever_usd_holder, &exchange.carbon_credits_holder, exchange.ever_usd_count)?;
//                         }
//                     }
//                     Ok(())
//                 })?;

//             Ok(())
//         }

//         #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 2)]
//         pub fn swap_everusd_bond_asset(origin, ever_usd_balance: EverUSDBalance, asset_balance: T::Balance, asset_id: AssetId<T>) -> DispatchResult {
//             let caller = ensure_signed(origin)?;

//             ensure!(pallet_evercity::Module::<T>::account_is_custodian(&caller), Error::<T>::InsufficientCarbonCreditsBalance);

//             Ok(())
//         }

//         #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 2)]
//         pub fn create_everusd_asset_mint_request(origin, asset_id: AssetId<T>, amount: T::Balance) -> DispatchResult {
//             let caller = ensure_signed(origin)?;
//             let asset_opt = pallet_assets::Module::<T>::get_asset_details(asset_id);
//             ensure!(asset_opt.is_some(), Error::<T>::AssetNotFound);
//             let mint_request = everusdasset::EverUSDAssetMinRequest::new(caller, asset_id, amount);

//             let new_id = match LastMintID::get().checked_add(1) {
//                 Some(id) => id,
//                 None => return Err(Error::<T>::ExchangeIdOwerflow.into()),
//             };
//             EverUSDAssetMintRequestById::<T>::insert(new_id, mint_request);
//             LastMintID::mutate(|x| *x = new_id);

//             Ok(())
//         }

//         #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 2)]
//         pub fn approve_everusd_asset_mint_request(origin, id: EverUSDAssetMintRequestId) -> DispatchResult {
//             let caller = ensure_signed(origin.clone())?;
//             ensure!(pallet_evercity::Module::<T>::account_is_custodian(&caller), Error::<T>::BadRole);

//             let mint_request = match EverUSDAssetMintRequestById::<T>::get(id) {
//                 Some(m) => m,
//                 None => panic!(),
//             };

//             let asset = match pallet_assets::Module::<T>::get_asset_details(mint_request.asset_id) {
//                 Some(a) => a,
//                 None => return Err(Error::<T>::AssetNotFound.into())
//             };
//             ensure!(*asset.get_owner() == caller, Error::<T>::BadRole);

//             let investor_static_lookup = <T::Lookup as StaticLookup>::unlookup(mint_request.account.clone());
//             let mint_call = pallet_assets::Call::<T>::mint(mint_request.asset_id, investor_static_lookup, mint_request.count_to_mint);
//             let result = mint_call.dispatch_bypass_filter(origin);
//             ensure!(!result.is_err(), Error::<T>::MintError);

//             Ok(())
//         }
//     }
// }