#![allow(clippy::from_over_into)]
use frame_support::{
    assert_noop, assert_ok, dispatch::DispatchResult, sp_io, sp_std::ops::RangeInclusive,
    Blake2_256, StorageHasher,
};

use crate::bond::transfer_bond_units;
use crate::tests::mock::*;
use crate::{
    BondId, BondImpactReportStruct, BondInnerStructOf, BondPeriodNumber, BondState, BondStructOf,
    BondUnitAmount, BondUnitPackage, BondUnitSaleLotStructOf, Error, EverUSDBalance, Module,
    AUDITOR_ROLE_MASK, DEFAULT_DAY_DURATION, ISSUER_ROLE_MASK, MASTER_ROLE_MASK,
};
use super::helpers::*;


#[test]
fn it_returns_true_for_correct_role_checks() {
    new_test_ext().execute_with(|| {
        assert_eq!(Evercity::account_is_master(&1), true);
        assert_eq!(Evercity::account_is_custodian(&2), true);
        assert_eq!(Evercity::account_is_issuer(&3), true);
        assert_eq!(Evercity::account_is_investor(&4), true);
        assert_eq!(Evercity::account_is_auditor(&5), true);
        assert_eq!(Evercity::account_is_manager(&8), true);
        assert_eq!(Evercity::account_is_issuer(&7), true);
        assert_eq!(Evercity::account_is_investor(&7), true);

        assert_eq!(Evercity::account_is_master(&100), false);
        assert_eq!(Evercity::account_is_custodian(&100), false);
        assert_eq!(Evercity::account_is_issuer(&100), false);
        assert_eq!(Evercity::account_is_investor(&100), false);
        assert_eq!(Evercity::account_is_auditor(&100), false);
        assert_eq!(Evercity::account_token_mint_burn_allowed(&100), false);
    });
}

#[test]
fn it_returns_false_for_incorrect_role_checks() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        //assert_ok!(AccountRegistry::insert(Origin::signed(1), EvercityAccountStruct {roles: 1u8, identity: 67u64}));
        // Read pallet storage and assert an expected result.
        assert_eq!(Evercity::account_is_auditor(&1), false);
        assert_eq!(Evercity::account_is_issuer(&2), false);
        assert_eq!(Evercity::account_is_investor(&3), false);
        assert_eq!(Evercity::account_is_custodian(&4), false);
        assert_eq!(Evercity::account_is_master(&5), false);
    });
}

#[test]
fn it_adds_new_account_with_correct_roles() {
    new_test_ext().execute_with(|| {
        Timestamp::set_timestamp(12345);

        assert_ok!(Evercity::account_add_with_role_and_data(
            Origin::signed(1),
            101,
            MASTER_ROLE_MASK,
            88u64
        ));
        assert_eq!(Evercity::account_is_master(&101), true);
        assert_eq!(Evercity::account_is_investor(&101), false);

        assert_ok!(Evercity::account_add_with_role_and_data(
            Origin::signed(1),
            102,
            AUDITOR_ROLE_MASK,
            89u64
        ));
        assert_eq!(Evercity::account_is_master(&102), false);
        assert_eq!(Evercity::account_is_auditor(&102), true);
    });
}

#[test]
fn it_correctly_sets_new_role_to_existing_account() {
    new_test_ext().execute_with(|| {
        // add new role to existing account (allowed only for master)
        assert_eq!(Evercity::account_is_issuer(&3), true);
        assert_ok!(Evercity::account_set_with_role_and_data(
            Origin::signed(1),
            3,
            AUDITOR_ROLE_MASK,
            88u64
        ));
        assert_eq!(Evercity::account_is_issuer(&3), true);
        assert_eq!(Evercity::account_is_auditor(&3), true);
        assert_eq!(Evercity::account_is_investor(&3), false);

        assert_eq!(Evercity::account_is_custodian(&2), true);
        assert_eq!(Evercity::account_is_issuer(&2), false);
        assert_ok!(Evercity::account_set_with_role_and_data(
            Origin::signed(1),
            2,
            ISSUER_ROLE_MASK,
            89u64
        ));
        assert_eq!(Evercity::account_is_custodian(&2), true);
        assert_eq!(Evercity::account_is_issuer(&2), true);
    });
}

#[test]
fn it_disable_account() {
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::account_add_with_role_and_data(
            Origin::signed(1),
            101,
            MASTER_ROLE_MASK,
            88u64
        ));
        assert_eq!(Evercity::account_is_master(&101), true);
        assert_ok!(Evercity::account_disable(Origin::signed(1), 101));

        assert_eq!(Evercity::account_is_master(&101), false);
    });
}

#[test]
fn it_try_disable_yourself() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Evercity::account_disable(Origin::signed(1), 1),
            RuntimeError::InvalidAction
        );
        assert_noop!(
            Evercity::account_set_with_role_and_data(Origin::signed(1), 1, 0, 0),
            RuntimeError::InvalidAction
        );
    });
}

#[test]
fn it_denies_add_and_set_roles_for_non_master() {
    new_test_ext().execute_with(|| {
        // trying to add account form non-master account
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(12345);
        assert_noop!(
            Evercity::account_add_with_role_and_data(
                Origin::signed(2),
                101,
                MASTER_ROLE_MASK,
                88u64
            ),
            RuntimeError::AccountNotAuthorized
        );

        assert_noop!(
            Evercity::account_set_with_role_and_data(Origin::signed(2), 3, ISSUER_ROLE_MASK, 88u64),
            RuntimeError::AccountNotAuthorized
        );
    });
}