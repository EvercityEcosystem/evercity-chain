use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
    dispatch::Vec,
};
use frame_support::sp_std::{
    cmp::{PartialEq}, 
};

// #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub type BatchAssetId = [u8; 32];

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
    INITIAL,
    AWAITING_VERIFICATION,
    VERIFIED,
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
    pub vintage_id: u16,
    pub amount: u32,
}

impl<AccountId> BatchAsset<AccountId> {
    pub fn new(owner: AccountId, registry_type: RegistryType, external_project_id: Vec<u8>, amount: u32) -> Self { 
        Self { owner, registry_type, serial_number: Default::default(), status: BatchStatus::INITIAL,
             external_project_id, vintage_id: Default::default(), amount } }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ExternalStandard {
    GOLD_STANDARD
}

impl Default for ExternalStandard {
    fn default() -> Self {
        todo!()
    }
}

// external outside third-party
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct ExternalProject {
    pub standard: ExternalStandard,
    pub methodology: Vec<u8>,
    pub region: Vec<u8>,
    pub method: Vec<u8>,
    pub emission_type: Vec<u8>,
    pub category: Vec<u8>,
    pub uri: Vec<u8>,
    pub vintages: Vec<Vintage>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct Vintage {
    pub name: Vec<u8>,
    pub start_time: Vec<u8>,
    pub end_time: Vec<u8>,
    pub total_vintage_quantity: Vec<u8>,
    pub uri: Vec<u8>,
}