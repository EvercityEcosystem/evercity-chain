use codec::{Encode, Decode};
use frame_support::RuntimeDebug;
use pallet_evercity_bonds::Expired;
use scale_info::TypeInfo;

/// Struct representing pack of carbon credits for sale.
/// Can include target bearer (to sell only to them)
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct CarbonCreditsPackageLot<AccountId, Moment, CCBalance, EverUSDAmount> {
    /// If set - only targer bearer can buy a lot, if None - anyone can buy
    pub target_bearer: Option<AccountId>,
    /// Lot available for sale only before deadline
    #[codec(compact)]
    pub deadline: Moment,
    /// Amount of Carbon Credits for sale in this lot
    #[codec(compact)]
    pub amount: CCBalance,
    /// Price per 1 Carbon Credit. Total price = amount*price_per_item
    #[codec(compact)]
    pub price_per_item: EverUSDAmount,
}

/// Wrapper of struct CarbonCreditsPackageLot representing pack of carbon credits for sale.
/// Can include target bearer (to sell only to them)
pub type CarbonCreditsPackageLotOf<T> = CarbonCreditsPackageLot<
    <T as frame_system::Config>::AccountId,
    <T as pallet_timestamp::Config>::Moment,
    crate::CarbonCreditsBalance<T>,
    pallet_evercity_bonds::EverUSDBalance,
>;

impl<AccountId, Moment: core::cmp::PartialOrd, CCBalance, EverUSDAmount> Expired<Moment> 
    for CarbonCreditsPackageLot<AccountId, Moment, CCBalance, EverUSDAmount> {
    fn is_expired(&self, now: Moment) -> bool {
        self.deadline < now
    }
}