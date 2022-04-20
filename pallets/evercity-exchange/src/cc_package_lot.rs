use codec::{Encode, Decode};
use frame_support::RuntimeDebug;
use pallet_evercity_bonds::Expired;

/// Struct representing pack of carbon credits for sale.
/// Can include target bearer (to sell only to them)
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct CarbonCreditsPackageLot<AccountId, Moment, CCAmount, EverUSDAmount> {
    /// If set - only targer bearer can buy a lot, if None - anyone can buy
    pub target_bearer: Option<AccountId>,
    /// Lot available for sale only before deadline
    pub deadline: Moment,
    /// Amount of Carbon Credits for sale in this lot
    pub amount: CCAmount,
    /// Total price of this lot
    pub price: EverUSDAmount,
}

/// Wrapper of struct CarbonCreditsPackageLot representing pack of carbon credits for sale.
/// Can include target bearer (to sell only to them)
pub type CarbonCreditsPackageLotOf<T> = CarbonCreditsPackageLot<
    <T as frame_system::Config>::AccountId,
    <T as pallet_timestamp::Config>::Moment,
    crate::CarbonCreditsBalance<T>,
    pallet_evercity_bonds::EverUSDBalance,
>;

impl<AccountId, Moment: core::cmp::PartialOrd, CCAmount, EverUSDAmount> Expired<Moment> 
    for CarbonCreditsPackageLot<AccountId, Moment, CCAmount, EverUSDAmount> {
    fn is_expired(&self, now: Moment) -> bool {
        self.deadline < now
    }
}