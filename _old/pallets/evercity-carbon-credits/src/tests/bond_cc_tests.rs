use crate::Error;
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_noop,};
use pallet_evercity_bonds::{bond::{CarbonUnitsMetadata, CarbonDistribution}, BondId};
use crate::standard::Standard;
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
        let carbon_distribution = CarbonDistribution{
            investors: 70_000,
            issuer: 30_000,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100000,
            carbon_distribution
        };

        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);

        let create_project_result = 
            CarbonCredits::create_bond_project(
                Origin::signed(issuer), 
                standard, 
                create_project_documentation_file(issuer), 
                bond_id
            );

        assert_ok!(create_project_result, ().into());
    });
}


#[test]
pub fn it_works_create_bond_project_bond_is_active() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 70_000,
            issuer: 30_000,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100000,
            carbon_distribution
        };

        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_active_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);

        let create_project_result = 
            CarbonCredits::create_bond_project(
                Origin::signed(issuer), 
                standard, 
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
        let carbon_distribution = CarbonDistribution{
            investors: 70_000,
            issuer: 30_000,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_finished_bond(bond_issuer, bond_id, bond.inner);
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        let standard = Standard::GOLD_STANDARD_BOND;

        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);

        let create_project_result = 
            CarbonCredits::create_bond_project(
                Origin::signed(not_issuer), 
                standard, 
                create_project_documentation_file(not_issuer), 
                bond_id
            );

        assert_noop!(create_project_result, RuntimeError::NotAnIssuer);
    });
}

#[test]
pub fn it_fails_create_bond_project_bond_not_finished_or_active() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 70_000,
            issuer: 30_000,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_not_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];

        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);

        let create_project_result = 
            CarbonCredits::create_bond_project(
                Origin::signed(issuer), 
                standard, 
                create_project_documentation_file(issuer), 
                bond_id
            );

        assert_noop!(create_project_result, RuntimeError::BondNotActiveOrFinished);
    });
}

#[test]
pub fn it_works_release_bond_carbon_credits1() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 10_000,
            issuer: 90_000,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100_000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);
        let cc_count = 1_000_000;
        let proj_id = 666;
        let asset_id = 1;
        CarbonCredits::create_test_bond_project(issuer, bond_id, cc_count, standard, proj_id, crate::project::REGISTERED, crate::annual_report::REPORT_ISSUED);
        let release_result = CarbonCredits::release_bond_carbon_credits(Origin::signed(issuer), proj_id, asset_id);
        let balance_investor1 = Assets::balance(asset_id, investor1);
        let balance_investor2 = Assets::balance(asset_id, investor2);
        let balance_investor3 = Assets::balance(asset_id, investor3);
        let balance_issuer = Assets::balance(asset_id, issuer);

        assert_eq!(balance_investor1, 50_000);
        assert_eq!(balance_investor2, 30_000);
        assert_eq!(balance_investor3, 20_000);
        assert_eq!(balance_issuer, 900_000);
        assert_ok!(release_result, ().into());
    });
}

#[test]
pub fn it_works_release_bond_carbon_credits1_bond_is_active() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 10_000,
            issuer: 90_000,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100_000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_active_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);
        let cc_count = 1_000_000;
        let proj_id = 666;
        let asset_id = 1;
        CarbonCredits::create_test_bond_project(issuer, bond_id, cc_count, standard, proj_id, crate::project::REGISTERED, crate::annual_report::REPORT_ISSUED);
        let release_result = CarbonCredits::release_bond_carbon_credits(Origin::signed(issuer), proj_id, asset_id);
        let balance_investor1 = Assets::balance(asset_id, investor1);
        let balance_investor2 = Assets::balance(asset_id, investor2);
        let balance_investor3 = Assets::balance(asset_id, investor3);
        let balance_issuer = Assets::balance(asset_id, issuer);

        assert_eq!(balance_investor1, 50_000);
        assert_eq!(balance_investor2, 30_000);
        assert_eq!(balance_investor3, 20_000);
        assert_eq!(balance_issuer, 900_000);
        assert_ok!(release_result, ().into());
    });
}

#[test]
pub fn it_works_release_bond_carbon_credits2() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let investor2 = 4;
        let investor3 = 5;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 100_000,
            issuer: 0,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100_000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);
        let cc_count = 1_000_000;
        let proj_id = 666;
        let asset_id = 1;
        CarbonCredits::create_test_bond_project(issuer, bond_id, cc_count, standard, proj_id, crate::project::REGISTERED, crate::annual_report::REPORT_ISSUED);
        let release_result = CarbonCredits::release_bond_carbon_credits(Origin::signed(issuer), proj_id, asset_id);
        let balance_investor1 = Assets::balance(asset_id, investor1);
        let balance_investor2 = Assets::balance(asset_id, investor2);
        let balance_investor3 = Assets::balance(asset_id, investor3);
        let balance_issuer = Assets::balance(asset_id, issuer);

        assert_eq!(balance_investor1, 500_000);
        assert_eq!(balance_investor2, 300_000);
        assert_eq!(balance_investor3, 200_000);
        assert_eq!(balance_issuer, 0);
        assert_ok!(release_result, ().into());
    });
}

#[test]
pub fn it_works_release_bond_carbon_credits3() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor1 = 3;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 100_000,
            issuer: 0,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100_000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);
        let cc_count = 1_000_000;
        let proj_id = 666;
        let asset_id = 1;
        CarbonCredits::create_test_bond_project(issuer, bond_id, cc_count, standard, proj_id, crate::project::REGISTERED, crate::annual_report::REPORT_ISSUED);
        let release_result = CarbonCredits::release_bond_carbon_credits(Origin::signed(issuer), proj_id, asset_id);
        let balance_investor1 = Assets::balance(asset_id, investor1);
        let balance_issuer = Assets::balance(asset_id, issuer);

        assert_eq!(balance_investor1, 1_000_000);
        assert_eq!(balance_issuer, 0);
        assert_ok!(release_result, ().into());
    });
}

#[test]
pub fn it_fails_release_bond_carbon_credits_project_not_registered() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor2 = 4;
        let investor3 = 5;
        let investor1 = 3;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 100_000,
            issuer: 0,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100_000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);
        let cc_count = 1_000_000;
        let proj_id = 666;
        let asset_id = 1;
        CarbonCredits::create_test_bond_project(issuer, bond_id, cc_count, standard, proj_id, 0, crate::annual_report::REPORT_ISSUED);
        let release_result = CarbonCredits::release_bond_carbon_credits(Origin::signed(issuer), proj_id, asset_id);

        assert_eq!(0, Assets::balance(asset_id, investor1));
        assert_eq!(0, Assets::balance(asset_id, investor2));
        assert_eq!(0, Assets::balance(asset_id, investor3));
        assert_eq!(0, Assets::balance(asset_id, issuer));
        assert_noop!(release_result, RuntimeError::ProjectNotRegistered);
    });
}

#[test]
pub fn it_fails_release_bond_carbon_credits_report_not_issued() {
    new_test_ext().execute_with(|| {
        let issuer = ROLES[1].0;
        let investor2 = 4;
        let investor3 = 5;
        let investor1 = 3;
        let bond_id: BondId = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1].into();
        let carbon_distribution = CarbonDistribution{
            investors: 100_000,
            issuer: 0,
            evercity: None,
            project_developer: None,
        };
        let carbon_metadata = CarbonUnitsMetadata{
            count: 100_000,
            carbon_distribution
        };
        let bond = get_test_bond(carbon_metadata);
        let _ = EvercityBonds::create_test_finished_bond(issuer, bond_id, bond.inner);
        let standard = Standard::GOLD_STANDARD_BOND;
        let units = vec![(investor1, 50), (investor2, 30), (investor3, 20)];
        EvercityBonds::add_test_bond_unit_packages(&bond_id, units);
        let cc_count = 1_000_000;
        let proj_id = 666;
        let asset_id = 1;
        CarbonCredits::create_test_bond_project(issuer, bond_id, cc_count, standard, proj_id, crate::project::REGISTERED, 0);
        let release_result = CarbonCredits::release_bond_carbon_credits(Origin::signed(issuer), proj_id, asset_id);

        assert_eq!(0, Assets::balance(asset_id, investor1));
        assert_eq!(0, Assets::balance(asset_id, investor2));
        assert_eq!(0, Assets::balance(asset_id, investor3));
        assert_eq!(0, Assets::balance(asset_id, issuer));
        assert_noop!(release_result, RuntimeError::ReportNotIssued);
    });
}