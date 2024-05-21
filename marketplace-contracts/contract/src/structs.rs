use casper_types::{account::AccountHash, Key, U512};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use alloc::vec::Vec;

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct ListingData {
    pub seller: Key,
    pub price: U512,
    pub expiration_time: Option<u64>,
}

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct AuctionData {
    pub seller: AccountHash,
    pub starting_price: U512,
    pub current_bid: U512,
    pub current_winner: AccountHash,
    pub end_time: u64,
}

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct OfferData {
    pub price: U512,
    pub expiration_time: u64,
}

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct RoyaltyData {
    pub percentage: u64,
    pub creator: Key,
}