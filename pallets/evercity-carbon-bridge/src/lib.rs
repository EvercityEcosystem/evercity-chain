#![cfg_attr(not(feature = "std"), no_std)]

mod trade_request;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use sp_std::{prelude::*};
use sp_runtime::{traits::{StaticLookup}
};
use pallet_evercity_bonds::bond::{BondId, BondState};

use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

pub use crate::pallet::*;

pub type TradeRequestId = u128;
pub type AssetId<T> = <T as pallet_assets::Config>::AssetId;
pub type CarbonCreditsId<T> = pallet_evercity_carbon_credits::AssetId<T>;
pub type CarbonCreditsBalance<T> = pallet_evercity_carbon_credits::Balance<T>;


#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsBondRelease<Balance> {
    pub amount: Balance,
    pub period: u32,
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*, traits::UnfilteredDispatchable,
	};
	use frame_system::pallet_prelude::*;
	// use crate::trade_request::{TradeRequest, HolderType, CARBON_CREDITS_HOLDER_APPROVED, ASSET_HOLDER_APPROVED};
	use super::*;
	// use pallet_evercity_assets;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T> (_);


    #[pallet::config]
	/// The module configuration trait.
	pub trait Config: 
		frame_system::Config +
		pallet_assets::Config + 
		pallet_evercity_assets::Config + 
		pallet_evercity_carbon_credits::Config + 
		pallet_evercity_bonds::Config + 
	{
		// type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}

	#[pallet::storage]
	pub(super) type BondCarbonReleaseRegistry<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BondId,
		CarbonCreditsBondRelease<CarbonCreditsBalance<T>>,
		OptionQuery
	>;

	#[pallet::error]
	pub enum Error<T> {
		BondNotFinished,
		CreateCCError,
		TransferCCError,
		BalanceIsZero,
		InvestmentIsZero,
		AlreadyReleased,
		NotAnIssuer,
    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", T::AssetId = "AssetId")]
	pub enum Event<T: Config> {
        /// \[TradeRequestId, AssetHolder, CarbonCreditsHolder, AssetId, CarbonCreditsId\]
        CarbonCreditsTradeRequestCreated(TradeRequestId, T::AccountId, T::AccountId, <T as pallet_assets::Config>::AssetId, CarbonCreditsId<T>),
		/// \[TradeRequestId\]
        CarbonCreditsTradeRequestAccepted(TradeRequestId),
    }
	
	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

    #[pallet::call]
	impl<T: Config> Pallet<T> where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128> {
		#[pallet::weight(10_000)]
		pub fn release_bond_carbon_credits(
			origin: OriginFor<T>, 
			carbon_credits_id: CarbonCreditsId<T>, 
			carbon_credits_count: CarbonCreditsBalance<T>, 
			bond_id: pallet_evercity_bonds::bond::BondId
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin.clone())?;
			let bond = pallet_evercity_bonds::Module::<T>::get_bond(&bond_id);
			ensure!(bond.issuer == caller, Error::<T>::NotAnIssuer);
			ensure!(bond.state != BondState::PREPARE, Error::<T>::BondNotFinished);

			let check_reg = BondCarbonReleaseRegistry::<T>::get(bond_id);
			ensure!(check_reg.is_none(), Error::<T>::AlreadyReleased);

			let bond_investment_tubple = pallet_evercity_bonds::Module::<T>::get_bond_account_investment(&bond_id);
			ensure!(bond_investment_tubple.len() != 0, Error::<T>::InvestmentIsZero);

			log::info!("====================================================================================");
			log::info!("======================= LENGTH IS {:?}", bond_investment_tubple.len());
			log::info!("======================= TUPLE VEC IS {:?}", bond_investment_tubple);

			for (i, j) in bond_investment_tubple.clone() {
				log::info!("======================= ACCOUNT {:?} IMPACT IS {:?}", i, j);
			}

			log::info!("====================================================================================");

			let total_packages = bond_investment_tubple.iter()
											.map(|(_, everusd)| everusd)
											.fold(0, |a, b| {a + b});

			log::info!("======================= TOTAL BALANCE IS {:?}", total_packages);

			ensure!(total_packages != 0, Error::<T>::BalanceIsZero);

			let parts = bond_investment_tubple
									.into_iter()
									.map(|(acc, everusd)| {
										// (acc, (everusd/total_everusd_balance) as f64)
										(acc, (everusd as f64)/(total_packages as f64) )
									})
									.filter(|(_, part)| *part != 0.0)
									.map(|(acc, everusd)| {
										// (acc, (everusd/total_everusd_balance) as f64)
										(acc, Self::divide_balance(everusd, carbon_credits_count))
									})
									.collect::<Vec<(T::AccountId, CarbonCreditsBalance<T>)>>();

			let create_cc_call = 
				pallet_evercity_carbon_credits::Module::<T>::create_bond_carbon_credits(caller, *bond_id, carbon_credits_id, carbon_credits_count);

			ensure!(create_cc_call.is_ok(), Error::<T>::CreateCCError);

			for (acc, bal) in parts {
				log::info!("======================================== trying to send {:?} to acccount: {:?} ===========================================", bal, acc);
				let res = 
					pallet_evercity_carbon_credits::Module::<T>::transfer_carbon_credits(
						origin.clone(), carbon_credits_id, acc, bal);
				log::info!("======================================== transfer result is:{:?} ===========================================", res);
			}

			let release = CarbonCreditsBondRelease {amount: carbon_credits_count, period: 0};
			BondCarbonReleaseRegistry::<T>::insert(bond_id, release);
			Ok(().into())
		}
    }

	impl<T: Config> Pallet<T> where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128> {
		pub fn u64_to_balance(num: u128) -> <T as pallet_evercity_assets::pallet::Config>::Balance where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128> {
			num.into()
		}

		pub fn balance_to_u128(bal: <T as pallet_evercity_assets::pallet::Config>::Balance ) -> u128 where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128> {
			bal.into()
		}

		pub fn divide_balance(
			percent: f64, 
			bal_amount: <T as pallet_evercity_assets::pallet::Config>::Balance
		) -> <T as pallet_evercity_assets::pallet::Config>::Balance  {
			let temp_u64 = ((Self::balance_to_u128(bal_amount) as f64) * percent) as u128;
			Self::u64_to_balance(temp_u64)
		}
	}
}

// По этапам:

// 1) Выполняется экстринзик с аргументами: AssetId, BondId, Amount
// Экстринзик переводит инвесторам мои AssetId в соответствии с процентами

// 2) Экстринзик учится дописывать в бонд информацию о финальной транзакции

// 3) Бонд при создании обещает схему выплаты КК и после вополнения экстринзика он следует этой схеме валидируя транзакцию распределения

// 4) Наступает сингулярность Бонда и КК на всем жизненном цикле бонда 