use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type RoleMask = u32;

pub const MASTER_ROLE_MASK: RoleMask = 1;

pub const CUSTODIAN_ROLE_MASK: RoleMask = 2;
pub const ISSUER_ROLE_MASK: RoleMask = 4;
pub const INVESTOR_ROLE_MASK: RoleMask = 8;
pub const AUDITOR_ROLE_MASK: RoleMask = 16;
pub const MANAGER_ROLE_MASK: RoleMask = 32;
pub const IMPACT_REPORTER_ROLE_MASK: RoleMask = 64;
pub const EMISSION_CREATOR_ROLE_MASK: RoleMask = 128;

// Carbon Credits Roles
// CC_ prefix Means - Carbon Credits
pub const CC_PROJECT_OWNER_ROLE_MASK: RoleMask = 256;
pub const CC_AUDITOR_ROLE_MASK: RoleMask = 512;
pub const CC_STANDARD_ROLE_MASK: RoleMask = 1024;
pub const CC_INVESTOR_ROLE_MASK: RoleMask = 2048;
pub const CC_REGISTRY_ROLE_MASK: RoleMask = 4096;

pub const ALL_ROLES_MASK: RoleMask = MASTER_ROLE_MASK
    | CUSTODIAN_ROLE_MASK
    | ISSUER_ROLE_MASK
    | INVESTOR_ROLE_MASK
    | AUDITOR_ROLE_MASK
    | MANAGER_ROLE_MASK
    | IMPACT_REPORTER_ROLE_MASK
    | EMISSION_CREATOR_ROLE_MASK 
    | CC_PROJECT_OWNER_ROLE_MASK
    | CC_AUDITOR_ROLE_MASK
    | CC_STANDARD_ROLE_MASK
    | CC_INVESTOR_ROLE_MASK
    | CC_REGISTRY_ROLE_MASK
    ;

#[inline]
pub const fn is_roles_correct(roles: RoleMask) -> bool {
    // max value of any roles combinations
    roles <= ALL_ROLES_MASK && roles > 0
}

#[inline]
pub const fn is_roles_mask_included(roles: RoleMask, const_mask: RoleMask) -> bool {
    roles <= const_mask && roles > 0
}

/// Main structure, containing account data: roles(bit mask), identity(external id), creation_time.
/// This structure is used to check and assign account roles
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct AccountStruct<Moment> {
    pub roles: RoleMask,
    #[codec(compact)]
    pub identity: u64,
    #[codec(compact)]
    pub create_time: Moment,
}

pub type EvercityAccountStructOf<T> =
    AccountStruct<<T as pallet_timestamp::Config>::Moment>;

impl<Moment> AccountStruct<Moment> {
    pub fn new(roles: RoleMask, identity: u64, create_time: Moment) -> Self {
        AccountStruct{
            roles,
            identity,
            create_time
        }
    }
}

// use impl_trait_for_tuples::impl_for_tuples;
// #[impl_trait_for_tuples::impl_for_tuples(30)]
// pub trait OnAddAccount<AccountId, Moment> {
//     fn on_add_account(account: &AccountId, data: &EvercityAccountStruct<Moment>);
// }
