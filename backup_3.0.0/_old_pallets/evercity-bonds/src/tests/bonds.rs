#![allow(clippy::from_over_into)]
use frame_support::{
    assert_noop, assert_ok,
    Blake2_256, StorageHasher,
};

use crate::tests::mock::*;
use crate::{
    BondId, BondImpactReportStruct, BondPeriodNumber, BondState, BondStructOf,
    BondUnitAmount, EverUSDBalance,
    DEFAULT_DAY_DURATION,
};
use super::helpers::*;

#[test]
fn bond_transfer_units() {
    new_test_ext().execute_with(|| {
        let mut from_package = create_bond_unit_package(vec![5, 2, 10, 1]);
        let mut to_package = create_bond_unit_package(vec![]);
        assert_ok!(Evercity::transfer_bond_units(
            &mut from_package,
            &mut to_package,
            3
        ));

        assert_eq!(bond_unit_package_amount(from_package), vec![10, 5]);
        assert_eq!(bond_unit_package_amount(to_package), vec![1, 2]);

        let mut from_package = create_bond_unit_package(vec![5, 2, 10, 1]);
        let mut to_package = create_bond_unit_package(vec![]);

        assert_ok!(Evercity::transfer_bond_units(
            &mut from_package,
            &mut to_package,
            10
        ));

        assert_eq!(bond_unit_package_amount(from_package), vec![8]);
        assert_eq!(bond_unit_package_amount(to_package), vec![1, 2, 5, 2]);

        let mut from_package = create_bond_unit_package(vec![5, 2, 10, 1]);
        let mut to_package = create_bond_unit_package(vec![]);

        assert_ok!(Evercity::transfer_bond_units(
            &mut from_package,
            &mut to_package,
            2
        ));

        assert_eq!(bond_unit_package_amount(from_package), vec![10, 5, 1]);
        assert_eq!(bond_unit_package_amount(to_package), vec![1, 1]);

        let mut from_package = create_bond_unit_package(vec![5, 2, 10, 1]);
        let mut to_package = create_bond_unit_package(vec![]);

        assert_noop!(
            Evercity::transfer_bond_units(&mut from_package, &mut to_package, 20),
            RuntimeError::BondParamIncorrect
        );
    });
}

#[test]
fn bond_validation() {
    new_test_ext().execute_with(|| {
        let bond = get_test_bond();
        assert_eq!(bond.inner.is_valid(DEFAULT_DAY_DURATION), true);
    });
}

#[test]
fn bond_stable_is_valid() {
    let bond = get_test_bond_stable().inner;
    assert_eq!(bond.is_valid(DEFAULT_DAY_DURATION), true);
}

#[test]
fn incorrect_bond_validation() {
    let process_test = |is_stable: bool| {
        assert_eq!(get_test_bond_incorrect(0, 12, 4_000_000_000_000, is_stable).inner.is_valid(DEFAULT_DAY_DURATION), false);
        assert_eq!(get_test_bond_incorrect(crate::bond::MIN_PAYMENT_PERIOD*DEFAULT_DAY_DURATION, 0, 4_000_000_000_000, is_stable).inner.is_valid(DEFAULT_DAY_DURATION), false);
        assert_eq!(get_test_bond_incorrect(crate::bond::MIN_PAYMENT_PERIOD*DEFAULT_DAY_DURATION, 12, 0, is_stable).inner.is_valid(DEFAULT_DAY_DURATION), false);
    };

    process_test(false);
    process_test(true);
}

#[test]
fn bond_with_carbon_is_valid() {
    let carbon_metadata1 = crate::bond::CarbonUnitsMetadata{
        count: 100_000,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 100_000,
            issuer: 0,
            evercity: None,
            project_developer: None,
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata2 = crate::bond::CarbonUnitsMetadata{
        count: 100_000,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 70_000,
            issuer: 30_000,
            evercity: None,
            project_developer: None,
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata3 = crate::bond::CarbonUnitsMetadata{
        count: 0,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 70_000,
            issuer: 20_000,
            evercity: Some((1, 10_000)),
            project_developer: None,
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata4 = crate::bond::CarbonUnitsMetadata{
        count: 0,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 70_000,
            issuer: 20_000,
            evercity: None,
            project_developer: Some((1, 10_000)),
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata5 = crate::bond::CarbonUnitsMetadata{
        count: 0,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 50_000,
            issuer: 20_000,
            evercity: Some((1, 20_000)),
            project_developer: Some((2, 10_000)),
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata6 = crate::bond::CarbonUnitsMetadata{
        count: 0,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 0,
            issuer: 0,
            evercity: Some((1, 50_000)),
            project_developer: Some((2, 50_000)),
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };

    assert!(get_test_bond_with_carbon_metadata(carbon_metadata1).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(get_test_bond_with_carbon_metadata(carbon_metadata2).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(get_test_bond_with_carbon_metadata(carbon_metadata3).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(get_test_bond_with_carbon_metadata(carbon_metadata4).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(get_test_bond_with_carbon_metadata(carbon_metadata5).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(get_test_bond_with_carbon_metadata(carbon_metadata6).inner.is_valid(DEFAULT_DAY_DURATION));
}

#[test]
fn bond_with_carbon_is_not_valid() {
    let carbon_metadata1 = crate::bond::CarbonUnitsMetadata{
        count: 100_000,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 100_000,
            issuer: 100_000,
            evercity: None,
            project_developer: None,
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata2 = crate::bond::CarbonUnitsMetadata{
        count: 100_000,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 0,
            issuer: 30_000,
            evercity: None,
            project_developer: None,
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata3 = crate::bond::CarbonUnitsMetadata{
        count: 0,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 70_000,
            issuer: 20_000,
            evercity: Some((1, 50_000)),
            project_developer: None,
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata4 = crate::bond::CarbonUnitsMetadata{
        count: 0,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 70_000,
            issuer: 20_000,
            evercity: None,
            project_developer: Some((1, 30_000)),
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };
    let carbon_metadata5 = crate::bond::CarbonUnitsMetadata{
        count: 0,
        carbon_distribution: crate::bond::CarbonDistribution{
            investors: 50_000,
            issuer: 20_000,
            evercity: Some((1, 20_000)),
            project_developer: Some((2, 20_000)),
        },
        account_investments: Vec::<(AccountId, u32)>::new()
    };

    assert!(!get_test_bond_with_carbon_metadata(carbon_metadata1).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(!get_test_bond_with_carbon_metadata(carbon_metadata2).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(!get_test_bond_with_carbon_metadata(carbon_metadata3).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(!get_test_bond_with_carbon_metadata(carbon_metadata4).inner.is_valid(DEFAULT_DAY_DURATION));
    assert!(!get_test_bond_with_carbon_metadata(carbon_metadata5).inner.is_valid(DEFAULT_DAY_DURATION));
}

#[test]
fn bond_with_carbon_redeem_ok() {
        // YMT - yield to maturity - total coupon yield after bond redemption
        const ACCOUNT: u64 = 3;
        const INVESTOR1: u64 = 4;
        const INVESTOR2: u64 = 6;
    
        let bondid: BondId = "BOND".into();
    
        new_test_ext().execute_with(|| {
            let c_bond = get_test_bond_stable_carbon();

            bond_grand_everusd();
            assert!(Evercity::evercity_balance().is_ok());
            let initial_balace1 = Evercity::balance_everusd(&INVESTOR1);
            let initial_balace2 = Evercity::balance_everusd(&INVESTOR2);
            bond_activate(bondid, ACCOUNT, c_bond.inner);
            assert!(Evercity::evercity_balance().is_ok());
    
            let chain_bond_item = Evercity::get_bond(&bondid);
            assert_eq!(chain_bond_item.active_start_date, 30000);
            assert_eq!(chain_bond_item.issued_amount, 1200);
    
            let num_periods = chain_bond_item.get_periods();
            // all period except start period will have interest rate = interest_rate_base_value
            // for start period interest rate will be  interest_rate_start_period_value
            for period in 0..num_periods - 1 {
                assert_ok!(Evercity::set_impact_data(
                    &bondid,
                    period,
                    chain_bond_item.inner.impact_data_baseline[period as usize].unwrap_or(0)
                ));
            }
              // go to the last period
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + days2timestamp(120 + chain_bond_item.inner.bond_duration * 30 + 1),
        );
        // add extra everusd to pay off coupon yield
        assert_ok!(add_token(ACCOUNT, 125_000_000_000_000));
        assert!(Evercity::evercity_balance().is_ok());

        let account_investments = Evercity::get_bond_account_investment(&bondid);
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT), bondid));
        assert!(Evercity::bond_check_invariant(&bondid));
        // withdraw coupon & principal value
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid
        ));
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR2),
            bondid
        ));

        assert!(Evercity::evercity_balance().is_ok());

        let chain_bond_item = Evercity::get_bond(&bondid);
        let yield1 = Evercity::balance_everusd(&INVESTOR1) - initial_balace1;
        let yield2 = Evercity::balance_everusd(&INVESTOR2) - initial_balace2;

        assert_eq!(
            yield1 + yield2 + Evercity::balance_everusd(&ACCOUNT),
            125_000_000_000_000
        );
        assert_eq!(yield1, yield2);
        assert_eq!(chain_bond_item.state, BondState::FINISHED);
        
        // check that carbon metadata stores bond distribution 
        assert_eq!(account_investments, chain_bond_item.inner.carbon_metadata.unwrap().account_investments);
    });
}


#[test]
fn bond_check_equation() {
    new_test_ext().execute_with(|| {
        let bond1 = get_test_bond();

        let mut bond2 = bond1.clone();
        assert_eq!(bond1.inner, bond2.inner);
        bond2.inner.docs_pack_root_hash_legal = Blake2_256::hash(b"").into();

        assert!(bond1.inner.is_financial_options_eq(&bond2.inner));
        assert_ne!(bond1.inner, bond2.inner);

        bond2.inner.docs_pack_root_hash_legal = bond1.inner.docs_pack_root_hash_legal;
        bond2.inner.payment_period += 1;

        assert!(!bond1.inner.is_financial_options_eq(&bond2.inner));
        assert_ne!(bond1.inner, bond2.inner);
    });
}

#[test]
fn bond_interest_min_max() {
    new_test_ext().execute_with(|| {
        let bond = get_test_bond();
        let impact_base_value = bond.inner.impact_data_baseline[0];
        // full amplitude
        assert_eq!(
            bond.calc_effective_interest_rate(impact_base_value.unwrap_or(0), impact_base_value.unwrap_or(0)),
            bond.inner.interest_rate_base_value
        );
        assert_eq!(
            bond.calc_effective_interest_rate(
                impact_base_value.unwrap_or(0),
                bond.inner.impact_data_max_deviation_cap.unwrap_or(0),
            ),
            bond.inner.interest_rate_margin_floor.unwrap_or(0)
        );
        assert_eq!(
            bond.calc_effective_interest_rate(
                impact_base_value.unwrap_or(0),
                bond.inner.impact_data_max_deviation_cap.unwrap_or(0) + 1,
            ),
            bond.inner.interest_rate_margin_floor.unwrap_or(0)
        );
        assert_eq!(
            bond.calc_effective_interest_rate(
                impact_base_value.unwrap_or(0),
                bond.inner.impact_data_max_deviation_floor.unwrap_or(0),
            ),
            bond.inner.interest_rate_margin_cap.unwrap_or(0)
        );
        assert_eq!(
            bond.calc_effective_interest_rate(
                impact_base_value.unwrap_or(0),
                bond.inner.impact_data_max_deviation_floor.unwrap_or(0) - 1,
            ),
            bond.inner.interest_rate_margin_cap.unwrap_or(0)
        );

        // partial amplitude
        assert_eq!(
            bond.calc_effective_interest_rate(impact_base_value.unwrap_or(0), 25000_u64),
            1500
        );
        assert_eq!(
            bond.calc_effective_interest_rate(impact_base_value.unwrap_or(0), 29000_u64),
            1100
        );

        assert_eq!(
            bond.calc_effective_interest_rate(impact_base_value.unwrap_or(0), 17000_u64),
            3000
        );
        assert_eq!(
            bond.calc_effective_interest_rate(impact_base_value.unwrap_or(0), 15000_u64),
            3666
        );
    });
}

#[test]
fn bond_period_interest_rate() {
    new_test_ext().execute_with(|| {
        let bond = get_test_bond();

        assert!(bond
            .inner
            .impact_data_baseline
            .iter()
            .all(|&v| v == Some(20000_u64)));

        let reports: Vec<BondImpactReportStruct> = vec![
            //missing report
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 20000_u64,
                signed: true,
            },
            //missing report
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            // worst result and maximal interest rate value
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 14000_u64,
                signed: true,
            },
            //missing report. it cannot make interest rate worse
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            // very good result lead to mininal interest rate
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 100000_u64,
                signed: true,
            },
            //first missing report.
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            //second missing report.
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
        ];

        assert_eq!(
            bond.inner.interest_rate_start_period_value,
            Some(Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 0))
        );

        assert_eq!(
            bond.inner.interest_rate_start_period_value.unwrap_or(0)
                + bond.inner.interest_rate_penalty_for_missed_report.unwrap_or(0),
            Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 1)
        );

        assert_eq!(
            bond.inner.interest_rate_base_value,
            Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 2)
        );

        assert_eq!(
            bond.inner.interest_rate_base_value
                + bond.inner.interest_rate_penalty_for_missed_report.unwrap_or(0),
            Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 3)
        );

        assert_eq!(
            bond.inner.interest_rate_margin_cap,
            Some(Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 4))
        );
        // missing report cannot increase insterested rate above maximal value
        assert_eq!(
            bond.inner.interest_rate_margin_cap,
            Some(Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 5))
        );

        assert_eq!(
            bond.inner.interest_rate_margin_floor,
            Some(Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 6))
        );

        assert_eq!(
            bond.inner.interest_rate_margin_floor.unwrap_or(0)
                + bond.inner.interest_rate_penalty_for_missed_report.unwrap_or(0),
            Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 7)
        );

        assert_eq!(
            bond.inner.interest_rate_margin_floor.unwrap_or(0)
                + 2 * bond.inner.interest_rate_penalty_for_missed_report.unwrap_or(0),
            Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 8)
        );
    });
}

#[test]
fn bond_create_with_small_start_period() {
    let bondid1: BondId = "B1".into();
    const ACCOUNT: u64 = 3;
    new_test_ext().execute_with(|| {
        let mut bond = get_test_bond().inner;
        bond.start_period = Some(bond.impact_data_send_period);
        assert!(bond.start_period.unwrap_or(0) < bond.payment_period);
        assert!(bond.start_period.unwrap_or(0) >= bond.impact_data_send_period);
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid1,
            bond
        ));
    })
}

#[test]
fn bond_create_with_min_period() {
    let bondid1: BondId = "B1".into();
    const ACCOUNT: u64 = 3;

    new_test_ext().execute_with(|| {
        let mut bond = get_test_bond().inner;
        bond.bond_finishing_period = DEFAULT_DAY_DURATION;
        bond.payment_period = DEFAULT_DAY_DURATION;
        bond.start_period = Some(DEFAULT_DAY_DURATION);
        bond.interest_pay_period = Some(DEFAULT_DAY_DURATION);
        bond.impact_data_send_period = DEFAULT_DAY_DURATION;

        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid1,
            bond
        ));
    })
}

#[test]
fn bond_create_series() {
    let bond = get_test_bond();
    let bondid1: BondId = "B1".into();
    let bondid2: BondId = "B2".into();
    let bondid3: BondId = "B3".into();

    const ACCOUNT: u64 = 3;

    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid1,
            bond.inner.clone()
        ));
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid2,
            bond.inner.clone()
        ));
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid3,
            bond.inner.clone()
        ));
        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid3, bond.inner.clone()),
            RuntimeError::BondAlreadyExists
        );
    });
}

#[test]
// unique case scenario
fn bond_buy_bond_uc() {
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const AUDITOR: u64 = 5;
    const INVESTOR1: u64 = 4;

    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 4_000_000_000_000_000));

        let mut bond = get_test_bond().inner;
        bond.mincap_deadline = 50_000;
        bond.bond_units_mincap_amount = 1000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond
        ));
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(50_000);
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            1000
        ));
        assert_ok!(Evercity::bond_set_auditor(
            Origin::signed(BOND_ARRANGER),
            bondid,
            AUDITOR
        ));
        assert_ok!(Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 2));

        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.issued_amount, 1000);
        assert_eq!(Evercity::balance_everusd(&ACCOUNT), 4_000_000_000_000_000);
        assert_eq!(Evercity::balance_everusd(&INVESTOR1), 0);
    });
}

#[test]
fn bond_try_create_by_nonissuer() {
    let bond = get_test_bond();
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        for acc in iter_accounts().filter(|acc| !EvercityAccounts::account_is_issuer(acc)) {
            assert_noop!(
                Evercity::bond_add_new(Origin::signed(acc), bondid, bond.inner.clone()),
                RuntimeError::AccountNotAuthorized
            );
        }
    });
}

#[test]
fn bond_try_create_incorrect_stable_or_unstable() {
    let process_test = |payment_period: u32, bond_duration: u32, bond_unit_base_prize: u64| {
        let stable_bond = get_test_bond_incorrect(payment_period, bond_duration, bond_unit_base_prize, true);
        let bond = get_test_bond_incorrect(payment_period, bond_duration, bond_unit_base_prize, false);
        let stable_bondid: BondId = "BOND_S".into();
        let bondid: BondId = "BOND".into();
    
        new_test_ext().execute_with(|| {
            for acc in iter_accounts().filter(|acc| EvercityAccounts::account_is_issuer(acc)) {
                assert_noop!(
                    Evercity::bond_add_new(Origin::signed(acc), stable_bondid, stable_bond.inner.clone()),
                    RuntimeError::BondParamIncorrect
                );
    
                assert_noop!(
                    Evercity::bond_add_new(Origin::signed(acc), bondid, bond.inner.clone()),
                    RuntimeError::BondParamIncorrect
                );
            }
        });
    };
    process_test(0, 12, 4_000_000_000_000,);
    process_test(crate::bond::MIN_PAYMENT_PERIOD*DEFAULT_DAY_DURATION, 0, 4_000_000_000_000);
    process_test(crate::bond::MIN_PAYMENT_PERIOD*DEFAULT_DAY_DURATION, 12, 0);
}

#[test]
fn bond_try_activate_without_release() {
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();

        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            get_test_bond().inner
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(days2timestamp(1));
        // try to buy some bonds in prepare state
        assert_noop!(
            Evercity::bond_unit_package_buy(Origin::signed(INVESTOR1), bondid, 0, 600),
            RuntimeError::BondStateNotPermitAction
        );
        // try to activate bond
        assert_noop!(
            Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 0),
            RuntimeError::BondStateNotPermitAction
        );
    })
}

#[test]
fn bond_try_activate_by_non_bond_arranger() {
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const AUDITOR: u64 = 5;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();

        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            get_test_bond().inner
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(days2timestamp(1));
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));
        // try to buy some bonds in prepare state
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            1000
        ));
        assert_ok!(Evercity::bond_set_auditor(
            Origin::signed(BOND_ARRANGER),
            bondid,
            AUDITOR
        ));
        // try to activate bond
        for acc in iter_accounts().filter(|acc| !EvercityAccounts::account_is_bond_arranger(acc)) {
            assert_noop!(
                Evercity::bond_activate(Origin::signed(acc), bondid, 0),
                RuntimeError::AccountNotAuthorized
            );
        }
        // make amend
        assert_ok!(Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 2));
    })
}

#[test]
fn bond_try_activate_without_auditor() {
    let mut bond = get_test_bond();
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const AUDITOR: u64 = 5;

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 50_000_000_000_000_000));

        bond.inner.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            1000
        ));
        assert_noop!(
            Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 1),
            RuntimeError::BondIsNotConfigured
        );
        // make amends
        assert_ok!(Evercity::bond_set_auditor(
            Origin::signed(BOND_ARRANGER),
            bondid,
            AUDITOR
        ));
        assert_ok!(Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 2));
    });
}

#[test]
fn bond_try_revoke_after_release() {
    const ACCOUNT: u64 = 3;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);

        assert_noop!(
            Evercity::bond_withdraw(Origin::signed(ACCOUNT), bondid),
            RuntimeError::BondStateNotPermitAction
        );
        assert_noop!(
            Evercity::bond_revoke(Origin::signed(ACCOUNT), bondid),
            RuntimeError::BondStateNotPermitAction
        );
    });
}

#[test]
fn bond_zero_send_period_is_stable() {
    let bond = get_test_bond_stable().inner;
    assert_eq!(bond.is_stable(), true);
}

#[test]
fn bond_nonzero_send_period_is_not_stable() {
    let bond = get_test_bond().inner;
    assert_eq!(bond.is_stable(), false);
}

#[test]
fn bond_stable_calc_bond_interest_rate() {
    let bond = get_test_bond_stable();
    let _base_value = bond.inner.interest_rate_base_value as u64;

    let reports: Vec<BondImpactReportStruct> = vec![
            //missing report
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            //missing report
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            // worst result and maximal interest rate value
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            //missing report. it cannot make interest rate worse
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            // very good result lead to mininal interest rate
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            //first missing report.
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
            //second missing report.
            BondImpactReportStruct {
                create_period: 0,
                impact_data: 0,
                signed: false,
            },
        ];

        assert_eq!(
            Evercity::calc_bond_interest_rate(&bond, reports.as_ref(), 0),
            bond.inner.interest_rate_base_value
        );
        
}

#[test]
fn bond_try_withdraw_before_deadline() {
    let mut bond = get_test_bond();
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 50_000_000_000_000_000));

        bond.inner.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            100
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(49000);
        assert_noop!(
            Evercity::bond_withdraw(Origin::signed(BOND_ARRANGER), bondid,),
            RuntimeError::BondStateNotPermitAction
        );
        // make amends
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(51000);
        assert_ok!(Evercity::bond_withdraw(Origin::signed(BOND_ARRANGER), bondid,));
        let chain_bond_item = Evercity::get_bond(&bondid);

        assert_eq!(chain_bond_item.state, BondState::PREPARE);
        assert_eq!(chain_bond_item.bond_credit, 0);
    });
}

#[test]
fn bond_try_withdraw_by_investor() {
    let mut bond = get_test_bond();
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 50_000_000_000_000_000));

        bond.inner.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            100
        ));

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(51000);
        assert_noop!(
            Evercity::bond_withdraw(Origin::signed(INVESTOR1), bondid,),
            RuntimeError::BondAccessDenied
        );

        // make amends
        assert_ok!(Evercity::bond_withdraw(Origin::signed(BOND_ARRANGER), bondid,));

        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.state, BondState::PREPARE);
        assert_eq!(chain_bond_item.issued_amount, 0);
        assert_eq!(chain_bond_item.bond_credit, 0);
        assert_eq!(chain_bond_item.bond_debit, 0);
        assert_eq!(
            Evercity::balance_everusd(&INVESTOR1),
            50_000_000_000_000_000
        );

        assert_eq!(Evercity::bond_packages(&bondid).is_empty(), true);
    });
}

#[test]
fn bond_try_manage_foreign_bond() {
    let mut bond = get_test_bond();
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const MANAGER: u64 = 8;

    new_test_ext().execute_with(|| {
        bond.inner.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));

        let mut update = get_test_bond().inner;
        update.mincap_deadline = 60000;

        for acc in iter_accounts().filter(|acc| *acc != ACCOUNT) {
            assert_noop!(
                Evercity::bond_update(Origin::signed(acc), bondid, 0, update.clone()),
                RuntimeError::BondAccessDenied
            );
        }
        // make amend
        assert_ok!(Evercity::bond_set_manager(
            Origin::signed(BOND_ARRANGER),
            bondid,
            MANAGER
        ));
        assert_ok!(Evercity::bond_update(
            Origin::signed(MANAGER),
            bondid,
            1,
            update
        ),);
    });
}

#[test]
fn bond_try_update_after_release() {
    let mut bond = get_test_bond();
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const AUDITOR: u64 = 5;

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 50_000_000_000_000_000));

        bond.inner.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));

        // release bond
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));

        // hashes can be changed
        let mut update = get_test_bond().inner;
        update.docs_pack_root_hash_finance = Blake2_256::hash(b"merkle tree hash").into();
        assert_ok!(Evercity::bond_update(
            Origin::signed(ACCOUNT),
            bondid,
            1,
            update
        ));

        // the others cannot. TODO add other fields to check
        let mut update = get_test_bond().inner;
        update.payment_period *= 2;
        assert_noop!(
            Evercity::bond_update(Origin::signed(ACCOUNT), bondid, 2, update),
            RuntimeError::BondStateNotPermitAction
        );
        let mut update = get_test_bond().inner;
        update.bond_units_base_price = 3_000_000_000_000;
        assert_noop!(
            Evercity::bond_update(Origin::signed(ACCOUNT), bondid, 2, update),
            RuntimeError::BondStateNotPermitAction
        );
        let mut update = get_test_bond().inner;
        let unwrapped_impact_data_baseline = update.impact_data_baseline[0].unwrap_or(0) + 1;
        update.impact_data_baseline[0] = Some(unwrapped_impact_data_baseline);
        assert_noop!(
            Evercity::bond_update(Origin::signed(ACCOUNT), bondid, 2, update),
            RuntimeError::BondStateNotPermitAction
        );

        // buy bonds
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            2,
            1200
        ));
        assert_ok!(Evercity::bond_set_auditor(
            Origin::signed(BOND_ARRANGER),
            bondid,
            AUDITOR
        ));
        // activate
        assert_ok!(Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 3));

        let mut update = get_test_bond().inner;
        update.docs_pack_root_hash_finance = Blake2_256::hash(b"merkle tree hash").into();
        // try change after activation
        assert_noop!(
            Evercity::bond_update(Origin::signed(ACCOUNT), bondid, 4, update),
            RuntimeError::BondStateNotPermitAction
        );
    });
}

#[test]
fn bond_try_activate_insufficient_fund_raising() {
    let mut bond = get_test_bond();
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const AUDITOR: u64 = 5;

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 50_000_000_000_000_000));

        bond.inner.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));

        // try activate before been issued
        assert_noop!(
            Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 0),
            RuntimeError::BondStateNotPermitAction
        );
        // try buy bonds before been issued
        assert_noop!(
            Evercity::bond_unit_package_buy(Origin::signed(INVESTOR1), bondid, 0, 100),
            RuntimeError::BondStateNotPermitAction
        );
        // release bond
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));
        // buy limited number of bonds
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            100
        ));
        assert_ok!(Evercity::bond_set_auditor(
            Origin::signed(BOND_ARRANGER),
            bondid,
            AUDITOR
        ));

        assert_noop!(
            Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 2),
            RuntimeError::BondParamIncorrect
        );
        // make amends
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            2,
            900
        ));
        assert_ok!(Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 2));
    });
}

#[test]
fn bond_try_activate_expired_fund_raising() {
    let mut bond = get_test_bond();
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const AUDITOR: u64 = 5;

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 50_000_000_000_000_000));
        assert!(bond.inner.mincap_deadline < days2timestamp(21));

        bond.inner.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));

        // release bond
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));
        // buy limited number of bonds
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            100
        ));
        assert_ok!(Evercity::bond_set_auditor(
            Origin::signed(BOND_ARRANGER),
            bondid,
            AUDITOR
        ));

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(days2timestamp(21));

        assert_noop!(
            Evercity::bond_activate(Origin::signed(BOND_ARRANGER), bondid, 2),
            RuntimeError::BondParamIncorrect
        );
        // workaround
        assert_ok!(Evercity::bond_withdraw(Origin::signed(ACCOUNT), bondid));
        assert_eq!(Evercity::bond_packages(&bondid).is_empty(), true);
    });
}

#[test]
fn bond_try_create_with_overflow() {
    const ACCOUNT: u64 = 3;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        let mut bond = get_test_bond().inner;
        bond.bond_units_maxcap_amount = BondUnitAmount::MAX - 1;

        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid, bond),
            RuntimeError::BondParamIncorrect
        );
    });
}

#[test]
fn bond_try_buy_unit_with_overflow() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        let amount = bond.bond_units_maxcap_amount;
        bond_release(bondid, ACCOUNT, bond);

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(100_000);
        assert_noop!(
            Evercity::bond_unit_package_buy(
                Origin::signed(INVESTOR1),
                bondid,
                2,
                BondUnitAmount::MAX - amount
            ),
            RuntimeError::BondParamIncorrect
        );
    });
}

#[test]
fn bond_calc_coupon_yield_basic() {
    const ACCOUNT: u64 = 3;
    let bondid: BondId = "BOND2".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);

        let mut chain_bond_item = Evercity::get_bond(&bondid);

        assert_eq!(chain_bond_item.active_start_date, 30000);
        // pass first (index=0) period
        let mut moment: Moment =
            30000_u64 + (chain_bond_item.inner.start_period.unwrap_or(0)) as u64 * 1000_u64 + 1_u64;
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(moment);

        assert_eq!(bond_current_period(&chain_bond_item, moment), 1);
        assert!(
            Evercity::calc_and_store_bond_coupon_yield(&bondid, &mut chain_bond_item, moment) > 0
        );
        // second call should return false
        assert!(
            !Evercity::calc_and_store_bond_coupon_yield(&bondid, &mut chain_bond_item, moment) > 0
        );

        // pass second (index=1) period
        moment += chain_bond_item.inner.payment_period as u64 * 1000_u64;
        assert_eq!(bond_current_period(&chain_bond_item, moment), 2);
        chain_bond_item.bond_debit = 2000;

        assert!(
            Evercity::calc_and_store_bond_coupon_yield(&bondid, &mut chain_bond_item, moment) > 0
        );

        let bond_yields = Evercity::get_coupon_yields(&bondid);

        assert_eq!(bond_yields.len(), 2);
        assert_eq!(
            bond_yields[0].interest_rate,
            chain_bond_item.inner.interest_rate_start_period_value.unwrap_or(0)
        );
        assert_eq!(bond_yields[0].total_yield, 29_983_561_643_520);

        assert_eq!(
            bond_yields[1].interest_rate,
            chain_bond_item.inner.interest_rate_start_period_value.unwrap_or(0)
                + chain_bond_item
                    .inner
                    .interest_rate_penalty_for_missed_report.unwrap_or(0)
        );
        assert_eq!(bond_yields[1].total_yield, 39_057_534_246_240);
    });
}

#[test]
fn bond_calc_coupon_yield_advanced() {
    const ACCOUNT1: u64 = 3;
    const ACCOUNT2: u64 = 7;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid1: BondId = "BOND1".into();
    let bondid2: BondId = "BOND2".into();

    fn deposit(account: u64, bond: BondId, amount: EverUSDBalance) -> BondStructOf<TestRuntime> {
        assert_ok!(Evercity::bond_deposit_everusd(
            Origin::signed(account),
            bond,
            amount
        ));
        Evercity::get_bond(&bond)
    }

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid1, ACCOUNT1, get_test_bond().inner);
        bond_activate(bondid2, ACCOUNT2, get_test_bond().inner);

        let chain_bond_item1 = Evercity::get_bond(&bondid1);
        let chain_bond_item2 = Evercity::get_bond(&bondid2);

        assert_eq!(
            chain_bond_item1.active_start_date,
            chain_bond_item2.active_start_date
        );

        let start_moment = chain_bond_item1.active_start_date;

        // set impact data
        for period in 0..12_usize {
            assert_ok!(Evercity::set_impact_data(
                &bondid1,
                period as BondPeriodNumber,
                chain_bond_item1.inner.impact_data_baseline[period].unwrap_or(0)
            ));

            assert_ok!(Evercity::set_impact_data(
                &bondid2,
                period as BondPeriodNumber,
                chain_bond_item2.inner.impact_data_baseline[period].unwrap_or(0)
            ));
        }
        Evercity::set_balance(&INVESTOR1, 0);
        Evercity::set_balance(&INVESTOR2, 0);

        Evercity::set_balance(&ACCOUNT1, 124668493149600 + 4000 * 600 * 2 * UNIT);
        Evercity::set_balance(&ACCOUNT2, 124668493149600 + 4000 * 600 * 2 * UNIT);

        let mut chain_bond_item1 = deposit(ACCOUNT1, bondid1, 20000 * UNIT);
        let mut chain_bond_item2 = deposit(ACCOUNT2, bondid2, 20000 * UNIT);

        let now = start_moment + (160 * DEFAULT_DAY_DURATION) as u64 * 1000;

        Evercity::calc_and_store_bond_coupon_yield(&bondid1, &mut chain_bond_item1, now);
        Evercity::calc_and_store_bond_coupon_yield(&bondid2, &mut chain_bond_item2, now);

        let bond_yield = Evercity::get_coupon_yields(&bondid1);
        println!("bond 1 = {:?}", bond_yield);
        let bond_yield = Evercity::get_coupon_yields(&bondid2);
        println!("bond 2 = {:?}", bond_yield);

        chain_bond_item1 = deposit(ACCOUNT1, bondid1, 8000 * UNIT);
        chain_bond_item2 = deposit(ACCOUNT2, bondid2, 20000 * UNIT);

        let now = start_moment + (220 * DEFAULT_DAY_DURATION) as u64 * 1000;

        Evercity::calc_and_store_bond_coupon_yield(&bondid1, &mut chain_bond_item1, now);
        Evercity::calc_and_store_bond_coupon_yield(&bondid2, &mut chain_bond_item2, now);

        let bond_yield = Evercity::get_coupon_yields(&bondid1);
        println!("bond 1 = {:?}", bond_yield);
        assert_eq!(
            bond_yield
                .into_iter()
                .map(|x| x.total_yield)
                .collect::<Vec<_>>(),
            [
                29983561643520,
                37873972602360,
                45764383561200,
                53654794520040
            ]
        );

        let bond_yield = Evercity::get_coupon_yields(&bondid2);
        println!("bond 2 = {:?}", bond_yield);

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(now);

        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid1,
        ));
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR2),
            bondid1,
        ));

        let balance1 = Evercity::balances_everusd(INVESTOR1);
        let balance2 = Evercity::balances_everusd(INVESTOR2);
        assert_eq!(balance1, balance2);
        assert_eq!(balance1, 14000 * UNIT);
        println!("balance investor1 {}, investor2 {}", balance1, balance2);

        deposit(ACCOUNT1, bondid1, 30000 * UNIT);

        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid1,
        ));
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR2),
            bondid1,
        ));

        let balance1 = Evercity::balances_everusd(INVESTOR1);
        let balance2 = Evercity::balances_everusd(INVESTOR2);
        assert_eq!(balance1, balance2);
        assert_eq!(balance1, 26827397260020); // 2 * 26827397260020 = 53654794520040
        println!("balance investor1 {}, investor2 {}", balance1, balance2);

        chain_bond_item1 = deposit(ACCOUNT1, bondid1, 20000 * UNIT);

        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid1,
        ));
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR2),
            bondid1,
        ));

        let balance1 = Evercity::balances_everusd(INVESTOR1);
        let balance2 = Evercity::balances_everusd(INVESTOR2);
        assert_eq!(balance1, balance2);
        assert_eq!(balance1, 26827397260020);
        println!("balance investor1 {}, investor2 {}", balance1, balance2);
        println!("{:?}", chain_bond_item1);

        let now = start_moment + ((12 * 30 + 120) * DEFAULT_DAY_DURATION) as u64 * 1000 + 100;
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(now);
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT1), bondid1));
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT2), bondid2));

        assert_eq!(Evercity::balances_everusd(ACCOUNT1), 0);
        assert_eq!(Evercity::balances_everusd(ACCOUNT2), 0);

        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid1,
        ));
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR2),
            bondid1,
        ));

        let balance1 = Evercity::balances_everusd(INVESTOR1);
        let balance2 = Evercity::balances_everusd(INVESTOR2);

        assert_eq!(balance1, balance2);
        assert_eq!(balance1, 124668493149600 / 2 + 4000 * 600 * UNIT);
        // check bond debt after been redeemed
        chain_bond_item1 = Evercity::get_bond(&bondid1);
        chain_bond_item2 = Evercity::get_bond(&bondid2);

        assert_eq!(chain_bond_item1.bond_debit, chain_bond_item1.bond_credit);
        assert_eq!(chain_bond_item2.bond_debit, chain_bond_item2.bond_credit);
    });
}

#[test]
fn bond_restore_from_bankrupt() {
    const ACCOUNT1: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid1: BondId = "BOND1".into();

    fn deposit(account: u64, bond: BondId, amount: EverUSDBalance) -> BondStructOf<TestRuntime> {
        assert_ok!(Evercity::bond_deposit_everusd(
            Origin::signed(account),
            bond,
            amount
        ));
        Evercity::get_bond(&bond)
    }

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid1, ACCOUNT1, get_test_bond().inner);
        let chain_bond_item1 = Evercity::get_bond(&bondid1);
        let start_moment = chain_bond_item1.active_start_date;

        for period in 0..12_usize {
            assert_ok!(Evercity::set_impact_data(
                &bondid1,
                period as BondPeriodNumber,
                chain_bond_item1.inner.impact_data_baseline[period].unwrap_or(0)
            ));
        }
        //reset balance
        Evercity::set_balance(&INVESTOR1, 0);
        Evercity::set_balance(&ACCOUNT1, 124668493149600 + 4000 * 600 * 2 * UNIT);

        let mut investor_balance = 0;

        let mut now = start_moment + (160 * DEFAULT_DAY_DURATION) as u64 * 1000;
        for _ in 0..11_usize {
            <pallet_timestamp::Module<TestRuntime>>::set_timestamp(now);
            deposit(ACCOUNT1, bondid1, 10000 * UNIT);

            assert_ok!(Evercity::bond_withdraw_everusd(
                Origin::signed(INVESTOR1),
                bondid1,
            ));
            let b = Evercity::balances_everusd(&INVESTOR1);
            assert!(b > investor_balance);
            investor_balance = b;
            println!("balance {:}", b);

            now += (DEFAULT_DAY_DURATION * 30) as u64 * 1000;
            let chain_bond_item1 = Evercity::get_bond(&bondid1);
            assert_eq!(chain_bond_item1.state, BondState::BANKRUPT);
        }

        deposit(ACCOUNT1, bondid1, 50000 * UNIT);
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(now);

        let chain_bond_item1 = Evercity::get_bond(&bondid1);
        assert_eq!(chain_bond_item1.state, BondState::ACTIVE);

        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT1), bondid1));
        let chain_bond_item1 = Evercity::get_bond(&bondid1);
        assert_eq!(chain_bond_item1.state, BondState::FINISHED);
        assert_eq!(chain_bond_item1.bond_credit, chain_bond_item1.bond_debit);
        assert_eq!(Evercity::balances_everusd(&ACCOUNT1), 0);
    });
}

#[test]
fn bond_withdraw_everusd() {
    const ACCOUNT1: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid1: BondId = "BOND1".into();

    fn deposit(account: u64, bond: BondId, amount: EverUSDBalance) -> BondStructOf<TestRuntime> {
        assert_ok!(Evercity::bond_deposit_everusd(
            Origin::signed(account),
            bond,
            amount
        ));
        Evercity::get_bond(&bond)
    }

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid1, ACCOUNT1, get_test_bond().inner);
        let chain_bond_item1 = Evercity::get_bond(&bondid1);
        let start_moment = chain_bond_item1.active_start_date;

        for period in 0..12_usize {
            assert_ok!(Evercity::set_impact_data(
                &bondid1,
                period as BondPeriodNumber,
                chain_bond_item1.inner.impact_data_baseline[period].unwrap_or(0)
            ));
        }
        //reset balance
        Evercity::set_balance(&INVESTOR1, 0);
        Evercity::set_balance(&ACCOUNT1, 124668493149600 + 4000 * 600 * 2 * UNIT);

        let mut now = start_moment + (130 * DEFAULT_DAY_DURATION) as u64 * 1000;
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(now);
        // 29983 UNIT in start period
        deposit(ACCOUNT1, bondid1, 30000 * UNIT);
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid1,
        ));

        let mut investor_balance = Evercity::balances_everusd(&INVESTOR1);
        println!("balance {:}", investor_balance);
        // after first non-start period
        now += (DEFAULT_DAY_DURATION * 30) as u64 * 1000;

        for m in 0..11_usize {
            // 7891 UNIT every payment period that is paid by two payments
            <pallet_timestamp::Module<TestRuntime>>::set_timestamp(now);
            deposit(ACCOUNT1, bondid1, 5891 * UNIT);

            assert_ok!(Evercity::bond_withdraw_everusd(
                Origin::signed(INVESTOR1),
                bondid1,
            ));

            let b = Evercity::balances_everusd(&INVESTOR1);
            assert!(b > investor_balance);
            investor_balance = b;
            println!("{} balance {:}", m, b);

            let chain_bond_item1 = Evercity::get_bond(&bondid1);
            assert_eq!(chain_bond_item1.state, BondState::BANKRUPT);

            deposit(ACCOUNT1, bondid1, 2000 * UNIT);
            assert_ok!(Evercity::bond_withdraw_everusd(
                Origin::signed(INVESTOR1),
                bondid1,
            ));

            let b = Evercity::balances_everusd(&INVESTOR1);
            assert!(b > investor_balance);
            investor_balance = b;
            println!("{} balance {:}", m, b);

            now += (DEFAULT_DAY_DURATION * 30) as u64 * 1000;
            let chain_bond_item1 = Evercity::get_bond(&bondid1);
            assert_eq!(chain_bond_item1.state, BondState::ACTIVE);
        }
    });
}

#[test]
fn bond_try_create_arbitrary_period() {
    let bondid: BondId = "BOND".into();
    const ACCOUNT: u64 = 3;

    new_test_ext().execute_with(|| {
        let mut bond = get_test_bond();
        let unwrapped_start_period = bond.inner.start_period.unwrap_or(0) + 1;
        bond.inner.start_period = Some(unwrapped_start_period);
        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid, bond.inner),
            RuntimeError::BondParamIncorrect
        );

        bond = get_test_bond();
        bond.inner.payment_period += 1;
        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid, bond.inner),
            RuntimeError::BondParamIncorrect
        );

        bond = get_test_bond();
        bond.inner.bond_finishing_period += 1;
        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid, bond.inner),
            RuntimeError::BondParamIncorrect
        );

        bond = get_test_bond();
        let unwrapped_interest_pay_period = bond.inner.interest_pay_period.unwrap_or(0) + 1;
        bond.inner.interest_pay_period = Some(unwrapped_interest_pay_period);
        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid, bond.inner),
            RuntimeError::BondParamIncorrect
        );

        bond = get_test_bond();
        bond.inner.impact_data_send_period += 1;
        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid, bond.inner),
            RuntimeError::BondParamIncorrect
        );
    });
}

#[test]
fn bond_try_release_without_fundraising_period() {
    let bondid: BondId = "BOND".into();
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;

    new_test_ext().execute_with(|| {
        let mut bond = get_test_bond();
        bond.inner.mincap_deadline = 100000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(100000);
        assert_noop!(
            Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0),
            RuntimeError::BondStateNotPermitAction
        );

        bond = get_test_bond();
        bond.inner.mincap_deadline = 200000;
        assert_ok!(Evercity::bond_update(
            Origin::signed(ACCOUNT),
            bondid,
            0,
            bond.inner
        ));
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 1));
    });
}

#[test]
fn bond_calc_redeemed_yield() {  
    // YMT - yield to maturity - total coupon yield after bond redemption
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;

    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        assert!(Evercity::evercity_balance().is_ok());
        let initial_balace1 = Evercity::balance_everusd(&INVESTOR1);
        let initial_balace2 = Evercity::balance_everusd(&INVESTOR2);
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);
        assert!(Evercity::evercity_balance().is_ok());

        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.active_start_date, 30000);
        assert_eq!(chain_bond_item.issued_amount, 1200);
        assert!(chain_bond_item
            .inner
            .impact_data_baseline
            .iter()
            .all(|&v| v == Some(20000_u64)));

        let num_periods = chain_bond_item.get_periods();
        // all period except start period will have interest rate = interest_rate_base_value
        // for start period interest rate will be  interest_rate_start_period_value
        for period in 0..num_periods - 1 {
            assert_ok!(Evercity::set_impact_data(
                &bondid,
                period,
                chain_bond_item.inner.impact_data_baseline[period as usize].unwrap_or(0)
            ));
        }
        // go to the last period
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + days2timestamp(120 + chain_bond_item.inner.bond_duration * 30 + 1),
        );
        // add extra everusd to pay off coupon yield
        assert_ok!(add_token(ACCOUNT, 125_000_000_000_000));
        assert!(Evercity::evercity_balance().is_ok());

        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT), bondid));
        assert!(Evercity::bond_check_invariant(&bondid));
        // withdraw coupon & principal value
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid
        ));
        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR2),
            bondid
        ));

        assert!(Evercity::evercity_balance().is_ok());

        let chain_bond_item = Evercity::get_bond(&bondid);
        let yield1 = Evercity::balance_everusd(&INVESTOR1) - initial_balace1;
        let yield2 = Evercity::balance_everusd(&INVESTOR2) - initial_balace2;

        assert_eq!(
            yield1 + yield2 + Evercity::balance_everusd(&ACCOUNT),
            125_000_000_000_000
        );
        assert_eq!(yield1, yield2);
        assert_eq!(yield1, 62_334_246_574_800);
        assert_eq!(Evercity::balance_everusd(&ACCOUNT), 331_506_850_400);

        assert_eq!(chain_bond_item.state, BondState::FINISHED);
        // @TODO descrees credit on redemption
        //assert_eq!(chain_bond_item.bond_credit, 0);
        //assert_eq!(chain_bond_item.bond_debit, 0);
    });
}

#[test]
fn bond_try_redeem_prior_maturity() {
    const BOND_ARRANGER: u64 = 9;
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;

    let bondid1: BondId = "BOND1".into();
    let bondid2: BondId = "BOND2".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        // bond before activation
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid1,
            get_test_bond().inner
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(days2timestamp(1));
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid1, 0));

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid1,
            1,
            600
        ));
        assert_noop!(
            Evercity::bond_redeem(Origin::signed(ACCOUNT), bondid1),
            RuntimeError::BondStateNotPermitAction
        );

        // active bond
        bond_activate(bondid2, ACCOUNT, get_test_bond().inner);

        // go to the end of the first period. n
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(days2timestamp(120 + 1));

        assert_ok!(add_token(ACCOUNT, 200_000_000_000_000));
        assert_noop!(
            Evercity::bond_redeem(Origin::signed(ACCOUNT), bondid2),
            RuntimeError::BondOutOfOrder
        );
    })
}

#[test]
fn bond_send_impact_reports() {
    const ACCOUNT: u64 = 3;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);
    });
}

#[test]
fn bond_periods() {
    let mut bond = get_test_bond();
    bond.state = BondState::ACTIVE;
    bond.active_start_date += 10;

    assert_eq!(bond.time_passed_after_activation(0), None);
    assert_eq!(
        bond.time_passed_after_activation(bond.active_start_date),
        Some((0, 0))
    );
    let start_period = bond.active_start_date + 120 * 1000 * DEFAULT_DAY_DURATION as u64;
    assert_eq!(bond.inner.start_period.unwrap_or(0), 120 * DEFAULT_DAY_DURATION);

    assert_eq!(
        bond.time_passed_after_activation(start_period),
        Some((120 * DEFAULT_DAY_DURATION, 1))
    );
    assert_eq!(
        bond.time_passed_after_activation(start_period - 1),
        Some((120 * DEFAULT_DAY_DURATION - 1, 0))
    );

    assert_eq!(bond.inner.payment_period, 30 * DEFAULT_DAY_DURATION);
    assert_eq!(
        bond.time_passed_after_activation(start_period + 30 * 1000 * DEFAULT_DAY_DURATION as u64),
        Some(((120 + 30) * DEFAULT_DAY_DURATION, 2))
    );
    assert_eq!(
        bond.time_passed_after_activation(start_period + 29 * 1000 * DEFAULT_DAY_DURATION as u64),
        Some(((120 + 29) * DEFAULT_DAY_DURATION, 1))
    );
    assert_eq!(
        bond.time_passed_after_activation(start_period + 1000 * DEFAULT_DAY_DURATION as u64),
        Some(((120 + 1) * DEFAULT_DAY_DURATION, 1))
    );
    assert_eq!(
        bond.time_passed_after_activation(start_period + 31 * 1000 * DEFAULT_DAY_DURATION as u64),
        Some(((31 + 120) * DEFAULT_DAY_DURATION, 2))
    );
    assert_eq!(
        bond.time_passed_after_activation(start_period + 310 * 1000 * DEFAULT_DAY_DURATION as u64),
        Some(((120 + 310) * DEFAULT_DAY_DURATION, 11))
    );

    assert_eq!(
        bond.time_passed_after_activation(4294967295000),
        Some((4294967294, 13))
    );

    assert_eq!(bond.time_passed_after_activation(6300000000000), None);
}

#[test]
fn bond_try_create_with_same_id() {
    let bond = get_test_bond();
    let bondid: BondId = "TEST".into();
    const ACCOUNT: u64 = 3;

    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner.clone()
        ));
        assert_noop!(
            Evercity::bond_add_new(Origin::signed(ACCOUNT), bondid, bond.inner.clone()),
            RuntimeError::BondAlreadyExists
        );
        assert_ok!(Evercity::bond_revoke(Origin::signed(ACCOUNT), bondid));
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner.clone()
        ));
    });
}

#[test]
fn bond_create_delete() {
    let bond = get_test_bond();
    let bondid: BondId = "TEST".into();

    const ACCOUNT: u64 = 3;
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner.clone()
        ));
        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(bond.inner, chain_bond_item.inner);

        assert_ok!(Evercity::bond_revoke(Origin::signed(ACCOUNT), bondid));
        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_ne!(bond.inner, chain_bond_item.inner);
        assert_eq!(chain_bond_item.inner, Default::default());
    });
}

fn bond_grand_everusd() {
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;

    assert_ok!(add_token(INVESTOR1, 50_000_000_000_000_000));
    assert_ok!(add_token(INVESTOR2, 50_000_000_000_000_000));
}

fn bond_release(bondid: BondId, acc: u64, mut bond: BondInnerStruct) -> BondStruct {
    const BOND_ARRANGER: u64 = 9;
    const AUDITOR: u64 = 5;
    bond.mincap_deadline = 50000;
    assert_ok!(Evercity::bond_add_new(Origin::signed(acc), bondid, bond));
    <pallet_timestamp::Module<TestRuntime>>::set_timestamp(10_000);
    assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));
    assert_ok!(Evercity::bond_set_auditor(
        Origin::signed(BOND_ARRANGER),
        bondid,
        AUDITOR
    ));
    Evercity::get_bond(&bondid)
}

fn bond_activate(bondid: BondId, acc: u64, mut bond: BondInnerStruct) {
    const AUDITOR: u64 = 5;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    const BOND_ARRANGER: u64 = 9;

    let investor1_balance = Evercity::balance_everusd(&INVESTOR1);
    let investor2_balance = Evercity::balance_everusd(&INVESTOR2);

    bond.mincap_deadline = 50000;
    assert_ok!(Evercity::bond_add_new(Origin::signed(acc), bondid, bond));
    <pallet_timestamp::Module<TestRuntime>>::set_timestamp(10_000);
    assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));
    let chain_bond_item = Evercity::get_bond(&bondid);
    assert_eq!(chain_bond_item.issued_amount, 0);

    // Buy two packages
    assert_ok!(Evercity::bond_unit_package_buy(
        Origin::signed(INVESTOR1),
        bondid,
        1,
        600
    ));

    assert!(Evercity::bond_check_invariant(&bondid));

    <pallet_timestamp::Module<TestRuntime>>::set_timestamp(20_000);
    assert_ok!(Evercity::bond_unit_package_buy(
        Origin::signed(INVESTOR2),
        bondid,
        1,
        600
    ));

    assert!(Evercity::bond_check_invariant(&bondid));

    let chain_bond_item = Evercity::get_bond(&bondid);
    assert_eq!(chain_bond_item.issued_amount, 1200);
    assert_eq!(chain_bond_item.bond_debit, 1200 * 4_000_000_000_000);
    assert_eq!(chain_bond_item.bond_debit, chain_bond_item.bond_credit);

    assert_ok!(Evercity::bond_set_auditor(
        Origin::signed(BOND_ARRANGER),
        bondid,
        AUDITOR
    ));

    // Activate bond
    <pallet_timestamp::Module<TestRuntime>>::set_timestamp(30000);
    assert_ok!(Evercity::bond_activate(
        Origin::signed(BOND_ARRANGER),
        bondid,
        chain_bond_item.nonce + 1
    ));
    let chain_bond_item = Evercity::get_bond(&bondid);

    assert_eq!(chain_bond_item.issued_amount, 1200);
    assert_eq!(chain_bond_item.bond_debit, 0);
    assert_eq!(chain_bond_item.bond_credit, 0);

    assert_eq!(Evercity::balance_everusd(&acc), 1200 * 4_000_000_000_000);

    assert_eq!(
        investor1_balance - Evercity::balance_everusd(&INVESTOR1),
        600 * 4_000_000_000_000
    );
    assert_eq!(
        investor2_balance - Evercity::balance_everusd(&INVESTOR2),
        600 * 4_000_000_000_000
    );
    // Try revoke
    assert_noop!(
        Evercity::bond_revoke(Origin::signed(acc), bondid),
        RuntimeError::BondStateNotPermitAction
    );
    // Try give back
    assert_noop!(
        Evercity::bond_unit_package_return(Origin::signed(INVESTOR1), bondid, 600),
        RuntimeError::BondStateNotPermitAction
    );
}

#[test]
fn bond_create_release_update() {
    let bond = get_test_bond();
    let bondid: BondId = "TEST".into();

    const ACCOUNT: u64 = 3;
    const BOND_ARRANGER: u64 = 9;
    const MANAGER: u64 = 8;
    new_test_ext().execute_with(|| {
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond.inner.clone()
        ));
        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.state, BondState::PREPARE);

        // set Manager
        assert_noop!(
            Evercity::bond_set_manager(Origin::signed(ACCOUNT), bondid, MANAGER),
            RuntimeError::AccountNotAuthorized
        );
        assert_ok!(Evercity::bond_set_manager(
            Origin::signed(BOND_ARRANGER),
            bondid,
            MANAGER
        ));
        // Manager can change bond_units_base_price
        let mut new_bond = bond.inner.clone();
        new_bond.bond_units_base_price = 100000;
        assert_ok!(Evercity::bond_update(
            Origin::signed(MANAGER),
            bondid,
            1,
            new_bond
        ));

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(10_000);

        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 2));
        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.state, BondState::BOOKING);
        assert_eq!(chain_bond_item.booking_start_date, 10_000);
        assert_eq!(chain_bond_item.manager, MANAGER);
        assert_eq!(chain_bond_item.inner.bond_units_base_price, 100_000);
    });
}

#[test]
fn bond_activate_bond_and_withdraw_bondfund() {
    const ACCOUNT: u64 = 3;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);
        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.state, BondState::ACTIVE);
        assert_eq!(chain_bond_item.active_start_date, 30000);
        assert_eq!(chain_bond_item.bond_debit, 0);
        assert_eq!(chain_bond_item.bond_credit, 0);

        assert_eq!(
            Evercity::balance_everusd(&ACCOUNT),
            1200 * 4_000_000_000_000
        );

        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.bond_debit, 0);
        assert_eq!(
            Evercity::balance_everusd(&ACCOUNT),
            1200 * 4_000_000_000_000
        );
        assert_eq!(Evercity::bond_packages(&bondid).is_empty(), false);
        let acquired_bond_units: BondUnitAmount = Evercity::bond_packages(&bondid)
            .iter()
            .map(|(_, packages)| {
                packages
                    .iter()
                    .map(|package| package.bond_units)
                    .sum::<BondUnitAmount>()
            })
            .sum::<BondUnitAmount>();

        assert_eq!(acquired_bond_units, 1200);
    });
}

#[test]
fn bond_buy_bond_units_after_activation() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(600_000);
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            3,
            400
        ));

        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(
            Evercity::balance_everusd(&ACCOUNT),
            1600 * 4_000_000_000_000
        ); // (600 + 600 + 400) * 4000
        assert_eq!(chain_bond_item.bond_debit, 0);
        assert_eq!(bond_current_period(&chain_bond_item, 600_000), 0);
    });
}

#[test]
fn bond_try_return_foreign_bonds() {
    const ACCOUNT1: u64 = 3;
    const ACCOUNT2: u64 = 7;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid1: BondId = "BOND1".into();
    let bondid2: BondId = "BOND2".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_release(bondid1, ACCOUNT1, get_test_bond().inner);
        bond_release(bondid2, ACCOUNT2, get_test_bond().inner);

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(600_000);
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid1,
            2,
            400
        ));
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR2),
            bondid2,
            2,
            400
        ));

        // bond1 != bond2
        assert_noop!(
            Evercity::bond_unit_package_return(Origin::signed(INVESTOR1), bondid2, 400),
            RuntimeError::BondParamIncorrect
        );

        // make amend
        assert_ok!(Evercity::bond_unit_package_return(
            Origin::signed(INVESTOR1),
            bondid1,
            400
        ));
        assert_ok!(Evercity::bond_unit_package_return(
            Origin::signed(INVESTOR2),
            bondid2,
            400
        ));
    });
}

#[test]
fn bond_return_bondunit_package() {
    const ACCOUNT: u64 = 3;
    const BOND_ARRANGER: u64 = 9;

    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;

    let bondid: BondId = "BOND0".into();

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 6_000_000_000_000_000));
        assert_ok!(add_token(INVESTOR2, 6_000_000_000_000_000));

        let mut bond = get_test_bond().inner;
        bond.mincap_deadline = 50000;

        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(10000);
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));
        assert!(Evercity::evercity_balance().is_ok());

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            600
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(20000);
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR2),
            bondid,
            1,
            600
        ));
        assert!(Evercity::evercity_balance().is_ok());

        let packages1 = Evercity::bond_holder_packages(&bondid, &INVESTOR1);
        assert_eq!(packages1.len(), 1);
        assert_eq!(packages1[0].bond_units, 600);
        assert_ok!(Evercity::bond_unit_package_return(
            Origin::signed(INVESTOR1),
            bondid,
            600
        ));

        let packages1 = Evercity::bond_holder_packages(&bondid, &INVESTOR1);
        assert_eq!(packages1.len(), 0);
        // you cannot give back part of the package
        assert_noop!(
            Evercity::bond_unit_package_return(Origin::signed(INVESTOR2), bondid, 100),
            RuntimeError::BondParamIncorrect
        );
        let packages2 = Evercity::bond_holder_packages(&bondid, &INVESTOR2);
        assert_eq!(packages2.len(), 1);
        assert!(Evercity::evercity_balance().is_ok());
        assert!(Evercity::bond_check_invariant(&bondid));
    });
}

#[test]
fn bond_return_partial_bondunit_package() {
    const ACCOUNT: u64 = 3;
    const BOND_ARRANGER: u64 = 9;

    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;

    let bondid: BondId = "BOND0".into();
    // investor1 = 100 + 100 + 200
    //    return 200 + 200
    // investor2 = 200
    //    return 200

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 6_000_000_000_000_000));
        assert_ok!(add_token(INVESTOR2, 6_000_000_000_000_000));

        let mut bond = get_test_bond().inner;
        bond.mincap_deadline = 50000;

        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond
        ));

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(20000);
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));
        assert!(Evercity::evercity_balance().is_ok());

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            200
        ));
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            100
        ));
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            100
        ));

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR2),
            bondid,
            1,
            200
        ));
        assert!(Evercity::bond_check_invariant(&bondid));

        assert_ok!(Evercity::bond_unit_package_return(
            Origin::signed(INVESTOR1),
            bondid,
            200
        ));
        assert_ok!(Evercity::bond_unit_package_return(
            Origin::signed(INVESTOR1),
            bondid,
            200
        ));
        assert!(Evercity::bond_check_invariant(&bondid));
        assert_noop!(
            Evercity::bond_unit_package_return(Origin::signed(INVESTOR2), bondid, 100),
            RuntimeError::BondParamIncorrect
        );

        assert_ok!(Evercity::bond_unit_package_return(
            Origin::signed(INVESTOR2),
            bondid,
            200
        ));
        assert!(Evercity::bond_check_invariant(&bondid));
    });
}

#[test]
fn bond_iter_periods() {
    const ACCOUNT: u64 = 3;
    let bondid: BondId = "BOND1".into();

    let mut ext = new_test_ext();
    ext.execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);
        let chain_bond_item = Evercity::get_bond(&bondid);

        let mut start = 0;
        let mut count = 0;
        for period in chain_bond_item.iter_periods() {
            assert_eq!(period.start_period, start);
            start = period.payment_period;
            count += 1;
        }

        assert_eq!(count, 14);
        assert_eq!(chain_bond_item.get_periods(), count - 1);
    });
}

#[test]
fn bond_cancel_after_release() {
    const ACCOUNT: u64 = 3;
    const BOND_ARRANGER: u64 = 9;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        assert_ok!(add_token(INVESTOR1, 10_000_000_000_000_000));
        assert_ok!(add_token(INVESTOR2, 10_000_000_000_000_000));

        let mut bond = get_test_bond().inner;
        bond.mincap_deadline = 50000;
        assert_ok!(Evercity::bond_add_new(
            Origin::signed(ACCOUNT),
            bondid,
            bond
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(10000);
        assert_ok!(Evercity::bond_release(Origin::signed(BOND_ARRANGER), bondid, 0));

        // Buy three packages
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            1,
            400
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(20_000);
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR2),
            bondid,
            1,
            200
        ));
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(30_000);
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR2),
            bondid,
            1,
            200
        ));

        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.issued_amount, 800);
        assert_eq!(chain_bond_item.bond_debit, 800 * 4_000_000_000_000);
        assert_eq!(chain_bond_item.bond_debit, chain_bond_item.bond_credit);

        assert_eq!(
            Evercity::balance_everusd(&INVESTOR1),
            10_000_000_000_000_000 - 400 * 4_000_000_000_000
        );
        assert_eq!(
            Evercity::balance_everusd(&INVESTOR2),
            10_000_000_000_000_000 - 400 * 4_000_000_000_000
        );

        // Bond unit packages

        let packages1 = Evercity::bond_holder_packages(&bondid, &INVESTOR1);
        let packages2 = Evercity::bond_holder_packages(&bondid, &INVESTOR2);

        assert_eq!(packages1.len(), 1);
        assert_eq!(packages2.len(), 2);

        assert_eq!(packages1[0].bond_units, 400);
        assert_eq!(packages2[0].bond_units, 200);
        assert_eq!(packages2[0].bond_units, 200);

        assert_eq!(packages1[0].acquisition, 0);
        assert_eq!(packages2[0].acquisition, 0);
        assert_eq!(packages2[1].acquisition, 0);

        // We raised up less than  bond_units_mincap_amount, so we should revoke the bond
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(60000);
        assert_ok!(Evercity::bond_withdraw(Origin::signed(BOND_ARRANGER), bondid));
        let chain_bond_item = Evercity::get_bond(&bondid);

        assert_eq!(chain_bond_item.issued_amount, 0);
        assert_eq!(chain_bond_item.state, BondState::PREPARE);
        assert_eq!(chain_bond_item.bond_debit, 0);
        assert_eq!(chain_bond_item.bond_credit, 0);

        assert_eq!(
            Evercity::balance_everusd(&INVESTOR1),
            10_000_000_000_000_000
        );
        assert_eq!(
            Evercity::balance_everusd(&INVESTOR2),
            10_000_000_000_000_000
        );

        let packages1 = Evercity::bond_holder_packages(&bondid, &INVESTOR1);
        let packages2 = Evercity::bond_holder_packages(&bondid, &INVESTOR2);

        assert_eq!(packages1.len(), 0);
        assert_eq!(packages2.len(), 0);
    });
}

#[test]
fn bond_impact_report_missing_data() {
    const ACCOUNT1: u64 = 3;

    let bondid1: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid1, ACCOUNT1, bond.clone());

        for &period in &[0, 1, 3, 5, 7, 9] {
            assert_ok!(Evercity::set_impact_data(
                &bondid1,
                period,
                bond.impact_data_baseline[period as usize].unwrap_or(0)
            ));
        }
        let chain_bond_item = Evercity::get_bond(&bondid1);
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + 1000_u64
                    * (bond.start_period.unwrap_or(0) + bond.bond_duration * bond.payment_period + 1) as u64,
        );
        assert_ok!(add_token(ACCOUNT1, 500_000_000_000_000));
        // force interest rate calculation
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT1), bondid1));

        let ref_interest_rate1 = vec![
            1900, 2000, 2000, 2400, 2000, 2400, 2000, 2400, 2000, 2400, 2000, 2400, 2800,
        ];
        for (calc_interest_rate, ref_interest_rate) in Evercity::get_coupon_yields(&bondid1)
            .iter()
            .map(|coupon| coupon.interest_rate)
            .zip(ref_interest_rate1)
        {
            assert_eq!(calc_interest_rate, ref_interest_rate);
        }
    });
}

#[test]
fn bond_impact_report_no_data() {
    const ACCOUNT1: u64 = 3;

    let bondid1: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid1, ACCOUNT1, bond.clone());

        let chain_bond_item = Evercity::get_bond(&bondid1);
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + 1000_u64
                    * (bond.start_period.unwrap_or(0) + bond.bond_duration * bond.payment_period + 1) as u64,
        );
        assert_ok!(add_token(ACCOUNT1, 500_000_000_000_000));
        // force interest rate calculation
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT1), bondid1));

        let ref_interest_rate = vec![
            1900, 2300, 2700, 3100, 3500, 3900, 4000, 4000, 4000, 4000, 4000, 4000, 4000,
        ];

        for (calc_interest_rate, ref_interest_rate) in Evercity::get_coupon_yields(&bondid1)
            .iter()
            .map(|coupon| coupon.interest_rate)
            .zip(ref_interest_rate)
        {
            assert_eq!(calc_interest_rate, ref_interest_rate);
        }
    });
}

#[test]
fn bond_interest_rate_rnd() {
    use rand::{
        self,
        distributions::{Distribution, Uniform},
    };

    const ACCOUNT: u64 = 3;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        let mut rng = rand::thread_rng();

        let mut bond = get_test_bond().inner;
        let impact_data_range = Uniform::new_inclusive(
            bond.impact_data_max_deviation_floor.unwrap_or(0),
            bond.impact_data_max_deviation_cap.unwrap_or(0),
        );
        for period in 0..bond.bond_duration as usize {
            bond.impact_data_baseline[period] = Some(impact_data_range.sample(&mut rng));
        }
        let periods: usize = bond.bond_duration as usize;
        //create bond
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, bond);

        for period in 0..periods {
            assert_ok!(Evercity::set_impact_data(
                &bondid,
                period as BondPeriodNumber,
                20000_u64
            ));
        }
        // force impact interesting rate calculation
        let chain_bond_item = Evercity::get_bond(&bondid);
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + 1000_u64
                    * (chain_bond_item.inner.start_period.unwrap_or(0)
                        + chain_bond_item.inner.bond_duration
                            * chain_bond_item.inner.payment_period
                        + 1) as u64,
        );

        assert_ok!(add_token(ACCOUNT, 500_000_000_000_000));
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT), bondid));

        //
        let bond_coupon_yields = Evercity::bond_coupon_yield(&bondid);
        let impact_reports = Evercity::impact_reports(&bondid);

        assert_eq!(bond_coupon_yields.len(), periods + 1);
        assert_eq!(
            bond_coupon_yields[0].interest_rate,
            chain_bond_item.inner.interest_rate_start_period_value.unwrap_or(0)
        );
        for period in 0..periods {
            let interest_rate = bond_coupon_yields[period + 1].interest_rate;
            let impact_data = impact_reports[period].impact_data;
            assert_eq!(impact_data, 20000_u64);
            // if impact data is less than baseline value then  interest rate is more than base value
            assert_eq!(
                impact_data < chain_bond_item.inner.impact_data_baseline[period].unwrap_or(0),
                interest_rate > chain_bond_item.inner.interest_rate_base_value
            );

            println!(
                "{}: impact_data={} <> baseline={}, base interest rate={} <> interest rate={}",
                period,
                impact_data,
                chain_bond_item.inner.impact_data_baseline[period].unwrap_or(0),
                chain_bond_item.inner.interest_rate_base_value,
                interest_rate
            )
        }
    });
}

#[test]
fn bond_impact_report_interest_rate() {
    const ACCOUNT1: u64 = 3;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT1, bond.clone());

        for (period, impact_data) in [
            bond.impact_data_baseline[0],
            Some(0),
            Some(60000),
            bond.impact_data_max_deviation_floor,
            bond.impact_data_max_deviation_cap,
            Some(25000),
            Some(16000),
        ]
        .iter()
        .enumerate()
        {
            assert_ok!(Evercity::set_impact_data(
                &bondid,
                period as BondPeriodNumber,
                impact_data.unwrap_or(0)
            ));
        }
        let chain_bond_item = Evercity::get_bond(&bondid);
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + 1000_u64
                    * (bond.start_period.unwrap_or(0) + bond.bond_duration * bond.payment_period + 1) as u64,
        );

        assert_ok!(add_token(ACCOUNT1, 500_000_000_000_000));

        //force interest rate calculation
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT1), bondid));

        let ref_interest_rate = vec![
            bond.interest_rate_start_period_value,
            Some(bond.interest_rate_base_value),
            bond.interest_rate_margin_cap,
            bond.interest_rate_margin_floor,
            bond.interest_rate_margin_cap,
            bond.interest_rate_margin_floor,
            Some(1500),
            Some(3333),
        ];
        for (calc_interest_rate, ref_interest_rate) in Evercity::get_coupon_yields(&bondid)
            .iter()
            .map(|coupon| coupon.interest_rate)
            .zip(ref_interest_rate)
        {
            assert_eq!(calc_interest_rate, ref_interest_rate.unwrap_or(0));
        }
    });
}

#[test]
fn bond_impact_report_send_approve() {
    const ACCOUNT1: u64 = 3;
    const AUDITOR: u64 = 5;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT1, bond.clone());

        let chain_bond_item = Evercity::get_bond(&bondid);

        for period in 0..bond.bond_duration {
            // day before end of the period
            <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
                chain_bond_item.active_start_date
                    + 1000_u64 * (bond.start_period.unwrap_or(0) + period * bond.payment_period - 1) as u64,
            );
            assert_ok!(Evercity::bond_impact_report_send(
                Origin::signed(ACCOUNT1),
                bondid,
                period,
                bond.impact_data_baseline[period as usize].unwrap_or(0)
            ));
            assert_ok!(Evercity::bond_impact_report_approve(
                Origin::signed(AUDITOR),
                bondid,
                period,
                bond.impact_data_baseline[period as usize].unwrap_or(0)
            ));
        }
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + 1000_u64
                    * (bond.start_period.unwrap_or(0) + bond.bond_duration * bond.payment_period + 1) as u64,
        );

        assert_ok!(add_token(ACCOUNT1, 500_000_000_000_000));
        //force interest rate calculation
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT1), bondid));

        let ref_interest_rate = vec![
            bond.interest_rate_start_period_value,
            Some(bond.interest_rate_base_value),
            Some(bond.interest_rate_base_value),
            Some(bond.interest_rate_base_value),
            Some(bond.interest_rate_base_value),
            Some(bond.interest_rate_base_value),
            Some(bond.interest_rate_base_value),
            Some(bond.interest_rate_base_value),
            Some(bond.interest_rate_base_value),
        ];
        for (calc_interest_rate, ref_interest_rate) in Evercity::get_coupon_yields(&bondid)
            .iter()
            .map(|coupon| coupon.interest_rate)
            .zip(ref_interest_rate)
        {
            assert_eq!(calc_interest_rate, ref_interest_rate.unwrap_or(0));
        }
    });
}

#[test]
fn bond_impact_report_try_approve_unauthorized() {
    const ACCOUNT1: u64 = 3;
    const AUDITOR: u64 = 5;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT1, bond.clone());

        let chain_bond_item = Evercity::get_bond(&bondid);
        // first period
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date + 1000_u64 * (bond.start_period.unwrap_or(0) - 1) as u64,
        );

        // send report
        assert_ok!(Evercity::bond_impact_report_send(
            Origin::signed(ACCOUNT1),
            bondid,
            0,
            1000
        ));

        for acc in iter_accounts().filter(|acc| *acc != AUDITOR) {
            assert_noop!(
                Evercity::bond_impact_report_approve(Origin::signed(acc), bondid, 0, 1000),
                RuntimeError::AccountNotAuthorized
            );
        }

        // make amend

        assert_ok!(Evercity::bond_impact_report_approve(
            Origin::signed(AUDITOR),
            bondid,
            0,
            1000
        ));
    });
}

#[test]
fn bond_impact_report_try_approve_unattended() {
    const ACCOUNT1: u64 = 3;
    const AUDITOR: u64 = 5;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT1, bond.clone());

        let chain_bond_item = Evercity::get_bond(&bondid);
        // first period
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date + 1000_u64 * (bond.start_period.unwrap_or(0) - 1) as u64,
        );
        // try approve without report
        assert_noop!(
            Evercity::bond_impact_report_approve(Origin::signed(AUDITOR), bondid, 0, 0),
            RuntimeError::BondParamIncorrect
        );

        // make amend
        assert_ok!(Evercity::bond_impact_report_send(
            Origin::signed(ACCOUNT1),
            bondid,
            0,
            0
        ));

        assert_ok!(Evercity::bond_impact_report_approve(
            Origin::signed(AUDITOR),
            bondid,
            0,
            0
        ));
    });
}

#[test]
fn bond_impact_report_outof_order() {
    const ACCOUNT1: u64 = 3;
    const AUDITOR: u64 = 5;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT1, bond.clone());

        let chain_bond_item = Evercity::get_bond(&bondid);

        for period in 0..bond.bond_duration {
            // before start of the report period
            <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
                chain_bond_item.active_start_date
                    + 1000_u64
                        * (bond.start_period.unwrap_or(0) + period * bond.payment_period
                            - bond.impact_data_send_period
                            - 1) as u64,
            );
            assert_noop!(
                Evercity::bond_impact_report_send(
                    Origin::signed(ACCOUNT1),
                    bondid,
                    period,
                    bond.impact_data_baseline[period as usize].unwrap_or(0)
                ),
                RuntimeError::BondOutOfOrder
            );

            // after current period end
            <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
                chain_bond_item.active_start_date
                    + 1000_u64 * (bond.start_period.unwrap_or(0) + period * bond.payment_period + 1) as u64,
            );

            assert_noop!(
                Evercity::bond_impact_report_send(
                    Origin::signed(ACCOUNT1),
                    bondid,
                    period,
                    bond.impact_data_baseline[period as usize].unwrap_or(0)
                ),
                RuntimeError::BondOutOfOrder
            );

            // between report period start and  current period end
            <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
                chain_bond_item.active_start_date
                    + 1000_u64 * (bond.start_period.unwrap_or(0) + period * bond.payment_period - 1000) as u64,
            );

            assert_ok!(Evercity::bond_impact_report_send(
                Origin::signed(ACCOUNT1),
                bondid,
                period,
                bond.impact_data_baseline[period as usize].unwrap_or(0)
            ));

            assert_ok!(Evercity::bond_impact_report_approve(
                Origin::signed(AUDITOR),
                bondid,
                period,
                bond.impact_data_baseline[period as usize].unwrap_or(0)
            ));
        }
    });
}

#[test]
fn bond_acquire_try_exceed_max() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid: BondId = "BOND1".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, get_test_bond().inner);

        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            3,
            599
        ));
        assert_noop!(
            Evercity::bond_unit_package_buy(Origin::signed(INVESTOR2), bondid, 3, 2),
            RuntimeError::BondParamIncorrect
        );

        // make amend
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            3,
            1
        ));
    });
}

#[test]
fn bond_acquire_try_own_bond() {
    const ACCOUNT1: u64 = 7;
    const ACCOUNT2: u64 = 3;
    let bondid1: BondId = "BOND1".into();
    let bondid2: BondId = "BOND2".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        bond_activate(bondid1, ACCOUNT1, get_test_bond().inner);
        bond_activate(bondid2, ACCOUNT2, get_test_bond().inner);
        let chain_bond_item = Evercity::get_bond(&bondid1);

        assert_eq!(chain_bond_item.issued_amount, 1200);

        assert_noop!(
            Evercity::bond_unit_package_buy(Origin::signed(ACCOUNT1), bondid1, 3, 1),
            RuntimeError::AccountNotAuthorized
        );

        let chain_bond_item = Evercity::get_bond(&bondid1);
        assert_eq!(chain_bond_item.issued_amount, 1200);

        // make amend by acquiring other bond
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(ACCOUNT1),
            bondid2,
            3,
            1
        ));
    });
}

#[test]
fn bond_acquire_try_after_redemption() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND0000".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT, bond.clone());
        let chain_bond_item = Evercity::get_bond(&bondid);

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + 1000_u64
                    * (bond.start_period.unwrap_or(0) + bond.bond_duration * bond.payment_period + 1) as u64,
        );
        // add everusd to pay off bond yield
        assert_ok!(add_token(ACCOUNT, 500_000_000_000_000));
        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT), bondid));

        assert_noop!(
            Evercity::bond_unit_package_buy(Origin::signed(INVESTOR1), bondid, 4, 2),
            RuntimeError::BondStateNotPermitAction
        );
    });
}

#[test]
fn bond_deposit_bond() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        assert!(Evercity::evercity_balance().is_ok());

        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT, bond.clone());
        let chain_bond_item = Evercity::get_bond(&bondid);

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date + 1000_u64 * (bond.start_period.unwrap_or(0) + 1) as u64,
        );

        assert_eq!(chain_bond_item.bond_debit, 0);
        assert_eq!(chain_bond_item.coupon_yield, 0);
        assert_eq!(
            Evercity::balance_everusd(&ACCOUNT),
            1200 * 4_000_000_000_000
        );

        assert_ok!(Evercity::bond_deposit_everusd(
            Origin::signed(ACCOUNT),
            bondid,
            100_000_000_000_000
        ));

        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.bond_debit, 100_000_000_000_000);
        assert_eq!(chain_bond_item.coupon_yield, 0);
        assert!(Evercity::evercity_balance().is_ok());

        assert_eq!(
            Evercity::balance_everusd(&ACCOUNT),
            1200 * 4_000_000_000_000 - 100_000_000_000_000
        );

        assert_ok!(Evercity::bond_withdraw_everusd(
            Origin::signed(INVESTOR1),
            bondid
        ));
        let chain_bond_item = Evercity::get_bond(&bondid);
        assert_eq!(chain_bond_item.coupon_yield, 14_991_780_821_760);
        assert_eq!(chain_bond_item.get_debt(), 0);
        // 1.9 % - (600 + 600) x 4000 usd - 120 days
        assert_eq!(chain_bond_item.bond_credit, 29_983_561_643_520);
        assert_eq!(
            chain_bond_item.get_free_balance(),
            100_000_000_000_000 - 29_983_561_643_520
        );
        assert!(Evercity::evercity_balance().is_ok());
    });
}

#[test]
fn bond_deposit_return_after_redemption() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        let bond = get_test_bond().inner;
        bond_activate(bondid, ACCOUNT, bond.clone());
        let chain_bond_item = Evercity::get_bond(&bondid);

        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(
            chain_bond_item.active_start_date
                + 1000_u64
                    * (bond.start_period.unwrap_or(0) + bond.bond_duration * bond.payment_period+ 1) as u64,
        );
        // add everusd to pay off bond yield
        assert_ok!(add_token(ACCOUNT, 500_000_000_000_000));

        assert_ok!(Evercity::bond_redeem(Origin::signed(ACCOUNT), bondid));

        assert_noop!(
            Evercity::bond_unit_package_buy(Origin::signed(INVESTOR1), bondid, 4, 2),
            RuntimeError::BondStateNotPermitAction
        );
    });
}

#[test]
fn bond_deposit_try_foreign() {
    const ACCOUNT1: u64 = 3;
    const ACCOUNT2: u64 = 7;

    let bondid1: BondId = "BOND1".into();
    let bondid2: BondId = "BOND2".into();

    new_test_ext().execute_with(|| {
        bond_grand_everusd();
        assert!(Evercity::evercity_balance().is_ok());

        let bond = get_test_bond().inner;
        bond_activate(bondid1, ACCOUNT1, bond.clone());
        bond_activate(bondid2, ACCOUNT2, bond);
        assert!(Evercity::evercity_balance().is_ok());

        assert_noop!(
            Evercity::bond_deposit_everusd(Origin::signed(ACCOUNT1), bondid2, 100_000_000_000_000),
            RuntimeError::BondAccessDenied
        );
    });
}

#[test]
fn bond_lot_bit_n_buy() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        let bond = get_test_bond().inner;
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, bond);

        assert!(Evercity::bond_check_invariant(&bondid));

        let lot = BondUnitSaleLotStruct {
            deadline: 100000,
            new_bondholder: Default::default(),
            bond_units: 600,
            amount: 600 * 3_000_000_000_000,
        };
        assert!(Evercity::evercity_balance().is_ok());
        assert_ok!(Evercity::bond_unit_lot_bid(
            Origin::signed(INVESTOR1),
            bondid,
            lot.clone()
        ));
        assert_ok!(Evercity::bond_unit_lot_settle(
            Origin::signed(INVESTOR2),
            bondid,
            INVESTOR1,
            lot
        ));
        assert!(Evercity::bond_check_invariant(&bondid));
        assert!(Evercity::evercity_balance().is_ok());
        let packages1 = Evercity::bond_holder_packages(&bondid, &INVESTOR1);
        let bond_units1: BondUnitAmount = packages1.iter().map(|p| p.bond_units).sum();
        let packages2 = Evercity::bond_holder_packages(&bondid, &INVESTOR2);
        let bond_units2: BondUnitAmount = packages2.iter().map(|p| p.bond_units).sum();

        assert_eq!(bond_units1, 0);
        assert_eq!(bond_units2, 1200);
    });
}

#[test]
fn bond_lot_paid_coupon() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        let bond = get_test_bond().inner;
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, bond.clone());
        let chain_bond_item = Evercity::get_bond(&bondid);

        // buy additional 200 + 100
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            3,
            100
        ));
        assert_ok!(Evercity::bond_unit_package_buy(
            Origin::signed(INVESTOR1),
            bondid,
            3,
            200
        ));
        assert!(Evercity::bond_check_invariant(&bondid));
        // first period
        let moment = chain_bond_item.active_start_date + 1000_u64 * (bond.start_period.unwrap_or(0) + 1) as u64;
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(moment);

        let (_, period) = chain_bond_item
            .time_passed_after_activation(moment)
            .unwrap();
        assert_eq!(period, 1);

        let lot = BondUnitSaleLotStruct {
            deadline: moment + 1,
            new_bondholder: Default::default(),
            bond_units: 400,
            amount: 400 * 3_000_000_000_000,
        };

        // deposit will be used to pay coupon
        assert_ok!(Evercity::bond_deposit_everusd(
            Origin::signed(ACCOUNT),
            bondid,
            100_000_000_000_000
        ));

        let balance1 = Evercity::balance_everusd(&INVESTOR1);

        assert!(Evercity::evercity_balance().is_ok());
        assert_ok!(Evercity::bond_unit_lot_bid(
            Origin::signed(INVESTOR1),
            bondid,
            lot.clone()
        ));
        assert_ok!(Evercity::bond_unit_lot_settle(
            Origin::signed(INVESTOR2),
            bondid,
            INVESTOR1,
            lot
        ));
        assert!(Evercity::evercity_balance().is_ok());

        let packages1 = Evercity::bond_holder_packages(&bondid, &INVESTOR1);
        let bond_units1: BondUnitAmount = packages1.iter().map(|p| p.bond_units).sum();

        let packages2 = Evercity::bond_holder_packages(&bondid, &INVESTOR2);
        let bond_units2: BondUnitAmount = packages2.iter().map(|p| p.bond_units).sum();

        assert_eq!(bond_units1, 500);
        assert_eq!(bond_units2, 1000);

        let bond_units1: Vec<_> = packages1.iter().map(|p| p.bond_units).collect();
        let bond_units2: Vec<_> = packages2.iter().map(|p| p.bond_units).collect();

        assert_eq!(bond_units1, vec![500]);
        assert_eq!(bond_units2, vec![600, 100, 200, 100]);
        // 1.9% - 120 days - (600 + 200 + 100) units x 4000 usd =22487.671 usd
        // @TODO calc coupon yield
        assert_eq!(
            Evercity::balance_everusd(&INVESTOR1) - balance1,
            400 * 3_000_000_000_000 + 22_487_671_232_640 // 1200000000000000
        );
    });
}

#[test]
fn bond_lot_try_buy_foreign() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        let bond = get_test_bond().inner;
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, bond);

        let lot = BondUnitSaleLotStruct {
            deadline: 100000,
            new_bondholder: 7,
            bond_units: 600,
            amount: 600 * 3_000_000_000_000,
        };
        assert!(Evercity::evercity_balance().is_ok());
        assert_ok!(Evercity::bond_unit_lot_bid(
            Origin::signed(INVESTOR1),
            bondid,
            lot.clone()
        ));
        assert_noop!(
            Evercity::bond_unit_lot_settle(Origin::signed(INVESTOR2), bondid, INVESTOR1, lot),
            RuntimeError::LotNotFound
        );
    });
}

#[test]
fn bond_lot_try_create_expired() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        let bond = get_test_bond().inner;
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, bond);

        let lot = BondUnitSaleLotStruct {
            deadline: 100000,
            new_bondholder: Default::default(),
            bond_units: 600,
            amount: 600 * 3_000_000_000_000,
        };
        // move forward
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(1000000 + 1);
        assert_noop!(
            Evercity::bond_unit_lot_bid(Origin::signed(INVESTOR1), bondid, lot),
            RuntimeError::LotParamIncorrect
        );
    });
}

#[test]
fn bond_lot_try_buy_expired() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    const INVESTOR2: u64 = 6;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        let bond = get_test_bond().inner;
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, bond);

        let lot = BondUnitSaleLotStruct {
            deadline: 100000,
            new_bondholder: Default::default(),
            bond_units: 600,
            amount: 600 * 3_000_000_000_000,
        };
        assert!(Evercity::evercity_balance().is_ok());
        assert_ok!(Evercity::bond_unit_lot_bid(
            Origin::signed(INVESTOR1),
            bondid,
            lot.clone()
        ));

        // move forward
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(1000000 + 1);

        assert_noop!(
            Evercity::bond_unit_lot_settle(Origin::signed(INVESTOR2), bondid, INVESTOR1, lot),
            RuntimeError::LotObsolete
        );
    });
}

#[test]
fn bond_lot_try_exceed_portfolio() {
    const ACCOUNT: u64 = 3;
    const INVESTOR1: u64 = 4;
    let bondid: BondId = "BOND".into();

    new_test_ext().execute_with(|| {
        let bond = get_test_bond().inner;
        bond_grand_everusd();
        bond_activate(bondid, ACCOUNT, bond);

        let lot = BondUnitSaleLotStruct {
            deadline: 100000,
            new_bondholder: Default::default(),
            bond_units: 500,
            amount: 600 * 3_000_000_000_000,
        };

        assert_ok!(Evercity::bond_unit_lot_bid(
            Origin::signed(INVESTOR1),
            bondid,
            lot.clone()
        ));
        assert_noop!(
            Evercity::bond_unit_lot_bid(Origin::signed(INVESTOR1), bondid, lot.clone()),
            RuntimeError::BalanceOverdraft
        );
        // make amend. make prior lots expired
        <pallet_timestamp::Module<TestRuntime>>::set_timestamp(100000 + 1);
        let mut lot = lot;
        lot.deadline = 100000 + 2;
        assert_ok!(Evercity::bond_unit_lot_bid(
            Origin::signed(INVESTOR1),
            bondid,
            lot
        ));
    });
}