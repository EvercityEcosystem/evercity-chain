use crate::Error;
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop, dispatch::{
    Vec, DispatchResultWithPostInfo,
}};
use crate::standard::Standard;
use crate::annual_report::*;
use pallet_evercity_accounts::accounts::*;
use crate::tests::helpers::*;
use sp_std::vec;

type RuntimeError = Error<TestRuntime>;


#[test]
pub fn func() {
    new_test_ext().execute_with(|| {
        let a = 1;
        EvercityBonds::aaa();
    });
}