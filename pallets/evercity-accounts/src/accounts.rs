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

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct AccountStruct {
    pub roles: RoleMask,
}

impl AccountStruct {
    pub fn new(roles: RoleMask) -> Self {
        AccountStruct{
            roles
        }
    }
}