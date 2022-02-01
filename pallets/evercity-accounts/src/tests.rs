use crate::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult, 
}};
use crate::accounts::*;

#[test]
fn it_works_account_add_with_role_and_data() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let assign_role_result = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        assert_ok!(assign_role_result, ());
    });
}

#[test]
fn it_fils_account_add_with_role_and_data_not_master() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let assign_role_result = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[1].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        assert_ne!(assign_role_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_fails_account_set_with_role_and_data_not_exits() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        assert_ne!(assign_role_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_works_account_set_with_role_and_data() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_AUDITOR_ROLE_MASK);
        assert!(EvercityAccounts::account_is_cc_investor(&some_new_account));
        assert_ok!(assign_role_result, ());
    });
}

#[test]
fn it_fails_account_set_with_role_and_data_not_master() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[1].0), some_new_account, CC_AUDITOR_ROLE_MASK);
        assert_ne!(assign_role_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_fails_account_set_with_master_role() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, MASTER_ROLE_MASK);
        assert_ne!(assign_role_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_works_roles_assigned_correctly_set_master() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CUSTODIAN_ROLE_MASK);
        let all_roles = vec![
                CUSTODIAN_ROLE_MASK, 
                ISSUER_ROLE_MASK, 
                INVESTOR_ROLE_MASK, 
                AUDITOR_ROLE_MASK, 
                MANAGER_ROLE_MASK, 
                IMPACT_REPORTER_ROLE_MASK, 
                EMISSION_CREATOR_ROLE_MASK,
                CC_PROJECT_OWNER_ROLE_MASK, 
                CC_AUDITOR_ROLE_MASK, 
                CC_STANDARD_ROLE_MASK, 
                CC_INVESTOR_ROLE_MASK, 
                CC_REGISTRY_ROLE_MASK
        ];

        all_roles.iter().for_each(|x| {
            let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, *x);
            assert_ok!(assign_role_result,());
        });

        assert!(EvercityAccounts::account_is_cc_project_owner(&some_new_account));
        assert!(EvercityAccounts::account_is_cc_auditor(&some_new_account));
        assert!(EvercityAccounts::account_is_cc_standard(&some_new_account));
        assert!(EvercityAccounts::account_is_cc_investor(&some_new_account));
        assert!(EvercityAccounts::account_is_cc_registry(&some_new_account));
    });
}

#[test]
fn it_works_account_set_with_master_role() {
    new_test_ext().execute_with(|| {
        let some_new_master_account = 666;
        let some_new_account = 1349;
        let set_master_result = EvercityAccounts::set_master(Origin::signed(ROLES[0].0), some_new_master_account);
        let assign_role_result = EvercityAccounts::account_add_with_role_and_data(Origin::signed(some_new_master_account), some_new_account, CC_PROJECT_OWNER_ROLE_MASK);

        assert_ok!(set_master_result, ());
        assert_ok!(assign_role_result, ());
        assert!(EvercityAccounts::account_is_master(&some_new_master_account));
        assert!(EvercityAccounts::account_is_cc_project_owner(&some_new_account));
    });
}

#[test]
fn it_fails_account_set_with_master_role_already_master() {
    new_test_ext().execute_with(|| {
        let some_new_master_account = 666;
        let _ = EvercityAccounts::set_master(Origin::signed(ROLES[0].0), some_new_master_account);
        let set_master_result = EvercityAccounts::set_master(Origin::signed(ROLES[0].0), some_new_master_account);

        assert_ne!(set_master_result, DispatchResult::Ok(()));
    });
}

#[test]
fn it_works_account_withraw_role() {
    new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_AUDITOR_ROLE_MASK);

        let withdraw_role_result = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);

        assert_ok!(assign_role_result, ());
        assert_ok!(withdraw_role_result, ());
        assert!(!EvercityAccounts::account_is_cc_investor(&some_new_account));
    });
}

#[test]
fn it_works_check_events() {
    new_test_ext_with_event().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        let add_account_event = last_event().unwrap();

        let _ = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_AUDITOR_ROLE_MASK);
        let set_account_event = last_event().unwrap();

        let _ = EvercityAccounts::set_master(Origin::signed(ROLES[0].0), some_new_account);
        let set_master_event = last_event().unwrap();

        let _ = EvercityAccounts::account_withdraw_role(Origin::signed(ROLES[0].0), some_new_account, CC_AUDITOR_ROLE_MASK);
        let withdraw_account_event = last_event().unwrap();

        assert_eq!(Event::pallet_evercity_accounts(crate::RawEvent::AccountAdd(ROLES[0].0, some_new_account, CC_INVESTOR_ROLE_MASK)), add_account_event);
        assert_eq!(Event::pallet_evercity_accounts(crate::RawEvent::AccountSet(ROLES[0].0, some_new_account, CC_AUDITOR_ROLE_MASK)), set_account_event);
        assert_eq!(Event::pallet_evercity_accounts(crate::RawEvent::MasterSet(ROLES[0].0, some_new_account)), set_master_event);
        assert_eq!(Event::pallet_evercity_accounts(crate::RawEvent::AccountWithdraw(ROLES[0].0, some_new_account, CC_AUDITOR_ROLE_MASK)), withdraw_account_event);
    });
}
