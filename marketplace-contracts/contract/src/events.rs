use alloc::string::String;
use casper_event_standard::{Event, Schemas};
use casper_types::{ContractHash, Key, U512};

#[derive(Event)]
pub struct NewListing {
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
    pub expiration_date: u64,
}

#[derive(Event)]
pub struct ListingBought {
    pub seller: Key,
    pub buyer: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
}

#[derive(Event)]
pub struct ListingCancelled {
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub timestamp: u64,
}

#[derive(Event)]
pub struct NewOffer {
    pub buyer: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
    pub expiration_date: u64
}

#[derive(Event)]
pub struct OfferCancelled {
    pub buyer: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub timestamp: u64,
}


#[derive(Event)]
pub struct OfferAccepted {
    pub buyer: Key,
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub price: U512,
    pub timestamp: u64,
}

#[derive(Event)]
pub struct AuctionStarted {
    pub seller: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub starting_price: U512,
    pub timestamp: u64,
    pub end_date: u64
}

#[derive(Event)]
pub struct AuctionEnded {
    pub seller: Key,
    pub winner: Key,
    pub contract_hash: ContractHash,
    pub token_id: String,
    pub ending_price: U512,
    pub timestamp: u64,
}

#[derive(Event)]
pub struct Bid {
    pub seller: Key,
    pub bidder: Key,
    pub contract_hash: ContractHash,
    pub bid_price: U512,
    pub token_id: String,
    pub timestamp: u64,
    pub new_end_timestamp: u64
}

#[derive(Event)]
pub struct RoyaltySet {
    pub recipient: Key,
    pub contract_hash: ContractHash,
    pub percentage: u64,
}

pub fn init_events() {
    let schemas = Schemas::new()
        .with::<NewListing>()
        .with::<ListingBought>()
        .with::<ListingCancelled>()
        .with::<Bid>()
        .with::<NewOffer>()
        .with::<OfferAccepted>()
        .with::<OfferCancelled>()
        .with::<AuctionEnded>()
        .with::<AuctionStarted>()
        .with::<RoyaltySet>()
        .with::<Bid>()
        .with::<NewOffer>();
    casper_event_standard::init(schemas);
}

pub fn emit_create_listing(data: NewListing) {
    casper_event_standard::emit(data);
}

pub fn emit_buy_listing(data: ListingBought) {
    casper_event_standard::emit(data);
}

pub fn emit_cancel_listing(data: ListingCancelled) {
    casper_event_standard::emit(data);
}

pub fn emit_make_offer(data: NewOffer) {
    casper_event_standard::emit(data);
}

pub fn emit_cancel_offer(data: OfferCancelled) {
    casper_event_standard::emit(data);
}

pub fn emit_accept_offer(data: OfferAccepted) {
    casper_event_standard::emit(data);
}


pub fn emit_auction_started(data: AuctionStarted) {
    casper_event_standard::emit(data);
}


pub fn emit_auction_ended(data: AuctionEnded) {
    casper_event_standard::emit(data);
}

pub fn emit_bid(data: Bid) {
    casper_event_standard::emit(data);
}


pub fn emit_royalty_set(data: RoyaltySet) {
    casper_event_standard::emit(data);
}
