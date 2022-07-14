use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, TypeInfo)]
pub struct CarbonCreditsBurnCertificate<AssetId, Balance> {
    /// Carbon Credit asset id
    pub asset_id: AssetId,
    /// Amount of burned Carbon Credits
    pub burn_amount: Balance,
}

impl<AssetId, Balance> CarbonCreditsBurnCertificate<AssetId, Balance>{
    pub fn new(asset_id: AssetId, burn_amount: Balance) -> Self {
        Self { 
            asset_id, 
            burn_amount
        }
    }
}