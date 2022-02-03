use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct TradeRequest<AccountId, AssetId, CarbonCreditId, AssetCount, CarbonCreditsCount> {
    pub asset_holder: AccountId,
    pub carbon_credits_holder: AccountId,
    pub asset_count: AssetCount,
    pub asset_id: AssetId,
    pub carbon_credits_count: CarbonCreditsCount,
    pub carbon_credits_id: CarbonCreditId,
    pub approved: ApproveMask,
}


impl<AccountId, AssetId, CarbonCreditId, AssetCount, CarbonCreditsCount> TradeRequest<AccountId, AssetId, CarbonCreditId, AssetCount, CarbonCreditsCount> {
    pub fn new(
        asset_holder: AccountId,
        carbon_credits_holder: AccountId,
        asset_count: AssetCount,
        asset_id: AssetId,
        carbon_credits_count: CarbonCreditsCount,
        carbon_credits_id: CarbonCreditId,
        approved: ApproveMask,
    ) -> Self {
            Self {
                asset_holder,
                carbon_credits_holder,
                asset_count,
                asset_id,
                carbon_credits_count,
                carbon_credits_id,
                approved,
            }
    }
}

pub type ApproveMask = u8;
pub const NO_APPROVES: ApproveMask = 0;
pub const ASSET_HOLDER_APPROVED: ApproveMask = 1;
pub const CARBON_CREDITS_HOLDER_APPROVED: ApproveMask = 2;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum HolderType {
    AssetHolder,
    CarbonCreditsHolder
}