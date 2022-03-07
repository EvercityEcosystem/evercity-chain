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
                // Project Events:

        /// \[ProjectOwner, ProjectId\]
        ProjectCreated(T::AccountId, ProjectId),
        /// \[ProjectOwner, ProjectId, FileId\]
        ProjectFileIdChanged(T::AccountId, ProjectId, FileId),
        /// \[ProjectOwner, ProjectId, OldStandard, NewStandard\]
        ProjectStandardChanged(T::AccountId, ProjectId, Standard, Standard),
        /// \[ProjectOwner, ProjectId\]
        ProjectSubmited(T::AccountId, ProjectId),
        /// \[Auditor, ProjectId\]
        ProjectSignedByAduitor(T::AccountId, ProjectId),
        /// \[StandardRoleAccount, ProjectId\]
        ProjectSignedByStandard(T::AccountId, ProjectId),
        /// \[Registry, ProjectId\]
        ProjectSignedByRegistry(T::AccountId, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        ProjectSignerAdded(T::AccountId, T::AccountId, RoleMask, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        ProjectSignerRemoved(T::AccountId, T::AccountId, RoleMask, ProjectId),

        // Annual Report Events:

        /// \[ProjectOwner, ProjectId\]
        AnnualReportCreated(T::AccountId, ProjectId),
        /// \[ProjectOwner, ProjectId\]
        AnnualReportDeleted(T::AccountId, ProjectId),
        // \[ProjectOwner, ProjectId, NewCount\]
        AnnualReportCreditsCountChanged(T::AccountId, ProjectId, T::Balance),
        /// \[ProjectOwner, ProjectId\]
        AnnualReportSubmited(T::AccountId, ProjectId), 
        /// \[Auditor, ProjectId\]
        AnnualReportSignedByAuditor(T::AccountId, ProjectId),
        /// \[StandardRoleAccount, ProjectId\]
        AnnualReportSignedByStandard(T::AccountId, ProjectId),
        /// \[Registry, ProjectId\]
        AnnualReportSignedByRegistry(T::AccountId, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        AnnualReportSignerAdded(T::AccountId, T::AccountId, RoleMask, ProjectId),
        /// \[ProjectOwner, Signer, Role, ProjectId\]
        AnnualReportSignerRemoved(T::AccountId, T::AccountId, RoleMask, ProjectId),

        // Carbon Credits Events:

        /// \[ProjectOwner, ProjectId, AssetId\]
        CarbonCreditsAssetCreated(T::AccountId, ProjectId, T::AssetId),
        /// \[ProjectOwner, AssetId\]
        CarbonCreditsMetadataChanged(T::AccountId, T::AssetId),
        /// \[ProjectOwner, ProjectId, AssetId\]
        CarbonCreditsMinted(T::AccountId, ProjectId, T::AssetId),
        /// \[CarbonCreditsHolder, AccountToTransfer, AssetId\]
        CarbonCreditsTransfered(T::AccountId, T::AccountId, T::AccountId),
        /// \[ProjectOwner, AssetId\]
        CarbonCreditsAssetBurned(T::AccountId, T::AssetId),
    }

    #[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

    #[pallet::error]
	pub enum Error<T> {
        // Project errors:

        /// Separate Error for project validation
        InvalidProjectState,

        // Account errors:

        /// Account does not have an auditor role in Accounts Pallet
        AccountNotAuditor,
        /// Account is not owner of the project or doenst have auditor role in Accounts Pallet
        AccountNotOwner,
        /// Account doesnt have Standard role in Accounts Pallet
        AccountNotStandard,
        /// Account doesnt have Registry role in Accounts Pallet 
        AccountNotRegistry,
        /// Account doesnt have Investor role in Accounts Pallet 
        AccountNotInvestor,
        /// Role if the account is incorrect
        AccountIncorrectRole,
        /// Account is not assigned as signer in given role
        AccountNotGivenRoleSigner,
        /// Account not owner of file
        AccountNotFileOwner,
        /// Account has already signed a project or annual report
        AccountAlreadySigned,

        // State machine errors

        /// Invalid State of the state machine
        InvalidState,
        /// Project does not exits in the storage
        ProjectNotExist,
        /// Project doesnt have Registered state
        ProjectNotRegistered,
        /// Annual reports of the project do not exist
        NoAnnualReports,
        /// State of an annual report doesnt equal to Issued
        NotIssuedAnnualReportsExist,

        // Asset error

        /// Error has occured when tried to create asset
        ErrorCreatingAsset,
        /// Error minting asset
        ErrorMintingAsset,
        /// Carbon credits are already created error
        CCAlreadyCreated,
        /// Carbon credits transfer failed
        TransferFailed,
        /// Carbon Credits asset burn failed
        BurnFailed,
        /// Bad parameters of metadata
        BadMetadataParameters,
        /// Set metadata parameters failed
        SetMetadataFailed,
        /// Annual report is not ready
        AnnualReportNotReady,
        /// Carbon Credits Ballance too low
        InsufficientCarbonCredits,

        // Passport Errors:

        /// There is no carbon credits passport in storage
        PassportNotExist,
        /// Project referenced by passport is equal to given
        BadPassportProject,
        /// Given Annual report index is bad 
        BadPassportAnnualReport,

        // Signer errors:

        /// Signer does not exist in Project required signers
        IncorrectProjectSigner,
        /// Signer does not exist in annual report required signers
        IncorrectAnnualReportSigner,

        // File errors
        IncorrectFileId,

        // Bond Validation Errors
        ProjectIsBond,
    }

    #[pallet::call]
	impl<T: Config> Pallet<T> {
        
    }
}