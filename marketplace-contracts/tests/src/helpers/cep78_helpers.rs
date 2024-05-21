use std::collections::BTreeMap;

use casper_engine_test_support::{ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNTS, DEFAULT_ACCOUNT_ADDR};
use casper_types::{account::AccountHash, runtime_args, ContractHash, Key, RuntimeArgs, U256};

use crate::helpers::{blockchain_helpers::get_contract_hash_from_account, constants::CEP78_WASM};


pub fn mint_cep_78(
    builder: &mut InMemoryWasmTestBuilder,
    cep47_hash: ContractHash,
    recipient: Key,
    ids: Vec<U256>,
) {

    let request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep47_hash,
        "mint",
        runtime_args! {
            "token_owner" => recipient,
            "token_meta_data" => "{\"name\":\"essa\"}"
        },
    )
    .build();
    builder.exec(request).expect_success().commit();
}

pub fn deploy_cep_78(builder: &mut InMemoryWasmTestBuilder) -> ContractHash {

        let key : Key = (*DEFAULT_ACCOUNT_ADDR).into();

        let request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CEP78_WASM,
            runtime_args! {
                "collection_name" => "hi",
                "collection_symbol" => "my symbol",
                "total_token_supply" => 100u64,
                "ownership_mode" => 2u8,
                "json_schema" => "",
                "nft_kind" => 0u8,
                "nft_metadata_kind" => 2u8,
                "identifier_mode" => 0u8,
                "metadata_mutability" => 0u8
            },
        )
        .build();
        builder.exec(request).expect_success().commit();
        let x = builder
        .get_expected_account(DEFAULT_ACCOUNTS.get(0).unwrap().account_hash())
        .named_keys().clone();

    

        println!("{:?}", x);
        get_contract_hash_from_account(builder, "cep78_contract_hash_hi")
    }



pub fn approve_cep_78(
        builder: &mut InMemoryWasmTestBuilder,
        caller: AccountHash,
        cep47_hash: ContractHash,
        recipient: Key,
        id: u64,
    ) {
        
        let request = ExecuteRequestBuilder::contract_call_by_hash(
            caller,
            cep47_hash,
            "approve",
            runtime_args! {
                "operator" => recipient,
                "token_id" => id,
            },
        )
        .build();
        builder.exec(request).expect_success().commit();
    }