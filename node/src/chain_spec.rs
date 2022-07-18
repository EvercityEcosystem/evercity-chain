use evercity_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY, 
	pallet_evercity_accounts::{accounts::{MASTER_ROLE_MASK, CUSTODIAN_ROLE_MASK, CC_PROJECT_OWNER_ROLE_MASK, 
		ISSUER_ROLE_MASK, CC_AUDITOR_ROLE_MASK, INVESTOR_ROLE_MASK, CC_STANDARD_ROLE_MASK, AUDITOR_ROLE_MASK, 
		CC_REGISTRY_ROLE_MASK, MANAGER_ROLE_MASK, IMPACT_REPORTER_ROLE_MASK}, self}, 
	EvercityAccountsConfig,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{traits::{IdentifyAccount, Verify}, app_crypto::Ss58Codec};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        MASTER_ROLE_MASK, 0
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        CUSTODIAN_ROLE_MASK|CC_PROJECT_OWNER_ROLE_MASK, 0
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        ISSUER_ROLE_MASK|CC_AUDITOR_ROLE_MASK, 0
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        INVESTOR_ROLE_MASK|CC_STANDARD_ROLE_MASK, 0
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Eve"),
                        AUDITOR_ROLE_MASK|CC_REGISTRY_ROLE_MASK, 0
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                        MANAGER_ROLE_MASK, 0
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Evercity"),
                        IMPACT_REPORTER_ROLE_MASK, 0
                    ),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			let master_account_id: AccountId =
                Ss58Codec::from_ss58check("5Fc6su9eJgm18K1LT2V5KnqfhaW9z9MmLszG5YvTzcVJ7sVL")
                    .unwrap();

			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				master_account_id.clone(),
				// Pre-funded accounts
                vec![(master_account_id.clone(), MASTER_ROLE_MASK, 0)],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
    evercity_accounts_roles: Vec<(AccountId, pallet_evercity_accounts::accounts::RoleMask, u64)>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: evercity_accounts_roles
			.iter()
			.map(|x| (x.0.clone(), 1 << 60))
			.collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		evercity_accounts: EvercityAccountsConfig {
            // set roles for each pre-set accounts (set role)
            genesis_account_registry: evercity_accounts_roles
        },
	}
}
