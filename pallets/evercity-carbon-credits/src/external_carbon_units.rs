use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use frame_support::sp_std::{
    cmp::{PartialEq}, 
};
use scale_info::TypeInfo;

/// Batch asset id to list in retirement details on external registry
pub type BatchAssetId = [u8; 32];
pub type ExternalProjectId = Vec<u8>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, TypeInfo)]
pub enum RegistryType {
    Cercarbono
}

impl Default for RegistryType {
    fn default() -> Self {
        Self::Cercarbono
    }
}

impl RegistryType {
    pub fn to_string(&self) -> &'static str {
        match self {
            RegistryType::Cercarbono => "Cercarbono",
        }
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, TypeInfo)]
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

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, TypeInfo)]
pub struct BatchAsset<AccountId> {
    pub owner: AccountId,
    pub registry_type: RegistryType,
    pub serial_number: Vec<u8>,
    pub status: BatchStatus,
    pub external_project_id: Vec<u8>,
    pub vintage_name: Vec<u8>,
    pub amount: u32,
    pub uri: Vec<u8>,
    pub ipfs_hash: Vec<u8>,
}

impl<AccountId> BatchAsset<AccountId> {
    pub fn new(owner: AccountId) -> Self { 
        Self { owner, registry_type: Default::default(), serial_number: Default::default(), status: BatchStatus::INITIAL,
             external_project_id: Default::default(), vintage_name: Default::default(), amount: Default::default(),
             uri: Default::default(), ipfs_hash: Default::default()
           } 
        }
     
    pub fn construct_external_project_id(&self) -> ExternalProjectId {
        let prefix = self.registry_type.to_string().as_bytes().to_vec();
        [prefix, br#"-"#.to_vec(), self.external_project_id.clone()].concat()
    }

    pub fn construct_carbon_asset_name(&self) -> Vec<u8> {
        let evercity_prefix = "EVERCITY-CO2-".as_bytes().to_vec();
        let external_id = self.construct_external_project_id();
        match self.vintage_name.len() {
            0 =>  [evercity_prefix, external_id].concat(),
            _ => [evercity_prefix, external_id, br#"-"#.to_vec(), self.vintage_name.clone()].concat()
        }
    }

    pub fn construct_carbon_asset_symbol(&self) -> Vec<u8> {
        let evercity_prefix = "EVR-CO2-".as_bytes().to_vec();
        let external_id = self.construct_external_project_id();
        match self.vintage_name.len() {
            0 =>  [evercity_prefix, external_id].concat(),
            _ => [evercity_prefix, external_id, br#"-"#.to_vec(), self.vintage_name.clone()].concat()
        }
    }
}