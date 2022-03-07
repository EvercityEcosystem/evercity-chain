#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub mod standard;
pub mod project;
pub mod annual_report;
pub mod required_signers;
pub mod carbon_credits_passport;
pub mod burn_certificate;

use sp_std::{prelude::*};
// use pallet_evercity_bonds::bond::{BondId, BondState};

use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
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
use project::{ProjectStruct, ProjectId};
use standard::Standard;
use pallet_evercity_filesign::file::{FileId};
use pallet_evercity_accounts::accounts::RoleMask;
use carbon_credits_passport::CarbonCreditsPassport;
use burn_certificate::CarbonCreditsBurnCertificate;

// use pallet_evercity_assets as pallet_assets;
// use pallet_evercity_accounts as accounts;

pub use crate::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use super::*;

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T> (_);

    #[pallet::config]
	/// The module configuration trait.
    pub trait Config: 
    frame_system::Config + 
    pallet_evercity_accounts::Config + 
    pallet_timestamp::Config + 
    pallet_evercity_assets::Config + 
    pallet_evercity_filesign::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", T::AssetId = "AssetId")]
	pub enum Event<T: Config> {
		// BondCarbonCreditsReleased(BondId, CarbonCreditsId<T>)
    }

    #[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

    #[pallet::error]
	pub enum Error<T> {
		BondNotFinished,
		CreateCCError,
		TransferCCError,
		BalanceIsZero,
		InvestmentIsZero,
		AlreadyReleased,
		NotAnIssuer,
		CarbonMetadataNotValid
    }

    #[pallet::call]
	impl<T: Config> Pallet<T> {
        
    }
}