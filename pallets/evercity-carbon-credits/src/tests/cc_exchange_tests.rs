use frame_support::assert_noop;
use frame_support::assert_ok;

use crate::tests::helpers::*;

pub const TEST_CARBON_CREDITS_COUNT: u64 = 15000;
pub const TEST_CARBON_CREDITS_DECIMAL: u8 = 0;

use crate::Error;
use crate::cc_package_lot::CarbonCreditsPackageLot;
use crate::tests::mock::*;

use super::mock::TestRuntime;


type RuntimeError = Error<TestRuntime>;

#[test]
fn it_works_create_cc_lot() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let cc_id = 666;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        let lots = CarbonCredits::lots(cc_holder, cc_id);

        assert_eq!(1, lots.len());
        assert_eq!(lot, lots[0]);
    });
}

#[test]
fn it_works_buy_cc_lot() {
    new_test_ext().execute_with(|| {
         // cc - carbon credits
        let everusd_holder = 111;
        let cc_id = 666;
        let everusd_balance = 6_000_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let total_price = cc_price*cc_amount;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_amount));
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(0, lots.len());
        assert_eq!(cc_amount, cc_holder_amount - cc_holder_amount_after);
        assert_eq!(total_price, everusd_balance - everusd_balance_after);
    });
}

#[test]
fn it_works_buy_cc_lot_partially() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let everusd_holder = 111;
        let cc_id = 666;
        let everusd_balance = 6_000_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let cc_to_buy: u64 = 10;
        let total_price = cc_price*cc_to_buy;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy));
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_amount - cc_to_buy, lots[0].amount);
        assert_eq!(cc_to_buy, cc_holder_amount - cc_holder_amount_after);
        assert_eq!(total_price, everusd_balance - everusd_balance_after);
    });
}

#[test]
fn it_works_create_cc_lot_several_times() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let carbon_credits_id = 666;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, carbon_credits_id, cc_holder, 1);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let cc_price2: u64 = 80_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let lot2 = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price2,
        };

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot.clone()));
        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot2.clone()));
        let lots = CarbonCredits::lots(cc_holder, carbon_credits_id);

        assert_eq!(2, lots.len());
        assert_eq!(lot, lots[0]);
        assert_eq!(lot2, lots[1]);
    });
}

#[test]
fn it_works_create_cc_lot_several_times_expired_deleted() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let carbon_credits_id = 666;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, carbon_credits_id, cc_holder, 1);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let cc_price2: u64 = 80_000_000_000;
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: 100,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let lot2 = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: 100,
            amount: cc_amount,
            price_per_item: cc_price2,
        };
        let lot3 = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: 10_000,
            amount: cc_amount,
            price_per_item: cc_price2,
        };

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot.clone()));
        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot2.clone()));
        // speed time
        Timestamp::set_timestamp(1_000);
        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot3.clone()));
        let lots = CarbonCredits::lots(cc_holder, carbon_credits_id);

        assert_eq!(1, lots.len());
        assert_eq!(lot3, lots[0]);
    });
}

#[test]
fn it_works_buy_cc_lot_several_times() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let everusd_holder = 111;
        let cc_id = 666;
        let everusd_balance = 6_000_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let cc_to_buy: u64 = 4;
        let total_price = cc_price*cc_to_buy;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot, cc_to_buy));
        let updated_lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount - cc_to_buy,
            price_per_item: cc_price,
        };
        assert_ok!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, updated_lot, cc_to_buy));
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_amount - cc_to_buy*2, lots[0].amount);
        assert_eq!(cc_to_buy*2, cc_holder_amount - cc_holder_amount_after);
        assert_eq!(total_price*2, everusd_balance - everusd_balance_after);
    });
}

#[test]
fn it_fails_create_lot_not_enough_cc() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let cc_id = 666;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);

        let cc_amount: u64 = cc_holder_amount+2;
        println!("cc amount = {}", cc_amount);
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };

        // let err = CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone());
        // println!("{:#?}", err);
        // assert_noop!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
        //     RuntimeError::InsufficientCarbonCreditsBalance);
        frame_support::assert_err!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
            RuntimeError::InsufficientCarbonCreditsBalance);
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        assert_eq!(0, lots.len());
    });
}

#[test]
fn it_fails_create_lot_private_to_seller() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let cc_id = 666;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);

        let cc_amount: u64 = 2;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: Some(cc_holder),
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };

        assert_noop!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
            RuntimeError::InvalidLotDetails);
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        assert_eq!(0, lots.len());
    });
}

#[test]
fn it_fails_create_lot_expired() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let cc_id = 666;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);

        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 1; 
        Timestamp::set_timestamp(1_000);
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };

        assert_noop!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
            RuntimeError::LotExpired);
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        assert_eq!(0, lots.len());
    });
}

#[test]
fn it_works_buy_private_lot() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let everusd_holder = 111;
        let cc_id = 666;
        let everusd_balance = 6_000_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: Some(everusd_holder),
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let cc_to_buy: u64 = 10;
        let total_price = cc_price*cc_to_buy;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy));
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_amount - cc_to_buy, lots[0].amount);
        assert_eq!(cc_to_buy, cc_holder_amount - cc_holder_amount_after);
        assert_eq!(total_price, everusd_balance - everusd_balance_after);
    });
}

#[test]
fn it_fails_buy_lot_not_enough_everusd() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let everusd_holder = 111;
        let cc_id = 666;
        let everusd_balance = 60_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100_000; 
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let cc_to_buy: u64 = 10;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_noop!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::InsufficientEverUSDBalance);
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_amount, lots[0].amount);
        assert_eq!(cc_holder_amount, cc_holder_amount_after);
        assert_eq!(everusd_balance, everusd_balance_after);
    });
}

#[test]
fn it_fails_buy_lot_expired() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let everusd_holder = 111;
        let cc_id = 666;
        let everusd_balance = 60_000_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100;
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let cc_to_buy: u64 = 10;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        // speed time
        Timestamp::set_timestamp(1_000); 
        assert_noop!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::LotExpired);
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_holder_amount, cc_holder_amount_after);
        assert_eq!(everusd_balance, everusd_balance_after);
    });
}

#[test]
fn it_fails_buy_lot_amount_too_big() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let everusd_holder = 111;
        let cc_id = 666;
        let everusd_balance = 60_000_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100;
        let lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let cc_to_buy: u64 = 10_000_000;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_noop!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::NotEnoughCarbonCreditsInLot);
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_holder_amount, cc_holder_amount_after);
        assert_eq!(everusd_balance, everusd_balance_after);
    });
}


#[test]
fn it_fails_buy_lot_is_private() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let everusd_holder = 111;
        let another_acc = 222;
        let cc_id = 666;
        let everusd_balance = 60_000_000_000_000;
        let (_, project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = Assets::balance(cc_id, cc_holder);
        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        EvercityBonds::set_balance(&another_acc, everusd_balance);
        
        let cc_amount: u64 = 20;
        let cc_price: u64 = 60_000_000_000;
        let deadline = 100;
        let lot = CarbonCreditsPackageLot {
            target_bearer: Some(another_acc),
            deadline: deadline,
            amount: cc_amount,
            price_per_item: cc_price,
        };
        let cc_to_buy: u64 = 10;

        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_noop!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::LotNotFound);
        let lots = CarbonCredits::lots(cc_holder, cc_id);
        let cc_holder_amount_after = Assets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_holder_amount, cc_holder_amount_after);
        assert_eq!(everusd_balance, everusd_balance_after);
    });
}