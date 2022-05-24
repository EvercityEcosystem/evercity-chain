#![allow(clippy::unused_unit)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod accounts;
#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;

use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    decl_event,
    dispatch::{
        DispatchResult,
        Vec,
    },
};
use frame_system::{
    ensure_signed,
};
use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};
use accounts::*;

type Timestamp<T> = pallet_timestamp::Module<T>;

pub trait Config: frame_system::Config + pallet_timestamp::Config  {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as CarbonCredits {
        /// Storage map for accounts, their roles and corresponding info
        AccountRegistry
            get(fn account_registry)
            config(genesis_account_registry):
            map hasher(blake2_128_concat) T::AccountId => EvercityAccountStructOf<T> ;

        LastID: u32;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// \[master, account, role\]
        AccountAdd(AccountId, AccountId, RoleMask),

        /// \[master, account, role\]
        AccountSet(AccountId, AccountId, RoleMask),

        /// \[master, account, role\]
        AccountWithdraw(AccountId, AccountId, RoleMask),

        /// \[master, account\]
        MasterSet(AccountId, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        // Account errors:
        AccountNotMaster,
        AccountNotAuditor,
        AccountNotOwner,
        AccountNotStandard,
        AccountNotRegistry,
        AccountNotInvestor,
        AccountToAddAlreadyExists,
        AccountRoleParamIncorrect,
        AccountNotExist,
        AccountRoleMasterIncluded,

        InvalidAction,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;
        
        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn account_add_with_role_and_data(origin, who: T::AccountId, role: RoleMask, identity: u64) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(!AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountToAddAlreadyExists);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::insert(who.clone(), AccountStruct::new(role, identity, Timestamp::<T>::get()));
            Self::deposit_event(RawEvent::AccountAdd(caller, who, role));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn account_set_with_role_and_data(origin, who: T::AccountId, role: RoleMask) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles |= role;
            });
            Self::deposit_event(RawEvent::AccountSet(caller, who, role));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn set_master(origin, who: T::AccountId) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(!Self::account_is_master(&who), Error::<T>::InvalidAction);
            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles |= MASTER_ROLE_MASK;
            });
            Self::deposit_event(RawEvent::MasterSet(caller, who));
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn account_withdraw_role(origin, who: T::AccountId, role: RoleMask) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles ^= role;
            });
            Self::deposit_event(RawEvent::AccountWithdraw(caller, who, role));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    /// <pre>
    /// Method: account_is_master(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Master role
    /// </pre>
    #[inline]
    pub fn account_is_master(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & MASTER_ROLE_MASK != 0
    }

     /// <pre>
    /// Method: account_is_cc_project_owner(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits project owner role
    /// </pre>
    #[inline]
    pub fn account_is_cc_project_owner(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_PROJECT_OWNER_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_auditor(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc hasc carbon credits auditor role
    /// </pre>
    #[inline]
    pub fn account_is_cc_auditor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_AUDITOR_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_standard(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits standard role
    /// </pre>
    #[inline]
    pub fn account_is_cc_standard(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_STANDARD_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_investor(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits investor role
    /// </pre>
    #[inline]
    pub fn account_is_cc_investor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_INVESTOR_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_cc_registry(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has carbon credits registry role
    /// </pre>
    #[inline]
    pub fn account_is_cc_registry(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CC_REGISTRY_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: accoount_is_selected_role(acc: &T::AccountId, role: RoleMask) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has some custom role
    /// </pre>
    #[inline]
    pub fn account_is_selected_role(acc: &T::AccountId, role: RoleMask) -> bool {
        AccountRegistry::<T>::get(acc).roles & role != 0
    }
}
