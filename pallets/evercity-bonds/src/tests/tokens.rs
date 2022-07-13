#![allow(clippy::from_over_into)]
use frame_support::{
    assert_noop, assert_ok,
};
use crate::tests::mock::*;
use crate::{
    EverUSDBalance,
};
use super::helpers::*;

#[test]
fn it_token_mint_create_with_confirm() {
    const ACCOUNT: u64 = 4; // INVESTOR
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::token_mint_request_create_everusd(
            Origin::signed(ACCOUNT),
            100000
        ));
        assert_eq!(Evercity::total_supply(), 0);

        assert_ok!(Evercity::token_mint_request_confirm_everusd(
            Origin::signed(CUSTODIAN_ID),
            ACCOUNT,
            100000
        ));
        assert_eq!(Evercity::total_supply(), 100000);
    });
}

#[test]
fn it_token_mint_create_with_revoke() {
    const ACCOUNT: u64 = 4; // INVESTOR
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::token_mint_request_create_everusd(
            Origin::signed(ACCOUNT), // INVESTOR
            100000
        ));

        assert_ok!(Evercity::token_mint_request_revoke_everusd(Origin::signed(
            ACCOUNT
        ),));

        assert_noop!(
            Evercity::token_mint_request_confirm_everusd(
                Origin::signed(CUSTODIAN_ID),
                ACCOUNT,
                100000
            ),
            RuntimeError::MintRequestDoesntExist
        );
    });
}

#[test]
fn it_token_mint_create_with_decline() {
    const ACCOUNT: u64 = 4; // INVESTOR
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::token_mint_request_create_everusd(
            Origin::signed(ACCOUNT),
            100000
        ));

        assert_ok!(Evercity::token_mint_request_decline_everusd(
            Origin::signed(CUSTODIAN_ID),
            ACCOUNT
        ));

        assert_noop!(
            Evercity::token_mint_request_revoke_everusd(Origin::signed(ACCOUNT)),
            RuntimeError::MintRequestDoesntExist
        );
    });
}

#[test]
fn it_token_mint_create_denied() {
    const ACCOUNT: u64 = 5; // AUDITOR
    new_test_ext().execute_with(|| {
        assert_noop!(
            Evercity::token_mint_request_create_everusd(Origin::signed(ACCOUNT), 100000),
            RuntimeError::AccountNotAuthorized
        );
    });
}

#[test]
fn it_token_mint_create_hasty() {
    const ACCOUNT: u64 = 4; // INVESTOR
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::token_mint_request_create_everusd(
            Origin::signed(ACCOUNT),
            100000
        ));

        assert_noop!(
            Evercity::token_mint_request_create_everusd(Origin::signed(ACCOUNT), 10),
            RuntimeError::MintRequestAlreadyExist
        );

        // make amend
        let ttl: u32 = <TestRuntime as crate::Config>::MintRequestTtl::get();
        <pallet_timestamp::Pallet<TestRuntime>>::set_timestamp(ttl.into());

        assert_ok!(Evercity::token_mint_request_create_everusd(
            Origin::signed(ACCOUNT),
            10
        ));
    });
}

#[test]
fn it_token_mint_create_toolarge() {
    const ACCOUNT: u64 = 4;
    new_test_ext().execute_with(|| {
        assert_noop!(
            Evercity::token_mint_request_create_everusd(
                Origin::signed(ACCOUNT), // INVESTOR
                EVERUSD_MAX_MINT_AMOUNT + 1
            ),
            RuntimeError::MintRequestParamIncorrect
        );
    });
}

#[test]
fn it_token_burn_mint_overflow() {
    const ACCOUNT: u64 = 4;
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::token_mint_request_create_everusd(
            Origin::signed(ACCOUNT),
            1000
        ));

        assert_ok!(Evercity::token_mint_request_confirm_everusd(
            Origin::signed(CUSTODIAN_ID),
            ACCOUNT,
            1000
        ));
        assert_noop!(
            Evercity::token_burn_request_create_everusd(
                Origin::signed(ACCOUNT),
                EverUSDBalance::MAX - 1000
            ),
            RuntimeError::BalanceOverdraft
        );
        // assert_noop!(
        //     Evercity::token_burn_request_confirm_everusd(
        //         Origin::signed(CUSTODIAN_ID),
        //         ACCOUNT,
        //         EverUSDBalance::MAX - 1000
        //     ),
        //     RuntimeError::BalanceOverdraft
        // );
    });
}

#[test]
fn it_token_mint_try_confirm_expired() {
    const ACCOUNT: u64 = 4;
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::token_mint_request_create_everusd(
            Origin::signed(ACCOUNT), // INVESTOR
            1000
        ));
        <pallet_timestamp::Pallet<TestRuntime>>::set_timestamp(days2timestamp(10));
        assert_noop!(
            Evercity::token_mint_request_confirm_everusd(
                Origin::signed(CUSTODIAN_ID),
                ACCOUNT,
                1000
            ),
            RuntimeError::MintRequestObsolete
        );
    });
}

// burn tokens

#[test]
fn it_token_burn_create_with_confirm() {
    const ACCOUNT: u64 = 4; // INVESTOR

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(ACCOUNT, 10000));

        assert_ok!(Evercity::token_burn_request_create_everusd(
            Origin::signed(ACCOUNT),
            10000
        ));

        assert_eq!(Evercity::total_supply(), 10000);

        assert_ok!(Evercity::token_burn_request_confirm_everusd(
            Origin::signed(CUSTODIAN_ID),
            ACCOUNT,
            10000
        ));

        assert_eq!(Evercity::total_supply(), 0);
        // duplicate confirmations is not allowed
        assert_noop!(
            Evercity::token_burn_request_confirm_everusd(
                Origin::signed(CUSTODIAN_ID),
                ACCOUNT,
                10000
            ),
            RuntimeError::BurnRequestDoesntExist
        );
    });
}

#[test]
fn it_token_burn_create_overrun() {
    const ACCOUNT: u64 = 3; // ISSUER
    const BALANCE: EverUSDBalance = 10000;

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(ACCOUNT, BALANCE));

        assert_noop!(
            Evercity::token_burn_request_create_everusd(Origin::signed(ACCOUNT), BALANCE + 1),
            RuntimeError::BalanceOverdraft
        );
    });
}

#[test]
fn it_token_burn_create_with_revoke() {
    const ACCOUNT: u64 = 3; // ISSUER

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(ACCOUNT, 10000));

        assert_ok!(Evercity::token_burn_request_create_everusd(
            Origin::signed(ACCOUNT),
            10000
        ));

        assert_ok!(Evercity::token_burn_request_revoke_everusd(Origin::signed(
            ACCOUNT
        ),));

        assert_noop!(
            Evercity::token_burn_request_confirm_everusd(
                Origin::signed(CUSTODIAN_ID),
                ACCOUNT,
                10000
            ),
            RuntimeError::BurnRequestDoesntExist
        );
    });
}

#[test]
fn it_token_burn_try_confirm_expired() {
    const ACCOUNT: u64 = 4;
    new_test_ext().execute_with(|| {
        assert_ok!(add_token(ACCOUNT, 10000));
        assert_ok!(Evercity::token_burn_request_create_everusd(
            Origin::signed(ACCOUNT), // INVESTOR
            1000
        ));
        <pallet_timestamp::Pallet<TestRuntime>>::set_timestamp(days2timestamp(10));
        assert_noop!(
            Evercity::token_burn_request_confirm_everusd(
                Origin::signed(CUSTODIAN_ID),
                ACCOUNT,
                1000
            ),
            RuntimeError::BurnRequestObsolete
        );
    });
}

#[test]
fn it_token_burn_hasty() {
    const ACCOUNT: u64 = 4; // INVESTOR

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(ACCOUNT, 10000));

        assert_ok!(Evercity::token_burn_request_create_everusd(
            Origin::signed(ACCOUNT),
            5000
        ));
        assert_noop!(
            Evercity::token_burn_request_create_everusd(Origin::signed(ACCOUNT), 10000),
            RuntimeError::BurnRequestAlreadyExist
        );

        // make amend
        let ttl: u32 = <TestRuntime as crate::Config>::BurnRequestTtl::get();
        <pallet_timestamp::Pallet<TestRuntime>>::set_timestamp(ttl.into());

        assert_ok!(Evercity::token_burn_request_create_everusd(
            Origin::signed(ACCOUNT),
            10000
        ));
    })
}