use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

use crate::approve_mask::ApproveMask;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct EverUSDTradeRequest<AccountId, CarbonCreditId, CarbonCreditsCount, EverUsdCount> {
    pub ever_usd_holder: AccountId,
    pub carbon_credits_holder: AccountId,
    pub ever_usd_count: EverUsdCount,
    pub carbon_credits_count: CarbonCreditsCount,
    pub carbon_credits_asset_id: CarbonCreditId,
    pub approved: ApproveMask,
}

impl<AccountId, CarbonCreditId, CarbonCreditsCount, EverUsdCount> EverUSDTradeRequest<AccountId, CarbonCreditId, CarbonCreditsCount, EverUsdCount> {
    pub fn new(ever_usd_holder: AccountId,
        carbon_credits_holder: AccountId,
        ever_usd_count: EverUsdCount,
        carbon_credits_asset_id: CarbonCreditId,
        carbon_credits_count: CarbonCreditsCount,
        approved: ApproveMask,
    ) -> Self {
            Self {
                ever_usd_holder,
                carbon_credits_holder,
                ever_usd_count,
                carbon_credits_asset_id,
                carbon_credits_count,
                approved,
            }
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum EverUSDTradeHolderType {
    EverUSDHolder,
    CarbonCreditsHolder
}