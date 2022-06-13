# 1. Evercity Carbon Credits Pallet

This repository contains source code of blockchain node, which is a main part of Evercity's Carbon Credits project.

# 2. Introduction

Evercity Carbon Credits Pallet allows to issue and monitor carbon credits - green financial instruments. The main goal of carbon credits is to increase investment in activities that reduce greenhouse gas emissions or remove carbon from the atmosphere. Each carbon credit represents one ton of CO2 (reduced or removed), which was verified by internationally accepted standards using robust methodologies. Main demand for carbon credits is now driven by investors and corporations who want to achieve net zero goals, as well as by blockchain projects that want to offset their carbon emissions. 

The value of the global carbon market increased by 20% in 2020 to $272 billion and is expected to grow further. However, there are currently several challenges that prevent market development: low transparency and fraud risk; low liquidity; double accounting, etc. Representatives of UNFCCC state that blockchain technology could help solve many of these problems, namely:
- strengthen monitoring, reporting and verification of the impacts of climate action
- improve transparency, traceability and cost-effectiveness of climate action
- build trust among climate actors
- make incentive mechanisms for climate action accessible to the poorest
- support mobilization of green finance

(https://unfccc.int/news/un-supports-blockchain-technology-for-climate-action)

At the same time, a challenge preventing rapid blockchain adoption in climate finance still lies in the high carbon footprint of main PoW blockchains including Ethereum. Parity Substrate blockchain has many advantages having a low carbon footprint, as well as enabling interoperability and scalability. 
 

# 3. Overview

Evercity Carbon Credits pallet allows issuing carbon credits according to any standard (or even creating own standard using customizable frameworks) as a result of interaction between various stakeholders: project owners, standard representatives, auditors and registries. We are replicating the globally accepted life cycle of carbon credits on blockchain making it more transparent, efficient and accessible. Key target audience of our product are project owners who issue carbon credits, companies who want to offset their emissions as well as blockchain projects who want to offset the carbon footprint of their transactions. 

# 4. Evercity carbon credits project main entities

Carbon Credits pallet has several main entities: 

### 4.1 Project 

Entity for signing carbon credits project documentation and creating annual reports 

### 4.2 Carbon Standard

Entity which determines the order of signature among three roles: CC_AUDITOR, CC_STANDARD, CC_REGISTRY

### 4.3 Annual Report 

Entity for confirming annual volume of carbon credit issuance

### 4.4 Carbon Credit Passport 

Entity for registering carbon credits as assets  

### 4.5 Carbon Offset Certificate 

Entity for granting certificates for carbon emissions offsetting using carbon credits

 ### 4.6 CarbonCreditsPackageLot 

 Struct representing pack of carbon credits for sale.
 Can include target bearer (to sell only to them). Lot has deadline, after whitch selling is impossible.



# 5. Evercity Roles and Carbon Creditis project scenario

### 5.1 Roles

The system of roles in Evercity is presented in Evercity accounts pallet https://github.com/EvercityEcosystem/evercity-accounts

- CC_PROJECT_OWNER: the role which can create carbon projects, annual reports and issue carbon credits
- CC_STANDARD; CC_AUDITOR; CC_REGISTRY: the roles which sign project documentation and annual reports (the order of signatures is determined by Carbon Standard entity)

### 5.2 Basic scenario

Here is the basic scenario on of carbon credits releasing and offetting:

- Project owner creates document and stores its hash into filesign pallet (extrinsic - pallet_evercity_filesign - create_new_file()). 
Any account can access this extrinsic.

- Project owner creates a Project in Carbon Credits pallet, choosing a Carbon Standard(extrinsic - create_project())

- Issuer can create bond-based project - also choosing a Carbon Standard (extrinsic - create_bond_project())

- Project owner can change project file id in a Project in Carbon Credits pallet to a new one. Available before signing starts(extrinsic - change_project_file_id()).
Only project owner account with CC_PROJECT_OWNER role mask can run this step. Also it must be the project owner and file owner in blockchain storage.

- Project owner adds signers and their roles to project(extrinsic - assign_project_signer()).
Only project owner account with CC_PROJECT_OWNER role mask can run this step. Also it must be the project owner in blockchain storage.

- Then project is signed by different stakeholders, the order depends on Carbon Standard. At the end, the project owner is ready for producing annual report for carbon credits issuance (extrinsic - sign_project())

- Project owner creates document and annual report in project with carbon credits asset_id and asset metadata (extrinsic - create_annual_report()).
Only project owner account with CC_PROJECT_OWNER role mask can run this step. Also it must be the project owner in blockchain storage.

- Project owner adds signers and their roles to annual report (extrinsic - assign_last_annual_report_signer()).
Only project owner account with CC_PROJECT_OWNER role mask can run this step. Also it must be the project owner in blockchain storage.

- Then starts report signing, the sign order depends on carbon credits standard (extrinsic - sign_last_annual_report()).
The role, which can access this step id defined by carbon credits standard. For example, gold standard  sequence is CC_PROJECT_OWNER -> CC_AUDITOR -> CC_STANDARD -> CC_REGISTRY. 
Also signers must be holed in blockchain storage.

- Then report is signed by different stakeholders, the order depends on Carbon Standard. (extrinsic - sign_last_annual_report())

- Then project owner can release carbon credits (extrinsic - release_carbon_credits() - if this project without bond, release_bond_carbon_credits() - if project is based on a bond)

- User can burn carbon credits (extrinsic - burn_carbon_credits()).
Any carbon credits holder can access this function.


Some other functions:

- Project owner can delete last annual report if it is not full signed(extrinsic - delete_last_annual_report())

- Project owner can remove account from project signers if it didnt sign the document (extrinsic - remove_project_signer())

- Project owner can remove account from last annual report signers if it didnt sign the document (extrinsic - remove_last_annual_report_signer())
 
 ### 5.3 Lot system 
 
 - Extrinsic: `create_carbon_credit_lot` - creates a lot with an expiration moment. Lot can be private - then only targer bearer can buy Carbon Credits from the lot. 

 - Extrinsic: `buy_carbon_credit_lot_units` - buys Carbon Credits from the lot. Can buy defined amount of it, or the whole lot.

### 5.4 Tokenization of external Carbon Units

1) User creates `BatchAsset` calling extrinsic `external_create_batch_asset`,
receives `BatchAssetId` from event.
2) User buys and retires/locks carbon units on the external registry and writes `BatchAssetId` to public retirement details. The user receives a serial number of retirement.
3) User updates `BatchAsset` calling extrinsic `external_update_batch_asset`, and list `registry_type` - registry name, `external_project_id`, `vintage_name`, `serial_number` from previous step, `amount` of bought carbon units.
4) User/or manager puts all project information on ipfs, then calls extrinsic `external_add_project_ipfs_link`.
5) Evercity manager verifies all information and calls extrinsic `external_verify_batch_asset`
6) Evercity carbon credits released. The user can burn them, or sell them via the lot system.

# 6. Pallet Carbon Credits documentation

### 6.1 Quickstart

The pallet is designed to work in the existing runtime

To start your environment from scratch use docker:

```bash
docker build ./
docker run -p 30300:30300 -p 9933:9933 -p 9944:9944 {IMAGE_ID}
```
Then check on https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/extrinsics

Types are described in the types.json file

### 6.2 Launching with existing runtime

Add to runtime cargo.toml

```toml
pallet-evercity-carbon-credits = { default-features = false, version = '0.2.0', git = 'https://github.com/EvercityEcosystem/evercity-chain' }
pallet-evercity-filesign = { default-features = false, version = '0.1.4', git = 'https://github.com/EvercityEcosystem/evercity-chain'}
pallet-evercity-assets = { default-features = false, version = '0.1.0', git = 'https://github.com/EvercityEcosystem/evercity-chain' }
pallet-evercity-accounts = { default-features = false, version = '0.1.8', git = 'https://github.com/EvercityEcosystem/evercity-chain' }
pallet-evercity-bonds ={ default-features = false, version = '0.1.3', git = 'https://github.com/EvercityEcosystem/evercity-chain' }
pallet-randomness-collective-flip = { default-features = false, version = '3.0.0' }
#...
[features]
default = ['std']

std = [
    #...
    'pallet-evercity-carbon-credits/std',
    'pallet-evercity-filesign/std',
    'pallet-evercity-assets/std',
    'pallet-evercity-accounts/std',
    'pallet-evercity-bonds/std',
    #...
]
```

Add to runtime constructing

```rust
pub use pallet_evercity_carbon_credits;
impl pallet_evercity_carbon_credits::Config for Runtime {
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
}
// dependency configs
const DEFAULT_DAY_DURATION: u32 = 86400; //seconds in 1 DAY
parameter_types! {
    pub const BurnRequestTtl: u32 = DEFAULT_DAY_DURATION as u32 * 7 * 1000;
    pub const MintRequestTtl: u32 = DEFAULT_DAY_DURATION as u32 * 7 * 1000;
    pub const MaxMintAmount: pallet_evercity_bonds::EverUSDBalance = 60_000_000_000_000_000;
    pub const TimeStep: pallet_evercity_bonds::BondPeriod = DEFAULT_DAY_DURATION;
}

impl pallet_evercity_bonds::Config for Runtime {
	type Event = Event;
    type BurnRequestTtl = BurnRequestTtl;
    type MintRequestTtl = MintRequestTtl;
    type MaxMintAmount = MaxMintAmount;
    type TimeStep = TimeStep;
    type WeightInfo = ();
    type OnAddBond = ();
}

pub use pallet_evercity_accounts;
impl pallet_evercity_accounts::Config for Runtime {
    type Event = Event;
}

parameter_types! {
    pub const AssetDepositBase: Balance = 0;
    pub const AssetDepositPerZombie: Balance = 0;
    pub const ApprovalDeposit: Balance = 0;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}
use pallet_evercity_assets;
impl pallet_evercity_assets::Config for Runtime {
    type Event = Event;
    type ABalance = u64;
    type AssetId = u64;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type AssetDepositBase = AssetDepositBase;
    type AssetDepositPerZombie = AssetDepositPerZombie;
    type StringLimit = StringLimit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type WeightInfo = pallet_evercity_assets::weights::SubstrateWeight<Runtime>;
}

use pallet_evercity_filesign;
impl pallet_evercity_filesign::Config for Runtime {
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
}
...
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        ...
        EvercityCarbonCredits: pallet_evercity_carbon_credits::{ Module, Call, Storage, Event<T>},
        ...
        // Add dependency pallets:
        Evercity: pallet_evercity_bonds::{Module, Call, Storage, Event<T>},
        EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Config<T>, Event<T>},
        EvercityFilesign: pallet_evercity_filesign::{ Module, Call, Storage, Event<T> },
        EvercityAssets: pallet_evercity_assets::{ Module, Storage, Event<T> },
	    ...
    }
);
```

Update chain spec for EvercityAccounts: [here](https://github.com/EvercityEcosystem/evercity-chain/tree/master/pallets/evercity-accounts#45-modify-chain-spec-set-genesisconfig)


### 6.3 Build

```bash
git clone https://github.com/EvercityEcosystem/carbon-credits
cd carbon-credits
make build
```

### 6.4 Run Unit Tests

```bash
make test
```

### 6.5 Launch linter

```bash
make lint
```