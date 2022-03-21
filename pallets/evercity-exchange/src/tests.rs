use frame_support::assert_noop;
use frame_support::assert_ok;

use helpers::*;

pub const TEST_CARBON_CREDITS_COUNT: u128 = 15000;
pub const TEST_CARBON_CREDITS_DECIMAL: u8 = 0;

use crate::Error;
use crate::mock::*;
use crate::everusd_trade_request::EverUSDTradeHolderType;

type RuntimeError = Error<TestRuntime>;

#[test]
fn it_works_exhange_cc_holder() {
    new_test_ext().execute_with(|| {
        let everusd_holder = 1;
        let carbon_credits_id = 666;
        let everusd_balance = 10000;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(
            Origin::signed(cc_holder), project_id, carbon_credits_id, cc_holder, 1);

        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_to_send = 600;
        let everusd_to_send = 500;
        let create_trade_request_result = 
            EvercityExchange::create_everusd_trade_request(
                Origin::signed(cc_holder), 
                everusd_holder,
                everusd_to_send,
                carbon_credits_id,
                cc_to_send,
                EverUSDTradeHolderType::CarbonCreditsHolder
            );
        
        let accept_trade_request_result = 
            EvercityExchange::accept_everusd_trade_request(Origin::signed(everusd_holder), 1, EverUSDTradeHolderType::EverUSDHolder);

        assert_eq!(everusd_to_send, EvercityBonds::get_balance(&cc_holder));
        assert_eq!(cc_to_send, CarbonCredits::balance(carbon_credits_id, everusd_holder));
        assert_ok!(create_trade_request_result, ().into());
        assert_ok!(accept_trade_request_result, ().into());
    });
}

#[test]
fn it_works_exhange_everusd_holder() {
    new_test_ext().execute_with(|| {
        let everusd_holder = 1;
        let carbon_credits_id = 666;
        let everusd_balance = 10000;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(
            Origin::signed(cc_holder), project_id, carbon_credits_id, cc_holder, 1);

        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_to_send = 600;
        let everusd_to_send = 500;
        let create_trade_request_result = 
            EvercityExchange::create_everusd_trade_request(
                Origin::signed(everusd_holder), 
                cc_holder,
                everusd_to_send,
                carbon_credits_id,
                cc_to_send,
                EverUSDTradeHolderType::EverUSDHolder
            );
        
        let accept_trade_request_result = 
            EvercityExchange::accept_everusd_trade_request(Origin::signed(cc_holder), 1, EverUSDTradeHolderType::CarbonCreditsHolder);

        assert_eq!(everusd_to_send, EvercityBonds::get_balance(&cc_holder));
        assert_eq!(cc_to_send, CarbonCredits::balance(carbon_credits_id, everusd_holder));
        assert_ok!(create_trade_request_result, ().into());
        assert_ok!(accept_trade_request_result, ().into());
    });
}

#[test]
fn it_fails_exhange_no_everusd_create() {
    new_test_ext().execute_with(|| {
        let everusd_holder = 1;
        let carbon_credits_id = 666;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(
            Origin::signed(cc_holder), project_id, carbon_credits_id, cc_holder, 1);

        // EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_to_send = 600;
        let everusd_to_send = 500;
        let create_trade_request_result = 
            EvercityExchange::create_everusd_trade_request(
                Origin::signed(cc_holder), 
                everusd_holder,
                everusd_to_send,
                carbon_credits_id,
                cc_to_send,
                EverUSDTradeHolderType::CarbonCreditsHolder
            );
        
        let accept_trade_request_result = 
            EvercityExchange::accept_everusd_trade_request(Origin::signed(everusd_holder), 1, EverUSDTradeHolderType::EverUSDHolder);

        assert_eq!(0, EvercityBonds::get_balance(&cc_holder));
        assert_eq!(0, CarbonCredits::balance(carbon_credits_id, everusd_holder));
        assert_noop!(create_trade_request_result, RuntimeError::InsufficientEverUSDBalance);
        assert_noop!(accept_trade_request_result, RuntimeError::TradeRequestNotFound);
    });
}

#[test]
fn it_fails_exhange_no_cc_create() {
    new_test_ext().execute_with(|| {
        let everusd_holder = 1;
        let carbon_credits_id = 666;
        let everusd_balance = 10000;
        let (_, cc_holder) = full_sign_annual_report_gold_standard();

        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_to_send = 600;
        let everusd_to_send = 500;
        let create_trade_request_result = 
            EvercityExchange::create_everusd_trade_request(
                Origin::signed(cc_holder), 
                everusd_holder,
                everusd_to_send,
                carbon_credits_id,
                cc_to_send,
                EverUSDTradeHolderType::CarbonCreditsHolder
            );
        
        let accept_trade_request_result = 
            EvercityExchange::accept_everusd_trade_request(Origin::signed(everusd_holder), 1, EverUSDTradeHolderType::EverUSDHolder);

        assert_eq!(0, EvercityBonds::get_balance(&cc_holder));
        assert_eq!(0, CarbonCredits::balance(carbon_credits_id, everusd_holder));
        assert_noop!(create_trade_request_result, RuntimeError::InsufficientCarbonCreditsBalance);
        assert_noop!(accept_trade_request_result, RuntimeError::TradeRequestNotFound);
    });
}

#[test]
fn it_fails_exhange_no_everusd_accept() {
    new_test_ext().execute_with(|| {
        let everusd_holder = 1;
        let carbon_credits_id = 666;
        let everusd_balance = 10000;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(
            Origin::signed(cc_holder), project_id, carbon_credits_id, cc_holder, 1);

        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_to_send = 600;
        let everusd_to_send = 500;
        let _ = 
            EvercityExchange::create_everusd_trade_request(
                Origin::signed(cc_holder), 
                everusd_holder,
                everusd_to_send,
                carbon_credits_id,
                cc_to_send,
                EverUSDTradeHolderType::CarbonCreditsHolder
            );

        EvercityBonds::set_balance(&everusd_holder, 0);
        let accept_trade_request_result = 
            EvercityExchange::accept_everusd_trade_request(Origin::signed(everusd_holder), 1, EverUSDTradeHolderType::EverUSDHolder);

        assert_eq!(0, EvercityBonds::get_balance(&cc_holder));
        assert_eq!(0, CarbonCredits::balance(carbon_credits_id, everusd_holder));
        assert_noop!(accept_trade_request_result, RuntimeError::InsufficientEverUSDBalance);
        // assert_noop!(accept_trade_request_result, RuntimeError::TradeRequestNotFound);
    });
}

#[test]
fn it_fails_exhange_no_cc_accept() {
    new_test_ext().execute_with(|| {
        let everusd_holder = 1;
        let carbon_credits_id = 666;
        let everusd_balance = 10000;
        let (project_id, cc_holder) = full_sign_annual_report_gold_standard();
        let _ = CarbonCredits::release_carbon_credits(
            Origin::signed(cc_holder), project_id, carbon_credits_id, cc_holder, 1);

        EvercityBonds::set_balance(&everusd_holder, everusd_balance);
        
        let cc_to_send = 600;
        let everusd_to_send = 500;
        let _ = 
            EvercityExchange::create_everusd_trade_request(
                Origin::signed(cc_holder), 
                everusd_holder,
                everusd_to_send,
                carbon_credits_id,
                cc_to_send,
                EverUSDTradeHolderType::CarbonCreditsHolder
            );

        let _ = CarbonCredits::transfer_carbon_credits(
            Origin::signed(cc_holder), carbon_credits_id, 555, TEST_CARBON_CREDITS_COUNT);

        let accept_trade_request_result = 
            EvercityExchange::accept_everusd_trade_request(Origin::signed(everusd_holder), 1, EverUSDTradeHolderType::EverUSDHolder);
        
        assert_eq!(0, EvercityBonds::get_balance(&cc_holder));
        assert_eq!(0, CarbonCredits::balance(carbon_credits_id, everusd_holder));
        assert_noop!(accept_trade_request_result, RuntimeError::InsufficientCarbonCreditsBalance);
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
            Origin::signed(owner), proj_id, create_annual_report_file(owner), TEST_CARBON_CREDITS_COUNT, get_test_carbon_credits_name() , get_test_carbon_credits_symbol(), TEST_CARBON_CREDITS_DECIMAL
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
