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


// impl<AccountId, AssetId, AssetCount, EverUsdCount> ExchangeStruct<AccountId, AssetId, AssetCount, EverUsdCount> {
//     pub fn new(ever_usd_holder: AccountId,
//         carbon_credits_holder: AccountId,
//         ever_usd_count: EverUsdCount,
//         carbon_credits_asset_id: AssetId,
//         carbon_credits_count: AssetCount,
//         approved: ApproveMask,
//     ) -> Self {
//             Self {
//                 ever_usd_holder,
//                 carbon_credits_holder,
//                 ever_usd_count,
//                 carbon_credits_asset_id,
//                 carbon_credits_count,
//                 approved,
//             }
//     }
// }

pub type ApproveMask = u8;
pub const ASSET_HOLDER_APPROVED: ApproveMask = 1;
pub const CARBON_CREDITS_HOLDER_APPROVED: ApproveMask = 2;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum HolderType {
    AssetHolder,
    CarbonCreditsHolder
}