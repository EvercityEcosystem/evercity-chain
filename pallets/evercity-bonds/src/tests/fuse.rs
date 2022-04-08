#![allow(clippy::from_over_into)]
use frame_support::{
    assert_noop, assert_ok, sp_io };
use crate::tests::mock::*;
use crate::{MASTER_ROLE_MASK};
use super::helpers::*;

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