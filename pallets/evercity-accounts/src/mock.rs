#![allow(clippy::from_over_into)]

use frame_support::{sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
}, traits::GenesisBuild};
use sp_core::H256;
use crate as pallet_evercity_accounts;
use crate::accounts::*;
use frame_support::parameter_types;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		EvercityAccounts: pallet_evercity_accounts::{Module, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Module, Call, Storage},
	}
);

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();

}

impl pallet_evercity_accounts::Config for TestRuntime {
    type Event = Event;
}

impl pallet_timestamp::Config for TestRuntime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
    pub const ExistentialDeposit: u64 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for TestRuntime {
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

// (AccountId, role)
pub static ROLES: [(u64, RoleMask, u64); 13] = [
    (1_u64, MASTER_ROLE_MASK, 0),
    (2_u64, CUSTODIAN_ROLE_MASK, 0),
    (3_u64, ISSUER_ROLE_MASK, 0),
    (4_u64, INVESTOR_ROLE_MASK, 0),
    (5_u64, AUDITOR_ROLE_MASK, 0),
    (6_u64, MANAGER_ROLE_MASK, 0),
    (7_u64, BOND_ARRANGER_ROLE_MASK, 0),
    (8_u64, IMPACT_REPORTER_ROLE_MASK, 0),
    (9_u64, CC_PROJECT_OWNER_ROLE_MASK, 0),
    (10_u64, CC_AUDITOR_ROLE_MASK, 0),
    (11_u64, CC_STANDARD_ROLE_MASK, 0),
    (12_u64, CC_INVESTOR_ROLE_MASK, 0),
    (13_u64, CC_REGISTRY_ROLE_MASK, 0),
];

//Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> frame_support::sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();
    pallet_balances::GenesisConfig::<TestRuntime> {
        // Provide some initial balances
        balances: ROLES.iter().map(|x| (x.0, 100000)).collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    frame_support::traits::GenesisBuild::assimilate_storage(&super::GenesisConfig::<TestRuntime> {
        // Accounts for tests
        genesis_account_registry: ROLES.to_vec()
    }, &mut t)
    .unwrap();

    t.into()
}


pub fn new_test_ext_with_event() -> frame_support::sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();
    pallet_balances::GenesisConfig::<TestRuntime> {
        // Provide some initial balances
        balances: ROLES.iter().map(|x| (x.0, 100000)).collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    super::GenesisConfig::<TestRuntime> {
        // Accounts for tests
        genesis_account_registry: ROLES.to_vec()
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// get and cut last event
#[allow(clippy::result_unit_err)] 
pub fn last_event() -> Result<Event, ()> {
	match System::events().pop() {
		Some(ev) => Ok(ev.event),
		None => Err(())
	}
}