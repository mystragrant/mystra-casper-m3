#![no_std]
#![no_main]
#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::{String, ToString};

use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage, system,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, contracts::NamedKeys, runtime_args, CLValue, ContractHash, Key, RuntimeArgs, URef, U256, U512
};
use constants::{
    ARG_BUY_PURSE, ARG_CREATOR, ARG_DURATION_MINUTES, ARG_OFFERER, ARG_PRICE, ARG_ROYALTIES_PERCENTAGE, ARG_TOKEN_CONTRACT, ARG_TOKEN_ID, CONTRACT_ACCESS_UREF, CONTRACT_KEY, CONTRACT_PACKAGE_NAME, CONTRACT_VERSION_KEY, KEY_INSTALLER, PURSE_AUCTIONS, PURSE_OFFERS, PURSE_REUSABLE
};
use entry_points::get_entry_points;
use events::{
    emit_accept_offer, emit_auction_ended, emit_auction_started, emit_bid, emit_buy_listing,
    emit_cancel_listing, emit_cancel_offer, emit_create_listing, emit_make_offer, emit_royalty_set,
    init_events, AuctionEnded, AuctionStarted, Bid, ListingBought, ListingCancelled, NewListing,
    NewOffer, OfferAccepted, OfferCancelled, RoyaltySet,
};
use structs::{AuctionData, ListingData, OfferData, RoyaltyData};
use utils::{
    get_auction_data, get_auction_dictionary, get_installer, get_installer_uref, get_listing_data,
    get_listing_dictionary, get_listing_key, get_offer_data, get_offer_dictionary, get_offer_key,
    get_purse, get_royalties_dictionary, get_token_owner, get_transfer_marketplace_address,
    minutes_to_milis, process_payment, transfer_approved, transfer_token,
};

mod constants;
mod entry_points;
mod error;
mod events;
mod structs;
mod utils;
use error::Error;


#[no_mangle]
pub extern "C" fn create_listing() -> () {
    // Read args
    let caller = Key::Account(runtime::get_caller());
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let price: U512 = runtime::get_named_arg(ARG_PRICE);
    let duration_in_minutes: u64 = runtime::get_named_arg(ARG_DURATION_MINUTES);
    let current_time: u64 = runtime::get_blocktime().into();
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();

    // Price must be greater than 0
    if price == U512::zero() {
        runtime::revert(Error::PriceSetToZero)
    }

    // Must be current owner of token
    let owner = get_token_owner(token_contract_hash, token_id);
    if owner != caller {
        runtime::revert(Error::PermissionDenied)
    }

    // Must approve token spending
    if transfer_approved(token_contract_hash, token_id, caller) == false {
        runtime::revert(Error::NeedsTransferApproval);
    }

    // Set expiration time if its greater than 0
    let expiration_time: Option<u64> = if duration_in_minutes > 0 {
        Some(current_time + minutes_to_milis(duration_in_minutes))
    } else {
        None
    };

    // Add new listing data
    let listing_data = ListingData {
        price: price,
        seller: owner,
        expiration_time: expiration_time,
    };
    let key = get_listing_key(token_contract_hash, token_id);
    storage::dictionary_put(get_listing_dictionary(), &key, listing_data);

    // Emit event
    emit_create_listing(NewListing {
        seller: caller,
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        price: price,
        timestamp: current_time,
        expiration_date: current_time + minutes_to_milis(duration_in_minutes),
    });
}

#[no_mangle]
pub extern "C" fn cancel_listing() -> () {
    // Read args
    let caller = Key::Account(runtime::get_caller());
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();

    // Must be token owner to cancel listing
    let owner: Key = get_token_owner(token_contract_hash, token_id);
    if owner != caller {
        runtime::revert(Error::PermissionDenied)
    }

    // Emit event
    emit_cancel_listing(ListingCancelled {
        seller: caller,
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        timestamp: runtime::get_blocktime().into(),
    });

    // Clear listing
    let key = get_listing_key(token_contract_hash, token_id);
    storage::dictionary_put(get_listing_dictionary(), &key, None::<ListingData>)
}

#[no_mangle]
pub extern "C" fn buy_listing() -> () {
    // Read args
    let buyer = Key::Account(runtime::get_caller());
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let buyer_purse: URef = runtime::get_named_arg(ARG_BUY_PURSE);
    let purse_balance: U512 = system::get_purse_balance(buyer_purse).unwrap();

    // Read listing data
    let key = get_listing_key(token_contract_hash, token_id);
    let listing_data: ListingData = get_listing_data(&key);

    // Checking purse balance and revert if lower than price
    if purse_balance < listing_data.price {
        runtime::revert(Error::BalanceInsufficient);
    }

    // Check if expiration time exists and revert if time passed
    match listing_data.expiration_time {
        Some(val) => {
            let current_time: u64 = runtime::get_blocktime().into();

            if current_time > val {
                runtime::revert(Error::ListingExpired)
            }
        }
        None => {}
    }

    // Transfer token and money between users
    process_payment(
        listing_data.price,
        buyer_purse,
        token_contract_string,
        listing_data.seller,
    );
    transfer_token(token_contract_hash, listing_data.seller, buyer, token_id);

    // Emit event
    emit_buy_listing(ListingBought {
        seller: listing_data.seller,
        buyer: buyer,
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        timestamp: runtime::get_blocktime().into(),
        price: listing_data.price,
    });

    // Clear listing data
    storage::dictionary_put(get_listing_dictionary(), &key, None::<ListingData>);
}

#[no_mangle]
pub extern "C" fn make_offer() -> () {
    // Read args
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let offerer_purse: URef = runtime::get_named_arg(ARG_BUY_PURSE);
    let duration_minutes: u64 = runtime::get_named_arg(ARG_DURATION_MINUTES);

    // Get purses data
    let purse_balance: U512 = system::get_purse_balance(offerer_purse).unwrap_or_revert();
    let offers_purse: URef = get_purse(PURSE_OFFERS);

    // If some offer already exists send money back to offerer
    let key = get_offer_key(token_contract_hash, token_id, runtime::get_caller());
    match storage::dictionary_get::<OfferData>(get_offer_dictionary(), &key) {
        Ok(d) => match d {
            Some(offer_data) => {
                system::transfer_from_purse_to_account(
                    offers_purse,
                    runtime::get_caller(),
                    offer_data.price,
                    None,
                )
                .unwrap_or_revert();
            }
            None => {}
        },
        Err(_error) => {}
    }

    // Transfer money from offerer to contract offer purse
    system::transfer_from_purse_to_purse(offerer_purse, offers_purse, purse_balance, None)
        .unwrap_or_revert();

    // Create offer data
    let offer = OfferData {
        price: purse_balance,
        expiration_time: u64::from(runtime::get_blocktime())
            + minutes_to_milis(duration_minutes),
    };

    // Emit event
    emit_make_offer(NewOffer {
        buyer: Key::Account(runtime::get_caller()),
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        price: purse_balance,
        timestamp: runtime::get_blocktime().into(),
        expiration_date: offer.expiration_time,
    });

    // Save offer data
    storage::dictionary_put(get_offer_dictionary(), &key, offer);
}

#[no_mangle]
pub extern "C" fn accept_offer() -> () {
    //Read args
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let offerer_account_string: String = runtime::get_named_arg(ARG_OFFERER);
    let offerer_account_hash: AccountHash =
        AccountHash::from_formatted_str(&offerer_account_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);

    // Load offer data
    let key: String = get_offer_key(token_contract_hash, token_id, offerer_account_hash);
    let offer_data = get_offer_data(&key);

    // Revert if offer time passed
    if u64::from(runtime::get_blocktime()) > offer_data.expiration_time {
        runtime::revert(Error::OfferExpired)
    }

    // Transfer money from purse to caller and transfer token
    process_payment(
        offer_data.price,
        get_purse(PURSE_OFFERS),
        token_contract_string,
        Key::Account(runtime::get_caller()),
    );
    transfer_token(
        token_contract_hash,
        Key::Account(runtime::get_caller()),
        Key::Account(offerer_account_hash),
        token_id,
    );

    // Clear offer data
    storage::dictionary_put(get_offer_dictionary(), &key, None::<OfferData>);

    // Emit event
    emit_accept_offer(OfferAccepted {
        buyer: Key::Account(offerer_account_hash),
        seller: Key::Account(runtime::get_caller()),
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        timestamp: runtime::get_blocktime().into(),
        price: offer_data.price,
    });
}

#[no_mangle]
pub extern "C" fn cancel_offer() -> () {
    // Read args
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);

    // Read offer data
    let key = get_offer_key(token_contract_hash, token_id, runtime::get_caller());
    let current_offer = get_offer_data(&key);

    // Transfer money from offer purse back to offerer
    system::transfer_from_purse_to_account(
        get_purse(PURSE_OFFERS),
        runtime::get_caller(),
        current_offer.price,
        None,
    )
    .unwrap_or_revert();

    // Emit event
    emit_cancel_offer(OfferCancelled {
        buyer: Key::Account(runtime::get_caller()),
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        timestamp: runtime::get_blocktime().into(),
    });

    // Clear offer data
    storage::dictionary_put(get_offer_dictionary(), &key, None::<OfferData>);
}

#[no_mangle]
pub extern "C" fn start_auction() -> () {
    // Get runtime args
    let caller = Key::Account(runtime::get_caller());
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let starting_price: U512 = runtime::get_named_arg(ARG_PRICE);
    let duration_in_minutes: u64 = runtime::get_named_arg(ARG_DURATION_MINUTES);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();

    // Token must not be listed
    let key = get_listing_key(token_contract_hash, token_id);
    match storage::dictionary_get::<ListingData>(get_listing_dictionary(), &key) {
        Ok(d) => match d {
            Some(_offer_data) => {
               runtime::revert(Error::TokenAlreadyOnListing)
            }
            None => {}
        },
        Err(_error) => {}
    }

    // Get current time
    let current_time: u64 = runtime::get_blocktime().into();

    // Create auction data
    let auction_data = AuctionData {
        current_bid: starting_price,
        starting_price: starting_price,
        seller: runtime::get_caller(),
        current_winner: runtime::get_caller(),
        end_time: current_time + minutes_to_milis(duration_in_minutes),
    };

    // Emit event
    emit_auction_started(AuctionStarted {
        seller: caller,
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        starting_price,
        timestamp: current_time,
        end_date: auction_data.end_time,
    });

    transfer_token(
        token_contract_hash,
        caller,
        get_transfer_marketplace_address(),
        token_id,
    );

    // Save auction data
    storage::dictionary_put(get_auction_dictionary(), &key, auction_data)
}

#[no_mangle]
pub extern "C" fn place_bid() -> () {
    // Get runtime args
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let buyer_purse: URef = runtime::get_named_arg(ARG_BUY_PURSE);
    let purse_balance: U512 = system::get_purse_balance(buyer_purse).unwrap();

    // Read auction data
    let key = get_listing_key(token_contract_hash, token_id);
    let mut auction_data: AuctionData = get_auction_data(&key);

    // Bid must be higher than current bid
    if purse_balance <= auction_data.current_bid {
        revert(Error::BidTooLow)
    }

    // Read current time
    let current_time: u64 = runtime::get_blocktime().into();

    // Revert if auction time passed
    if current_time > auction_data.end_time {
        runtime::revert(Error::AuctionEnded);
    }

    // If time until end is smaller than 10 minutes set end time 10 minutes from now
    if current_time - auction_data.end_time < minutes_to_milis(10) {
        auction_data.end_time = current_time + minutes_to_milis(10);
    }

    // Read auction purse
    let auctions_purse: URef = get_purse(PURSE_AUCTIONS);

    // Send transfer to previous bidder, if current_bid == starting_price there are no bidders yet
    if auction_data.current_bid != auction_data.starting_price {
        system::transfer_from_purse_to_account(
            auctions_purse,
            auction_data.current_winner,
            auction_data.current_bid,
            None,
        )
        .unwrap_or_revert();
    }

    // Transfer money from user to auctions purse
    system::transfer_from_purse_to_purse(buyer_purse, auctions_purse, purse_balance, None)
        .unwrap_or_revert();

    // Update auction data
    auction_data.current_bid = purse_balance;
    auction_data.current_winner = runtime::get_caller();

    // Emit event
    emit_bid(Bid {
        seller: Key::Account(auction_data.seller),
        bidder: Key::Account(runtime::get_caller()),
        contract_hash: token_contract_hash,
        bid_price: purse_balance,
        token_id: token_id.to_string(),
        timestamp: current_time,
        new_end_timestamp: auction_data.end_time,
    });

    // Save updated auction data
    storage::dictionary_put(get_auction_dictionary(), &key, auction_data)
}

#[no_mangle]
pub extern "C" fn end_auction() -> () {
    // Read runtime args
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);

    // Get auction data
    let key = get_listing_key(token_contract_hash, token_id);
    let auction_data: AuctionData = get_auction_data(&key);

    // Read current time
    let current_time: u64 = runtime::get_blocktime().into();

    // Revert if auction is not finished but allow to cancel (check this only if someone bidded at least once)
    if current_time < auction_data.end_time && auction_data.current_bid != auction_data.starting_price {
        runtime::revert(Error::AuctionNotFinished);
    }

    // If someone already bidded, transfer assets, else transfer token back to user
    if auction_data.current_bid != auction_data.starting_price {
        process_payment(
            auction_data.current_bid,
            get_purse(PURSE_AUCTIONS),
            token_contract_string,
            Key::Account(auction_data.current_winner),
        );
        transfer_token(
            token_contract_hash,
            get_transfer_marketplace_address(),
            Key::Account(auction_data.current_winner),
            token_id,
        );
    } else {
        
        transfer_token(
            token_contract_hash,
            get_transfer_marketplace_address(),
            Key::Account(auction_data.seller),
            token_id,
        );

    }

    emit_auction_ended(AuctionEnded {
        seller: Key::Account(auction_data.seller),
        winner: Key::Account(auction_data.current_winner),
        contract_hash: token_contract_hash,
        token_id: token_id.to_string(),
        ending_price: auction_data.current_bid,
        timestamp: runtime::get_blocktime().into(),
    });

    storage::dictionary_put(get_listing_dictionary(), &key, None::<AuctionData>)
}

#[no_mangle]
pub extern "C" fn set_royalties() -> () {
    // Get runtime args
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let percentage: u64 = runtime::get_named_arg(ARG_ROYALTIES_PERCENTAGE);
    let creator_string: String = runtime::get_named_arg(ARG_CREATOR);
    let creator_account_hash: AccountHash =
        AccountHash::from_formatted_str(&creator_string).unwrap();
    let creator_key: Key = creator_account_hash.into();

    // Only installer can set royalties
    let installer: Key = get_installer();
    if Key::Account(runtime::get_caller()) != installer {
        runtime::revert(Error::CallerNotInstaller);
    }

    // Create royalty data
    let royalty_data = RoyaltyData {
        percentage: percentage,
        creator: creator_key,
    };

    // Emit event
    emit_royalty_set(RoyaltySet {
        recipient: creator_key,
        contract_hash: ContractHash::from_formatted_str(&token_contract_string).unwrap(),
        percentage: percentage,
    });

    // Save royalty data
    storage::dictionary_put(
        get_royalties_dictionary(),
        &token_contract_string,
        royalty_data,
    )
}

#[no_mangle]
pub extern "C" fn init() -> () {
    // Set up CES events
    init_events();

    // Init reusable purse
    get_purse(PURSE_REUSABLE);
}

#[no_mangle]
pub extern "C" fn get_reusable_purse() -> () {
    runtime::ret(CLValue::from_t(get_purse(PURSE_REUSABLE)).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn call() {
    let mut named_keys = NamedKeys::new();
    named_keys.insert(KEY_INSTALLER.to_string(), get_installer_uref().into());

    let (stored_contract_hash, contract_version) = storage::new_contract(
        get_entry_points(),
        Some(named_keys),
        Some(CONTRACT_PACKAGE_NAME.to_string()),
        Some(CONTRACT_ACCESS_UREF.to_string()),
    );

    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());

    // Call init 
    runtime::call_contract::<()>(stored_contract_hash, "init", runtime_args! {});
}
