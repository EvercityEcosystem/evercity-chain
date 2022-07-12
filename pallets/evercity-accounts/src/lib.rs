#![cfg_attr(not(feature = "std"), no_std)]

pub mod accounts;
#[cfg(test)]
pub mod mock;
#[cfg(test)]    
pub mod tests;

use accounts::*;
type Timestamp<T> = pallet_timestamp::Pallet<T>;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
    { }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// <pre>
        /// Method: set_master()
        /// Arguments: origin: AccountId - transaction caller
        /// Access: anyone
        ///
        /// Set caller master role if master not set in chain_spec. Can be set only once.
        /// </pre>
        #[pallet::weight(T::DbWeight::get().reads_writes(2, 1) + 10_000)]
        pub fn set_master(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            Fuse::<T>::try_mutate(|fuse|->DispatchResultWithPostInfo{
                if *fuse {
                    Err( Error::<T>::InvalidAction.into() )
                }else{
                    AccountRegistry::<T>::insert(&caller, AccountStruct::new(MASTER_ROLE_MASK, 0, Timestamp::<T>::get()));
                    *fuse = true;
                    Ok(().into())
                }
            })
        }
        
        /// <pre>
        /// Method: account_add_with_role_and_data(origin, who: T::AccountId, role: RoleMask, identity: u64)
        /// Arguments:  origin: AccountId - transaction caller
        ///             who: AccountId - id of account to add to accounts registry of platform
        ///             role: RoleMask - role(s) of account (see ALL_ROLES_MASK for allowed roles)
        ///             identity: u64 - reserved field for integration with external platforms
        /// Access: Master role
        ///
        /// Adds new account with given role(s). Roles are set as bitmask. Contains parameter
        /// "identity", planned to use in the future to connect accounts with external services like
        /// KYC providers
        /// </pre>
        #[pallet::weight(T::DbWeight::get().reads_writes(2, 1) + 10_000)]
        pub fn account_add_with_role_and_data(
            origin: OriginFor<T>,
            who: T::AccountId, 
            role: RoleMask, 
            #[pallet::compact] identity: u64
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(!AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountToAddAlreadyExists);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);

            AccountRegistry::<T>::insert(who.clone(), AccountStruct::new(role, identity, Timestamp::<T>::get()));
            Self::deposit_event(Event::<T>::AccountAdd(caller, who, role, identity));
            Ok(().into())
        }

        /// <pre>
        /// Method: account_set_with_role_and_data(origin, who: T::AccountId, role: RoleMask)
        /// Arguments:  origin: AccountId - transaction caller
        ///             who: AccountId - account to modify
        ///             role: RoleMask - role(s) of account (see ALL_ROLES_MASK for allowed roles)
        /// Access: Master role
        ///
        /// Modifies existing account, assigning new role(s) to it
        /// </pre>
        #[pallet::weight(T::DbWeight::get().reads_writes(2, 1) + 10_000)]
        pub fn account_set_with_role_and_data(
            origin: OriginFor<T>, 
            who: T::AccountId, 
            role: RoleMask
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);

            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles |= role;
            });
            Self::deposit_event(Event::<T>::AccountSet(caller, who, role));
            Ok(().into())
        }

        /// <pre>
        /// Method: add_master_role(origin, who: T::AccountId)
        /// Arguments:  origin: AccountId - transaction caller
        ///             who: AccountId - account to modify
        /// Access: Master role
        ///
        /// Modifies existing account, assigning MASTER role(s) to it
        /// </pre>
        #[pallet::weight(T::DbWeight::get().reads_writes(2, 1) + 10_000)]
        pub fn add_master_role(
            origin: OriginFor<T>,
            who: T::AccountId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(!Self::account_is_master(&who), Error::<T>::InvalidAction);

            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles |= MASTER_ROLE_MASK;
            });
            Self::deposit_event(Event::<T>::MasterSet(caller, who));
            Ok(().into())
        }

        /// <pre>
        /// Method: account_withdraw_role(origin, who: T::AccountId, role: RoleMask)
        /// Arguments:  origin: AccountId - transaction caller
        ///             who: AccountId - account to modify
        ///             role: RoleMask - role(s) of account (see ALL_ROLES_MASK for allowed roles)
        /// Access: Master role
        ///
        /// Modifies existing account, removing role(s) from it
        /// </pre>
        #[pallet::weight(T::DbWeight::get().reads_writes(2, 1) + 10_000)]
        pub fn account_withdraw_role(
            origin: OriginFor<T>,
            who: T::AccountId, 
            role: RoleMask
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);
            ensure!(is_roles_correct(role), Error::<T>::AccountRoleParamIncorrect);
            ensure!(!is_roles_mask_included(role, MASTER_ROLE_MASK), Error::<T>::AccountRoleMasterIncluded);
            AccountRegistry::<T>::mutate(who.clone(),|acc|{
                acc.roles ^= role;
            });
            Self::deposit_event(Event::<T>::AccountWithdraw(caller, who, role));
            Ok(().into())
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
        #[pallet::weight(T::DbWeight::get().reads_writes(4, 1) + 10_000)]
        pub fn account_disable(
            origin: OriginFor<T>,
            who: T::AccountId
        ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(Self::account_is_master(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(caller != who, Error::<T>::InvalidAction);
            ensure!(AccountRegistry::<T>::contains_key(&who), Error::<T>::AccountNotExist);

            AccountRegistry::<T>::mutate(&who,|acc|{
                acc.roles = 0; // set no roles
            });

            Self::deposit_event(Event::<T>::AccountDisable(caller, who));
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// \[master, account, role, identity\]
        AccountAdd(T::AccountId, T::AccountId, RoleMask, u64),

        /// \[master, account, role\]
        AccountSet(T::AccountId, T::AccountId, RoleMask),

        /// \[master, account, role\]
        AccountWithdraw(T::AccountId, T::AccountId, RoleMask),

        /// \[master, account\]
        MasterSet(T::AccountId, T::AccountId),
        /// \[master, account\]
        AccountDisable(T::AccountId, T::AccountId),
    }

    /// Old name generated by `decl_event`.
    #[deprecated(note="use `Event` instead")]
    pub type RawEvent<T> = Event<T>;

    #[pallet::error]
    pub enum Error<T> {
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

    #[pallet::storage]
    #[pallet::getter(fn fuse)]
    pub(super) type Fuse<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Storage map for accounts, their roles and corresponding info
    #[pallet::storage]
    #[pallet::getter(fn account_registry)]
    pub(super) type AccountRegistry<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        T::AccountId, 
        EvercityAccountStructOf<T>, 
        ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
            // Storage map for accounts, their roles and corresponding info
            pub genesis_account_registry: Vec<(T::AccountId, RoleMask, u64)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                genesis_account_registry: Default::default(),
            }
        }
    }


    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) { 
            let builder : fn(&Self) -> _ = | config |!
            config.genesis_account_registry.is_empty(); 
            let data = &builder(self);
            let v : &bool = data;
            <Fuse<T> as frame_support::storage::StorageValue<bool>>::put::<&bool>(v);
             
            let data = &self.genesis_account_registry; 
            let data : &frame_support::sp_std::vec::Vec<(T::AccountId, RoleMask, u64)> = data;
            data.iter().for_each(|(k, roles, identity) |{
                let acc = EvercityAccountStructOf::<T> { roles: *roles, identity: *identity, create_time: Default::default() };
                <AccountRegistry<T,> as frame_support::storage::StorageMap<T
                ::AccountId, EvercityAccountStructOf<T>>>::insert::<&T::
                    AccountId, &EvercityAccountStructOf<T>>(k, &acc);
            });
        }
    }
}

impl<T: Config> Pallet<T> {

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
    /// Method: account_is_bond_arranger(acc: &T::AccountId) -> bool
    /// Arguments: acc: AccountId - account id to check
    ///
    /// Checks if the acc has global Bond Arranger role (BOND_ARRANGER_ROLE_MASK) 
    /// </pre>
    pub fn account_is_bond_arranger(acc: &T::AccountId) -> bool {
        AccountRegistry::<T>::get(acc).roles & BOND_ARRANGER_ROLE_MASK != 0
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