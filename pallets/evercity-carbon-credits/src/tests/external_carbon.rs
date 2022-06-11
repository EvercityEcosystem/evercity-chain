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
        let external_project_id: Vec<u8> = "1234v".as_bytes().to_vec();
       // let amount = 1_000;
        assert_ok!(CarbonCredits::create_batch_asset(Origin::signed(acc), RegistryType::Cercarbono, external_project_id));
        let event = System::events().pop().unwrap().event;
        let batch_id = CarbonCredits::get_random_batch_id(&acc);
        assert_eq!(Event::pallet_carbon_credits(crate::Event::BatchAssetCreated(acc, batch_id)), event);

        let asset = CarbonCredits::batch_assets(batch_id).unwrap();
        assert_eq!(acc, asset.owner);
        //assert_eq!(amount, asset.amount);
        assert_eq!(RegistryType::Cercarbono, asset.registry_type);
        assert_eq!(BatchStatus::INITIAL, asset.status);
    });
}

#[test]
fn it_works_create_batch_asset_() {
    new_test_ext().execute_with(|| {});
}