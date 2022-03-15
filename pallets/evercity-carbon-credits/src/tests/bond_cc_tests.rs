use crate::Error;
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop, dispatch::{
    Vec, DispatchResultWithPostInfo,
}};
use pallet_evercity_bonds::{bond::BondInnerStruct, BondStruct, BondId};
use crate::standard::Standard;
use crate::annual_report::*;
use pallet_evercity_accounts::accounts::*;
use crate::tests::helpers::*;
use sp_std::vec;

type RuntimeError = Error<TestRuntime>;

#[test]
pub fn it_works_create_bond_project() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let bond = get_test_bond();
        let _ = EvercityBonds::create_test_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];

        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);

        let create_project_result = 
            CarbonCredits::create_bond_project(
                Origin::signed(issuer), 
                standard.clone(), 
                create_project_documentation_file(issuer), 
                bond_id
            );

        assert_ok!(create_project_result, ().into());
    });
}

#[test]
pub fn it_fails_create_bond_project_not_bond_issuer() {
    new_test_ext().execute_with(|| {
        let bond_issuer = 500;
        let not_issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let bond = get_test_bond();
        let _ = EvercityBonds::create_test_finished_bond(bond_issuer, bond_id, bond.inner);
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        let standard = Standard::GOLD_STANDARD_BOND;

        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);

        let create_project_result = 
            CarbonCredits::create_bond_project(
                Origin::signed(not_issuer), 
                standard.clone(), 
                create_project_documentation_file(not_issuer), 
                bond_id
            );

        assert_noop!(create_project_result, RuntimeError::NotAnIssuer);
    });
}

#[test]
pub fn it_fails_create_bond_project_bond_not_finished() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let bond = get_test_bond();
        let _ = EvercityBonds::create_test_not_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];

        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);

        let create_project_result = 
            CarbonCredits::create_bond_project(
                Origin::signed(issuer), 
                standard.clone(), 
                create_project_documentation_file(issuer), 
                bond_id
            );

        assert_noop!(create_project_result, RuntimeError::BondNotFinished);
    });
}