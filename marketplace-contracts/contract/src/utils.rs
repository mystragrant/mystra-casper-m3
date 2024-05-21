use alloc::{
    format, str,
    string::{String, ToString},
    vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage,
        system,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, runtime_args, system::CallStackElement, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};

use crate::constants::{ARG_TOKEN_STANDARD, KEY_INSTALLER};
use crate::{error::Error, AuctionData, ListingData, OfferData, RoyaltyData};

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = runtime::get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert_with(4);

    let package_hash = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => *contract_package_hash,
        _ => runtime::revert(5),
    };

    package_hash
}

pub fn contract_hash() -> ContractHash {
    let call_stacks = runtime::get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert_with(4);

    let hash = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash,
        } => *contract_hash,
        _ => runtime::revert(5),
    };

    hash
}

pub fn transfer_approved(token_contract_hash: ContractHash, token_id: U256, owner: Key) -> bool {
    let token_standard: u8 = runtime::get_named_arg("token_standard");

    if (token_standard == 0u8) {
        let approved = runtime::call_contract::<Option<Key>>(
            token_contract_hash,
            "get_approved",
            runtime_args! {
            "owner" => owner,
            "token_id" => token_id
            },
        )
        .unwrap_or_revert_with(Error::NeedsTransferApproval);
        let approved_hash = approved.into_hash().unwrap_or_revert();

        contract_package_hash().value() == approved_hash
    } else if token_standard == 1u8 {
        let approved = runtime::call_contract::<Option<Key>>(
            token_contract_hash,
            "get_approved",
            runtime_args! {
            "owner" => owner,
            "token_id" => token_id.as_u64()
            },
        )
        .unwrap_or_revert();

        let approved_hash = approved.into_hash().unwrap_or_revert_with(4);

        contract_hash().value() == approved_hash
    } else {
        false
    }
}

pub fn get_token_owner(token_contract_hash: ContractHash, token_id: U256) -> Key {
    let token_standard: u8 = runtime::get_named_arg("token_standard");

    if token_standard == 0u8 {
        runtime::call_contract::<Option<Key>>(
            token_contract_hash,
            "owner_of",
            runtime_args! {
              "token_id" => token_id
            },
        )
        .unwrap_or_revert()
    } else {
        runtime::call_contract::<Key>(
            token_contract_hash,
            "owner_of",
            runtime_args! {
              "token_id" => token_id.as_u64()
            },
        )
    }
}

pub fn transfer_token(contract_hash: ContractHash, owner: Key, to: Key, token_id: U256) -> () {
    let token_standard: u8 = runtime::get_named_arg("token_standard");

    if token_standard == 0u8 {
        runtime::call_contract::<()>(
            contract_hash,
            "transfer_from",
            runtime_args! {
              "sender" => owner,
              "recipient" => to,
              "token_ids" => vec![token_id],
            },
        )
    } else {
        runtime::call_contract::<()>(
            contract_hash,
            "transfer",
            runtime_args! {
              "source_key" => owner,
              "target_key" => to,
              "token_id" =>
              token_id.as_u64(),
            },
        )
    }
}

pub fn get_dictionary_uref(key: &str) -> URef {
    match runtime::get_key(key) {
        Some(uref_key) => uref_key.into_uref().unwrap_or_revert(),
        None => storage::new_dictionary(key).unwrap_or_revert(),
    }
}

pub fn get_installer_uref() -> URef {
    match runtime::get_key(KEY_INSTALLER) {
        Some(uref_key) => uref_key.into_uref().unwrap_or_revert(),
        None => storage::new_uref(Key::Account(runtime::get_caller())),
    }
}

pub fn get_installer() -> Key {
    storage::read(get_installer_uref())
        .unwrap_or_revert()
        .unwrap_or_revert()
}

pub fn get_listing_key(token_contract_hash: ContractHash, token_id: U256) -> String {
    let key_string = format!(
        "{}_{}",
        token_contract_hash.to_string(),
        token_id.to_string()
    );
    let hashed = runtime::blake2b(key_string);
    hex::encode(hashed)
}

pub fn get_listing_dictionary() -> URef {
    get_dictionary_uref("listings")
}

pub fn get_royalties_dictionary() -> URef {
    get_dictionary_uref("royalties")
}

pub fn get_offer_key(
    token_contract_hash: ContractHash,
    token_id: U256,
    bidder: AccountHash,
) -> String {
    let key_string = format!(
        "{}_{bidder}_{}",
        token_contract_hash.to_string(),
        token_id.to_string()
    );
    let hashed = runtime::blake2b(key_string);
    hex::encode(hashed)
}

pub fn get_offer_dictionary() -> URef {
    get_dictionary_uref("offers")
}

pub fn get_auction_dictionary() -> URef {
    get_dictionary_uref("auctions")
}

pub fn get_purse(purse_name: &str) -> URef {
    let purse = if !runtime::has_key(&purse_name) {
        let purse = system::create_purse();
        runtime::put_key(&purse_name, purse.into());
        purse
    } else {
        let destination_purse_key =
            runtime::get_key(&purse_name).unwrap_or_revert_with(Error::OfferPurseRetrieval);
        match destination_purse_key.as_uref() {
            Some(uref) => *uref,
            None => runtime::revert(Error::OfferPurseRetrieval),
        }
    };
    return purse;
}

pub fn get_listing_data(key: &str) -> ListingData {
    let listing: ListingData = match storage::dictionary_get(get_listing_dictionary(), &key) {
        Ok(item) => match item {
            None => runtime::revert(Error::ListingDoesntExist),
            Some(value) => value,
        },
        Err(_error) => runtime::revert(Error::ListingCancelledOrFinished),
    };

    listing
}

pub fn get_offer_data(key: &str) -> OfferData {
    let offer: OfferData = match storage::dictionary_get(get_offer_dictionary(), &key) {
        Ok(item) => match item {
            None => runtime::revert(Error::OfferDoesntExist),
            Some(value) => value,
        },
        Err(_error) => runtime::revert(Error::OfferCancelledOrFinished),
    };

    offer
}

pub fn get_auction_data(key: &str) -> AuctionData {
    let auction: AuctionData = match storage::dictionary_get(get_auction_dictionary(), &key) {
        Ok(item) => match item {
            None => runtime::revert(Error::AuctionDoesntExist),
            Some(value) => value,
        },
        Err(_error) => runtime::revert(Error::AuctionCancelledOrFinished),
    };

    auction
}

pub fn get_royalty_data(key: &str) -> RoyaltyData {
    let royalty: RoyaltyData = match storage::dictionary_get(get_royalties_dictionary(), &key) {
        Ok(item) => match item {
            None => RoyaltyData {
                percentage: 0u64,
                creator: Key::Account(runtime::get_caller()),
            },
            Some(value) => value,
        },
        Err(_error) => runtime::revert(Error::AuctionCancelledOrFinished),
    };

    royalty
}

pub fn process_payment(
    price: U512,
    from_purse: URef,
    token_contract_hash_string: String,
    to: Key,
) -> () {
    let royalty: RoyaltyData = get_royalty_data(&token_contract_hash_string);

    let creator_part = price * U512::from(royalty.percentage) / U512::from(100);
    let marketplace_part = price * U512::from(10) / U512::from(100);

    system::transfer_from_purse_to_account(
        from_purse,
        to.into_account().unwrap_or_revert(),
        price - creator_part - marketplace_part,
        None,
    )
    .unwrap_or_revert();

    if (royalty.percentage > 0) {
        system::transfer_from_purse_to_account(
            from_purse,
            royalty.creator.into_account().unwrap_or_revert(),
            creator_part,
            None,
        )
        .unwrap_or_revert();
    }

    system::transfer_from_purse_to_account(
        from_purse,
        get_installer().into_account().unwrap_or_revert(),
        marketplace_part,
        None,
    )
    .unwrap_or_revert();
}

pub fn minutes_to_milis(minutes: u64) -> u64 {
    minutes * 60000
}

pub fn get_transfer_marketplace_address() -> Key {
    let token_standard: u8 = runtime::get_named_arg(ARG_TOKEN_STANDARD);

    if token_standard == 0u8 {
        contract_package_hash().into()
    } else if token_standard == 1u8 {
        contract_hash().into()
    } else {
        contract_hash().into()
    }
}
