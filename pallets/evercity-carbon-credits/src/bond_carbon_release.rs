use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use scale_info::TypeInfo;

/// Struct representing Carbon Credits release for the bond.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, TypeInfo)]
pub struct CarbonCreditsBondRelease<Balance> {
    /// Amount of released Carbon Credits.
    pub amount: Balance,
}