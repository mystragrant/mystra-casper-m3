use alloc::{string::String, vec};
use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
    URef, U256, U512,
};

pub fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "create_listing",
        vec![
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("price", U512::cl_type()),
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("duration_minutes", u64::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "cancel_listing",
        vec![
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("contract_hash", String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_reusable_purse",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "buy_listing",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("buy_purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "init",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "make_offer",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("buy_purse", URef::cl_type()),
            Parameter::new("duration_minutes", u64::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "accept_offer",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("offerer", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "cancel_offer",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "start_auction",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("price", U512::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_royalties",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("royalties_percentage", U256::cl_type()),
            Parameter::new("creator", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "place_bid",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
            Parameter::new("buy_purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "end_auction",
        vec![
            Parameter::new("contract_hash", String::cl_type()),
            Parameter::new("token_id", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
