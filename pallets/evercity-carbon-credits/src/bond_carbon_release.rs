use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsBondRelease<Balance> {
    pub amount: Balance,
}