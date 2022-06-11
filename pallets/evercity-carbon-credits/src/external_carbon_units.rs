use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use frame_support::sp_std::{
    cmp::{PartialEq}, 
};

/// Batch asset id to list in retirement details on external registry
pub type BatchAssetId = [u8; 32];
pub type ExternalProjectId = Vec<u8>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum RegistryType {
    Cercarbono
}

impl Default for RegistryType {
    fn default() -> Self {
        Self::Cercarbono
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BatchStatus {
    /// Initial creation
    INITIAL,
    /// All details included, awaiting verification
    AWAITING_VERIFICATION,
    /// Verified, last status
    VERIFIED,
    /// Rejected because details incorrect
    REJECTED,
}

impl Default for BatchStatus {
    fn default() -> Self {
        Self::INITIAL
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct BatchAsset<AccountId> {
    pub owner: AccountId,
    pub registry_type: RegistryType,
    pub serial_number: Vec<u8>,
    pub status: BatchStatus,
    pub external_project_id: Vec<u8>,
    pub vintage_name: Vec<u8>,
    pub amount: u32,
}

impl<AccountId> BatchAsset<AccountId> {
    pub fn new(owner: AccountId) -> Self { 
        Self { owner, registry_type: Default::default(), serial_number: Default::default(), status: BatchStatus::INITIAL,
             external_project_id: Default::default(), vintage_name: Default::default(), amount: Default::default() } }
}

// external outside third-party
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct ExternalProject {
    pub uri: Vec<u8>,
    pub hash_link: Vec<u8>,
    pub vintages: Vec<Vintage>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct Vintage {
    pub name: Vec<u8>,
    pub hash_link: Vec<u8>,
    pub uri: Vec<u8>,
}

impl Vintage {
}

pub fn construct_external_project_id(registry_type: RegistryType, project_id: Vec<u8>) -> ExternalProjectId {
    todo!()
}

impl ExternalProject {

    pub fn construct_carnon_asset_name(&self, vintage_id: usize) {
        todo!()
    }
}