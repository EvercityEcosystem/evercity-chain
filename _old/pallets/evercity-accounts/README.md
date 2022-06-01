# 1. Evercity Accounts Pallet

This repositary contains source code of blockchain node, which is a main part of Evercity's Accounts. The pallet has a purely technical purpose and is used for interaction with other pallets.

# 2. Evercity accounts main entities

Accounts pallet has several entities: 

### 2.1 AccountStruct 

Is the main entity for accounts, containing rolemask of account in pallet storage

### 2.2 RoleMask 

Each Evercity account can can accommodate one or more roles:

- MASTER: the administrative role that can assign roles to accounts
- CC_PROJECT_OWNER: the role which can create carbon projects, annual report and issue caebon credits
- CC_AUDITOR: the role to sign project documentation and annual reports according to carbon credits standard
- CC_STANDARD: the role to sign project documentation and annual reports according to carbon credits standard
- CC_INVESTOR: carbon credits investor
- CC_REGISTRY: the role to sign project documentation and annual reports according to carbon credits standard

# 3. Evercity Account pallet can do several things

- Set MASTER role on account
- Set any non-MASTER role
- Add additional non-MASTER role on account
- Withraw any non-MASTER role

# 4. Accounts documentation

### 4.1 Runtime methods

<!-- Methods of pallet-evercity are described in Rust documentation [here](http://51.15.47.43/doc/pallet_evercity/) [TEMP] -->

### 4.2 Build

```bash
git clone https://github.com/EvercityEcosystem/evercity-accounts
cd evercity-accounts
make build
```
### 4.3 Add to runtime cargo.toml

```toml
pallet-evercity-accounts = { default-features = false, version = '0.1.7', git = 'https://github.com/EvercityEcosystem/evercity-accounts' }
#...
[features]
default = ['std']

std = [
    #...
    'pallet-evercity-accounts/std',
    #...
]
```

### 4.4 Add to runtime constructing

```rust
pub use pallet_evercity_accounts;
impl pallet_evercity_accounts::Config for Runtime {
    type Event = Event;
}
...
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        ...
        EvercityAccounts: pallet_evercity_accounts::{ Module, Call, Storage, Config<T>, Event<T>},
        ...
    }
);
```

### 4.5 Check on smart sustainable bond node

```bash
git clone https://github.com/EvercityEcosystem/smart-sustainable-bond.git
cd smart-sustainable-bond
git checkout add_carbon_credits #temporary
make run
```

### 4.6 Run Unit Tests

```bash
make test
```

### 4.7 Launch linter

```bash
make lint
```
