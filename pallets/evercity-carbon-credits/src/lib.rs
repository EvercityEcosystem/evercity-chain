#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub mod standard;
pub mod project;
pub mod annual_report;
pub mod required_signers;
pub mod carbon_credits_passport;
pub mod burn_certificate;
pub mod bond_carbon_release;
#[cfg(test)]    
pub mod tests;

use sp_std::{prelude::*};
use frame_support::{
    dispatch::{
        DispatchResult,
        Vec,
    },
    traits::UnfilteredDispatchable,
};
use sp_runtime::traits::StaticLookup;
use project::{ProjectStruct, ProjectId};
use standard::Standard;
use pallet_evercity_filesign::file::{FileId};
use pallet_evercity_accounts::accounts::RoleMask;
use carbon_credits_passport::CarbonCreditsPassport;
use burn_certificate::CarbonCreditsBurnCertificate;

// use pallet_evercity_assets as pallet_assets;
use pallet_evercity_accounts as accounts;

pub use crate::pallet::*;

type Timestamp<T> = pallet_timestamp::Module<T>;
pub type AssetId<T> = <T as pallet_evercity_assets::Config>::AssetId;
pub type Balance<T> = <T as pallet_evercity_assets::Config>::Balance;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
    use pallet_evercity_bonds::{bond::BondState, BondId};
	use crate::bond_carbon_release::CarbonCreditsBondRelease;

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
        pallet_evercity_filesign::Config +
        pallet_evercity_bonds::Config
    {
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
        CarbonCreditsTransfered(T::AccountId, T::AccountId, T::AssetId),
        /// \[ProjectOwner, AssetId\]
        CarbonCreditsAssetBurned(T::AccountId, T::AssetId),



        BondCarbonCreditsReleased(BondId, <T as pallet_evercity_assets::Config>::AssetId)
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
        ProjectIsNotBond,

        BondNotFinished,
		CreateCCError,
		TransferCCError,
		BalanceIsZero,
		InvestmentIsZero,
		AlreadyReleased,
		NotAnIssuer,
		CarbonMetadataNotValid
    }

    #[pallet::storage]
	pub(super) type ProjectById<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		ProjectStruct<T::AccountId, T, T::Balance>,
		OptionQuery
	>;

    #[pallet::storage]
	pub(super) type LastID<T: Config> = StorageValue<
        _, 
		ProjectId,
        ValueQuery
	>;

    #[pallet::storage]
	pub(super) type CarbonCreditPassportRegistry<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		AssetId<T>,
		CarbonCreditsPassport<AssetId<T>>,
		OptionQuery
	>;

    #[pallet::storage]
	pub(super) type BurnCertificates<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Vec<CarbonCreditsBurnCertificate<AssetId<T>, T::Balance>>,
		ValueQuery
	>;


	#[pallet::storage]
	pub(super) type BondCarbonReleaseRegistry<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BondId,
		CarbonCreditsBondRelease<T::Balance>,
		OptionQuery
	>;


    // EXTRINSICS:
    #[pallet::call]
	impl<T: Config> Pallet<T> where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128> {
        /// <pre>
        /// Method: create_project(standard: Standard, file_id: FileId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            standard: Standard - Carbon Credits Standard
        ///            file_id: FileId - id of file in filesign pallet
        /// Access: Project Owner Role
        ///
        /// Creates new project with relation to PDD file in filesign
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn create_project(origin: OriginFor<T>, standard: Standard, file_id: Option<FileId>) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            if let Some(id) = file_id {
                ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(id, &caller), Error::<T>::AccountNotFileOwner);
            }
            let new_id = LastID::<T>::get() + 1;
            let new_project = ProjectStruct::<<T as frame_system::Config>::AccountId, T, T::Balance>::new(caller.clone(), new_id, standard, file_id);
            <ProjectById<T>>::insert(new_id, new_project);
            LastID::<T>::mutate(|x| *x = x.checked_add(1).unwrap());

            // SendEvent
            Self::deposit_event(Event::ProjectCreated(caller, new_id));
            Ok(().into())
        }

        /// <pre>
        /// Method: create_project(standard: Standard, file_id: FileId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            standard: Standard - Carbon Credits Standard
        ///            file_id: FileId - id of file in filesign pallet
        /// Access: Project Owner Role
        ///
        /// Creates new project with relation to PDD file in filesign
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn create_bond_project(
            origin: OriginFor<T>, 
            standard: Standard, 
            file_id: Option<FileId>, 
            bond_id: BondId
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            if let Some(id) = file_id {
                ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(id, &caller), Error::<T>::AccountNotFileOwner);
            }
            let bond = pallet_evercity_bonds::Module::<T>::get_bond(&bond_id);
			ensure!(bond.issuer == caller, Error::<T>::NotAnIssuer);
			ensure!(bond.state == BondState::FINISHED , Error::<T>::BondNotFinished);
            let new_id = LastID::<T>::get() + 1;
            let new_project = 
                ProjectStruct::new_with_bond(caller.clone(), new_id, standard, file_id, *bond_id);
            <ProjectById<T>>::insert(new_id, new_project);
            LastID::<T>::mutate(|x| *x = x.checked_add(1).unwrap());

            // SendEvent
            Self::deposit_event(Event::ProjectCreated(caller, new_id));
            Ok(().into())
        }

        /// <pre>
        /// Method: change_project_file_id(standard: Standard, file_id: FileId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Carbon Credits Standard
        ///            file_id: FileId - id of file in filesign pallet
        /// Access: Project Owner Role
        ///
        /// Changes project file id, availible before signing starts
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn change_project_file_id(origin: OriginFor<T>, project_id: ProjectId, file_id: FileId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(file_id, &caller), Error::<T>::AccountNotFileOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(project.state == project::PROJECT_OWNER_SIGN_PENDING, Error::<T>::InvalidState);
                            project.file_id = Some(file_id);
                        }
                    }
                    Ok(())
                })?;
            Self::deposit_event(Event::ProjectFileIdChanged(caller, project_id, file_id));
            Ok(().into())
        }

        /// <pre>
        /// Method: assign_project_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project 
        ///
        /// assign signer, that is required for signing project documentation
        /// also adds signer to filesign PDD 
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn assign_project_signer(
            origin: OriginFor<T>, 
            signer: T::AccountId, 
            role: RoleMask, 
            project_id: ProjectId
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            project.assign_required_signer((signer.clone(), role));
                            let file_id = match project.file_id {
                                None => return Err(Error::<T>::IncorrectFileId.into()),
                                Some(id) => id
                            };
                            pallet_evercity_filesign::Module::<T>::assign_signer(origin, file_id, signer.clone())?;
                        }
                    }
                    Ok(())
                })?;
            Self::deposit_event(Event::ProjectSignerAdded(caller, signer, role, project_id));
            Ok(().into())
        }

        /// <pre>
        /// Method: remove_project_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project 
        ///
        /// remove signer, that is was added for signing project documentation
        /// also deletes signer from filesign PDD 
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 2))]
        pub fn remove_project_signer(
            origin: OriginFor<T>, 
            signer: T::AccountId, 
            role: RoleMask, 
            project_id: ProjectId
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(project.is_required_signer((signer.clone(), role)), Error::<T>::AccountNotGivenRoleSigner);
                            let file_id = match project.file_id {
                                None => return Err(Error::<T>::IncorrectFileId.into()),
                                Some(id) => id
                            };
                            // Check if signer did not already sign the project
                            let has_signed = pallet_evercity_filesign::Module::<T>::address_has_signed_the_file(file_id, &signer);
                            ensure!(!has_signed, Error::<T>::AccountAlreadySigned);
                            project.remove_required_signer((signer.clone(), role));
                            // delete from filesign:
                            pallet_evercity_filesign::Module::<T>::delete_signer(origin, file_id, signer.clone())?;
                        }
                    }
                    Ok(())
            })?;
            Self::deposit_event(Event::ProjectSignerRemoved(caller, signer, role, project_id));
            Ok(().into())
        }

        /// <pre>
        /// Method: sign_project(project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id - id of the project
        ///
        /// Access: Required Signer with signer role 
        ///
        /// Signs project documentation, changing state of the project state machine
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2))]
        pub fn sign_project(origin: OriginFor<T>, project_id: ProjectId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin.clone())?;
            let mut event_opt: Option<Event<T>> = None;
            ProjectById::<T>::try_mutate(
                project_id, |project_option| -> DispatchResult {
                    match project_option {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            let project_documentation_file_id = match project.file_id {
                                None => return Err(Error::<T>::IncorrectFileId.into()),
                                Some(id) => id
                            };
                            ensure!(pallet_evercity_filesign::Module::<T>::address_is_signer_for_file(project_documentation_file_id, &caller), 
                                Error::<T>::IncorrectProjectSigner);
                            Self::change_project_state(project, caller, &mut event_opt)?;
                            pallet_evercity_filesign::Module::<T>::sign_latest_version(origin, project_documentation_file_id)?;
                        }
                    }
                    Ok(())
                })?;
            if let Some(event) = event_opt {
                Self::deposit_event(event);
            }
            Ok(().into())
        }


        /// <pre>
        /// Method: create_annual_report(project_id: ProjectId, file_id: FileId, carbon_credits_count: T::Balance)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Id of project, where to create annual report
        ///            file_id: FileId - Id of pre created file of annual report document
        ///            carbon_credits_count - count of carbon credits to release after signing
        ///
        ///
        /// Access: Owner of the project
        ///
        /// Create annual report entity with link to annual report file
        /// 
        /// </pre> 
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 1))]
        pub fn create_annual_report(
            origin: OriginFor<T>, 
            project_id: ProjectId, 
            file_id: FileId, 
            carbon_credits_count: T::Balance,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ensure!(pallet_evercity_filesign::Module::<T>::address_is_owner_for_file(file_id, &caller), Error::<T>::AccountNotFileOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_option| -> DispatchResult {
                    match project_option {
                        None => Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(project.state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
                            ensure!(project.annual_reports.iter()
                                        .all(|x| x.state == annual_report::REPORT_ISSUED),
                                Error::<T>::NotIssuedAnnualReportsExist
                            );
                            let meta = annual_report::CarbonCreditsMeta::new(name, symbol, decimals);
                            ensure!(meta.is_metadata_valid(), Error::<T>::BadMetadataParameters);
                            project.annual_reports
                                .push(annual_report::AnnualReportStruct::<T::AccountId, T, T::Balance>::new(file_id, carbon_credits_count, Timestamp::<T>::get(), meta));
                            Ok(())
                        }
                    }
             })?;
            // SendEvent
            Self::deposit_event(Event::AnnualReportCreated(caller, project_id));
            Ok(().into())
        }

        /// <pre>
        /// Method: create_annual_report_with_file(
        ///             project_id: ProjectId, 
        ///             file_id: FileId, 
        ///             filehash: pallet_evercity_filesign::file::H256,
        ///             tag: Vec<u8>,
        ///             carbon_credits_count: T::Balance,
        ///             name: Vec<u8>,
        ///             symbol: Vec<u8>,
        ///             decimals: u8,
        ///             ) 
        /// 
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Id of project, where to create annual report
        ///            file_id: FileId - Id of file to create in filesign
        ///            filehash: pallet_evercity_filesign::file::H256 - file hash
        ///            tag: Vec<u8> - file tag
        ///            carbon_credits_count: T::Balance
        ///            name: Vec<u8> - name of carbon credits, part of metadata
        ///            symbol: Vec<u8> - symbol
        ///            decimals: u8 - number of decimals
        ///
        ///
        /// Access: Owner of the project
        ///
        /// Create annual report entity with link to annual report file
        /// 
        /// </pre> 
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 1))]
        pub fn create_annual_report_with_file(
            origin: OriginFor<T>, 
            project_id: ProjectId, 
            file_id: FileId, 
            filehash: pallet_evercity_filesign::file::H256,
            tag: Vec<u8>,
            carbon_credits_count: T::Balance,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin.clone())?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            let meta = annual_report::CarbonCreditsMeta::new(name, symbol, decimals);
            ensure!(meta.is_metadata_valid(), Error::<T>::BadMetadataParameters);
            ProjectById::<T>::try_mutate(
                project_id, |project_option| -> DispatchResult {
                    match project_option {
                        None => Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(project.state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
                            ensure!(project.annual_reports.iter()
                                        .all(|x| x.state == annual_report::REPORT_ISSUED),
                                Error::<T>::NotIssuedAnnualReportsExist
                            );
                            pallet_evercity_filesign::Module::<T>::create_new_file(origin, tag, filehash, Some(file_id))?;
                            project.annual_reports
                                        .push(annual_report::AnnualReportStruct::<T::AccountId, T, T::Balance>::new(file_id, carbon_credits_count, Timestamp::<T>::get(), meta));
                            Ok(())
                        }
                    }
             })?;
            // SendEvent
            Self::deposit_event(Event::AnnualReportCreated(caller, project_id));
            Ok(().into())
        }
        

        /// <pre>
        /// Method: change_report_carbon_credits_count(project_id: ProjectId, new_carbon_credits_count: T::Balance)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Id of project, where to create annual report
        ///            new_carbon_credits_count - new count of carbon credits to release after signing
        ///
        ///
        /// Access: Owner of the project
        ///
        /// Change annual report balance. Can only be changed in preparing step
        /// 
        /// </pre> 
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
        pub fn change_report_carbon_credits_count(
            origin: OriginFor<T>, 
            project_id: ProjectId, 
            new_carbon_credits_count: T::Balance
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            // Ensure, that report in initial preparing state REPORT_PROJECT_OWNER_SIGN_PENDING
                            ensure!(proj.annual_reports[len - 1].state == annual_report::REPORT_PROJECT_OWNER_SIGN_PENDING, Error::<T>::InvalidState);
                            proj.annual_reports[len - 1].change_carbon_credits_count(new_carbon_credits_count);
                        }
                    }
                    Ok(())
             })?;
            // SendEvent
            Self::deposit_event(Event::AnnualReportCreditsCountChanged(caller, project_id, new_carbon_credits_count));
            Ok(().into())
        }

        /// <pre>
        /// Method: delete_last_annual_report(project_id: ProjectId: T::Balance)
        /// Arguments: origin: AccountId - Transaction caller
        ///            project_id: ProjectId - Id of project, where to create annual report
        ///
        ///
        /// Access: Owner of the project
        ///
        /// Deletes project's last annual report if it is not issued
        /// 
        /// </pre> 
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
        pub fn delete_last_annual_report(
            origin: OriginFor<T>, 
            project_id: ProjectId
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            // Ensure, that report not in final issued state
                            // To prevent deleting ready reports, that have its carbon credits released
                            ensure!(proj.annual_reports[len - 1].state != annual_report::REPORT_ISSUED, Error::<T>::InvalidState);
                            proj.annual_reports.remove(len - 1);
                        }
                    }
                    Ok(())
             })?;
            // SendEvent
            Self::deposit_event(Event::AnnualReportDeleted(caller, project_id));
            Ok(().into())
        }


        /// <pre>
        /// Method: assign_last_annual_report_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId))
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project
        /// assign signer, that is required for signing фттгфд кузщке document
        /// also adds signer to filesign document 
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 2))]
        pub fn assign_last_annual_report_signer(
            origin: OriginFor<T>, 
            signer: T::AccountId, 
            role: RoleMask, 
            project_id: ProjectId
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            proj.annual_reports[len - 1].assign_required_signer((signer.clone(), role));
                            // Assign signer in filesign pallet:
                            pallet_evercity_filesign::Module::<T>::assign_signer(origin.clone(), proj.annual_reports[len - 1].file_id, signer.clone())?;
                        }
                    }
                    Ok(())
             })?;
            Self::deposit_event(Event::AnnualReportSignerAdded(caller, signer, role, project_id));
            Ok(().into())
        }


        /// <pre>
        /// Method: remove_last_annual_report_signer(signer: T::AccountId, role: RoleMask, project_id: ProjectId))
        /// Arguments: origin: AccountId - Transaction caller
        ///            signer: T::AccountId - assign signer account
        ///            role - Role of the signer
        ///            project_id - id of the project
        ///
        /// Access: Owner of the project
        /// remove signer, that was added for signing фттгфд кузщке document
        /// also deletes signer to filesign document 
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 2))]
        pub fn remove_last_annual_report_signer(
            origin: OriginFor<T>, 
            signer: T::AccountId, 
            role: RoleMask, 
            project_id: ProjectId
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin.clone())?;
            ensure!(pallet_evercity_accounts::Module::<T>::account_is_selected_role(&signer, role), Error::<T>::AccountIncorrectRole);
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate  {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(proj) => {
                            ensure!(proj.owner == caller, Error::<T>::AccountNotOwner);
                            let len = proj.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            // Check if signer did not already sign the project
                            ensure!(proj.annual_reports[len - 1].is_required_signer((signer.clone(), role)), Error::<T>::AccountNotGivenRoleSigner);
                            let has_signed = pallet_evercity_filesign::Module::<T>::address_has_signed_the_file(proj.annual_reports[len - 1].file_id, &signer);
                            ensure!(!has_signed, Error::<T>::AccountAlreadySigned);
                            proj.annual_reports[len - 1].remove_required_signer((signer.clone(), role));
                            // delete signer in filesign pallet
                            pallet_evercity_filesign::Module::<T>::delete_signer(origin.clone(), proj.annual_reports[len - 1].file_id, signer.clone())?;
                        }
                    }
                    Ok(())
             })?;
            Self::deposit_event(Event::AnnualReportSignerRemoved(caller, signer, role, project_id));
            Ok(().into())
        }

        /// <pre>
        /// Method: sign_last_annual_report(project_id: ProjectId)
        /// Arguments: origin: AccountId - Transaction caller
        ///
        /// Access: Assigned signer
        ///
        /// Signs annual repor document, changing state of the project state machine
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 2))]
        pub fn sign_last_annual_report(origin: OriginFor<T>, project_id: ProjectId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin.clone())?;
            let mut event_opt: Option<Event<T>> = None;
            ProjectById::<T>::try_mutate(
                project_id, |project_to_mutate| -> DispatchResult {
                    match project_to_mutate {
                        None => return Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            let len = project.annual_reports.len();
                            ensure!(len > 0, Error::<T>::NoAnnualReports);
                            let annual_report_file_id = project.annual_reports[len - 1].file_id;
                            ensure!(pallet_evercity_filesign::Module::<T>::address_is_signer_for_file(annual_report_file_id, &caller), 
                                Error::<T>::IncorrectAnnualReportSigner);
                            Self::change_project_annual_report_state(project, caller, &mut event_opt)?;
                            pallet_evercity_filesign::Module::<T>::sign_latest_version(origin, 
                                annual_report_file_id)?;
                        }
                    }
                    Ok(())
            })?;
            if let Some(event) = event_opt {
                Self::deposit_event(event);
            }
            Ok(().into())
        }


        /// <pre>
        /// Method: release_carbon_credits(
        ///         project_id: ProjectId
        ///         asset_id: <T as pallet_assets::Config>::AssetId,
        ///         new_carbon_credits_holder: T::AccountId,
        ///         min_balance: <T as pallet_assets::Config>::Balance,
        ///     )
        /// 
        /// Arguments: origin: AccountId - Transaction caller
        ///            asset_id: <T as pallet_assets::Config>::AssetId - Asset Id in assets pallet
        ///            new_carbon_credits_holder - carbo credits holder, can be other than project owner
        ///            min_balance - min balance for assets pallet
        ///
        /// Access: Project owner
        ///
        /// Creates assets in assets pallet, creates carbon credits passport and calls mint in assets pallet
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(5, 4))]
        pub fn release_carbon_credits(
            origin: OriginFor<T>, 
            project_id: ProjectId,
            asset_id: <T as pallet_evercity_assets::Config>::AssetId,
            new_carbon_credits_holder: T::AccountId,
            min_balance: <T as pallet_evercity_assets::Config>::Balance,
        ) -> DispatchResultWithPostInfo {
            let project_owner = ensure_signed(origin.clone())?;
            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&project_owner), Error::<T>::AccountNotOwner);

            ProjectById::<T>::try_mutate(
                project_id, |project_option| -> DispatchResult {
                    match project_option {
                        None => Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == project_owner, Error::<T>::AccountNotOwner);
                            ensure!(project.state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
                            ensure!(project.get_bond_id().is_none(), Error::<T>::ProjectIsBond);
        
                            // Check that there is at least one annual report
                            let reports_len = project.annual_reports.len();
                            ensure!(reports_len > 0,    
                                Error::<T>::NoAnnualReports
                            );
        
                            // ensure that carbon credits not released, and then set it to released state
                            let last_annual_report = &mut project.annual_reports[reports_len - 1];
                            ensure!(!last_annual_report.is_carbon_credits_released(), Error::<T>::CCAlreadyCreated);
                            last_annual_report.set_carbon_credits_released();
            
                            // Create Asset:
                            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(new_carbon_credits_holder);
                            let create_asset_call = pallet_evercity_assets::Call::<T>::create(asset_id, new_carbon_credits_holder_source, 0, min_balance);
                            let create_asset_result = create_asset_call.dispatch_bypass_filter(origin.clone());
                            ensure!(!create_asset_result.is_err(), Error::<T>::ErrorCreatingAsset);
        
          
                            // Set metadata from annual report
                            // Changing metadata of annual report to empty struct
                            let mut meta = annual_report::CarbonCreditsMeta::default();
                            sp_std::mem::swap(&mut meta, &mut last_annual_report.carbon_credits_meta);
                            let set_metadata_call = pallet_evercity_assets::Call::<T>::set_metadata(asset_id, 
                                                    meta.name, 
                                                    meta.symbol, 
                                                    meta.decimals
                                                );
        
                            let result = set_metadata_call.dispatch_bypass_filter(origin.clone());
                            ensure!(!result.is_err(), {
                                // destroy if failed
                                let _ = pallet_evercity_assets::Call::<T>::destroy(asset_id, 0);
                                Error::<T>::SetMetadataFailed
                            });
        
                            // Mint Carbon Credits
                            let cc_amount = last_annual_report.carbon_credits_count();
                            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(project_owner.clone());
                            let mint_call = pallet_evercity_assets::Call::<T>::mint(asset_id, new_carbon_credits_holder_source, cc_amount);
                            let result = mint_call.dispatch_bypass_filter(origin);
                            ensure!(!result.is_err(), {
                                // destroy if failed
                                let _ = pallet_evercity_assets::Call::<T>::destroy(asset_id, 0);
                                Error::<T>::ErrorMintingAsset
                            });
        
                            // Create passport
                            <CarbonCreditPassportRegistry<T>>::insert(asset_id, CarbonCreditsPassport::new(asset_id, project_id, project.annual_reports.len()));
                            Ok(())
                        }
                    }
             })?;
    
            Self::deposit_event(Event::CarbonCreditsMinted(project_owner, project_id, asset_id));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
		pub fn release_bond_carbon_credits(
			origin: OriginFor<T>,
            project_id: ProjectId,
			asset_id: <T as pallet_evercity_assets::Config>::AssetId, 
			carbon_credits_count: T::Balance, 
			bond_id: pallet_evercity_bonds::bond::BondId
		) -> DispatchResultWithPostInfo {
			let project_owner = ensure_signed(origin.clone())?;
			let bond = pallet_evercity_bonds::Module::<T>::get_bond(&bond_id);
			ensure!(bond.issuer == project_owner, Error::<T>::NotAnIssuer);
			ensure!(bond.state == BondState::FINISHED , Error::<T>::BondNotFinished);

            ProjectById::<T>::try_mutate(
                project_id, |project_option| -> DispatchResult {
                    match project_option {
                        None => Err(Error::<T>::ProjectNotExist.into()),
                        Some(project) => {
                            ensure!(project.owner == project_owner, Error::<T>::AccountNotOwner);
                            ensure!(project.state == project::REGISTERED, Error::<T>::ProjectNotRegistered);
                            ensure!(project.get_bond_id().is_some(), Error::<T>::ProjectIsNotBond);
        
                            // Check that there is at least one annual report
                            let reports_len = project.annual_reports.len();
                            ensure!(reports_len > 0,    
                                Error::<T>::NoAnnualReports
                            );
        
                            // ensure that carbon credits not released, and then set it to released state
                            let last_annual_report = &mut project.annual_reports[reports_len - 1];
                            ensure!(!last_annual_report.is_carbon_credits_released(), Error::<T>::CCAlreadyCreated);
                            last_annual_report.set_carbon_credits_released();
            
                            // Create Asset:
                            let new_carbon_credits_holder_source = 
                                <T::Lookup as StaticLookup>::unlookup(project_owner.clone());
                            let create_asset_call = 
                                pallet_evercity_assets::Call::<T>::create(asset_id, new_carbon_credits_holder_source, 0, Self::u128_to_balance(1));
                            let create_asset_result = create_asset_call.dispatch_bypass_filter(origin.clone());
                            ensure!(!create_asset_result.is_err(), Error::<T>::ErrorCreatingAsset);
        
          
                            // Set metadata from annual report
                            // Changing metadata of annual report to empty struct
                            let mut meta = annual_report::CarbonCreditsMeta::default();
                            sp_std::mem::swap(&mut meta, &mut last_annual_report.carbon_credits_meta);
                            let set_metadata_call = pallet_evercity_assets::Call::<T>::set_metadata(asset_id, 
                                                    meta.name, 
                                                    meta.symbol, 
                                                    meta.decimals
                                                );
        
                            let result = set_metadata_call.dispatch_bypass_filter(origin.clone());
                            ensure!(!result.is_err(), {
                                // destroy if failed
                                let _ = pallet_evercity_assets::Call::<T>::destroy(asset_id, 0);
                                Error::<T>::SetMetadataFailed
                            });
        
                            // Mint Carbon Credits
                            let cc_amount = last_annual_report.carbon_credits_count();
                            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(project_owner.clone());
                            let mint_call = pallet_evercity_assets::Call::<T>::mint(asset_id, new_carbon_credits_holder_source, cc_amount);
                            let result = mint_call.dispatch_bypass_filter(origin.clone());
                            ensure!(!result.is_err(), {
                                // destroy if failed
                                let _ = pallet_evercity_assets::Call::<T>::destroy(asset_id, 0);
                                Error::<T>::ErrorMintingAsset
                            });
        
                            // Create passport
                            <CarbonCreditPassportRegistry<T>>::insert(asset_id, CarbonCreditsPassport::new(asset_id, project_id, project.annual_reports.len()));

                            // SPREAD 
                            let carbon_metadata = match bond.inner.carbon_metadata {
                                Some(c) => c,
                                None => return Err(Error::<T>::CarbonMetadataNotValid.into())
                            };

                            let check_reg = BondCarbonReleaseRegistry::<T>::get(bond_id);
                            ensure!(check_reg.is_none(), Error::<T>::AlreadyReleased);

                            let bond_investment_tuples = pallet_evercity_bonds::Module::<T>::get_bond_account_investment(&bond_id);
                            ensure!(bond_investment_tuples.len() != 0, Error::<T>::InvestmentIsZero);

                            let total_packages = bond_investment_tuples.iter()
                                                            .map(|(_, everusd)| everusd)
                                                            .fold(0, |a, b| {a + b});

                            ensure!(total_packages != 0, Error::<T>::BalanceIsZero);

                            let all_investors_percent = (carbon_metadata.carbon_distribution.investors as f64)/(100_000 as f64);

                            let investor_parts = bond_investment_tuples
                                                    .into_iter()
                                                    .map(|(acc, everusd)| {
                                                        (acc, ((everusd as f64)/(total_packages as f64))*all_investors_percent )
                                                    })
                                                    .filter(|(_, part)| *part != 0.0)
                                                    .map(|(acc, everusd)| {
                                                        (acc, Self::divide_balance(everusd, carbon_credits_count))
                                                    })
                                                    .collect::<Vec<(T::AccountId, T::Balance)>>();

                            // sends the part of balance
                            let proceed_send = |account: T::AccountId, part: i32| {
                                let perc = (part as f64)/(100_000 as f64);
                                if perc != 0.0 {
                                    let balance_to_send = Self::divide_balance(perc, carbon_credits_count);
                                    let _ = 
                                        Self::transfer_carbon_credits(
                                            origin.clone(), asset_id, account, balance_to_send);
                                }
                            };

                            // send to issuer
                            proceed_send(bond.issuer, carbon_metadata.carbon_distribution.issuer);
                            // send to evercity
                            match carbon_metadata.carbon_distribution.evercity {
                                None => {},
                                Some((acc, perc)) => {
                                    proceed_send(acc, perc);
                                }
                            }
                            // send to project developer
                            match carbon_metadata.carbon_distribution.project_developer {
                                None => {},
                                Some((acc, perc)) => {
                                    proceed_send(acc, perc);
                                }
                            }

                            // send to investors
                            for (acc, bal) in investor_parts {
                                let _ = 
                                    Self::transfer_carbon_credits(
                                        origin.clone(), asset_id, acc, bal);
                            }

                            let release = CarbonCreditsBondRelease {amount: carbon_credits_count};
                            BondCarbonReleaseRegistry::<T>::insert(bond_id, release);
                            Ok(())
                        }
                    }
             })?;
			Self::deposit_event(Event::BondCarbonCreditsReleased(bond_id, asset_id));
			Ok(().into())
		}



        /// <pre>
        /// Method: transfer_carbon_credits(
        ///    asset_id: <T as pallet_assets::Config>::AssetId, 
        ///    new_carbon_credits_holder: T::AccountId, 
        ///    amount: T::Balance
        ///) 
        /// Arguments: origin: AccountId - Transaction caller
        ///
        /// Access: Carbon Credits holder
        ///
        ///  Transfers carbon creadits of asset id in given amount to an adress
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
        pub fn transfer_carbon_credits(
            origin: OriginFor<T>, 
            asset_id: <T as pallet_evercity_assets::Config>::AssetId, 
            new_carbon_credits_holder: T::AccountId, 
            amount: T::Balance
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin.clone())?;
            // check passport creds
            let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
            ensure!(passport.is_some(), Error::<T>::PassportNotExist);

            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(new_carbon_credits_holder.clone());
            let transfer_call = pallet_evercity_assets::Call::<T>::transfer(asset_id, new_carbon_credits_holder_source, amount);
            let result = transfer_call.dispatch_bypass_filter(origin);
            ensure!(!result.is_err(), Error::<T>::TransferFailed);

            Self::deposit_event(Event::CarbonCreditsTransfered(owner, new_carbon_credits_holder, asset_id));
            Ok(().into())
        }


        /// <pre>
        /// Method: burn_carbon_credits(
        ///    asset_id: <T as pallet_assets::Config>::AssetId, 
        ///    amount: T::Balance
        ///) 
        /// Arguments: origin: AccountId - Transaction caller
        ///            asset_id - id of asset_id
        ///            amount - amount to burn
        ///
        /// Access: Holder of carbon credits
        ///
        /// Burns amount of carbon credits
        /// 
        /// </pre>
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 2))]
        pub fn burn_carbon_credits(
            origin: OriginFor<T>, 
            asset_id: <T as pallet_evercity_assets::Config>::AssetId, 
            amount: T::Balance
        ) -> DispatchResultWithPostInfo {
            let credits_holder = ensure_signed(origin.clone())?;
            // check passport creds
            let passport = CarbonCreditPassportRegistry::<T>::get(asset_id);
            ensure!(passport.is_some(), Error::<T>::PassportNotExist);
            ensure!(pallet_evercity_assets::Pallet::<T>::balance(asset_id, credits_holder.clone()) >= amount,
                Error::<T>::InsufficientCarbonCredits
            );
            BurnCertificates::<T>::try_mutate(
                credits_holder.clone(), |certificates| -> DispatchResult {
                    match certificates.iter_mut().find(|x| x.asset_id == asset_id) {
                        Some(cert) => {
                            cert.burn_amount += amount;
                        },
                        None => {
                            certificates.push(CarbonCreditsBurnCertificate::new(asset_id, amount));
                        }
                    }

                    let burn_call = pallet_evercity_assets::Call::<T>::burn_self_assets(asset_id, amount);
                    let result = burn_call.dispatch_bypass_filter(origin);
                    ensure!(!result.is_err(), Error::<T>::BurnFailed);
                    Ok(())
                }
            )?;
            Self::deposit_event(Event::CarbonCreditsAssetBurned(credits_holder, asset_id));
            Ok(().into())
        }
    }



    // IMPL PALLET
    impl<T: Config> Pallet<T> {
        /// Changes state of a project by signing
        fn change_project_state(
            project: &mut ProjectStruct<T::AccountId, T, T::Balance>, 
            caller: T::AccountId, 
            event: &mut Option<Event<T>>
        ) -> DispatchResult {
            match &mut project.get_standard() {
                // Project Owner submits PDD (changing status to Registration) => 
                // => Auditor Approves PDD => Standard Certifies PDD => Registry Registers PDD (changing status to Issuance)
                Standard::GOLD_STANDARD => {
                    match project.state {
                        project::PROJECT_OWNER_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(project.is_ready_for_signing(), Error::<T>::IncorrectFileId);
                            ensure!(Self::is_correct_project_signer(project, caller.clone(), accounts::accounts::CC_PROJECT_OWNER_ROLE_MASK), 
                                Error::<T>::IncorrectProjectSigner);
                            project.state = project::AUDITOR_SIGN_PENDING;
                            project.status = project::ProjectStatus::REGISTRATION;
                            *event = Some(Event::ProjectSubmited(caller, project.id));
                        },
                        project::AUDITOR_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                            ensure!(Self::is_correct_project_signer(project, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK), 
                                Error::<T>::IncorrectProjectSigner);
                            project.state = project::STANDARD_SIGN_PENDING;
                            *event = Some(Event::ProjectSignedByAduitor(caller, project.id));
                        },
                        project::STANDARD_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                            ensure!(Self::is_correct_project_signer(project, caller.clone(), accounts::accounts::CC_STANDARD_ROLE_MASK), 
                                Error::<T>::IncorrectProjectSigner);
                            project.state = project::REGISTRY_SIGN_PENDING;
                            *event = Some(Event::ProjectSignedByStandard(caller, project.id));
                        },
                        project::REGISTRY_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                            ensure!(Self::is_correct_project_signer(project, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK), 
                                Error::<T>::IncorrectProjectSigner);
                            project.state = project::REGISTERED;
                            project.status = project::ProjectStatus::ISSUANCE;
                            *event = Some(Event::ProjectSignedByRegistry(caller, project.id));
                        },
                        _ => return Err(Error::<T>::InvalidState.into())
                    }
                    Ok(())
                },
                Standard::GOLD_STANDARD_BOND => {
                    match project.state {
                        project::PROJECT_OWNER_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(project.is_ready_for_signing(), Error::<T>::IncorrectFileId);
                            ensure!(Self::is_correct_project_signer(project, caller.clone(), accounts::accounts::CC_PROJECT_OWNER_ROLE_MASK), 
                                Error::<T>::IncorrectProjectSigner);
                            project.state = project::AUDITOR_SIGN_PENDING;
                            project.status = project::ProjectStatus::REGISTRATION;
                            *event = Some(Event::ProjectSubmited(caller, project.id));
                        },
                        project::AUDITOR_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                            ensure!(Self::is_correct_project_signer(project, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK), 
                                Error::<T>::IncorrectProjectSigner);
                            project.state = project::STANDARD_SIGN_PENDING;
                            *event = Some(Event::ProjectSignedByAduitor(caller, project.id));
                        },
                        project::REGISTRY_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                            ensure!(Self::is_correct_project_signer(project, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK), 
                                Error::<T>::IncorrectProjectSigner);
                            project.state = project::REGISTERED;
                            project.status = project::ProjectStatus::ISSUANCE;
                            *event = Some(Event::ProjectSignedByRegistry(caller, project.id));
                        },
                        _ => return Err(Error::<T>::InvalidState.into())
                    }
                    Ok(())
                }
            }
        }

        /// Changes state of an annual report by signing
        fn change_project_annual_report_state(
            project: &mut ProjectStruct<T::AccountId, T, T::Balance>, 
            caller: T::AccountId, 
            event: &mut Option<Event<T>>
        ) -> DispatchResult {
            let standard = project.get_standard().clone();
            let report = match project.annual_reports.last_mut(){
                None => return Err(Error::<T>::NoAnnualReports.into()),
                Some(rep) => rep
            };
            match standard {
                // Project Owner sends report for verification =>  Auditor provides and submits verification report => 
                // Standard Approves carbon credit issuance => Registry issues carbon credits
                Standard::GOLD_STANDARD  => {
                    match report.state {
                        annual_report::REPORT_PROJECT_OWNER_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(Self::is_correct_annual_report_signer(report, caller.clone(), accounts::accounts::CC_PROJECT_OWNER_ROLE_MASK),
                                Error::<T>::IncorrectProjectSigner);
                            ensure!(report.carbon_credits_meta.is_metadata_valid(), Error::<T>::BadMetadataParameters);
                            report.state = annual_report::REPORT_AUDITOR_SIGN_PENDING;
                            *event = Some(Event::AnnualReportSubmited(caller, project.id));
                        },
                        annual_report::REPORT_AUDITOR_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                            ensure!(Self::is_correct_annual_report_signer(report, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK),
                                Error::<T>::IncorrectProjectSigner);
                            report.state = annual_report::REPORT_STANDARD_SIGN_PENDING;
                            *event = Some(Event::AnnualReportSignedByAuditor(caller, project.id));
                        },
                        annual_report::REPORT_STANDARD_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_standard(&caller), Error::<T>::AccountNotStandard);
                            ensure!(Self::is_correct_annual_report_signer(report, caller.clone(), accounts::accounts::CC_STANDARD_ROLE_MASK),
                                Error::<T>::IncorrectProjectSigner);
                            report.state = annual_report::REPORT_REGISTRY_SIGN_PENDING;
                            *event = Some(Event::AnnualReportSignedByStandard(caller, project.id));
                        },
                        annual_report::REPORT_REGISTRY_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                            ensure!(Self::is_correct_annual_report_signer(report, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK),
                                Error::<T>::IncorrectProjectSigner);
                            report.state = annual_report::REPORT_ISSUED;
                            *event = Some(Event::AnnualReportSignedByRegistry(caller, project.id));
                        },
                        _ => return Err(Error::<T>::InvalidState.into())
                    }
                    Ok(())
                },
                Standard::GOLD_STANDARD_BOND => {
                    match report.state {
                        annual_report::REPORT_PROJECT_OWNER_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_project_owner(&caller), Error::<T>::AccountNotOwner);
                            ensure!(project.owner == caller, Error::<T>::AccountNotOwner);
                            ensure!(Self::is_correct_annual_report_signer(report, caller.clone(), accounts::accounts::CC_PROJECT_OWNER_ROLE_MASK),
                                Error::<T>::IncorrectProjectSigner);
                            ensure!(report.carbon_credits_meta.is_metadata_valid(), Error::<T>::BadMetadataParameters);
                            report.state = annual_report::REPORT_AUDITOR_SIGN_PENDING;
                            *event = Some(Event::AnnualReportSubmited(caller, project.id));
                        },
                        annual_report::REPORT_AUDITOR_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_auditor(&caller), Error::<T>::AccountNotAuditor);
                            ensure!(Self::is_correct_annual_report_signer(report, caller.clone(), accounts::accounts::CC_AUDITOR_ROLE_MASK),
                                Error::<T>::IncorrectProjectSigner);
                            report.state = annual_report::REPORT_STANDARD_SIGN_PENDING;
                            *event = Some(Event::AnnualReportSignedByAuditor(caller, project.id));
                        },
                        annual_report::REPORT_REGISTRY_SIGN_PENDING => {
                            ensure!(accounts::Module::<T>::account_is_cc_registry(&caller), Error::<T>::AccountNotRegistry);
                            ensure!(Self::is_correct_annual_report_signer(report, caller.clone(), accounts::accounts::CC_REGISTRY_ROLE_MASK),
                                Error::<T>::IncorrectProjectSigner);
                            report.state = annual_report::REPORT_ISSUED;
                            *event = Some(Event::AnnualReportSignedByRegistry(caller, project.id));
                        },
                        _ => return Err(Error::<T>::InvalidState.into())
                    }
                    Ok(())
                }
            }
        }

        fn is_correct_project_signer(project: &ProjectStruct<T::AccountId, T, T::Balance>, account: T::AccountId, role: RoleMask) -> bool {
            pallet_evercity_accounts::Module::<T>::account_is_selected_role(&account, role) &&
            project.is_required_signer((account, role))
        }

        fn is_correct_annual_report_signer(annual_report: &annual_report::AnnualReportStruct<T::AccountId, T, T::Balance>, account: T::AccountId, role: RoleMask) -> bool {
            pallet_evercity_accounts::Module::<T>::account_is_selected_role(&account, role) &&
            annual_report.is_required_signer((account, role))
        }

        pub fn balance(asset_id: AssetId<T>, account_id: T::AccountId) -> T::Balance {
            pallet_evercity_assets::Module::<T>::balance(asset_id, account_id)
        }
    
        #[cfg(test)]
        pub fn get_proj_by_id(id: ProjectId) -> Option<ProjectStruct<T::AccountId, T, T::Balance>> {
            ProjectById::<T>::get(id)
        }
    
        #[cfg(test)]
        pub fn get_passport_by_assetid(asset_id: AssetId<T>) -> Option<CarbonCreditsPassport<AssetId<T>>> {
            CarbonCreditPassportRegistry::<T>::get(asset_id)
        }
    
        #[cfg(test)]
        pub fn get_certificates_by_account(account: T::AccountId) -> Vec<CarbonCreditsBurnCertificate<AssetId<T>, T::Balance>> {
            BurnCertificates::<T>::get(account)
        }
    
        #[cfg(test)]
        pub fn create_test_carbon_credits(
            account_id: T::AccountId, 
            cc_amount: T::Balance, 
            asset_id: T::AssetId, 
            fake_project_id: ProjectId
        ) -> DispatchResult {
            let new_carbon_credits_holder_source = <T::Lookup as StaticLookup>::unlookup(account_id.clone());
            let mint_call = pallet_evercity_assets::Call::<T>::mint(asset_id, new_carbon_credits_holder_source, cc_amount);
            let origin = frame_system::RawOrigin::Signed(account_id).into();
            let _ = mint_call.dispatch_bypass_filter(origin);
            <CarbonCreditPassportRegistry<T>>::insert(asset_id, CarbonCreditsPassport::new(asset_id, fake_project_id, 1));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> where <T as pallet_evercity_assets::pallet::Config>::Balance: From<u128> + Into<u128> { 
        pub fn u128_to_balance(num: u128) -> <T as pallet_evercity_assets::pallet::Config>::Balance {
			num.into()
		}

		pub fn balance_to_u128(bal: <T as pallet_evercity_assets::pallet::Config>::Balance ) -> u128 {
			bal.into()
		}

		pub fn divide_balance(
			percent: f64, 
			bal_amount: <T as pallet_evercity_assets::pallet::Config>::Balance
		) -> <T as pallet_evercity_assets::pallet::Config>::Balance  {
			let temp_u64 = ((Self::balance_to_u128(bal_amount) as f64) * percent) as u128;
			Self::u128_to_balance(temp_u64)
		}
    }
}