use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

/// Struct representing Carbon Credits release for the bond.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsBondRelease<Balance> {
    /// Amount of released Carbon Credits.
    pub amount: Balance,
}