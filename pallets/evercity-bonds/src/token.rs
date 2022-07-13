use crate::bond::{EverUSDBalance, Expired};
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::{
        RuntimeDebug,
    },
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use scale_info::TypeInfo;

/// Structure, created by Issuer or Investor to receive EverUSD on her balance
/// by paying USD to Custodian. Then Custodian confirms request, adding corresponding
/// amount to mint request creator's balance
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, TypeInfo)]
pub struct TokenMintRequestStruct<Moment> {
    #[codec(compact)]
    pub amount: EverUSDBalance,
    #[codec(compact)]
    pub deadline: Moment,
}

impl<Moment: core::cmp::PartialOrd> Expired<Moment> for TokenMintRequestStruct<Moment> {
    fn is_expired(&self, now: Moment) -> bool {
        self.deadline <= now
    }
}

pub type TokenMintRequestStructOf<T> =
    TokenMintRequestStruct<<T as pallet_timestamp::Config>::Moment>;

/// Structure, created by Issuer or Investor to burn EverUSD on her balance
/// and receive corresponding amount of USD from Custodian.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, TypeInfo)]
pub struct TokenBurnRequestStruct<Moment> {
    #[codec(compact)]
    pub amount: EverUSDBalance,
    #[codec(compact)]
    pub deadline: Moment,
}

impl<Moment: core::cmp::PartialOrd> Expired<Moment> for TokenBurnRequestStruct<Moment> {
    fn is_expired(&self, now: Moment) -> bool {
        self.deadline <= now
    }
}

pub type TokenBurnRequestStructOf<T> =
    TokenBurnRequestStruct<<T as pallet_timestamp::Config>::Moment>;