use frame_support::assert_noop;
use frame_support::assert_ok;

use helpers::*;

pub const TEST_CARBON_CREDITS_COUNT: u64 = 15000;
pub const TEST_CARBON_CREDITS_DECIMAL: u8 = 0;

use crate::Error;
use crate::cc_package_lot::CarbonCreditsPackageLot;
use crate::mock::*;


type RuntimeError = Error<TestRuntime>;

#[test]
fn it_works_create_cc_lot() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let cc_id = 666;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        let lots = EvercityExchange::lots(cc_holder, cc_id);

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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_amount));
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy));
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot.clone()));
        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot2.clone()));
        let lots = EvercityExchange::lots(cc_holder, carbon_credits_id);

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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot.clone()));
        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot2.clone()));
        // speed time
        Timestamp::set_timestamp(1_000);
        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), carbon_credits_id, lot3.clone()));
        let lots = EvercityExchange::lots(cc_holder, carbon_credits_id);

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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot, cc_to_buy));
        let updated_lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: deadline,
            amount: cc_amount - cc_to_buy,
            price_per_item: cc_price,
        };
        assert_ok!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, updated_lot, cc_to_buy));
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);

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

        // let err = EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone());
        // println!("{:#?}", err);
        // assert_noop!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
        //     RuntimeError::InsufficientCarbonCreditsBalance);
        frame_support::assert_err!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
            RuntimeError::InsufficientCarbonCreditsBalance);
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        assert_eq!(0, lots.len());
    });
}

#[test]
fn it_fails_create_lot_private_to_seller() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let cc_id = 666;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
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

        assert_noop!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
            RuntimeError::InvalidLotDetails);
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        assert_eq!(0, lots.len());
    });
}

#[test]
fn it_fails_create_lot_expired() {
    new_test_ext().execute_with(|| {
        // cc - carbon credits
        let cc_id = 666;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
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

        assert_noop!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()),
            RuntimeError::LotExpired);
        let lots = EvercityExchange::lots(cc_holder, cc_id);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_ok!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy));
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_noop!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::InsufficientEverUSDBalance);
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        // speed time
        Timestamp::set_timestamp(1_000); 
        assert_noop!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::LotExpired);
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_noop!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::NotEnoughCarbonCreditsInLot);
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
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
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(Origin::signed(cc_holder), project_id, cc_id, cc_holder, 1);
        let cc_holder_amount = EvercityAssets::balance(cc_id, cc_holder);
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

        assert_ok!(EvercityExchange::create_carbon_credit_lot(Origin::signed(cc_holder), cc_id, lot.clone()));
        assert_noop!(EvercityExchange::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), cc_holder, 
            cc_id, lot.clone(), cc_to_buy), RuntimeError::LotNotFound);
        let lots = EvercityExchange::lots(cc_holder, cc_id);
        let cc_holder_amount_after = EvercityAssets::balance(cc_id, cc_holder);
        let everusd_balance_after = EvercityBonds::balance_everusd(&everusd_holder);

        assert_eq!(1, lots.len());
        assert_eq!(cc_holder_amount, cc_holder_amount_after);
        assert_eq!(everusd_balance, everusd_balance_after);
    });
}

// helpers funcs
mod helpers {
    use pallet_evercity_carbon_credits::annual_report::*;
    use pallet_evercity_carbon_credits::project::ProjectId;
    use pallet_evercity_carbon_credits::standard::Standard;
    use pallet_evercity_filesign::file::FileId;
    use pallet_evercity_filesign::file::H256;
    use super::*;

    pub(crate) fn get_test_carbon_credits_name() -> Vec<u8> {
        "CarbonToken".to_owned().as_bytes().to_vec()
    }
    
    pub(crate) fn get_test_carbon_credits_symbol() -> Vec<u8> {
        "CT".to_owned().as_bytes().to_vec()
    }
    
    pub(crate) fn create_project_documentation_file(account: u64) -> Option<FileId> {
        let filehash = H256::from([0x66; 32]);
        let file_id = Some([6; 16]);
        let _ = EvercityFilesign::create_new_file(Origin::signed(account), "my_project_documentation".to_owned().as_bytes().to_vec(), filehash, file_id);
        file_id
    }
    
    pub(crate) fn create_annual_report_file(account: u64) -> FileId {
        let filehash = H256::from([0x88; 32]);
        let file_id = [9; 16];
        let _ = EvercityFilesign::create_new_file(Origin::signed(account), "my_annual_report".to_owned().as_bytes().to_vec(), filehash, Some(file_id));
        file_id
    }
    
    /// Return tuple -> (project, project_id, project_owner)
    pub(crate) fn get_registerd_project_and_owner_gold_standard() -> (ProjectId, u64) {
        get_project_and_owner_and_custom_signers(assign_project_mock_users_required_signers_gold_standard)
    }
    
    pub(crate) fn get_project_and_owner_and_custom_signers<F>(sign_func: F) -> (ProjectId, u64) where F: Fn(ProjectId) {
        let owner = ROLES[1].0;
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
        let standard = Standard::GOLD_STANDARD;
    
        let _ = CarbonCredits::create_project(Origin::signed(owner), standard, create_project_documentation_file(owner));
        sign_func(1);
    
        let _ = CarbonCredits::sign_project(Origin::signed(owner), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(auditor), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(standard_acc), 1);
        let _ = CarbonCredits::sign_project(Origin::signed(registry), 1);
    
        (1, owner)
    }
    
    /// Return tuple -> (project_id, project_owner)
    pub(crate) fn full_sign_annual_report_gold_standard() -> (ProjectId, u64) {
        get_annual_report_and_owner_custom_signers(assign_annual_report_mock_users_required_signers_gold_standard)
    }
    
    pub(crate) fn get_annual_report_and_owner_custom_signers<F>(sign_func: F) -> (ProjectId, u64) where F: Fn(ProjectId) {
        let (proj_id, owner) = get_registerd_project_and_owner_gold_standard();
        let auditor = ROLES[2].0;
        let standard_acc = ROLES[3].0;
        let registry = ROLES[5].0;
    
        let _ = CarbonCredits::create_annual_report(
            Origin::signed(owner), 
            proj_id, 
            create_annual_report_file(owner), 
            TEST_CARBON_CREDITS_COUNT, 
            get_test_carbon_credits_name(), 
            get_test_carbon_credits_symbol(), 
            TEST_CARBON_CREDITS_DECIMAL
        );
        sign_func(proj_id);
    
        let tuple_vec = vec![
            (owner, REPORT_AUDITOR_SIGN_PENDING),
            (auditor, REPORT_STANDARD_SIGN_PENDING),
            (standard_acc, REPORT_REGISTRY_SIGN_PENDING),
            (registry, REPORT_ISSUED)
        ];
    
        tuple_vec.iter()
            .map(|account_state_tuple| {
                let acc = account_state_tuple.0;
                let state = account_state_tuple.1;
                let result = CarbonCredits::sign_last_annual_report(Origin::signed(acc), proj_id);
    
                (acc, state, result)
            })
            .for_each(|account_state_result_tuple|{
                let _ = account_state_result_tuple.0;
                let _ = account_state_result_tuple.1;
                let _ = account_state_result_tuple.2;
            });
    
        (1, owner)
    }
    
    pub(crate) fn assign_project_mock_users_required_signers_gold_standard(project_id: ProjectId) {
        let owner = ROLES[1].0;
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_project_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
    }
    
    pub(crate) fn assign_annual_report_mock_users_required_signers_gold_standard(project_id: ProjectId) {
        let owner = ROLES[1].0;
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[1].0, ROLES[1].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[2].0, ROLES[2].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[3].0, ROLES[3].1, project_id);
        let _ = CarbonCredits::assign_last_annual_report_signer(Origin::signed(owner), ROLES[5].0, ROLES[5].1, project_id);
    }
}
