use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use pallet_evercity_bonds::BondId;
use crate::standard::Standard;
use crate::annual_report::*;
use pallet_evercity_filesign::file::FileId;
use frame_support::sp_std::{
    cmp::{
        PartialEq}, 
};
use crate::required_signers::RequiredSigner;
use scale_info::TypeInfo;

/// Project states for project state machine.
/// 
/// Possible project states: 
/// - `PROJECT_OWNER_SIGN_PENDING` = 1
/// - `AUDITOR_SIGN_PENDING` = 2
/// - `STANDARD_SIGN_PENDING` = 4
/// - `INVESTOR_SIGN_PENDING` = 8
/// - `REGISTRY_SIGN_PENDING` = 16
/// - `REGISTERED` = 32
/// - `EVERCITY_SIGN_PENDING` = 64
pub type ProjectStateMask = u16;
pub const PROJECT_OWNER_SIGN_PENDING: ProjectStateMask = 1;
pub const AUDITOR_SIGN_PENDING: ProjectStateMask = 2;
pub const STANDARD_SIGN_PENDING: ProjectStateMask = 4;
pub const INVESTOR_SIGN_PENDING: ProjectStateMask = 8;
pub const REGISTRY_SIGN_PENDING: ProjectStateMask = 16;
pub const REGISTERED: ProjectStateMask = 32;

// Evercity Bond Special Masks
pub const EVERCITY_SIGN_PENDING: ProjectStateMask = 64;

pub type ProjectId = u32;

/// Main struct for projects
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, TypeInfo)]
pub struct ProjectStruct<AccountId, Moment, Balance> where AccountId: PartialEq + Clone, Balance: Clone {
    /// Owner of the project. Creates project, assigns auditor (and other) signers for the project.
    /// Creates annual reports and assigns auditor (and other) signers for it.
    /// Releases Carbon Credits when annual report is signed.
    pub owner: AccountId,
    /// Project Id.
    pub id: ProjectId,
    /// Status for project.
    pub status: ProjectStatus,
    /// State shows that project is awaiting for specific signers.
    pub state: ProjectStateMask,
    /// Id of file with project documentation.
    pub file_id: Option<FileId>,
    /// List of annual reports.
    pub annual_reports: Vec<AnnualReportStruct<AccountId, Moment, Balance>>,
    /// Signers that should sign to transit project to `REGISTERED` state.
    required_signers: Vec<RequiredSigner<AccountId>>,
    /// Carbon Credits industry standard.
    standard: Standard,
    /// Bond Id if project connected to green bond.
    bond_id: Option<BondId>,
}

impl<AccountId, Moment, Balance> ProjectStruct<AccountId, Moment, Balance> where AccountId: PartialEq + Clone, Moment: pallet_timestamp::Config, Balance: Clone {
    /// constructor for project
    pub fn new(owner: AccountId, id: u32, standard: Standard, file_id: Option<FileId>) -> Self {
        ProjectStruct{
            file_id, 
            owner,
            id,
            standard,
            status: ProjectStatus::default(), 
            state: PROJECT_OWNER_SIGN_PENDING,
            annual_reports: Vec::new(),
            required_signers: Vec::new(),
            bond_id: None
        }
    }

    pub fn new_with_bond(owner: AccountId, id: u32, standard: Standard, file_id: Option<FileId>, bond_id: BondId) -> Self {
        ProjectStruct{
            file_id, 
            owner,
            id,
            standard,
            status: ProjectStatus::default(), 
            state: PROJECT_OWNER_SIGN_PENDING,
            annual_reports: Vec::new(),
            required_signers: Vec::new(),
            bond_id: Some(bond_id),
        }
    }

    // Standart must be guaranted immutable for lifetime of the progect on register and issuance step 
    pub fn get_standard(&self) -> &Standard {
        &self.standard
    }

    // Standart must be guaranted immutable for lifetime of the progect on register and issuance step 
    pub fn set_new_standard(&mut self, new_standard: Standard) {
        if self.state == PROJECT_OWNER_SIGN_PENDING {
            self.standard = new_standard
        }
    }

    pub fn assign_required_signer(&mut self, signer: RequiredSigner<AccountId>) {
        if !self.required_signers.iter().any(|(acc, role)| *acc == signer.0 && *role == signer.1) {
            self.required_signers.push(signer);
        }
    }

    pub fn remove_required_signer(&mut self, signer: RequiredSigner<AccountId>) {
        let index = match self.required_signers.iter().position(|a| *a == signer) {
            Some(i) => i,
            None => {
                return;
            }
        };

        self.required_signers.remove(index);
    }

    pub fn is_required_signer(&self, signer: RequiredSigner<AccountId>) -> bool {
        self.required_signers.iter().any(|(acc, role)| *acc == signer.0 && *role == signer.1)
    }

    pub fn is_ready_for_signing(&self) -> bool {
        self.file_id.is_some()
    }

    pub fn get_bond_id(&self) -> Option<BondId> {
        self.bond_id
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum ProjectStatus {
    /// When project is created
    PREPARING,
    /// When project awaits signs from auditors
    REGISTRATION,
    /// When project registered (signed by all kind of auditors)
    ISSUANCE,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::PREPARING
    }
}