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

type Evercity = Module<TestRuntime>;
type Timestamp = pallet_timestamp::Module<TestRuntime>;
type Moment = <TestRuntime as pallet_timestamp::Config>::Moment;
type BondInnerStruct = BondInnerStructOf<TestRuntime>;
type BondStruct = BondStructOf<TestRuntime>;
type RuntimeError = Error<TestRuntime>;
type AccountId = <TestRuntime as frame_system::Config>::AccountId;
type BondUnitSaleLotStruct = BondUnitSaleLotStructOf<TestRuntime>;


#[test]
fn fuse_is_blone() {
    new_test_ext().execute_with(|| {
        let fuse = Evercity::fuse();
        assert_eq!(fuse, true);

        assert_noop!(
            Evercity::set_master(Origin::signed(2),),
            RuntimeError::InvalidAction
        );
    })
}

#[test]
fn fuse_is_intact_on_bare_storage() {
    let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap()
        .into();

    ext.execute_with(|| {
        assert_eq!(Evercity::fuse(), false);

        assert_noop!(
            Evercity::account_add_with_role_and_data(Origin::signed(1), 101, MASTER_ROLE_MASK, 0),
            RuntimeError::AccountNotAuthorized
        );
        assert_ok!(Evercity::set_master(Origin::signed(1),));
        // make amend
        assert_ok!(Evercity::account_add_with_role_and_data(
            Origin::signed(1),
            101,
            MASTER_ROLE_MASK,
            0
        ));

        assert_eq!(Evercity::fuse(), true);
        assert_noop!(
            Evercity::set_master(Origin::signed(2),),
            RuntimeError::InvalidAction
        );
    });
}