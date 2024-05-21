use std::usize;

use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNTS,
    DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE,
};
use casper_execution_engine::{
    core::engine_state::GenesisAccount, storage::global_state::in_memory::InMemoryGlobalState,
};
use casper_types::{
    account::AccountHash, runtime_args, ContractHash, Key, Motes, PublicKey, RuntimeArgs,
    SecretKey, U512,
};
use sha2::{Digest, Sha256};

pub fn get_account_balance(
    builder: &mut InMemoryWasmTestBuilder,
    account_hash: AccountHash,
) -> U512 {
    let account_result = builder
        .query(None, Key::Account(account_hash), &[])
        .expect("Should have account data");
    let account = account_result.as_account().expect("Should be account");
    let main_purse = account.main_purse();

    builder.get_purse_balance(main_purse)
}

/// Retrieves the contract hash from the default account's storage by a given key
pub fn get_contract_hash_from_account(
    builder: &mut InMemoryWasmTestBuilder,
    key: &str,
) -> ContractHash {
    builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get(key)
        .expect("must have contract hash key")
        .into_hash()
        .map(ContractHash::new)
        .expect("must get contract hash")
}

pub fn get_user(builder: &mut InMemoryWasmTestBuilder, id: u8) -> AccountHash {
    if (id > 1) {
        let mut pem_string = "";

        if (id == 2) {
            pem_string = r#"-----BEGIN PRIVATE KEY-----
            MC4CAQAwBQYDK2VwBCIEIEMdODqbgKz7VZnbb2C+ba9Cm+4Qxcf18UoohV+xHwqI
            -----END PRIVATE KEY-----
            "#;
        } else if(id == 3) {
            pem_string = r#"-----BEGIN PRIVATE KEY-----
            MC4CAQAwBQYDK2VwBCIEIKxk7W5EmiZXX9Ce1sX7e1QJTZzwnQzEE5Eoiv3qHjNP
            -----END PRIVATE KEY-----            
            "#;
        }

        let acc = create_funded_dummy_account(builder, &pem_string);

        acc
    } else {
        let index: usize = id.into();
        DEFAULT_ACCOUNTS.get(index).unwrap().account_hash()
    }
}

fn create_dummy_key_pair(pem: &str) -> (SecretKey, PublicKey) {
    let secret_key = SecretKey::from_pem(pem).expect("failed to create secret key");
    let public_key = PublicKey::from(&secret_key);
    (secret_key, public_key)
}

pub(crate) fn create_funded_dummy_account(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    account_string: &str,
) -> AccountHash {
    let (_, account_public_key) = create_dummy_key_pair(account_string);
    let account = account_public_key.to_account_hash();

    let transfer = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            "amount" => 100_000_000_000_000u64,
            "target" => account,
            "id" => Option::<u64>::None,
        },
    )
    .build();
    builder.exec(transfer).expect_success().commit();
    account
}
