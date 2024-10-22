#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ApiError, ContractHash, Key, RuntimeArgs, URef, U256, U512,
};

#[no_mangle]
pub extern "C" fn call() {
    let token_id: U256 = runtime::get_named_arg("token_id");
    let contract_hash: String = runtime::get_named_arg("bid_contract_hash");
    let marketplace_hash: String = runtime::get_named_arg("marketplace_hash");
    let amount: U512 = runtime::get_named_arg("amount");

    let contract_hash_parsed: ContractHash =
        ContractHash::from_formatted_str(&marketplace_hash).unwrap();

    let deposit_purse: URef = system::create_purse();

    system::transfer_from_purse_to_purse(account::get_main_purse(), deposit_purse, amount, None)
        .unwrap_or_revert();

    runtime::call_contract(
        contract_hash_parsed,
        "place_bid",
        runtime_args! {
         "contract_hash" => contract_hash,
         "token_id" => token_id,
         "buy_purse" => deposit_purse
        },
    )
}
