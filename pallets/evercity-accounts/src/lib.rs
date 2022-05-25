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
        Fuse get(fn fuse)
            build(|config| !config.genesis_account_registry.is_empty()):
            bool;
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
        /// \[master, account, role, identity\]
        AccountAdd(AccountId, AccountId, RoleMask, u64),

        /// \[master, account, role\]
        AccountSet(AccountId, AccountId, RoleMask),

        /// \[master, account, role\]
        AccountWithdraw(AccountId, AccountId, RoleMask),

        /// \[master, account\]
        MasterSet(AccountId, AccountId),
        /// \[master, account\]
        AccountDisable(AccountId, AccountId),
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
        /// Account not authorized(doesn't have a needed role, or doesnt present in AccountRegistry at all)
        AccountNotAuthorized,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = T::DbWeight::get().reads_writes(2,1)]
        fn set_master(origin) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            Fuse::try_mutate(|fuse|->DispatchResult{
                if *fuse {
                    Err( Error::<T>::InvalidAction.into() )
                }else{
                    AccountRegistry::<T>::insert(&caller, AccountStruct::new(MASTER_ROLE_MASK, 0, Timestamp::<T>::get()));
                    *fuse = true;
                    Ok(())
                }
            })
        }
        
        #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        pub fn account_add_with_role_and_data(origin, who: T::AccountId, role: RoleMask, #[compact] identity: u64) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(!AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountToAddAlreadyExists);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::insert(who.clone(), AccountStruct::new(role, identity, Timestamp::<T>::get()));
            Self::deposit_event(RawEvent::AccountAdd(caller, who, role, identity));
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

        // #[weight = 10_000 + T::DbWeight::get().reads_writes(2, 1)]
        // pub fn set_master(origin, who: T::AccountId) -> DispatchResult {
        //     let caller = ensure_signed(origin)?;
        //     ensure!(caller != who, Error::<T>::InvalidAction);
        //     ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
        //     ensure!(!Self::account_is_master(&who), Error::<T>::InvalidAction);
        //     AccountRegistry::<T>::mutate(who.clone(),|acc|{
        //         acc.roles |= MASTER_ROLE_MASK;
        //     });
        //     Self::deposit_event(RawEvent::MasterSet(caller, who));
        //     Ok(())
        // }

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

        
        /// <pre>
        /// Method: account_disable(who: AccountId)
        /// Arguments: origin: AccountId - transaction caller
        ///            who: AccountId - account to disable
        /// Access: Master role
        ///
        /// Disables all roles of account, setting roles bitmask to 0.
        /// Accounts are not allowed to perform any actions without role,
        /// but still have its data in blockchain (to not loose related entities)
        /// </pre>
       // #[weight = <T as Config>::WeightInfo::account_disable()]
        #[weight = 10_000 + T::DbWeight::get().reads_writes(4, 1)]
        fn account_disable(origin, who: T::AccountId) -> DispatchResult {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotMaster);
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);

            AccountRegistry::<T>::mutate(&who,|acc|{
                acc.roles = 0; // set no roles
            });

            Self::deposit_event(RawEvent::AccountDisable(caller, who));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    // fn account_add(account: &T::AccountId, mut data: EvercityAccountStructOf<T>) {
    //     data.create_time = Timestamp::<T>::get();
    //     AccountRegistry::<T>::insert(account, &data);
    //     T::OnAddAccount::on_add_account(account, &data);
    // }

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
    /// Method: account_is_custodian(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Custodian role
    /// </pre>
    pub fn account_is_custodian(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & CUSTODIAN_ROLE_MASK != 0
    }

     /// <pre>
    /// Method: account_is_issuer(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Issuer role
    /// </pre>
    pub fn account_is_issuer(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & ISSUER_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_investor(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Investor role
    /// </pre>
    pub fn account_is_investor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & INVESTOR_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_auditor(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Auditor role
    /// </pre>
    pub fn account_is_auditor(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & AUDITOR_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_manager(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Manager role
    /// </pre>
    pub fn account_is_manager(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & MANAGER_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_is_impact_reporter(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc has global Impact Reporter role
    /// </pre>
    pub fn account_is_impact_reporter(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & IMPACT_REPORTER_ROLE_MASK != 0
    }

    /// <pre>
    /// Method: account_token_mint_burn_allowed(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - checked account id
    ///
    /// Checks if the acc can create burn and mint tokens requests(INVESTOR or ISSUER)
    /// </pre>
    pub fn account_token_mint_burn_allowed(acc: &T::AccountId) -> bool {
        const ALLOWED_ROLES_MASK: RoleMask = INVESTOR_ROLE_MASK | ISSUER_ROLE_MASK;
        AccountRegistry::<T>::get(acc).roles & ALLOWED_ROLES_MASK != 0
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
