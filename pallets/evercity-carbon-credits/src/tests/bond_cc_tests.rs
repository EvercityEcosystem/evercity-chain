use crate::Error;
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop, dispatch::{
    Vec, DispatchResultWithPostInfo,
}};
use pallet_evercity_bonds::{bond::BondInnerStruct, BondStruct};
use crate::standard::Standard;
use crate::annual_report::*;
use pallet_evercity_accounts::accounts::*;
use crate::tests::helpers::*;
use sp_std::vec;

type RuntimeError = Error<TestRuntime>;


#[test]
pub fn func() {
    new_test_ext().execute_with(|| {
        let issuer = 1;
        let investor1 = 2;
        let investor2 = 3;
        let investor3 = 4;

        // let inner_bond = get_test_inner_bond();
        let bond = get_test_bond();
    });
}