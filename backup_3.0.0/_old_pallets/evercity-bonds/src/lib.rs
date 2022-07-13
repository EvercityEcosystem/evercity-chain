//! <div>
//! Crate "pallet-evercity" implements functions for processing
//! the lifecycle of "green bonds", that copy financial mechanics
//! form traditional bonds, but allowing the dynamical change of
//! bond's interest rate, depending of "good/bad" reports from
//! green project, that this bond represents.
//!
//! Highlevel description of protocol:
//!
//! Bond - is the main entity of all protocol. Bond owns a pack of Bond Units, that
//! can be bought by Investors (by spending their EverUSD tokens). Bond Units later
//! can be used by Investors to claim corresponding coupon yield from bond later or be sold to
//! other Investors.
//! Single bond is the finite automation, passing states: [PREPARE]<->[BOOKING]->[ACTIVE or BANKRUPT]->[FINISHED].
//! In PREPARE state Issuer and Managers prepare all Bond parameters: official documents,
//! configure coupon yield conditions, periods, etc.
//! In BOOKING state all financial paramters are fixed, and Investors can buy packs of Bond Units
//! (or return them back at any time). If some minimal amount of Bond Units was bought, bond
//! becomes ACTIVE.
//! In ACTIVE state Bond Units can be bought and sold by Investors on free market for EverUSD.
//! Also, periodically sending EverUSD to bond's balance, and these EverUSDs can be claimed by
//! Investors, that own them.
//! BANKRUPT state is the same state that ACTIVE, but bond don't have enough EverUSD to pay all its
//! debts, in this case Investors can claim only avalable funds, depending of owned Bond Units.
//! In FINISHED state bond cosidered closed, no operations with Bond Units are possible, Investors
//! can claim all coupon yield and principal debt of bond form its balance.
//!
//! Example scenario:

//! More cases and information are presented in Evercity project materials.
//! Here we describe only example scenario with function calls and their meaning.
//!
//!  - Issuer creates bond: <i>bond_add_new(BondId, BondInnerStructOf<T>)</i>, initial bond status is PREPARE
//!  - Bond in PREPARE state:
//!    - Bond Arranger assigns Manager to help Issuer to configure Bond: <i>bond_add_new(BondId, BondInnerStructOf<T>)</i>
//!    - Manager or Issuer modifies Bond<i>bond_update(BondId, u64, BondInnerStructOf<T>)</i>
//!    - "bond_update()" can be called multiple times, allowing many updates of bond structure,
//!      until all business requirements are met
//!    - Bond Arranger confirms that Bond is correct, moves Bond to go to BOOKING state: <i>bond_release(BondId, u64)</i>
//!
//!  - Bond in BOOKING state
//!    - Investors see Bond on platform, and each buys some amount Bond Units: (many
//!      calls) <i>bond_unit_package_buy(BondId, u64, BondUnitAmount)</i>
//!    - Some of Investors refuse, returning their bought BondUnitsPackage-s: (many calls)
//!      <i>bond_unit_package_return(BondId, BondUnitAmount)</i>
//!    - If Investors have bought NOT enough BondUnits until "mincap_deadline", Bond Arranger, Issuer or Manager withdraws it back
//!      to PREPARE state: <i>bond_withdraw(BondId)</i>. Bond cannot be "canceled" until deadline.
//!      All pre-bought Bond Units can be returned by Investors
//!    - If Investors bought enough BondUnits until deadline, Bond Arranger moves the Bond to ACTIVE state:
//!      <i>bond_activate(BondId, u64)</i>
//!    - Date, when bond becomes ACTIVE (BOOKING->ACTIVE) - is a bond start time. All next periods
//!      will be calculated using this time as start moment
//!      - During activation Bond transfers all EverUSD, received from Investors to bond Issuer
//!      - During activation the structure, holding info about each payment_period is created:
//!        <i>[(start_period_data), (period_1_data), (period_2_data), ..., (period_N_data)]</i>.
//!        This structure(array, fixed size) will hold accumulated coupon yield values, confirmed
//!        impact data, recieved for given period, etc)
//!    - If deadline is passed, but "mincap" was not reached, bond returns to PREPARE state when
//!      Bond Arranger, Issuer or Manager calls <i>bond_withdraw(BondId)</i>
//!      - \[TODO\] "bond_withdraw" logic can be changed, we're refactoring its logic currently
//!  - Bond in ACTIVE state
//!    - passes start_period, while interest_rate is fixed and Issuer constructs project in real world
//!    - first payment period is coming, Issuer prepares report data about project and sum in EverUSD to pay to Investors
//!    - It's the end of period, now it's time to send impact_data, it will affect NEXT payment period
//!    - Issuer sends impact_data to bond: <i>bond_impact_report_send(BondId, BondPeriodNumber,
//!      u64)</i>
//!    - Auditor confirms data, sent by Issuer: <i>bond_impact_report_approve(BondId,
//!      BondPeriodNumber, u64)</i>
//!    - Confirmed impact data will later result in change of interest rate for NEXT period
//!    - First payment_period begins (start period passed). Confirmed impact_data in previous
//!      period results in calculation of current period coupon yield.
//!    - the beginning of payment period is the time for Issuer to pay coupon interest rate to
//!      Investors. Effective coupoun yield rate is calculated, using impcat_data of previous
//!      period. Any operation with bond units and EverUSD balance will update "bond_credit",
//!      representing overall accrued bond's debt to Investors
//!    - Issuer sends EverUSD in bond using: <i>bond_deposit_everusd(BondId, EverUSDBalance)</i>
//!      - All EverUSD, sent by Issuer are placed on bond's balance, added to bond_debit, then,
//!        function <i>calc_and_store_bond_coupon_yield(...)</i> increases "bond_credit",
//!        summarizing coupon yields for all previous periods. Later, difference between "bond_debit"
//!        and "bond_credit" is used to calculate bond state (ACTIVE or BANKRUPT) and to calculate
//!        amount ov EverUSD, that Investors can withdraw from bond at the current moment of time
//!    - After all calculations, bond saves coupin rates and correct amounts of EverUSD that must be paid
//!      to Investors for each of already passed payment_periods
//!    - each Investor calls <i>bond_withdraw_everusd(BondId, EverUSDBalance)</i>,
//!    requesting coupon yield from bond.
//!    - If there is not enough EverUSDfrom Issuer, any function working with debit/credit or
//!    operating with Bond Units moves bond to BANKRUPT state
//!  - Bond in BANKRUPT state
//!    - bond still accept <i>bond_deposit_everusd(BondId, EverUSDBalance)</i> from Issuer,
//!      increasing bond_debit
//!    - in BANKRUPT state Investors can withdraw only part of coupon yield, corresponding
//!      to amount of Bond Units they own
//!  - Bond in ACTIVE state(finishing period)
//!    - after all payment_period passed maturity period begins. It's time form Issuer to pay
//!      full bond debt back to Investors
//!    - Investor accumulates needed amount of EverUSD on his address and calls
//!      <i>bond_redeem(BondId)</i> function.
//!      - function "bond_redeem" recalculates all yields per each period, summarizes them with
//!      full maturity debt and transfers needed amount of EverUSD from Issuer's balance
//!        - if Issuer already deposited a lot of EverUSD on bond's balance, "bond_redeem()" will
//!          pay a "charge" to Issuer, instead of withdrawing EverUSD from his balance
//!        - if there is not enough EverUSD on Issuer's balance - "bond_redeem()" fails with
//!          "unsufficient balance" error
//!      - if there is enough of EverUSD to pay all debts, all coupn yields are recalculated,
//!        actual "bond_debit" and "bond_credit" are stored, and function moves bond to FINISHED
//!        state
//!   - Bond in FINISHED state
//!      - move to FINISHED state is possible ONLY if bond's debt was fully covered by EverUSDs,
//!        taken from bond Issuer's balance
//!        - if Issuer don't have enough EverUSDs on his balance to pay pricipal debt, his call to
//!          <i>bond_redeem(BondId)</i> will be unsuccessful, and bond will be in BANKRUPT state
//!      - full debt of Issuer is taken from his balance and added to "bond_debit"
//!      - Investors can call <i>bond_withdraw_everusd(BondId, EverUSDBalance)</i> to receive
//!        all their EverUSDs, including total accrued coupon yield and principal debt
//! </div>

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::too_many_arguments)]
#![recursion_limit = "256"]

use pallet_evercity_accounts as accounts;

use crate::bond::{
    AccountYield, BondInnerStructOf, BondPeriodNumber, BondState,
    BondUnitAmount, BondUnitSaleLotStructOf, OnAddBond,

};
pub use crate::bond::{
    BondId, BondImpactReportStruct, BondPeriod, BondStruct, BondStructOf, BondUnitPackage,
    DEFAULT_DAY_DURATION, EverUSDBalance, Expired, MIN_BOND_DURATION,
};

pub use default_weight::WeightInfo;
use frame_support::debug::native;
use frame_support::{
    dispatch::Vec,
    dispatch::{DispatchResult,},
    sp_std::cmp::{min,},
    sp_std::result::Result,
};
pub use period::{PeriodDataStruct, PeriodYield};

type Timestamp<T> = pallet_timestamp::Module<T>;

/// EverUSD = USD * ( 10 ^ EVERUSD_DECIMALS )
pub const EVERUSD_DECIMALS: u64 = 9;
/// Bank's year in days
const INTEREST_RATE_YEAR: u64 = 365;
/// Gas limit settings for purge mint/burn requests
const MAX_PURGE_REQUESTS: usize = 100;


/// Evercity project types
/// All these types must be put in CUSTOM_TYPES part of config for polkadot.js
/// to be correctly presented in DApp
pub mod bond;
mod default_weight;
#[cfg(test)]
pub mod ledger;

pub mod token;
pub mod period;
pub mod runtime_api;
#[cfg(test)]
mod tests;

use token::*;

macro_rules! ensure_active {
    ($f:expr, $err:expr) => {
        match ($f) {
            Some(v) => v,
            None => {
                return $err.into();
            }
        }
    };
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config
            + pallet_timestamp::Config
            + pallet_evercity_accounts::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        #[pallet::constant]
        type BurnRequestTtl: Get<u32>;
        #[pallet::constant]
        type MintRequestTtl: Get<u32>;
        #[pallet::constant]
        type MaxMintAmount: Get<EverUSDBalance>;
        #[pallet::constant]
        type TimeStep: Get<BondPeriod>;
        type WeightInfo: WeightInfo;
        type OnAddBond: OnAddBond<Self::AccountId, Self::Moment, Self::Hash>;    
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {  }



    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// \[account, everusd\]
        MintRequestCreated(T::AccountId, EverUSDBalance),
        /// \[account, everusd\]
        MintRequestRevoked(T::AccountId, EverUSDBalance),
        /// \[custodian, account, everusd\]
        MintRequestConfirmed(T::AccountId, T::AccountId, EverUSDBalance),
        /// \[custodian, account, everusd\]
        MintRequestDeclined(T::AccountId, T::AccountId, EverUSDBalance),
        /// \[account, everusd\]
        BurnRequestCreated(T::AccountId, EverUSDBalance),
        /// \[account, everusd\]
        BurnRequestRevoked(T::AccountId, EverUSDBalance),
        /// \[custodian, account, everusd\]
        BurnRequestConfirmed(T::AccountId, T::AccountId, EverUSDBalance),
        /// \[custodian,account, everusd\]
        BurnRequestDeclined(T::AccountId, T::AccountId, EverUSDBalance),
        // Bond events
        /// \[issuer,bond\]
        BondAdded(T::AccountId, BondId),
        /// \[sender,bond\]
        BondChanged(T::AccountId, BondId),
        /// \[issuer,bond\]
        BondRevoked(T::AccountId, BondId),
        /// \[sender,bond\]
        BondReleased(T::AccountId, BondId),
        /// \[sender,bond,bondfund\]
        BondActivated(T::AccountId, BondId, EverUSDBalance),
        /// \[issuer,bond\]
        BondWithdrawal(T::AccountId, BondId),
        /// \[issuer,bond,bondfund\]
        BondRedeemed(T::AccountId, BondId, EverUSDBalance),
        /// \[sender,bond,credit,debit\]
        BondBankrupted(T::AccountId, BondId, EverUSDBalance, EverUSDBalance),
        /// \[sender,bond,everusd\]
        BondWithdrawEverUSD(T::AccountId, BondId, EverUSDBalance),
        /// \[issuer,bond,everusd\]
        BondDepositEverUSD(T::AccountId, BondId, EverUSDBalance),
        /// \[bondholder,bond,units,everusd\]
        BondUnitSold(T::AccountId, BondId, u32, EverUSDBalance),
        /// \[bondholder,bond,units,everusd\]
        BondUnitReturned(T::AccountId, BondId, u32, EverUSDBalance),
        /// \[issuer,bond,period,impact_data\]
        BondImpactReportSent(T::AccountId, BondId, BondPeriodNumber, u64),
        /// \[auditor,bond,period,impact_data\]
        BondImpactReportApproved(T::AccountId, BondId, BondPeriodNumber, u64),
        /// \[bond,everusd\]
        BondCouponYield(BondId, EverUSDBalance),
        /// \[bondholder, bond, lot\]
        BondSaleLotBid(T::AccountId, BondId, BondUnitSaleLotStructOf::<T>),
        /// \[from, to, bond, lot\]
        BondSaleLotSettle(T::AccountId, T::AccountId, BondId, BondUnitSaleLotStructOf::<T>),
    }

    /// Old name generated by `decl_event`.
    #[deprecated(note="use `Event` instead")]
    pub type RawEvent<T> = Event<T>;

    #[pallet::error]
    pub enum Error<T> {
        /// Potentially dangerous action
        InvalidAction,
        /// Account tried to use more EverUSD  than was available on the balance
        BalanceOverdraft,
        /// Account was already added and present in AccountRegistry
        AccountToAddAlreadyExists,
        /// Account not authorized(doesn't have a needed role, or doesnt present in AccountRegistry at all)
        AccountNotAuthorized,
        /// Account does not exist in AccountRegistry
        AccountNotExist,
        /// Role parameter is invalid (bit mask of available roles includes non-existent role)
        AccountRoleParamIncorrect,
        /// Account already created one mint request, only one allowed at a time(to be changed in future)
        MintRequestAlreadyExist,
        /// Mint request for given account doesnt exist
        MintRequestDoesntExist,
        /// Incorrect parameters for mint request(miant amount > MAX_MINT_AMOUNT)
        MintRequestParamIncorrect,
        /// Account already created one burn request, only one allowed at a time(to be changed in future)
        BurnRequestAlreadyExist,
        /// Mint request for given account doesnt exist
        BurnRequestDoesntExist,
        /// Incorrect parameters for mint request(mint amount > MAX_MINT_AMOUNT)
        BurnRequestParamIncorrect,
        /// Burn request exists but outdated
        BurnRequestObsolete,
        /// Mint request exists but outdated
        MintRequestObsolete,
        /// Bond with same ticker already exists
        /// Every bond on the platform has unique BondId: 8 bytes, like "MUSKPWR1" or "SOLGEN02"
        BondAlreadyExists,
        /// Incorrect bond parameters (many different cases)
        BondParamIncorrect,
        /// Incorrect bond ticker provided or bond has been revoked
        BondNotFound,
        /// Requested action in bond is not permitted for this account
        BondAccessDenied,
        /// Current bond state doesn't permit the requested action
        BondStateNotPermitAction,
        /// Action requires some bond options to be properly initialized
        BondIsNotConfigured,
        /// Requested action is not allowed in current period of time
        BondOutOfOrder,
        /// Bond version is outdated
        BondNonceObsolete,
        /// Bid lot not found
        LotNotFound,
        /// Bid lot expired
        LotObsolete,
        /// Incorrect parameter for the bond sale lot
        LotParamIncorrect,
    }


    /// Total supply of EverUSD token. Sum of all token balances in system
    #[pallet::storage]
    #[pallet::getter(fn total_supply_everusd)]
    pub(super) type TotalSupplyEverUSD<T: Config> = StorageValue<_, EverUSDBalance, ValueQuery>;

    /// Storage map for EverUSD token balances
    #[pallet::storage]
    #[pallet::getter(fn balances_everusd)]
    pub(super) type BalanceEverUSD<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, EverUSDBalance, ValueQuery>;

    /// Storage map for EverUSD token mint requests (see TokenMintRequestStruct)
    #[pallet::storage]
    #[pallet::getter(fn mint_request_everusd)]
    pub(super) type MintRequestEverUSD<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, TokenMintRequestStructOf<T>, ValueQuery>;     

    /// Storage map for EverUSD token burn requests (see TokenBurnRequestStruct)
    #[pallet::storage]
    #[pallet::getter(fn burn_request_everusd)]
    pub(super) type BurnRequestEverUSD<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, TokenBurnRequestStructOf<T>, ValueQuery>;     

    /// Structure for storing all platform bonds.
    /// BondId is now a ticker [u8; 8]: 8-bytes unique identifier like "MUSKPWR1" or "WINDGEN2"
    #[pallet::storage]
    #[pallet::getter(fn bond_registry)]
    pub(super) type BondRegistry<T: Config> = StorageMap<_, Blake2_128Concat, BondId, BondStructOf<T>, ValueQuery>;

    /// Investor's Bond units (packs of bond_units, received at the same time, belonging to Investor)
    #[pallet::storage]
    #[pallet::getter(fn bond_unit_registry)]
    pub(super) type BondUnitPackageRegistry<T: Config> = StorageDoubleMap<_, Blake2_128Concat, BondId, Blake2_128Concat, T::AccountId, Vec<BondUnitPackage>, ValueQuery>;

    /// Bond coupon yield storage
    /// Every element has total bond yield of passed period recorded on accrual basis
    #[pallet::storage]
    #[pallet::getter(fn bond_coupon_yield)]
    pub(super) type BondCouponYield<T: Config> = StorageMap<_, Blake2_128Concat, BondId, Vec<PeriodYield>, ValueQuery>;

    /// Bondholder's last requested coupon yield for given period and bond
    #[pallet::storage]
    #[pallet::getter(fn bond_last_coupon_yield)]
    pub(super) type BondLastCouponYield<T: Config> = StorageDoubleMap<_, Blake2_128Concat, BondId, Blake2_128Concat, T::AccountId, AccountYield, ValueQuery>;

    /// Bond sale lots for each bond
    #[pallet::storage]
    #[pallet::getter(fn bond_unit_lots)]
    pub(super) type BondUnitPackageLot<T: Config> = StorageDoubleMap<_, Blake2_128Concat, BondId, Blake2_128Concat, T::AccountId, Vec<BondUnitSaleLotStructOf<T>>, ValueQuery>;

    /// Bond impact report storage
    #[pallet::storage]
    #[pallet::getter(fn impact_reports)]
    pub(super) type BondImpactReport<T: Config> = StorageMap<_, Blake2_128Concat, BondId, Vec<BondImpactReportStruct>, ValueQuery>;


    #[pallet::call]
    impl<T: Config> Pallet<T>
            // TODO_MAYBE_WHERE_CLAUSE
    {
        // Token balances manipulation functions

        /// <pre>
        /// Method: token_mint_request_create_everusd(origin: OriginFor<T>, amount_to_mint: EverUSDBalance)
        /// Arguments:  origin: T::AccountId - transaction caller
        ///             amount_to_mint: EverUSDBalance - amount of tokens to mint
        /// Access: Investor or Issuer role
        ///
        /// Creates a request to mint given amount of EverUSD tokens on caller's balance.
        /// Custodian account confirms request after receiving payment in USD from target account's owner
        /// It's possible to create only one request per account. Mint request has a time-to-live
        /// and becomes invalidated after it.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_mint_request_create_everusd())]
        pub fn token_mint_request_create_everusd(origin: OriginFor<T>, #[pallet::compact] amount_to_mint: EverUSDBalance) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_token_mint_burn_allowed(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(amount_to_mint <= T::MaxMintAmount::get(), Error::<T>::MintRequestParamIncorrect);

            MintRequestEverUSD::<T>::try_mutate(&caller, |request|->DispatchResult{
                let now = Timestamp::<T>::get();
                if !request.is_expired(now) {
                    Err( Error::<T>::MintRequestAlreadyExist.into() )
                }else{
                    *request = TokenMintRequestStruct{
                        amount: amount_to_mint,
                        deadline: now + T::MintRequestTtl::get().into(),
                    };
                    Self::deposit_event(Event::<T>::MintRequestCreated(caller.clone(), amount_to_mint));
                    Ok(())
                }
            })?;
            Ok(().into())
        }

        /// <pre>
        /// Method: token_mint_request_revoke_everusd(origin)
        /// Arguments: origin: T::AccountId - transaction caller
        /// Access: Investor or Issuer role
        ///
        /// Revokes and deletes currently existing mint request, created by caller's account
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_mint_request_revoke_everusd())]
        pub fn token_mint_request_revoke_everusd(origin: OriginFor<T>,) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(MintRequestEverUSD::<T>::contains_key(&caller), Error::<T>::MintRequestDoesntExist);
            let _amount = MintRequestEverUSD::<T>::get(&caller).amount;
            MintRequestEverUSD::<T>::remove(&caller);
            Self::deposit_event(Event::<T>::MintRequestRevoked(caller, _amount));
            Ok(().into())
        }

        /// <pre>
        /// Method: token_mint_request_confirm_everusd(origin: OriginFor<T>, who: T::AccountId, amount: EverUSDBalance)
        /// Arguments:  origin: T::AccountId - transaction caller
        ///             who: T::AccountId - target account
        ///             amount: EverUSDBalance - amount of tokens to mint, confirmed by Custodian
        /// Access: Custodian role
        ///
        /// Confirms the mint request of account, creating "amount" of tokens on its balance.
        /// (note) Amount of tokens is sent as parameter to avoid data race problem, when
        /// Custodian can confirm unwanted amount of tokens, because attacker is modified mint request
        /// while Custodian makes a decision
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_mint_request_confirm_everusd())]
        pub fn token_mint_request_confirm_everusd(origin: OriginFor<T>, who: T::AccountId, #[pallet::compact] amount: EverUSDBalance) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_custodian(&caller),Error::<T>::AccountNotAuthorized);
            ensure!(MintRequestEverUSD::<T>::contains_key(&who), Error::<T>::MintRequestDoesntExist);
            let mint_request = MintRequestEverUSD::<T>::get(&who);
            let now = Timestamp::<T>::get();
            ensure!(!mint_request.is_expired(now), Error::<T>::MintRequestObsolete);

            // add tokens to user's balance and total supply of EverUSD
            let amount_to_add = mint_request.amount;
            ensure!(amount_to_add==amount,Error::<T>::MintRequestParamIncorrect );

            Self::balance_add(&who, amount_to_add)?;

            TotalSupplyEverUSD::<T>::try_mutate(|total|->DispatchResult{
                *total = total.checked_add(amount_to_add).ok_or( Error::<T>::BalanceOverdraft )?;
                Ok(().into())
            })?;

            MintRequestEverUSD::<T>::remove(&who);
            Self::deposit_event(Event::<T>::MintRequestConfirmed(caller, who, amount_to_add));
            Self::purge_expired_mint_requests(now);
            Ok(().into())
        }

        /// <pre>
        /// Method: token_mint_request_decline_everusd(origin: OriginFor<T>, who: T::AccountId)
        /// Arguments:  origin: T::AccountId - transaction caller
        ///             who: T::AccountId - target account
        /// Access: Custodian role
        ///
        /// Declines and deletes the mint request of account (Custodian)
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_mint_request_decline_everusd())]
        pub fn token_mint_request_decline_everusd(origin: OriginFor<T>, who: T::AccountId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_custodian(&caller),Error::<T>::AccountNotAuthorized);
            ensure!(MintRequestEverUSD::<T>::contains_key(&who), Error::<T>::MintRequestDoesntExist);
            let amount = MintRequestEverUSD::<T>::get(&who).amount;
            MintRequestEverUSD::<T>::remove(&who);
            Self::deposit_event(Event::<T>::MintRequestDeclined(caller, who, amount));
            Ok(().into())
        }

        /// <pre>
        /// Method: token_burn_request_create_everusd(origin: OriginFor<T>, amount_to_burn: EverUSDBalance)
        /// Arguments:  origin: T::AccountId - transaction caller
        ///             amount_to_burn: EverUSDBalance - amount of tokens to burn
        /// Access: Investor or Issuer role
        ///
        /// Creates a request to burn given amount of EverUSD tokens on caller's balance.
        /// Custodian account confirms request after sending payment in USD to target account's owner
        /// It's possible to create only one request per account. Burn request has a time-to-live
        /// and becomes invalidated after it.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_burn_request_create_everusd())]
        pub fn token_burn_request_create_everusd(origin: OriginFor<T>, #[pallet::compact]  amount_to_burn: EverUSDBalance) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_token_mint_burn_allowed(&caller), Error::<T>::AccountNotAuthorized);

            let current_balance = BalanceEverUSD::<T>::get(&caller);
            ensure!(amount_to_burn <= current_balance, Error::<T>::BalanceOverdraft);

            BurnRequestEverUSD::<T>::try_mutate(&caller,|request|->DispatchResult{
                let now = Timestamp::<T>::get();
                if !request.is_expired( now ) {
                    Err( Error::<T>::BurnRequestAlreadyExist.into() )
                }else{
                    *request = TokenBurnRequestStruct {
                        amount: amount_to_burn,
                        deadline: now +  T::BurnRequestTtl::get().into(),
                    };
                    Self::deposit_event(Event::<T>::BurnRequestCreated(caller.clone(), amount_to_burn));
                    Ok(())
                }
            })?;
            Ok(().into())
        }

        /// <pre>
        /// Method: token_burn_request_revoke_everusd(origin)
        /// Arguments: origin: T::AccountId - transaction caller
        /// Access: Investor or Issuer role
        ///
        /// Revokes and deletes currently existing burn request, created by caller's account
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_burn_request_revoke_everusd())]
        pub fn token_burn_request_revoke_everusd(origin: OriginFor<T>,) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(BurnRequestEverUSD::<T>::contains_key(&caller), Error::<T>::BurnRequestDoesntExist);
            let amount = BurnRequestEverUSD::<T>::get(&caller).amount;
            BurnRequestEverUSD::<T>::remove(&caller);
            Self::deposit_event(Event::<T>::BurnRequestRevoked(caller, amount));
            Ok(().into())
        }

        /// <pre>
        /// Method: token_burn_request_confirm_everusd(origin: OriginFor<T>, who: T::AccountId, amount: EverUSDBalance)
        /// Arguments:  origin: T::AccountId - transaction caller
        ///             who: T::AccountId - target account
        ///             amount: EverUSDBalance - amount of tokens to mint, confirmed by Custodian
        /// Access: Custodian role
        ///
        /// Confirms the burn request of account, destroying "amount" of tokens on its balance.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_burn_request_confirm_everusd())]
        pub fn token_burn_request_confirm_everusd(origin: OriginFor<T>, who: T::AccountId, #[pallet::compact]  amount: EverUSDBalance) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_custodian(&caller),Error::<T>::AccountNotAuthorized);
            ensure!(BurnRequestEverUSD::<T>::contains_key(&who), Error::<T>::BurnRequestDoesntExist);
            let burn_request = BurnRequestEverUSD::<T>::get(&who);
            let now = Timestamp::<T>::get();
            ensure!(!burn_request.is_expired(now), Error::<T>::BurnRequestObsolete);
            // remove tokens from user's balance and decrease total supply of EverUSD
            let amount_to_sub = burn_request.amount;
            // prevent unacceptable commit
            ensure!(amount_to_sub==amount, Error::<T>::MintRequestParamIncorrect );

            Self::balance_sub(&who, amount_to_sub)?;
            TotalSupplyEverUSD::<T>::mutate(|total|{
                *total-=amount_to_sub;
            });

            BurnRequestEverUSD::<T>::remove(&who);
            Self::deposit_event(Event::<T>::BurnRequestConfirmed(caller, who, amount_to_sub));
            Self::purge_expired_burn_requests(now);
            Ok(().into())
        }

        /// <pre>
        /// Method: token_burn_request_decline_everusd(origin: OriginFor<T>, who: T::AccountId)
        /// Arguments:  origin: T::AccountId - transaction caller
        ///             who: T::AccountId - target account
        /// Access: Custodian role
        ///
        /// Declines and deletes the burn request of account (Custodian)
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::token_burn_request_decline_everusd())]
        pub fn token_burn_request_decline_everusd(origin: OriginFor<T>, who: T::AccountId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_custodian(&caller),Error::<T>::AccountNotAuthorized);
            ensure!(BurnRequestEverUSD::<T>::contains_key(&who), Error::<T>::BurnRequestDoesntExist);
            let amount = BurnRequestEverUSD::<T>::get(&who).amount;
            BurnRequestEverUSD::<T>::remove(&who);
            Self::deposit_event(Event::<T>::BurnRequestDeclined(caller, who, amount));
            Ok(().into())
        }

        // Bonds handling functions

        /// <pre>
        /// Method: bond_add_new(origin: OriginFor<T>, origin: OriginFor<T>, bond: BondId, body: BondInnerStruct)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            body: BondInnerStruct
        /// Access: Issuer role
        ///
        /// Creates new bond with given BondId (8 bytes) and pack of parameters, set by BondInnerStruct.
        /// Bond is created in BondState::PREPARE, and can be modified many times until it becomes ready
        /// for next BondState::BOOKING (when most of BondInnerStruct parameters cannot be changed, and
        /// Investors can buy bond units). There are many checks inside (see function bond.is_valid()).
        /// All time periods must be a multiple of "time_step"(which is equal to DEFAULT_DAY_DURATION),
        /// none of periods cannot be "out of bounds", amount of "impact_baselines" must be equal the
        /// amount of "payment_periods", all min-max deviations must be correct
        /// (max_deviation > baseline, min_deviation < baseline), etc...
        /// If all checks were passed, bond object is created in BondRegistry, receives state "PREPARE"
        /// and awaits when account with Bond Arranger role allows it to be moved to state BOOKING
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_add_new())]
        pub fn bond_add_new(origin: OriginFor<T>, bond: BondId, body: BondInnerStructOf<T> ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_issuer(&caller),Error::<T>::AccountNotAuthorized);
            ensure!(body.is_valid(T::TimeStep::get()), Error::<T>::BondParamIncorrect );
            ensure!(!BondRegistry::<T>::contains_key(&bond), Error::<T>::BondAlreadyExists);

            let now = Timestamp::<T>::get();

            let mut item = BondStruct{
                    inner: body,
                    creation_date: now,
                    issuer: caller.clone(),
                    nonce: 0,
                    .. Default::default()
            };
            T::OnAddBond::on_add_bond(&bond, &mut item);
            BondRegistry::<T>::insert(&bond, item);

            Self::deposit_event(Event::<T>::BondAdded(caller, bond));
            Ok(().into())
        }

        /// <pre>
        /// Method: bond_set_manager(origin: OriginFor<T>, bond: BondId, acc: T::AccountId)
        /// Arguments: origin: T::AccountId - transaction caller, assigner
        ///            bond: BondId - bond identifier
        ///            acc: T::AccountId - assignee account
        /// Access: Bond Arranger role
        ///
        /// Assigns target account to be the manager of the bond. Manager can make
        /// almost the same actions with bond as Issuer, instead of most important,
        /// like issuing the bond to market (moving it to states: BOOKING, ACTIVE, etc...).
        /// Manager helps Issuer to manage bond parameters, work with documents. Also, Manager
        /// can return bond back from BOOKING to PREPARE state (f.e. if mincap of sold Bond Units
        /// is not reached)
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_set())]
        pub fn bond_set_manager(origin: OriginFor<T>, bond: BondId, acc: T::AccountId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            // Bond Auxiliary roles can be set only by Bond Arranger role
            ensure!(accounts::Module::<T>::account_is_bond_arranger(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(accounts::Module::<T>::account_is_manager(&acc), Error::<T>::AccountRoleParamIncorrect);

            Self::with_bond(&bond, |item|{
                ensure!(
                    matches!(item.state, BondState::PREPARE),
                    Error::<T>::BondStateNotPermitAction
                );
                item.manager = acc;
                item.nonce += 1;
                Self::deposit_event(Event::<T>::BondChanged(caller, bond));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_set_auditor(origin: OriginFor<T>, bond: BondId, acc: T::AccountId)
        /// Arguments: origin: T::AccountId - transaction caller, assigner
        ///            bond: BondId - bond identifier
        ///            acc: T::AccountId - assignee
        /// Access: Bond Arranger role
        ///
        /// Assigns target account to be the auditor of the bond. Auditor confirms
        /// impact data coming in bond, and performs other verification-related actions.
        /// Assigned account must have a role AUDITOR. impact_data for given bond can be
        /// confirmed only by assigned Auditor.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_set())]
        pub fn bond_set_auditor(origin: OriginFor<T>, bond: BondId, acc: T::AccountId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            // Bond auxiliary roles can be set only by Bond Arranger role
            ensure!(accounts::Module::<T>::account_is_bond_arranger(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(accounts::Module::<T>::account_is_auditor(&acc), Error::<T>::AccountRoleParamIncorrect);

            Self::with_bond(&bond, |item|{
                ensure!(
                    matches!(item.state, BondState::PREPARE | BondState::BOOKING),
                    Error::<T>::BondStateNotPermitAction
                );
                item.auditor = acc;
                item.nonce += 1;
                Self::deposit_event(Event::<T>::BondChanged(caller, bond));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_set_impact_reporter(origin: OriginFor<T>, bond: BondId, acc: T::AccountId)
        /// Arguments: origin: T::AccountId - transaction caller, assigner
        ///            bond: BondId - bond identifier
        ///            acc: T::AccountId - assignee
        /// Access: only accounts with Bond Arranger role
        ///
        /// Assigns an account to be a publisher of impact_data for this bond. Only assigned
        /// by Bond Arranger, target account must have IMPACT_REPORTER role.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_set())]
        pub fn bond_set_impact_reporter(origin: OriginFor<T>, bond: BondId, acc: T::AccountId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            // Bond auxiliary roles can be set only by Bond Arranger role
            ensure!(accounts::Module::<T>::account_is_bond_arranger(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(accounts::Module::<T>::account_is_impact_reporter(&acc), Error::<T>::AccountRoleParamIncorrect);

            Self::with_bond(&bond, |item|{
                item.impact_reporter = acc;
                item.nonce += 1;
                Self::deposit_event(Event::<T>::BondChanged(caller, bond));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_update(origin: OriginFor<T>, origin: OriginFor<T>, bond: BondId, body: BondInnerStruct)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            nonce: u64 - bond nonce
        ///            body: BondInnerStruct
        /// Access: bond Issuer or bond Manager
        ///
        /// Updates bond data. Being released bond can be changed only in part, not related to financial
        /// options(issuer cannot change rates, caps, prices after bon release on market). Only document hashes
        /// can be changed to update information about bond, allowing to re-verify documentary history of bond
        /// during its lifetime.
        /// Function accepts same information as bond_add(), but requires, that only allowed options was
        /// changed (calling "is_financial_options_eq()" with previous version of bond)
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_update())]
        pub fn bond_update(origin: OriginFor<T>, bond: BondId,#[pallet::compact] nonce: u64, body: BondInnerStructOf<T>) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(body.is_valid(T::TimeStep::get()), Error::<T>::BondParamIncorrect );
            // Bond can be update only by Owner or assigned Manager
            Self::with_bond(&bond, |item|{
                ensure!(item.nonce == nonce, Error::<T>::BondNonceObsolete );
                // preserving the bond_units_base_price value
                ensure!(
                    matches!(item.state, BondState::PREPARE | BondState::BOOKING),
                    Error::<T>::BondStateNotPermitAction
                );
                ensure!(
                    item.issuer == caller || item.manager == caller ,
                    Error::<T>::BondAccessDenied
                );
                // Financial data shall not be changed after release
                if item.state == BondState::BOOKING {
                    ensure!( item.inner.is_financial_options_eq(&body), Error::<T>::BondStateNotPermitAction );
                }
                item.inner = body;
                item.nonce += 1;
                Self::deposit_event(Event::<T>::BondChanged(caller, bond));

                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_release(origin: OriginFor<T>, bond: BondId)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            nonce: u64 - bond nonce
        /// Access: only accounts with Bond Arranger role
        ///
        /// Releases the bond on the market starting presale.
        /// Moves bond form PREPARE to BOOKING state, allowing investors to buy
        /// packs of bond units(BU), denying any changes of financial parameters,
        /// and waiting until "mincap" of bond units will be booked. If it happens,
        /// bond can further be move to ACTIVE state, allowing Investors to trade
        /// their Bond Units on free market with other Investors/
        /// Function requires PREPARE state, not expired "mincap_deadline" to be successful.
        /// Other checks are: nonce and "is_valid()", to avoid release of incorrect bond
        /// </pre>

        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_release())]
        pub fn bond_release(origin: OriginFor<T>, bond: BondId, #[pallet::compact]  nonce: u64) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            // Bond can be released only by Bond Arranger role
            ensure!(accounts::Module::<T>::account_is_bond_arranger(&caller), Error::<T>::AccountNotAuthorized);
            Self::with_bond(&bond, |item|{
                ensure!(item.nonce == nonce, Error::<T>::BondNonceObsolete );
                ensure!(item.state == BondState::PREPARE, Error::<T>::BondStateNotPermitAction);
                ensure!(item.inner.is_valid(T::TimeStep::get()), Error::<T>::BondParamIncorrect);

                let now = Timestamp::<T>::get();
                // Ensure booking deadline is in the future
                ensure!(item.inner.mincap_deadline>now, Error::<T>::BondStateNotPermitAction);

                item.booking_start_date = now;
                item.state = BondState::BOOKING;
                item.nonce += 1;
                Self::deposit_event(Event::<T>::BondReleased(caller, bond));
                Ok(().into())
            })
        }


        /// <pre>
        /// Method: bond_unit_package_buy(origin: OriginFor<T>, bond: BondId, unit_amount: BondUnitAmount )
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            nonce: u64 - bond nonce
        ///            unit_amount: BondUnitAmount - amount of bond units
        /// Access: only accounts with Investor role
        ///
        /// Buy bond units from bond in BOOKING, ACTIVE or BANKRUPT state.
        /// Investor passes amount of BUs he wants to buy. Function calculates price of BUs
        /// and adds new BondUnitsPackage, with amount of BUs that was bought. BondUnitsPackage
        /// contains:
        ///  - amount of BUs bought
        ///  - aquisition time(number of period when this BUs package was bought)
        ///  - accrued coupon yield(EverUSD): internal field, calculated and saved only when any
        ///    financial operations with this package are performed
        ///
        /// Bond must be in BOOKING, ACTIVE, BANKRUPT state, amount of Bond Units
        /// should not except "bond_units_maxcap_amount"
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_unit_package_buy())]
        pub fn bond_unit_package_buy(origin: OriginFor<T>, bond: BondId,#[pallet::compact]  nonce: u64,#[pallet::compact] unit_amount: BondUnitAmount ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_investor(&caller), Error::<T>::AccountNotAuthorized);
            Self::with_bond(&bond, |mut item|{
                ensure!(item.nonce == nonce, Error::<T>::BondNonceObsolete);
                ensure!(
                    matches!(item.state, BondState::BANKRUPT | BondState::ACTIVE | BondState::BOOKING),
                    Error::<T>::BondStateNotPermitAction
                );
                // issuer cannot buy his own bonds
                ensure!(item.issuer != caller, Error::<T>::AccountNotAuthorized);

                let issued_amount = unit_amount.checked_add(item.issued_amount)
                    .ok_or(Error::<T>::BalanceOverdraft)?;

                ensure!(
                    issued_amount <= item.inner.bond_units_maxcap_amount,
                    Error::<T>::BondParamIncorrect
                );

                let package_value =  item.par_value( unit_amount ) ;

                Self::balance_sub(&caller, package_value)?;

                let now = Timestamp::<T>::get();

                // get the number of seconds after bond activation.
                // zero value if the bond has not activated yet
                let (acquisition,_) = item.time_passed_after_activation( now ).unwrap_or( (0,0) );
                // @FIXME assess the costs of current array struct for storing packages and
                // compare them with a more efficient way to store data
                BondUnitPackageRegistry::<T>::mutate(&bond, &caller, |packages|{
                    packages.push(
                        BondUnitPackage{
                             bond_units: unit_amount,
                             acquisition,
                             coupon_yield: 0,
                        }
                    );
                });

                item.issued_amount = issued_amount;


                if matches!(item.state, BondState::ACTIVE | BondState::BANKRUPT) {
                    item.bond_debit += package_value;
                    // in BondState::ACTIVE or BondState::BANKRUPT received everusd
                    // can be forwarded to pay off the debt
                    // @TODO add postdispatch weight
                    Self::calc_and_store_bond_coupon_yield(&bond, &mut item, now);
                    // surplus to the issuer balance
                    let free_balance = item.get_free_balance();
                    if free_balance > 0 {
                        item.bond_debit -= free_balance;
                        Self::balance_add(&item.issuer, free_balance)?;
                    }
                }else{
                    // in BondState::PREPARE just increase assets and liabilities of the Bond
                    item.increase( package_value );
                }

                Self::deposit_event(Event::<T>::BondUnitSold(caller.clone(), bond, unit_amount, package_value));

                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_unit_package_return(origin: OriginFor<T>, bond: BondId, unit_amount: BondUnitAmount )
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            unit_amount: BondUnitAmount - amount of bond units
        ///
        /// Access: only accounts with Investor role, holding bond units
        /// In BOOKING state Investor can return "unit_amount" of BUs, and return EverUSD paid.
        /// Removes BondUnitsPackages, owned by Investor, matching amount on BUs, he wants to return.
        /// BUs are returned to common pool, decreasing "issued_amount" of bond.
        /// </pre>
        // Investor gives back bond units and withdraw tokens
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_unit_package_return())]
        pub fn bond_unit_package_return(origin: OriginFor<T>, bond: BondId,#[pallet::compact]  unit_amount: BondUnitAmount ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_investor(&caller), Error::<T>::AccountNotAuthorized);
            ensure!(unit_amount > 0, Error::<T>::BondParamIncorrect);
            // Active Bond cannot be withdrawn
            Self::with_bond(&bond, |item|{
                ensure!(item.state == BondState::BOOKING, Error::<T>::BondStateNotPermitAction );
                ensure!(item.issued_amount >= unit_amount, Error::<T>::BondParamIncorrect);
                let package_value = item.par_value( unit_amount ) ;
                ensure!(item.bond_credit >= package_value, Error::<T>::BondParamIncorrect);

                BondUnitPackageRegistry::<T>::try_mutate(&bond, &caller, |packages|->DispatchResult{
                    ensure!(!packages.is_empty(), Error::<T>::BondParamIncorrect);
                    if packages.iter().map(|item| item.bond_units).sum::<BondUnitAmount>() == unit_amount {
                        packages.clear();
                        Ok(().into())
                    } else if let Some(index) = packages.iter().position(|item| item.bond_units == unit_amount ){
                        packages.remove( index );
                        Ok(().into())
                    } else {
                        Err( Error::<T>::BondParamIncorrect.into() )
                    }
                })?;

                item.decrease( package_value );
                item.issued_amount -= unit_amount;

                Self::balance_add(&caller, package_value)?;
                Self::deposit_event(Event::<T>::BondUnitReturned(caller, bond, unit_amount, package_value));

                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_withdraw(origin: OriginFor<T>, bond: BondId)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///
        /// Access: accounts with Bond Arranger role, bond Issuer, or bond Manager
        /// Can be called after the bond was released but not raised enough capacity after deadline.
        /// In BOOKING state only. If bond haven't reached "bond_units_mincap_amount" (Investors
        /// haven't bought enough of BUs), bond managers(Issuer, Manager) or Bond Arranger can return
        /// bond in PREPARE state, denying acquisiton of new bond units, and allowing
        /// team to change parameters of bond and then try to release it with more suitable
        /// for Investors parameters. Cannot be called until "mincap_deadline"
        /// </pre>
        // Called after the Bond was released but not raised enough tokens until the deadline
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_withdraw())]
        pub fn bond_withdraw(origin: OriginFor<T>, bond: BondId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            // Bond issuer, bond Manager, or Bond Arranger can do it
            Self::with_bond(&bond, |item|{
                ensure!( item.state == BondState::BOOKING, Error::<T>::BondStateNotPermitAction );
                // Ensure the Bond raises less then bond_units_mincap_amount bond units
                ensure!(item.inner.bond_units_mincap_amount > item.issued_amount, Error::<T>::BondParamIncorrect);
                ensure!(
                    item.issuer == caller || item.manager == caller || accounts::Module::<T>::account_is_bond_arranger(&caller) ,
                    Error::<T>::BondAccessDenied
                );
                let now = Timestamp::<T>::get();
                // Ensure booking deadline is in the future
                ensure!(item.inner.mincap_deadline <= now, Error::<T>::BondStateNotPermitAction);

                item.state = BondState::PREPARE;
                item.nonce += 1;
                assert!(item.bond_credit == item.par_value(item.issued_amount));
                // @TODO make it lazy. this implementation do much work to restore balances
                // that is too CPU and memory expensive.
                // For each bondholder
                for (bondholder, package) in BondUnitPackageRegistry::<T>::iter_prefix(&bond){
                      let bondholder_total_amount: BondUnitAmount = package.iter()
                      .map(|item| item.bond_units )
                      .sum();

                      item.issued_amount -= bondholder_total_amount;

                      let transfer = item.par_value( bondholder_total_amount ) ;
                      item.decrease(transfer);

                      Self::balance_add(&bondholder, transfer)?;
                }
                assert!(item.bond_credit == 0);
                assert!(item.issued_amount == 0);

                BondUnitPackageRegistry::<T>::remove_prefix(&bond);

                Self::deposit_event(Event::<T>::BondWithdrawal(caller, bond));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_activate(origin: OriginFor<T>, bond: BondId)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            nonce: u64 - bond nonce
        /// Access: only accounts with Bond Arranger role
        ///
        /// Activates the bond after it raised minimum capacity of bond units, opening
        /// BondUnitsPackages, owned by Investors, to be traded of free market. Function
        /// moves bond from BOOKING to ACTIVE state, and transfers all the sum(EverUSD),
        /// paid by Investors to bond Issuer. Also, function creates the fixed size array of
        /// BondImpactReportStruct-s with amount of elements, equal to amount of payment periods.
        /// This array will be used to store future impact_report_data and effective
        /// coupon_yield_rate (depending on impact_report_data for each period). Requires that
        /// "bond_units_mincap_amount" was reached.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_activate())]
        pub fn bond_activate(origin: OriginFor<T>, bond: BondId,#[pallet::compact]  nonce: u64) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            //Bond can be activated only by Bond Arranger
            ensure!(accounts::Module::<T>::account_is_bond_arranger(&caller), Error::<T>::AccountNotAuthorized);
            //if it's raised enough bond units during bidding process
            Self::with_bond(&bond, |item|{
                ensure!(item.nonce == nonce, Error::<T>::BondNonceObsolete );
                ensure!(item.state == BondState::BOOKING, Error::<T>::BondStateNotPermitAction);
                ensure!(item.inner.bond_units_mincap_amount <= item.issued_amount, Error::<T>::BondParamIncorrect);
                // auditor should be assigned before
                ensure!(item.auditor != Default::default(), Error::<T>::BondIsNotConfigured);

                let now = Timestamp::<T>::get();
                item.state = BondState::ACTIVE;
                item.nonce += 1;
                item.active_start_date = now;
                // Decrease liabilities by value of fund
                assert_eq!(item.bond_credit, item.par_value( item.issued_amount ) );
                assert!(item.bond_credit == item.bond_debit);
                item.bond_credit = 0 ;

                // create impact report struct.
                // the total number or reports is equal to the number of periods.
                // start period coupon interest isn't calculated using impact data.
                let mut reports: Vec<BondImpactReportStruct> = Vec::new();
                // reports length cannot be larger than item.inner.impact_data_baseline
                // because bond_duration is limited in "is_valid()" function.
                reports.resize( item.inner.bond_duration  as usize, Default::default() );

                BondImpactReport::<T>::insert(&bond, &reports);

                // withdraw all available bond fund
                let amount = item.bond_debit;
                Self::balance_add(&item.issuer, item.bond_debit)?;
                item.bond_debit = 0;

                Self::deposit_event(Event::<T>::BondActivated(caller, bond, amount));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_impact_report_send(origin: OriginFor<T>, bond: BondId, impact_data: u64 )
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            impact_data: u64 - report value
        /// Access: bond Issuer or ImpactReporter, assigned to the bond
        ///
        /// Saves inital impact_report_data for current period, for later confirmation by Auditor.
        /// Data must be send during correct period (depending on "impact_data_send_period" and "payment_period"
        /// properties of bond). Also, impact_data for this period must be not confirmed by Auditor yet.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_impact_report_send())]
        pub fn bond_impact_report_send(origin: OriginFor<T>, bond: BondId,#[pallet::compact] period: BondPeriodNumber,#[pallet::compact] impact_data: u64 ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            let now = Timestamp::<T>::get();
            let moment = {
                let item = BondRegistry::<T>::get(bond);
                ensure!(item.issuer == caller || item.impact_reporter == caller, Error::<T>::BondAccessDenied );
                ensure!(Self::is_report_in_time(&item, now, period), Error::<T>::BondOutOfOrder );
                item.time_passed_after_activation(now).map(|(moment, _period)| moment ).unwrap()
            };

            let index: usize = period as usize;
            BondImpactReport::<T>::try_mutate(&bond, |reports|->DispatchResult {

                ensure!(index < reports.len() && !reports[index].signed, Error::<T>::BondParamIncorrect);

                reports[index].create_period = moment;
                reports[index].impact_data = impact_data;

                Self::deposit_event(Event::<T>::BondImpactReportSent( caller, bond, period, impact_data));
                Ok(())
            })?;
            Ok(().into())
        }

        /// <pre>
        /// Method: bond_impact_report_approve(origin: OriginFor<T>, bond: BondId, period: u64, impact_data: u64 )
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            period: u32 - report period starting from 0
        ///            impact_data: u64 - report value
        ///
        /// Access: only Auditor assigned to the bond
        /// Confirms "impact_report_data", that already being sent by the bond Issuer or ImpactReporter.
        /// Auditor sends period number and exact value of impact_data, he confirms. Confirmation must be
        /// send during the correct period (depending on "impact_data_send_period" and "payment_period"
        /// properties of bond)
        /// </pre>
        // Auditor signs impact report
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_impact_report_approve())]
        pub fn bond_impact_report_approve(origin: OriginFor<T>, bond: BondId,#[pallet::compact] period: BondPeriodNumber,#[pallet::compact]  impact_data: u64 ) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_auditor(&caller), Error::<T>::AccountNotAuthorized);
            let now = Timestamp::<T>::get();
            {
                let item = BondRegistry::<T>::get(bond);
                ensure!(item.auditor == caller, Error::<T>::BondAccessDenied );
                ensure!(Self::is_report_in_time(&item, now, period), Error::<T>::BondOutOfOrder );
            }

            let index: usize = period as usize;
            BondImpactReport::<T>::try_mutate(&bond, |reports|->DispatchResult {

                ensure!(index < reports.len(), Error::<T>::BondParamIncorrect );
                let report = &reports[index];
                ensure!(report.create_period > 0 , Error::<T>::BondParamIncorrect);
                ensure!(!report.signed && report.impact_data == impact_data,
                 Error::<T>::BondParamIncorrect
                );

                reports[index].signed = true;

                Self::deposit_event(Event::<T>::BondImpactReportApproved( caller, bond, period, impact_data));
                Ok(())
            })?;
            Ok(().into())
        }

        /// <pre>
        /// Method: bond_redeem(origin: OriginFor<T>, bond: BondId)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///
        /// Access: any account
        /// Makes the bond reached maturity date. Requires the bond Issuer to pay back
        /// redemption yield. Function moves bond from ACTIVE or BANKRUPT state to FINISHED.
        /// Function checks correct time (all payment_periods must be passed),
        /// calculates total bond credit and debit, summarizing all debts and yields,
        /// and transfers needed sum from/to Issuer's(!) EverUSD balance (calculated
        /// using "bond_debit", "bond_credit" and "issued_amount" (bond price)). Any account
        /// can call this function
        /// If all operations are successful, bond_debit will be fully covered EverUSD, paid
        /// by Issuer and Issuer don't have any obligations. Investors now can
        /// withdraw all their accrued coupon yield and parts of bond maturity debt
        /// Bond becomes FINISHED.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_redeem())]
        pub fn bond_redeem(origin: OriginFor<T>, bond: BondId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            let now = Timestamp::<T>::get();
            Self::with_bond(&bond, |mut item|{
                ensure!( matches!(item.state, BondState::ACTIVE|BondState::BANKRUPT), Error::<T>::BondStateNotPermitAction );

                match item.time_passed_after_activation(now){
                    Some((_, period))  if period == item.get_periods() => (),
                    _ => return Err( Error::<T>::BondOutOfOrder.into() ),
                };

                // if bond has carbon metadata writes accounts investments in it
                match &mut item.inner.carbon_metadata {
                    None => (),
                    Some(metadata) => {
                        metadata.account_investments = Self::get_bond_account_investment(&bond);
                    }
                }

                Self::calc_and_store_bond_coupon_yield(&bond, &mut item, now);
                // now bond_credit has YTM ( yield to mature )
                let amount = item.bond_credit + item.par_value( item.issued_amount ) ;
                if amount <= item.bond_debit {
                    // withdraw free balance
                    Self::balance_add(&item.issuer, item.bond_debit - amount)?;
                }else{
                    let transfer = amount - item.bond_debit;
                    // pay off debt
                    Self::balance_sub(&item.issuer, transfer)?;
                }
                let ytm = item.bond_credit;
                item.bond_credit = amount;
                //item.coupon_yield = amount;
                item.bond_debit = amount;
                item.state = BondState::FINISHED;
                item.nonce += 1;
                Self::deposit_event(Event::<T>::BondRedeemed(caller, bond, ytm));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_declare_bankrupt(origin: OriginFor<T>, bond: BondId)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        /// Access: Bond Arranger role
        ///
        /// Marks the bond as bankrupt, moving it from ACTIVE to BANKRUPT state.
        /// Function checks, that "get_debt()" of bond is > 0 (bond_credit > bond_debit),
        /// and that "interest_pay_period" is not active(Issuer still have a chance to pay debt).
        /// Then function calculates and stores all accumulated coupon_yield and marks bond as BANKRUPT
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_declare_bankrupt())]
        pub fn bond_declare_bankrupt(origin: OriginFor<T>, bond: BondId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            // Only Bond Arranger role can declare bankrupt
            ensure!(accounts::Module::<T>::account_is_bond_arranger(&caller), Error::<T>::AccountNotAuthorized);

            Self::with_bond(&bond, |mut item|{
                ensure!(item.state == BondState::ACTIVE, Error::<T>::BondStateNotPermitAction);
                ensure!(item.get_debt() > 0, Error::<T>::BondParamIncorrect );
                let now = Timestamp::<T>::get();
                ensure!( !Self::is_interest_pay_period(&item, now),Error::<T>::BondOutOfOrder );
                Self::calc_and_store_bond_coupon_yield(&bond, &mut item, now);

                item.state = BondState::BANKRUPT;
                item.nonce += 1;
                Self::deposit_event(Event::<T>::BondBankrupted(caller.clone(), bond, item.bond_credit, item.bond_debit));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_accrue_coupon_yield(origin: OriginFor<T>, bond: BondId)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        /// Access: any account
        ///
        /// Calculates total bond coupon yield(EverUSD) and stores it in "bond_credit"
        /// by calculating effective interest rates for each passed payment_period.
        /// This function is a call to "lazy" function "calc_and_store_bond_coupon_yield()"
        /// that is called in many operations, changing Investors BondUnitsPackage-s (like buy/sell BUs).
        /// Have the complexity O(N), where N - amount of BondUnitsPackage-s
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_accrue_coupon_yield())]
        pub fn bond_accrue_coupon_yield(origin: OriginFor<T>, bond: BondId) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            Self::with_bond(&bond, |mut item|->DispatchResultWithPostInfo {
                let now = Timestamp::<T>::get();
                let processed: u64 = Self::calc_and_store_bond_coupon_yield(&bond, &mut item, now) as u64;
                Ok(Some( T::DbWeight::get().reads_writes(processed+2, processed+1) ).into())
            })
        }

        /// <pre>
        /// Method: bond_revoke(origin: OriginFor<T>, bond: BondId)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        /// Access: Bond Issuer or Manager, assigned to the bond
        ///
        /// Totally removes bond from BondsRegistry before it was issued(in PREPARE state)
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_revoke())]
        pub fn bond_revoke(origin: OriginFor<T>, bond: BondId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            // Bond can be revoked only by Owner or by Manager assigned to the Bond
            // Bond should be in Prepare state, so no bids can exist at this time
            ensure!( BondRegistry::<T>::contains_key(&bond), Error::<T>::BondNotFound );
            let item = BondRegistry::<T>::get(bond);
            ensure!(item.issuer == caller || item.manager == caller, Error::<T>::BondAccessDenied);
            ensure!(item.state == BondState::PREPARE, Error::<T>::BondStateNotPermitAction);
            assert!( BondRegistry::<T>::contains_key(bond) );
            BondRegistry::<T>::remove( &bond );

            Self::deposit_event(Event::<T>::BondRevoked(caller, bond));
            Ok(().into())
        }

        /// <pre>
        /// Method: bond_withdraw_everusd(origin: OriginFor<T>, bond: BondId, amount: EverUSDBalance)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///
        /// Access: Bond Issuer or any Investor
        ///
        /// This function is mainly used by Investors to recieve accrued coupon yield in any moment.
        /// Also, this function is used by bond Issuer to withdraw EverUSD from `free bond balance`.
        /// Available only in ACTIVE, BANKRUPT of FINISHED states.
        /// If caller is Issuer of this bond, "get_free_balance()" amount of EverUSD tokens are transfered
        /// to bond.issuer address. If caller is the Investor of bond, then, in FINISHED state he
        /// receives all bond debt (principal value + coupon yield), or coupon yield only
        /// (by calling "request_coupon_yield()") if bond still ACTIVE or BANKRUPT. If amount
        /// of EverUSD on bond's balance is not enough to pay to Investors, bond moves to BANKRUPT state.
        /// </pre>
        //  @TODO add parameter beneficiary:T::AccountId  who will receive coupon yield
        //  @TODO consider separate functions for Issuer and Investor
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_withdraw_everusd())]
        pub fn bond_withdraw_everusd(origin: OriginFor<T>, bond: BondId) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            Self::with_bond(&bond, |mut item|{
                ensure!( matches!(item.state , BondState::ACTIVE | BondState::BANKRUPT | BondState::FINISHED), Error::<T>::BondStateNotPermitAction);

                let now = Timestamp::<T>::get();
                Self::calc_and_store_bond_coupon_yield(&bond, &mut item, now);

                let amount: EverUSDBalance = if item.issuer == caller {
                    // issuer withdraw bond fund
                    let amount = item.get_free_balance();
                    if amount>0{
                        Self::balance_add(&item.issuer, amount)?;
                        // it's safe to do unchecked subtraction
                        item.bond_debit -= amount;
                    }
                    amount
                }else if item.state == BondState::FINISHED {
                    // investor (bondholder) withdraw principal value
                    Self::redeem_bond_units(&bond, &mut item, &caller)
                }else{
                    // investor (bondholder) withdraw coupon yield
                    // set bankrupt state if bond fund cannot pay off
                    if item.state == BondState::ACTIVE && item.get_debt()>0 && !Self::is_interest_pay_period(&item, now){
                        item.state = BondState::BANKRUPT;
                        Self::deposit_event(Event::<T>::BondBankrupted(caller.clone(), bond, item.bond_credit, item.bond_debit ));
                    }

                    Self::request_coupon_yield(&bond, &mut item, &caller)
                };

                if amount>0{
                    Self::deposit_event(Event::<T>::BondWithdrawEverUSD(caller, bond, amount));
                }
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_deposit_everusd(origin: OriginFor<T>, bond: BondId, amount: EverUSDBalance)
        /// Arguments: origin: T::AccountId - transaction caller
        ///            bond: BondId - bond identifier
        ///            amount: EverUSDBalance - the number of EverUSD  deposited to bond fund
        /// Access: Bond issuer
        ///
        /// Transfers `amount` of EverUSD tokens from bond Issuer's balance to the bond fund
        /// to pay coupon yield and principal debt to Investors. Bond must be in ACTIVE or BANKRUPT
        /// state. Transfered EverUSD amount is added to bond_debit, and will be distributed between
        /// Investors, according to their ownership of Bond Units.
        /// "Lazy" call for function "calc_and_store_bond_coupon_yield()", that calculates and stores
        /// coupon yield amounts for each payment_period, where it's possible
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_deposit_everusd())]
        pub fn bond_deposit_everusd(origin: OriginFor<T>, bond: BondId,#[pallet::compact]  amount: EverUSDBalance) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;
            Self::with_bond(&bond, |mut item|{
                ensure!(
                    matches!(item.state , BondState::ACTIVE | BondState::BANKRUPT),
                    Error::<T>::BondStateNotPermitAction
                );
                ensure!(item.issuer == caller, Error::<T>::BondAccessDenied);

                Self::balance_sub(&caller, amount)?;

                item.bond_debit = item.bond_debit.checked_add(amount)
                    .ok_or( Error::<T>::BondParamIncorrect )?;
                let now = Timestamp::<T>::get();
                Self::calc_and_store_bond_coupon_yield(&bond, &mut item, now);
                if item.state == BondState::BANKRUPT && !item.is_shortage(){
                    item.state = BondState::ACTIVE;
                }

                Self::deposit_event(Event::<T>::BondDepositEverUSD(caller, bond, amount));
                Ok(().into())
            })
        }

        /// <pre>
        /// Method: bond_unit_lot_bid(origin: OriginFor<T>, bond: BondId, lot: BondUnitSaleLotStruct)
        /// Arguments: origin: T::AccountId - bond unit bondholder
        ///            bond: BondId - bond identifier
        ///            lot: BondUnitSaleLotStruct - lot data
        /// Access: any account, holding Bond Units (having BondUnitsPackage-s)
        ///
        /// Creates sale lot: pack of Bond Units to sale for given price. Accepts BondUnitSaleLotStruct,
        /// containing:
        ///   - "bond_units": amount of selling Bond Units
        ///   - "amount": price of whole lot in EverUSD
        ///   - "deadline": moment, after that lot cannot be sold
        ///   - "new_bondholder": (optional) target buyer(to restrict sale of this lot to given buyer)
        /// Function checks, that seller have BondUnitsPackage-s, containing enough BUs for lot creation,
        /// expiration date. Then, creates new lot in BondUnitPackageLot registry. Later, buyers can
        /// choose this lot and buy it.
        /// Also, function purges expired lots for this bond and seller from BondUnitPackageLot storage.
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_unit_lot_bid())]
        pub fn bond_unit_lot_bid(origin: OriginFor<T>, bond: BondId, lot: BondUnitSaleLotStructOf<T>) -> DispatchResultWithPostInfo{
            let caller = ensure_signed(origin)?;
            let now = Timestamp::<T>::get();
            // @TODO - maybe restrict this operation only to Investors?
            ensure!(!lot.is_expired(now), Error::<T>::LotParamIncorrect);

            let packages = BondUnitPackageRegistry::<T>::get(&bond, &caller);
            // how many bond units does the caller have
            let total_bond_units: BondUnitAmount = packages.iter()
            .map(|package| package.bond_units)
            .sum();

            ensure!(total_bond_units>=lot.bond_units && lot.bond_units>0, Error::<T>::BondParamIncorrect );

            // all lots of the caller.
            let mut lots: Vec<_> = BondUnitPackageLot::<T>::get(&bond, &caller);
            // purge expired lots
            lots.retain(|lot| !lot.is_expired(now) );

            let total_bond_units_inlot: BondUnitAmount = lots.iter().map(|lot| lot.bond_units).sum();
            // prevent new bid if the caller doesn't have enough bond units
            ensure!(total_bond_units>= total_bond_units_inlot+lot.bond_units, Error::<T>::BalanceOverdraft);

            lots.push(
                lot.clone()
            );
            // save  lots
            BondUnitPackageLot::<T>::insert(&bond, &caller, lots);
            Self::deposit_event(Event::<T>::BondSaleLotBid(caller, bond, lot));
            Ok(().into())
        }

        /// <pre>
        /// Method: bond_unit_lot_settle(origin: OriginFor<T>, bond: BondId,bondholder: T::AccountId, lot: BondUnitSaleLotStruct)
        /// Arguments: origin: T::AccountId - bond unit bondholder
        ///            bond: BondId - bond identifier
        ///            bondholder: Current bondholder of of bond
        ///            lot: BondUnitSaleLotStruct - lot data
        /// Access: any account with Investor role
        ///
        /// Buy the lot created by bond_unit_lot_bid() call. Lot should not be expired.
        /// If "new_bondholder" field of lot is set, only given account can buy this lot.
        /// Buyer should have enough EverUSD to buy the lot. When lot is sold,
        /// expired lots (fixed amount) are purged from BondUnitPackageLot registry.
        ///
        /// Before transfer of EverUSD, function caluclates and stores bond coupon yield for bondholder(seller)
        /// and caller(buyer), because this deal changes BondUnitsPackages of buyer and seller
        /// (buyer receives "newer" BondUnitsPackage, that buyer owned)
        /// </pre>
        #[pallet::weight(<T as pallet::Config>::WeightInfo::bond_unit_lot_settle())]
        pub fn bond_unit_lot_settle(origin: OriginFor<T>, bond: BondId, bondholder: T::AccountId, lot: BondUnitSaleLotStructOf<T>)->DispatchResultWithPostInfo{
            let caller = ensure_signed(origin)?;
            ensure!(accounts::Module::<T>::account_is_investor(&caller), Error::<T>::AccountNotAuthorized);
            let now = Timestamp::<T>::get();
            // prevent expired lots sales
            ensure!(!lot.is_expired( now ), Error::<T>::LotObsolete);

            ensure!(lot.new_bondholder == Default::default() || lot.new_bondholder == caller, Error::<T>::LotNotFound);
            let balance = Self::balance_everusd(&caller);
            // ensure caller has enough tokens on its balance
            ensure!(lot.amount <= balance , Error::<T>::BalanceOverdraft);

            BondUnitPackageLot::<T>::try_mutate(&bond, &bondholder, |lots|->DispatchResult{
                if let Some(index) = lots.iter().position(|item| item==&lot ){
                     lots.remove( index );
                     if !lots.is_empty() {
                        // purge expired lots
                        lots.retain( |item| !item.is_expired( now ) );
                     }
                     // @TODO optimize out access to balances
                     BondRegistry::<T>::mutate(bond, |mut item|{
                        Self::calc_and_store_bond_coupon_yield(&bond, &mut item, now);
                        Self::request_coupon_yield(&bond, &mut item, &bondholder);
                        Self::request_coupon_yield(&bond, &mut item, &caller);
                     });

                     let mut from_packages = BondUnitPackageRegistry::<T>::get(&bond, &bondholder);
                     let mut to_packages = BondUnitPackageRegistry::<T>::get(&bond, &caller);
                     // transfer lot.bond_units from bondholder to caller
                     Self::transfer_bond_units(&mut from_packages, &mut to_packages, lot.bond_units)?;
                     // store new packages
                     BondUnitPackageRegistry::<T>::insert(&bond, &bondholder, from_packages);
                     BondUnitPackageRegistry::<T>::insert(&bond, &caller, to_packages);

                     // pay off deal
                     Self::balance_sub(&caller, lot.amount)?;
                     Self::balance_add(&bondholder, lot.amount)?;
                     Self::deposit_event(Event::<T>::BondSaleLotSettle(caller, bondholder.clone(), bond, lot));
                     Ok(())
                }else{
                    Err(Error::<T>::BondParamIncorrect.into())
                }
            })?;
            Ok(().into())
        }
    }

    impl<T: Config> Module<T> {

        // @TESTME try to compare sort performance with binaryheap
        // @TODO try to find the package with exact match at fist

        /// <pre>
        /// Method: transfer_bond_units(from_packages, to_packages, lot_bond_units)
        /// Arguments: from_packages: &mut Vec<BondUnitPackage> - pack of BU packages(seller), BUs should be transfered "from"
        ///            to_packages: &mut Vec<BondUnitPackage> - pack of BU packages(buyer), BUs should be transfered "to"
        ///            lot_bond_units: BondUnitAmount -  amount of BUs to transfer
        ///
        /// Internal function, called when a lot with given amount of BUs is sold, and "lot_bond_units" should be transfered from
        /// seller's BUs packages pack to buyer's BUs packages. Functions accumulates needed amount of BUs,
        /// by removing and modifying seller's packages, beginning from last package
        /// </pre>
        pub(crate) fn transfer_bond_units(
            from_packages: &mut Vec<BondUnitPackage>,
            to_packages: &mut Vec<BondUnitPackage>,
            mut lot_bond_units: BondUnitAmount,
        ) -> DispatchResult {
            from_packages.sort_by_key(|package| core::cmp::Reverse(package.bond_units));

            while lot_bond_units > 0 {
                // last element has smallest number of bond units
                let mut last = from_packages
                    .pop()
                    .ok_or(Error::<T>::BondParamIncorrect)?;
                let (bond_units, acquisition, coupon_yield) = if last.bond_units > lot_bond_units {
                    last.bond_units -= lot_bond_units;
                    let bond_units = lot_bond_units;
                    let acquisition = last.acquisition;
                    lot_bond_units = 0;
                    from_packages.push(last);
                    (bond_units, acquisition, 0)
                } else {
                    lot_bond_units -= last.bond_units;
                    (last.bond_units, last.acquisition, last.coupon_yield)
                };

                to_packages.push(BondUnitPackage {
                    bond_units,
                    acquisition,
                    coupon_yield,
                });
            }
            from_packages.shrink_to_fit();
            Ok(())
        }

        /// <pre>
        /// Method: balance_everusd(acc: &T::AccountId) -> EverUSDBalance
        /// Arguments: acc: T::AccountId - account id
        ///
        /// Returns account's balance as the number of EverUSD tokens.
        /// Amount is returned as unsigned integer in smallest units of EverUSD
        ///  "1" EverUSD is presented as "1 000 000 000"(depends on EVERUSD_DECIMALS == 9)
        /// </pre>
        pub fn balance_everusd(acc: &T::AccountId) -> EverUSDBalance {
            BalanceEverUSD::<T>::get(acc)
        }
    
        /// <pre>
        /// Method: total_supply() -> EverUSDBalance
        /// Arguments: none
        ///
        /// Returns the total number of EverUSD tokens, owned by all holders, that was
        /// supplied by the custodian. Every mint() operation inreases total_supply, while
        /// burn() - decreases it. Initial total_supply of EverUSD == 0
        /// </pre>
        #[cfg(test)]
        pub fn total_supply() -> EverUSDBalance {
            TotalSupplyEverUSD::<T>::get()
        }
    
        /// <pre>
        /// Method: get_bond(bond: BondId) -> bond: BondId) -> BondStruct
        /// Arguments: bond: BondId - bond unique identifier
        ///
        ///  Returns bond structure if it was found in BondRegistry by given BondId
        /// </pre>
        // #[cfg(test)]
        pub fn get_bond(bond: &BondId) -> BondStructOf<T> {
            BondRegistry::<T>::get(bond)
        }
    
        #[cfg(test)]
        pub fn bond_check_invariant(bond: &BondId) -> bool {
            let (bond_units, coupon_yield) = BondUnitPackageRegistry::<T>::iter_prefix_values(bond)
                .fold((0, 0), |acc, packages| {
                    packages.iter().fold(acc, |acc, package| {
                        (acc.0 + package.bond_units, acc.1 + package.coupon_yield)
                    })
                });
            let bond = BondRegistry::<T>::get(bond);
    
            bond.issued_amount == bond_units && bond.coupon_yield == coupon_yield
        }
    
        #[cfg(test)]
        pub fn bond_holder_packages(bond: &BondId, bondholder: &T::AccountId) -> Vec<BondUnitPackage> {
            BondUnitPackageRegistry::<T>::get(bond, bondholder)
        }
    
        pub fn bond_impact_data(bond: &BondId) -> Vec<BondImpactReportStruct> {
            BondImpactReport::<T>::get(bond)
        }
    
        #[cfg(test)]
        pub fn bond_packages(id: &BondId) -> std::collections::HashMap<T::AccountId, Vec<BondUnitPackage>>
        where
            <T as frame_system::Config>::AccountId: std::hash::Hash,
        {
            BondUnitPackageRegistry::<T>::iter_prefix(id).collect()
        }
    
        /// <pre>
        /// Same as BondRegistry::<T>::mutate(bond, f).
        /// Unlike BondRegistry::<T>::mutate(bond, f) `with_bond` doesn't write to storage
        /// if call returns error or bond key doesn't exist in the registry
        /// </pre>
        pub fn with_bond<R, E: From<Error<T>>, F: FnOnce(&mut BondStructOf<T>) -> Result<R, E>>(
            bond: &BondId,
            f: F,
        ) -> Result<R, E> {
            ensure!(
                BondRegistry::<T>::contains_key(bond),
                Error::<T>::BondNotFound
            );
    
            BondRegistry::<T>::try_mutate(bond, |mut item| f(&mut item))
        }
    
        /// <pre>
        /// Increase account balance by `amount` EverUSD
        /// </pre>
        pub fn balance_add(who: &T::AccountId, amount: EverUSDBalance) -> DispatchResult {
            BalanceEverUSD::<T>::try_mutate(who, |balance| -> DispatchResult {
                *balance = balance
                    .checked_add(amount)
                    .ok_or(Error::<T>::BalanceOverdraft)?;
                Ok(())
            })
        }
    
        /// <pre>
        /// Decrease account balance by `amount` EverUSD
        /// </pre>
        pub fn balance_sub(who: &T::AccountId, amount: EverUSDBalance) -> DispatchResult {
            BalanceEverUSD::<T>::try_mutate(who, |balance| -> DispatchResult {
                *balance = balance
                    .checked_sub(amount)
                    .ok_or(Error::<T>::BalanceOverdraft)?;
                Ok(())
            })
        }
    
        /// Transfers everusd to an account
        #[inline]
        pub fn transfer_everusd(from: &T::AccountId, to: &T::AccountId, amount: EverUSDBalance) -> DispatchResult{ 
            Self::balance_sub(from, amount)?;
            Self::balance_add(to, amount)?;
            Ok(())
        }
    
        /// Gets bon unit investmets per account
        #[inline]
        pub fn get_bond_account_investment(bond_id: &BondId) -> Vec<(T::AccountId, u32)> {
            BondUnitPackageRegistry::<T>::iter()
                                        .filter(|(x, _, _)|{ x == bond_id })
                                        .map(|(_, y, z)| 
                                                {(y, z.iter()
                                                        .map(|x| x.bond_units)
                                                        .reduce(|a, b| a + b)
                                                        .unwrap_or(0))}
                                        )
                                        .collect::<Vec<(T::AccountId, u32)>>()
        }
    
        /// <pre>
        /// Deletes expired burn requests.
        /// Process less or equal than MAX_PURGE_REQUESTS expired requests
        /// </pre>
        pub fn purge_expired_burn_requests(before: T::Moment) {
            let to_purge: Vec<_> = BurnRequestEverUSD::<T>::iter()
                .filter(|(_, request)| request.is_expired(before))
                .map(|(acc, _)| acc)
                .take(MAX_PURGE_REQUESTS)
                .collect();
    
            for acc in to_purge {
                BurnRequestEverUSD::<T>::remove(acc);
            }
        }
    
        /// <pre>
        /// Deletes expired mint requests from the queue.
        /// Process less or equal than MAX_PURGE_REQUESTS expired requests
        /// </pre>
        pub fn purge_expired_mint_requests(before: T::Moment) {
            let to_purge: Vec<_> = MintRequestEverUSD::<T>::iter()
                .filter(|(_, request)| request.is_expired(before))
                .map(|(acc, _)| acc)
                .take(MAX_PURGE_REQUESTS)
                .collect();
    
            for acc in to_purge {
                MintRequestEverUSD::<T>::remove(acc);
            }
        }
    
        #[cfg(test)]
        pub fn get_coupon_yields(bond: &BondId) -> Vec<PeriodYield> {
            BondCouponYield::<T>::get(bond)
        }
    
        /// <pre>
        /// Returns combination of impact data and interest_rate for given BondId.
        /// </pre>
        pub fn get_impact_reports(bond: BondId) -> Vec<PeriodDataStruct> {
            let impact_data = BondImpactReport::<T>::get(bond);
            let coupon_yields = BondCouponYield::<T>::get(bond);
            coupon_yields
                .into_iter()
                .zip(impact_data.into_iter())
                .map(|(coupon_yields, impact_data)| PeriodDataStruct {
                    interest_rate: coupon_yields.interest_rate,
                    create_period: impact_data.create_period,
                    impact_data: impact_data.impact_data,
                    signed: impact_data.signed,
                })
                .collect()
        }
    
        /// <pre>
        /// Gets account everusd balance
        /// </pre>
        #[inline]
        pub fn get_balance(acc: &T::AccountId) -> EverUSDBalance {
            BalanceEverUSD::<T>::get(acc)
        }
    
        /// <pre>
        /// Calculates bond coupon yield and store accrued values in BondCouponYield::<T> map.
        /// Calculated values are stored in array, holding data for each payment_period.
        /// Function also updats bond's "bond_credit" value to the currently accrued coupon yield.
        /// Returns the number of processed periods.
        /// Common function complexity is O(N), where N is the number of issued bond unit packages.
        /// For each BondUnitsPackage of bond (containing payment_period when it was bought),
        /// the "package_yield" is calculated (yield per one bond unit), and then used to summarise
        /// yields of all bond's BondUnitsPackages.
        /// </pre>
        pub fn calc_and_store_bond_coupon_yield(
            id: &BondId,
            bond: &mut BondStructOf<T>,
            now: <T as pallet_timestamp::Config>::Moment,
        ) -> usize {
            let (_, period) = ensure_active!(bond.time_passed_after_activation(now), false);
            // here is current pay period
            let period = period as usize;
            // @TODO refactor. use `mutate` method instead  of get+insert
            let mut bond_yields = BondCouponYield::<T>::get(id);
            // get last accrued coupon yield
            let mut total_yield = bond_yields
                .last()
                .map(|period_yield| period_yield.total_yield)
                .unwrap_or(0);
            // period should be ended up before we can calc it
            if bond_yields.len() >= period {
                // term hasn't come yet (if period=0 )
                // or current period has been calculated
                bond.bond_credit = total_yield;
                return 0;
            }
            let time_step = T::TimeStep::get();
    
            let reports = BondImpactReport::<T>::get(id);
            assert!(reports.len() + 1 >= period);
    
            let mut processed: usize = 0;
            while bond_yields.len() < period {
                // index - accrued period number
                let index = bond_yields.len();
    
                let interest_rate = if bond.inner.is_stable() {
                    bond.inner.interest_rate_base_value
                } else {
                    if index == 0 {
                        // There is no periods and data yet, set start period interest rate value
                        bond.inner.interest_rate_start_period_value.unwrap_or(0)
                    } else if reports[index - 1].signed {
                        // There is confirmed impact_data about this period
                        // Calculate interest rate, based on impact_data and baseline,min,max parameters of bond
                        bond.calc_effective_interest_rate(
                            bond.inner.impact_data_baseline[index - 1].unwrap_or(0),
                            reports[index - 1].impact_data,
                        )
                    } else {
                        // Report is missed, apply penalty for missed report(but not more than interest_rate_margin_cap)
                        min(
                            bond_yields[index - 1].interest_rate
                                + bond.inner.interest_rate_penalty_for_missed_report.unwrap_or(0),
                            bond.inner.interest_rate_margin_cap.unwrap_or(0),
                        )
                    }
                };
    
                let package_yield = bond.inner.bond_units_base_price / 1000
                    * interest_rate as EverUSDBalance
                    / INTEREST_RATE_YEAR;
    
                // calculate yield for period equal to bond_yields.len()
                let period_coupon_yield: EverUSDBalance = match bond
                    .period_desc(index as BondPeriodNumber)
                {
                    Some(period_desc) => {
                        // for every bond bondholder
                        BondUnitPackageRegistry::<T>::iter_prefix(id)
                            .map(|(_bondholder, packages)| {
                                // flat_map
                                // for every package
                                packages
                                    .iter()
                                    .map(|package| {
                                        // @TODO use checked arithmetics
                                        package_yield
                                            * package.bond_units as EverUSDBalance
                                            * (period_desc.duration(package.acquisition) / time_step)
                                                as EverUSDBalance
                                            / 100
                                    })
                                    .sum::<EverUSDBalance>()
                            })
                            .sum()
                    }
                    None => {
                        // @TODO  it's best panic instead of return false
                        return 0;
                    }
                };
    
                total_yield += period_coupon_yield;
    
                bond_yields.push(PeriodYield {
                    total_yield,
                    interest_rate,
                    //coupon_yield_before: 0,
                });
                processed += 1;
                Self::deposit_event(Event::<T>::BondCouponYield(*id, total_yield));
            }
            // save current liability in bond_credit field
            bond.bond_credit = total_yield;
            BondCouponYield::<T>::insert(id, bond_yields);
    
            Self::deposit_event(Event::<T>::BondCouponYield(*id, total_yield));
            processed
        }
    
        /// <pre>
        /// Redeem bond units, get principal value, and coupon yield in the balance
        /// Function summarizes data from all passed periods,
        /// calculates coupon yield for each BondUnitsPackage, owned by bondholder.
        /// For each BondUnitsPackage, owned by "bondholder"(containing "payment_period"
        /// when it was bought), the "package_yield" value is calculated (yield per one bond unit).
        /// This "package_yield" then used to summarize yields of all bond's BondUnitsPackages.
        /// </pre>
        pub fn redeem_bond_units(
            id: &BondId,
            bond: &mut BondStructOf<T>,
            bondholder: &T::AccountId,
        ) -> EverUSDBalance {
            let packages = BondUnitPackageRegistry::<T>::take(id, &bondholder);
            let time_step = T::TimeStep::get();
            let bond_yields = BondCouponYield::<T>::get(id);
            assert!(!bond_yields.is_empty());
            // calc coupon yield
            let mut payable: EverUSDBalance = bond_yields
                .iter()
                .enumerate()
                .map(|(i, bond_yield)| {
                    let period_desc = bond.period_desc(i as BondPeriodNumber).unwrap();
                    let package_yield = bond.inner.bond_units_base_price / 1000
                        * bond_yield.interest_rate as EverUSDBalance
                        / INTEREST_RATE_YEAR;
                    packages
                        .iter()
                        .map(|package| {
                            package_yield
                                * package.bond_units as EverUSDBalance
                                * (period_desc.duration(package.acquisition) / time_step)
                                    as EverUSDBalance
                                / 100
                        })
                        .sum::<EverUSDBalance>()
                })
                .sum::<EverUSDBalance>();
    
            let (bond_units, paid_yield): (BondUnitAmount, EverUSDBalance) =
                packages.iter().fold((0, 0), |acc, package| {
                    (acc.0 + package.bond_units, acc.1 + package.coupon_yield)
                });
            // substrate paid coupon
            payable -= paid_yield;
            // add principal value
            payable += bond.par_value(bond_units);
            bond.coupon_yield += payable;
    
            Self::balance_add(bondholder, payable).unwrap();
    
            payable
        }
    
        /// <pre>
        /// Transfer accrued coupon yield into bondholder balance
    
        /// Calculates "non-spent" amount of EverUSD, that bondholder can receive,
        /// (gathering separate yields from all his BondUnitsPackage-s), transfers
        /// EverUSD to bondholder and stores information about "aready-transfered"
        /// amount of EverUSD for given bondholder.
        /// For each BondUnitsPackage, owned by "bondholder"(containing "payment_period"
        /// when it was bought), the "package_yield" value is calculated (yield per one bond unit).
        /// This "package_yield" then used to summarize yields of all bond's BondUnitsPackages
        /// and transfers this amount to bondholder from bond's balance.
        /// </pre>
        pub fn request_coupon_yield(
            id: &BondId,
            bond: &mut BondStructOf<T>,
            bondholder: &T::AccountId,
        ) -> EverUSDBalance {
            let bond_yields = BondCouponYield::<T>::get(id);
    
            let total_yield = bond_yields
                .last()
                .map(|period_yield| period_yield.total_yield)
                .unwrap_or(0);
    
            if total_yield == 0 || bond.bond_debit == 0 {
                return 0;
            }
    
            debug_assert!(!bond_yields.is_empty());
    
            let current_coupon_yield = min(bond.bond_debit, total_yield);
    
            let mut last_bondholder_coupon_yield = BondLastCouponYield::<T>::get(id, bondholder);
            debug_assert!(current_coupon_yield >= last_bondholder_coupon_yield.coupon_yield);
            debug_assert!(bond_yields.len() > last_bondholder_coupon_yield.period_num as usize);
    
            if last_bondholder_coupon_yield.coupon_yield == current_coupon_yield {
                // no more accrued coupon yield
                return 0;
            }
    
            let time_step = T::TimeStep::get();
            let mut payable = 0;
    
            let mut prev_total_yield = if last_bondholder_coupon_yield.period_num == 0 {
                0
            } else {
                bond_yields[last_bondholder_coupon_yield.period_num as usize - 1].total_yield
            };
    
            for (i, bond_yield) in bond_yields
                .iter()
                .enumerate()
                .skip(last_bondholder_coupon_yield.period_num as usize)
            {
                native::info!(
                    "period {}. period_yield {}-{}, current total coupon yield  {}, bondholder {};",
                    i,
                    prev_total_yield,
                    bond_yield.total_yield,
                    current_coupon_yield,
                    last_bondholder_coupon_yield.coupon_yield,
                );
    
                if last_bondholder_coupon_yield.coupon_yield >= current_coupon_yield {
                    break;
                }
                // don't open next period till the current period be fully repaid
                if current_coupon_yield <= prev_total_yield {
                    break;
                }
                // current period accrued coupon yield
                let accrued_yield = bond_yield.total_yield.saturating_sub(prev_total_yield);
                prev_total_yield = bond_yield.total_yield;
    
                let coupon_yield = if current_coupon_yield >= bond_yield.total_yield {
                    bond_yield.total_yield
                } else {
                    current_coupon_yield
                };
                // to be distributed among bondholders
                let installment =
                    coupon_yield.saturating_sub(last_bondholder_coupon_yield.coupon_yield);
    
                last_bondholder_coupon_yield.coupon_yield = coupon_yield;
                last_bondholder_coupon_yield.period_num = i as BondPeriodNumber;
    
                if installment == 0 {
                    continue;
                }
    
                debug_assert!(installment <= accrued_yield);
    
                let package_yield = bond.inner.bond_units_base_price / 1000
                    * bond_yield.interest_rate as EverUSDBalance
                    / INTEREST_RATE_YEAR;
    
                let period_desc = bond.period_desc(i as BondPeriodNumber).unwrap();
    
                BondUnitPackageRegistry::<T>::mutate(id, &bondholder, |packages| {
                    for package in packages.iter_mut() {
                        let accrued = package_yield
                            * package.bond_units as EverUSDBalance
                            * (period_desc.duration(package.acquisition) / time_step) as EverUSDBalance
                            / 100;
    
                        let package_coupon_yield = if installment == accrued_yield {
                            accrued
                        } else {
                            (installment as u128 * accrued as u128 / accrued_yield as u128) as u64
                        };
    
                        payable += package_coupon_yield;
                        package.coupon_yield += package_coupon_yield;
                    }
                });
            }
    
            bond.coupon_yield = bond.coupon_yield.saturating_add(payable);
    
            BondLastCouponYield::<T>::insert(id, &bondholder, last_bondholder_coupon_yield);
            Self::balance_add(bondholder, payable).unwrap();
            payable
        }
    
        /// <pre>
        /// Returns effective coupon interest rate for `period`.
        /// Iterates through periods, calculating interest_rate for each period
        /// based on data of previous period(data can be absent). In case of absence
        /// of impact data, "interest_rate_penalty_for_missed_report" is applied
        /// to each new "empty" period, until "interest_rate_margin_cap" is reached.
        /// Common complexity is O(1), O(N) in worst case(when no reports were sent)
        /// </pre>
        #[cfg(test)]
        pub fn calc_bond_interest_rate(
            bond: &BondStructOf<T>,
            reports: &[BondImpactReportStruct],
            period: usize,
        ) -> bond::BondInterest {
            assert!(reports.len() >= period);
            if bond.inner.is_stable() {
                bond.inner.interest_rate_base_value            
            } else {
                let mut missed_periods = 0;
                let mut interest: bond::BondInterest = bond.inner.interest_rate_start_period_value.unwrap_or(0);
    
                for (report, baseline) in reports[0..period]
                    .iter()
                    .zip(bond.inner.impact_data_baseline[0..period].iter())
                    .rev()
                {
                    if report.signed {
                        interest = bond.calc_effective_interest_rate(baseline.unwrap_or(0), report.impact_data);
                        break;
                    }
                    missed_periods += 1;
                }
    
                min(
                    bond.inner.interest_rate_margin_cap.unwrap_or(0),
                    interest + missed_periods * bond.inner.interest_rate_penalty_for_missed_report.unwrap_or(0),
                )
            }
        }
        /// <pre>
        /// Checks if a report comes at the right time.
        /// Impact data (and confirmation by Auditor) must be sent
        /// after "impact_data_send_period" seconds before end of a payment_period
        /// and before end of a payment_period.
        /// </pre>
        pub fn is_report_in_time(
            bond: &BondStructOf<T>,
            now: <T as pallet_timestamp::Config>::Moment,
            period: BondPeriodNumber,
        ) -> bool {
            // get  the number of seconds from bond activation
            let (moment, _current_period) =
                ensure_active!(bond.time_passed_after_activation(now), false);
            // impact report should be sent and signed not early than interval for send report begins
            // and not later than current period ends
            bond.period_desc(period)
                .map(|desc| moment >= desc.impact_data_send_period && moment < desc.payment_period)
                .unwrap_or(false)
        }
    
        /// <pre>
        /// Checks if it's a right time for Issuer to pay interest for this period
        /// Payment must be sent before "interest_pay_period" seconds are passed
        /// after payment_period beginning
        /// </pre>
        pub fn is_interest_pay_period(
            bond: &BondStructOf<T>,
            now: <T as pallet_timestamp::Config>::Moment,
        ) -> bool {
            let (moment, period) = ensure_active!(bond.time_passed_after_activation(now), true);
    
            bond.period_desc(period)
                .map(|desc| moment < desc.interest_pay_period)
                .unwrap_or(true)
        }
    
        #[cfg(debug_assertions)]
        pub fn set_balance(who: &T::AccountId, amount: EverUSDBalance) {
            BalanceEverUSD::<T>::insert(who, amount)
        }
    
        #[cfg(test)]
        pub fn set_impact_data(
            bond: &BondId,
            period: BondPeriodNumber,
            impact_data: u64,
        ) -> DispatchResultWithPostInfo {
            BondImpactReport::<T>::try_mutate(&bond, |reports| -> DispatchResult {
                let index = period as usize;
    
                reports[index].signed = true;
                reports[index].impact_data = impact_data;
                reports[index].create_period = 1; //dirty hack. test require nonzero value
    
                Ok(())
            })?;
            Ok(().into())
        }
    
        #[cfg(test)]
        pub fn evercity_balance() -> ledger::EvercityBalance {
            let account: EverUSDBalance = BalanceEverUSD::<T>::iter_values().sum();
            let bond_fund: EverUSDBalance = BondRegistry::<T>::iter_values()
                .map(|bond| bond.bond_debit - bond.coupon_yield)
                .sum();
    
            ledger::EvercityBalance {
                supply: TotalSupplyEverUSD::<T>::get(),
                account,
                bond_fund,
            }
        }
    
        #[cfg(debug_assertions)]
        pub fn create_test_finished_bond(issuer: T::AccountId, bond_id: BondId, inner: BondInnerStructOf<T>) -> Result<(), ()> {
            let now = Timestamp::<T>::get();
            let item = BondStruct {
                    inner,
                    creation_date: now,
                    issuer,
                    nonce: 0,
                    state: BondState::FINISHED, 
                    .. Default::default()
            };
            BondRegistry::<T>::insert(&bond_id, item);
            Ok(().into())
        }
    
        #[cfg(debug_assertions)]
        pub fn create_test_active_bond(issuer: T::AccountId, bond_id: BondId, inner: BondInnerStructOf<T>) -> Result<(), ()> {
            let now = Timestamp::<T>::get();
            let item = BondStruct {
                    inner,
                    creation_date: now,
                    issuer,
                    nonce: 0,
                    state: BondState::ACTIVE, 
                    .. Default::default()
            };
            BondRegistry::<T>::insert(&bond_id, item);
            Ok(().into())
        }
    
        #[cfg(debug_assertions)]
        pub fn create_test_not_finished_bond(issuer: T::AccountId, bond_id: BondId, inner: BondInnerStructOf<T>) -> Result<(), ()> {
            let now = Timestamp::<T>::get();
            let item = BondStruct {
                    inner,
                    creation_date: now,
                    issuer,
                    nonce: 0,
                    .. Default::default()
            };
            BondRegistry::<T>::insert(&bond_id, item);
            Ok(().into())
        }
    
        #[cfg(debug_assertions)]
        pub fn add_test_bond_unit_packages(bond_id: &BondId, units: Vec<(T::AccountId, BondUnitAmount)>) {
            for (acc, unit_amount) in units {
                BondUnitPackageRegistry::<T>::mutate(bond_id, &acc, |packages|{
                    packages.push(
                        BondUnitPackage{
                             bond_units: unit_amount,
                             acquisition: 0,
                             coupon_yield: 0,
                        }
                    );
                })
            }
        }
    }
}