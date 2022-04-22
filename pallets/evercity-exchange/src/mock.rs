#![allow(clippy::from_over_into)]

use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_support::parameter_types;
use sp_core::H256;
use crate as pallet_evercity_exchange;
use pallet_evercity_accounts::accounts::*;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{ Module, Call, Config, Storage, Event<T> },
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		CarbonCredits: pallet_evercity_carbon_credits::{ Module, Call, Storage, Event<T> },
		EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Event<T> },
		Timestamp: pallet_timestamp::{ Module, Call, Storage, Inherent},
        EvercityAssets: pallet_evercity_assets::{ Module, Call, Storage, Event<T> },
        EvercityFilesign: pallet_evercity_filesign::{ Module, Call, Storage, Event<T> },
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        EvercityExchange: pallet_evercity_exchange::{ Module, Call, Storage, Event<T> },
        EvercityBonds: pallet_evercity_bonds::{Module, Call, Storage, Event<T>},
	}
);

const DEFAULT_DAY_DURATION: u32 = 60; // 86400; seconds in 1 DAY

parameter_types! {
    pub const BurnRequestTtl: u32 = DEFAULT_DAY_DURATION as u32 * 7 * 1000;
    pub const MintRequestTtl: u32 = DEFAULT_DAY_DURATION as u32 * 7 * 1000;
    pub const MaxMintAmount: pallet_evercity_bonds::EverUSDBalance = 60_000_000_000_000_000;
    pub const TimeStep: pallet_evercity_bonds::BondPeriod = DEFAULT_DAY_DURATION;
}

impl pallet_evercity_bonds::Config for TestRuntime {
    type Event = Event;
    type BurnRequestTtl = BurnRequestTtl;
    type MintRequestTtl = MintRequestTtl;
    type MaxMintAmount = MaxMintAmount;
    type TimeStep = TimeStep;
    type WeightInfo = ();
    type OnAddAccount = ();
    type OnAddBond = ();
}


impl pallet_evercity_exchange::Config for TestRuntime {
    type Event = Event;
}

type AccountId = u64;

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
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl pallet_evercity_carbon_credits::Config for TestRuntime {
	type Event = Event;
}

impl pallet_evercity_accounts::Config for TestRuntime {
	type Event = Event;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 6000 / 2;
}

impl pallet_timestamp::Config for TestRuntime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

// ballances
parameter_types! {
    pub const ExistentialDeposit: u64 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for TestRuntime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

// Asset Pallet Configs
pub type Balance = u128;
pub type CCAmount = u64;

parameter_types! {
    pub const AssetDeposit: CCAmount = 1; 
    pub const ApprovalDeposit: CCAmount = 1;
    pub const StringLimit: u32 = 50;
    /// https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
    pub const MetadataDepositBase: CCAmount = 1;
    pub const MetadataDepositPerByte: CCAmount = 1;
}

impl pallet_evercity_assets::Config for TestRuntime {
    type Event = Event;
    type Balance = CCAmount;
    type AssetId = u32;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureSigned<AccountId>;
    type AssetDepositBase = AssetDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type AssetDepositPerZombie = AssetDeposit;
    type StringLimit = StringLimit;
    type WeightInfo = ();
}

impl pallet_evercity_filesign::Config for TestRuntime {
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
}

// (AccountId, role)
pub static ROLES: [(u64, RoleMask); 6] = [
    (1_u64, MASTER_ROLE_MASK),
    (2_u64, CC_PROJECT_OWNER_ROLE_MASK),
    (3_u64, CC_AUDITOR_ROLE_MASK),
    (4_u64, CC_STANDARD_ROLE_MASK),
    (5_u64, CC_INVESTOR_ROLE_MASK),
    (6_u64, CC_REGISTRY_ROLE_MASK),
];

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> frame_support::sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();

    pallet_balances::GenesisConfig::<TestRuntime> {
        // Provide some initial balances
        balances: ROLES.iter().map(|x| (x.0, 10000000)).collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

	pallet_evercity_accounts::GenesisConfig::<TestRuntime> {
        // Accounts for tests
        genesis_account_registry: ROLES
            .iter()
            .map(|(acc, role)| {
                (
                    *acc,
                    AccountStruct {
                        roles: *role
                    },
                )
            })
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}

// Get events list
#[allow(dead_code)]
fn events() -> Vec<Event> {
    let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();
    System::reset_events();
    evt
}