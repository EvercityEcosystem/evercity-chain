#![allow(clippy::from_over_into)]

use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_support::parameter_types;
use pallet_evercity_bonds::{bond::BondInnerStructOf, BondStructOf};
use sp_core::H256;
use crate as pallet_carbon_credits;
use pallet_evercity_accounts::accounts::*;
pub use pallet_evercity_assets as pallet_assets;


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
		CarbonCredits: pallet_carbon_credits::{ Module, Call, Storage, Event<T> },
		EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Event<T> },
		Timestamp: pallet_timestamp::{ Module, Call, Storage, Inherent},
        Assets: pallet_assets::{ Module, Call, Storage, Event<T> },
        EvercityFilesign: pallet_evercity_filesign::{ Module, Call, Storage, Event<T> },
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
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
    type OnAddBond = ();
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
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl pallet_carbon_credits::Config for TestRuntime {
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
    pub const ExistentialDeposit: u128 = 0;
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

pub type Balance = u128;

parameter_types! {
    pub const AssetDeposit: Balance = 1; 
    pub const ApprovalDeposit: Balance = 1;
    pub const StringLimit: u32 = 50;
    /// https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
    pub const MetadataDepositBase: Balance = 1;
    pub const MetadataDepositPerByte: Balance = 1;
}

pub type CCAmount = u64;

impl pallet_assets::Config for TestRuntime {
    type Event = Event;
    type ABalance = CCAmount;
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
                        roles: *role,
                        identity: 0,
                        create_time: 0,
                    },
                )
            })
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}

// Build genesis storage for event testing
pub fn new_test_ext_with_event() -> frame_support::sp_io::TestExternalities {
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
                        roles: *role,
                        identity: 0,
                        create_time: 0,
                    },
                )
            })
            .collect(),
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

// Get events list
#[allow(dead_code)]
fn events() -> Vec<Event> {
    let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();
    System::reset_events();
    evt
}



type BondInnerStruct = BondInnerStructOf<TestRuntime>;
type BondStruct = BondStructOf<TestRuntime>;

pub fn get_test_bond(carbon_metadata: pallet_evercity_bonds::bond::CarbonUnitsMetadata<AccountId>) -> BondStruct {
    const PERIODS: usize = 12;
    BondStruct {
        inner: BondInnerStruct {
            docs_pack_root_hash_main: Default::default(),
            docs_pack_root_hash_legal: Default::default(),
            docs_pack_root_hash_finance: Default::default(),
            docs_pack_root_hash_tech: Default::default(),

            impact_data_type: Default::default(),
            impact_data_baseline: vec![None; PERIODS],
            impact_data_max_deviation_cap: None,
            impact_data_max_deviation_floor: None,
            interest_rate_penalty_for_missed_report: None,

            interest_rate_base_value: 2000,   // 2.0%
            interest_rate_margin_cap: None,
            interest_rate_margin_floor: None,
            interest_rate_start_period_value: None,
            start_period: None,
            payment_period: 1,
            interest_pay_period: None,
            mincap_deadline: (20 * DEFAULT_DAY_DURATION * 1000) as u64,
            impact_data_send_period: 0,
            bond_duration: 1,         // PERIODS periods for 30 days
            bond_finishing_period: 14 * DEFAULT_DAY_DURATION,   // 14 days after mature date

            bond_units_mincap_amount: 1000,
            bond_units_maxcap_amount: 1800,
            bond_units_base_price: 4_000_000_000_000,
            carbon_metadata: Some(carbon_metadata),
        },

        issuer: 0,
        manager: 0,
        auditor: 0,
        impact_reporter: 0,

        issued_amount: 0,
        booking_start_date: Default::default(),
        active_start_date: Default::default(),
        creation_date: Default::default(),
        state: pallet_evercity_bonds::bond::BondState::FINISHED,

        bond_debit: 0,
        bond_credit: 0,
        coupon_yield: 0,
        nonce: 0,
    }
}