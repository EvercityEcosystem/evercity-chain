use frame_support::assert_noop;
use frame_support::assert_ok;

use crate::tests::helpers::*;

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

fn create_update_batch_asset() -> BatchAssetId {
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
fn it_works_create_batch_asset_() {
    new_test_ext().execute_with(|| {
        let batch_id = create_update_batch_asset();
        
    });
}