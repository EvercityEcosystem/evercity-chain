use frame_support::assert_noop;
use frame_support::assert_ok;

use crate::cc_package_lot::CarbonCreditsPackageLot;

pub const TEST_CARBON_CREDITS_COUNT: u64 = 15000;
pub const TEST_CARBON_CREDITS_DECIMAL: u8 = 0;

use crate::Error;
use crate::external_carbon_units::*;
use crate::tests::mock::*;
use sp_runtime::traits::Header;

use super::mock::TestRuntime;


type RuntimeError = Error<TestRuntime>;

fn setup_blocks(blocks: u64) {
    let mut parent_hash = System::parent_hash();

    for i in 1..(blocks + 1) {
        System::reset_events();
        System::initialize(&i, &parent_hash, &Default::default(), Default::default());

        let header = System::finalize();
        parent_hash = header.hash();
        System::set_block_number(*header.number());
    }
}

#[test]
fn random_batch_id_ok() {
    new_test_ext().execute_with(|| {
        setup_blocks(38);
        let acc = 4;
        <frame_system::Module<TestRuntime>>::set_extrinsic_index(1);
        let rnd = CarbonCredits::get_random_batch_id(&acc);
        assert!(rnd.len() == 32);
        assert_eq!(&rnd[0..13], "EVERCITY-1.0-".as_bytes());
        assert_ne!(&rnd[13..30], &[0; 16]);
        assert_eq!(&rnd[29..], &[0; 3]);
    });
}

#[test]
fn it_works_create_batch_asset() {
    new_test_ext().execute_with(|| {
        setup_blocks(38);
        let acc = 1;

        assert_ok!(CarbonCredits::external_create_batch_asset(Origin::signed(acc)));
        let event = System::events().pop().unwrap().event;
        let batch_id = CarbonCredits::get_random_batch_id(&acc);
        assert_eq!(Event::pallet_carbon_credits(crate::Event::BatchAssetCreated(acc, batch_id)), event);

        let asset = CarbonCredits::batch_asset(batch_id).unwrap();
        assert_eq!(acc, asset.owner);
        assert_eq!(BatchStatus::INITIAL, asset.status);
    });
}

#[test]
fn it_works_update_batch_asset() {
    new_test_ext().execute_with(|| {
        setup_blocks(38);
        let acc = 1;
        let external_project_id: Vec<u8> = "1234v".as_bytes().to_vec();
        let vintage_name = "1989-1990".as_bytes().to_vec();
        let serial_number = "1-2345-901".as_bytes().to_vec();
        let amount = 50;

        assert_ok!(CarbonCredits::external_create_batch_asset(Origin::signed(acc)));
        let event = System::events().pop().unwrap().event;
        let batch_id = CarbonCredits::get_random_batch_id(&acc);
        assert_eq!(Event::pallet_carbon_credits(crate::Event::BatchAssetCreated(acc, batch_id)), event);

        let asset = CarbonCredits::batch_asset(batch_id).unwrap();
        assert_eq!(acc, asset.owner);

        assert_ok!(CarbonCredits::external_update_batch_asset(Origin::signed(acc), batch_id, RegistryType::Cercarbono,
            external_project_id.clone(), Some(vintage_name.clone()), serial_number.clone(), amount));
        let event = System::events().pop().unwrap().event;
        assert_eq!(Event::pallet_carbon_credits(crate::Event::BatchAssetUpdated(batch_id)), event);
        let asset = CarbonCredits::batch_asset(batch_id).unwrap();
        assert_eq!(acc, asset.owner);
        assert_eq!(BatchStatus::AWAITING_VERIFICATION, asset.status);
        assert_eq!(external_project_id, asset.external_project_id);
        assert_eq!(vintage_name, asset.vintage_name);
        assert_eq!(serial_number, asset.serial_number);
        assert_eq!(amount, asset.amount);
    });
}

#[test]
fn it_fails_update_batch_asset_not_found() {
    new_test_ext().execute_with(|| {
        setup_blocks(38);
        let acc = 1;
        let external_project_id: Vec<u8> = "1234v".as_bytes().to_vec();
        let vintage_name = "1989-1990".as_bytes().to_vec();
        let serial_number = "1-2345-901".as_bytes().to_vec();
        let amount = 50;

        let batch_id = CarbonCredits::get_random_batch_id(&acc);

        assert_noop!(CarbonCredits::external_update_batch_asset(Origin::signed(acc), batch_id, RegistryType::Cercarbono,
            external_project_id.clone(), Some(vintage_name.clone()), serial_number.clone(), amount),
            RuntimeError::BatchNotFound);
        let asset = CarbonCredits::batch_asset(batch_id);
        assert_eq!(None, asset);
    });
}
const ASSET_OWNER: u64 = 3;
fn create_update_batch_asset() -> BatchAssetId {
    setup_blocks(38);
    let external_project_id: Vec<u8> = "1234v".as_bytes().to_vec();
    let vintage_name = "1989-1990".as_bytes().to_vec();
    let serial_number = "1-2345-901".as_bytes().to_vec();
    let amount = 50;

    assert_ok!(CarbonCredits::external_create_batch_asset(Origin::signed(ASSET_OWNER)));
    let event = System::events().pop().unwrap().event;
    let batch_id = CarbonCredits::get_random_batch_id(&ASSET_OWNER);
    assert_eq!(Event::pallet_carbon_credits(crate::Event::BatchAssetCreated(ASSET_OWNER, batch_id)), event);

    let asset = CarbonCredits::batch_asset(batch_id).unwrap();
    assert_eq!(ASSET_OWNER, asset.owner);

    assert_ok!(CarbonCredits::external_update_batch_asset(Origin::signed(ASSET_OWNER), batch_id, RegistryType::Cercarbono,
        external_project_id.clone(), Some(vintage_name.clone()), serial_number.clone(), amount));
    batch_id
}

fn create_full_batch_asset() -> BatchAssetId {
    let batch_id = create_update_batch_asset();
    let uri = "htps://uiuiw".as_bytes().to_vec();
    let hash = "12ac23e22b".as_bytes().to_vec();

    assert_ok!(CarbonCredits::external_add_project_ipfs_link(Origin::signed(2), batch_id, uri.clone(), hash.clone()));
    let asset = CarbonCredits::batch_asset(batch_id).unwrap();
    assert_eq!(uri, asset.uri);
    assert_eq!(hash.clone(), asset.ipfs_hash);
    let event = System::events().pop().unwrap().event;
    assert_eq!(Event::pallet_carbon_credits(crate::Event::BatchAssetAddedIpfsLink(batch_id.clone(), hash)), event);
    batch_id
}

#[test]
fn it_works_add_ipfs_link() {
    new_test_ext().execute_with(|| {
        let batch_id = create_update_batch_asset();
        let uri = "htps://uiuiw".as_bytes().to_vec();
        let hash = "12ac23e22b".as_bytes().to_vec();

        assert_ok!(CarbonCredits::external_add_project_ipfs_link(Origin::signed(2), batch_id, uri.clone(), hash.clone()));
        let asset = CarbonCredits::batch_asset(batch_id).unwrap();
        assert_eq!(uri, asset.uri);
        assert_eq!(hash.clone(), asset.ipfs_hash);
        let event = System::events().pop().unwrap().event;
        assert_eq!(Event::pallet_carbon_credits(crate::Event::BatchAssetAddedIpfsLink(batch_id, hash)), event);
    });
}

#[test]
fn it_fails_add_ipfs_link_not_found() {
    new_test_ext().execute_with(|| {
        setup_blocks(38);
        let acc = 1;
        let batch_id = CarbonCredits::get_random_batch_id(&acc);
        let uri = "htps://uiuiw".as_bytes().to_vec();
        let hash = "12ac23e22b".as_bytes().to_vec();

        assert_noop!(CarbonCredits::external_add_project_ipfs_link(Origin::signed(2), batch_id, uri.clone(), hash.clone()),
            RuntimeError::BatchNotFound);
        let asset = CarbonCredits::batch_asset(batch_id);
        assert_eq!(None, asset);
    });
}

#[test]
fn it_works_verify_batch_asset() {
    new_test_ext().execute_with(|| {
        let batch_id = create_full_batch_asset();
        let asset_id = 456;
        let min_balance = 1;
        let manager = 7;

        assert_ok!(CarbonCredits::external_verify_batch_asset(Origin::signed(manager), batch_id, asset_id, min_balance));
        let balance = Assets::balance(asset_id, ASSET_OWNER);
        assert_eq!(50, balance);
        let passport = CarbonCredits::get_passport_by_assetid(asset_id).unwrap();
        assert_eq!(Some(batch_id), passport.get_batch_asset_id());
    });
}

#[test]
fn it_fails_verify_batch_asset_not_manager() {
    new_test_ext().execute_with(|| {
        let batch_id = create_full_batch_asset();
        let asset_id = 456;
        let min_balance = 1;
        let not_manager = 2;

        assert_noop!(CarbonCredits::external_verify_batch_asset(Origin::signed(not_manager), batch_id, asset_id, min_balance),
            RuntimeError::AccountIncorrectRole);
        let balance = Assets::balance(asset_id, ASSET_OWNER);
        assert_eq!(0, balance);
        let passport = CarbonCredits::get_passport_by_assetid(asset_id);
        assert_eq!(None, passport);
    });
}

#[test]
fn it_works_verify_batch_asset_create_lot_buy_lot() {
    new_test_ext().execute_with(|| {
        // export external carbon
        let batch_id = create_full_batch_asset();
        let asset_id = 456;
        let min_balance = 1;
        let manager = 7;

        assert_ok!(CarbonCredits::external_verify_batch_asset(Origin::signed(manager), batch_id, asset_id, min_balance));
        let balance = Assets::balance(asset_id, ASSET_OWNER);
        assert_eq!(50, balance);

        // create lot
        let new_lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: 100_000,
            amount: 20,
            price_per_item: 10_000_000_000,
        };
        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(ASSET_OWNER), asset_id, new_lot.clone()));
        let lots = CarbonCredits::lots(ASSET_OWNER, asset_id);
        assert!(lots.is_some());

        // buy lot
        let everusd_holder = 5;
        EvercityBonds::set_balance(&everusd_holder, 1_000_000_000_000);
        assert_ok!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), ASSET_OWNER, 
        asset_id, new_lot, 20));
        
        assert_eq!(30, Assets::balance(asset_id, ASSET_OWNER));
        assert_eq!(20, Assets::balance(asset_id, everusd_holder));
    });
}

#[test]
fn it_works_verify_batch_asset_create_lot_burn_cc() {
    new_test_ext().execute_with(|| {
        // export external carbon
        let batch_id = create_full_batch_asset();
        let asset_id = 456;
        let min_balance = 1;
        let manager = 7;

        assert_ok!(CarbonCredits::external_verify_batch_asset(Origin::signed(manager), batch_id, asset_id, min_balance));
        assert_eq!(50, Assets::balance(asset_id, ASSET_OWNER));

        // burn some
        assert_ok!(CarbonCredits::burn_carbon_credits(Origin::signed(ASSET_OWNER), asset_id, 5));
        assert_eq!(45, Assets::balance(asset_id, ASSET_OWNER));

        // create lot
        let new_lot = CarbonCreditsPackageLot {
            target_bearer: None,
            deadline: 100_000,
            amount: 20,
            price_per_item: 10_000_000_000,
        };
        assert_ok!(CarbonCredits::create_carbon_credit_lot(Origin::signed(ASSET_OWNER), asset_id, new_lot.clone()));
        let lots = CarbonCredits::lots(ASSET_OWNER, asset_id);
        assert!(lots.is_some());

        // buy lot
        let everusd_holder = 5;
        EvercityBonds::set_balance(&everusd_holder, 1_000_000_000_000);
        assert_ok!(CarbonCredits::buy_carbon_credit_lot_units(Origin::signed(everusd_holder), ASSET_OWNER, 
        asset_id, new_lot, 20));
        
        assert_eq!(25, Assets::balance(asset_id, ASSET_OWNER));
        assert_eq!(20, Assets::balance(asset_id, everusd_holder));

        // burn some
        assert_ok!(CarbonCredits::burn_carbon_credits(Origin::signed(ASSET_OWNER), asset_id, 15));
        assert_eq!(10, Assets::balance(asset_id, ASSET_OWNER));

        assert_ok!(CarbonCredits::burn_carbon_credits(Origin::signed(everusd_holder), asset_id, 10));
        assert_eq!(10, Assets::balance(asset_id, everusd_holder));
    });
}