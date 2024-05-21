use std::collections::BTreeMap;

use casper_engine_test_support::{ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{account::AccountHash, runtime_args, RuntimeArgs, ContractHash, Key, U256};

use super::{blockchain_helpers::get_contract_hash_from_account, constants::CEP47_WASM};


pub fn mint_cep_47(
    builder: &mut InMemoryWasmTestBuilder,
    cep47_hash: ContractHash,
    recipient: Key,
    ids: Vec<U256>,
) {
    let mut meta = BTreeMap::new();
    meta.insert("rarity".to_string(), "Epic".to_string());

    let request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep47_hash,
        "mint",
        runtime_args! {
            "recipient" => recipient,
            "token_metas" => vec![meta],
            "token_ids" => ids,
        },
    )
    .build();
    builder.exec(request).expect_success().commit();
}

pub fn deploy_cep_47(builder: &mut InMemoryWasmTestBuilder) -> ContractHash {
        let mut meta = BTreeMap::new();
        meta.insert("rarity".to_string(), "Epic".to_string());

        let request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CEP47_WASM,
            runtime_args! {
                "name" => "my nft",
                "symbol" => "my symbol",
                "meta" => meta,
                "contract_name" => "MYNFT"

            },
        )
        .build();
        builder.exec(request).expect_success().commit();
        get_contract_hash_from_account(builder, "MYNFT_contract_hash")
    }

pub fn approve_cep_47(
        builder: &mut InMemoryWasmTestBuilder,
        caller: AccountHash,
        cep47_hash: ContractHash,
        recipient: Key,
        ids: Vec<U256>,
        
    ) {
        let request = ExecuteRequestBuilder::contract_call_by_hash(
            caller,
            cep47_hash,
            "approve",
            runtime_args! {
                "spender" => recipient,
                "token_ids" => ids,
            },
        )
        .build();
        builder.exec(request).expect_success().commit();
    }