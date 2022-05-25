#![allow(clippy::from_over_into)]
use frame_support::{
     dispatch::DispatchResult, sp_std::ops::RangeInclusive,
};
use crate::tests::mock::*;
use crate::{
    BondInnerStructOf, BondStructOf,
    BondUnitAmount, BondUnitPackage, BondUnitSaleLotStructOf, Error, EverUSDBalance, DEFAULT_DAY_DURATION,
};

//////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test uses pack of accounts, pre-set in new_test_ext in mock.rs:
// (1, EvercityAccountStruct { roles: MASTER,            identity: 10u64}), // MASTER    (accountId: 1)
// (2, EvercityAccountStruct { roles: CUSTODIAN,         identity: 20u64}), // CUSTODIAN (accountID: 2)
// (3, EvercityAccountStruct { roles: ISSUER,            identity: 30u64}), // ISSUER   (accountID: 3)
// (4, EvercityAccountStruct { roles: INVESTOR,          identity: 40u64}), // INVESTOR  (accountId: 4)
// (5, EvercityAccountStruct { roles: AUDITOR,           identity: 50u64}), // AUDITOR   (accountId: 5)
// (7, EvercityAccountStruct { roles: ISSUER | ISSUER,   identity: 70u64}), // ISSUER   (accountId: 5)
// (8, EvercityAccountStruct { roles: MANAGER,           identity: 80u64}), // MANAGER   (accountId: 8)
// (101+ : some external accounts
//////////////////////////////////////////////////////////////////////////////////////////////////////////
//pub type Timestamp = pallet_timestamp::Module<TestRuntime>;
pub type Moment = <TestRuntime as pallet_timestamp::Config>::Moment;
pub type BondInnerStruct = BondInnerStructOf<TestRuntime>;
pub type BondStruct = BondStructOf<TestRuntime>;
pub type RuntimeError = Error<TestRuntime>;
pub type AccountId = <TestRuntime as frame_system::Config>::AccountId;
pub type BondUnitSaleLotStruct = BondUnitSaleLotStructOf<TestRuntime>;


pub fn bond_current_period(bond: &BondStruct, now: Moment) -> u32 {
    bond.time_passed_after_activation(now).unwrap().1
}

/// Auxiliary function that replenish account balance
pub fn add_token(id: AccountId, amount: EverUSDBalance) -> DispatchResult {
    Evercity::token_mint_request_create_everusd(Origin::signed(id), amount)?;
    Evercity::token_mint_request_confirm_everusd(Origin::signed(CUSTODIAN_ID), id, amount)
}

/// Converts days into milliseconds
pub fn days2timestamp(days: u32) -> Moment {
    (days * DEFAULT_DAY_DURATION) as u64 * 1000_u64
}

/// Returns all accounts
pub fn iter_accounts() -> RangeInclusive<u64> {
    1_u64..=9
}

pub fn bond_unit_package_amount(package: Vec<BondUnitPackage>) -> Vec<BondUnitAmount> {
    package.into_iter().map(|item| item.bond_units).collect()
}

pub fn create_bond_unit_package(amount: Vec<BondUnitAmount>) -> Vec<BondUnitPackage> {
    amount
        .into_iter()
        .map(|bond_units| BondUnitPackage {
            bond_units,
            acquisition: 0,
            coupon_yield: 0,
        })
        .collect()
}

pub const CUSTODIAN_ID: u64 = 2;