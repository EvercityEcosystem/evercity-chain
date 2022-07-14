use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
use scale_info::TypeInfo;

/// Carbon Credits industry standard
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum Standard {
    /// Variant for the Gold Standard for bond projects
    GOLD_STANDARD_BOND,
    /// Variant for the Gold Standard
    GOLD_STANDARD,
}

impl Default for Standard {
    fn default() -> Standard {
        Standard::GOLD_STANDARD
    }
}