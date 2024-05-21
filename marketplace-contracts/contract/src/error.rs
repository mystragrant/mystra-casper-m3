use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    NeedsTransferApproval = 0,
    PermissionDenied = 1,
    BalanceInsufficient = 2,
    OfferPurseRetrieval = 3,
    PriceSetToZero = 4,
    OfferDoesntExistOrCancelled = 5,
    ListingExpired = 6,
    BidTooLow = 7,
    AuctionEnded = 8,
    ListingDoesntExist = 9,
    ListingCancelledOrFinished = 10,
    OfferDoesntExist = 11,
    OfferCancelledOrFinished = 12,
    AuctionDoesntExist = 13,
    AuctionCancelledOrFinished = 14,
    AuctionNotFinished = 15,
    CallerNotInstaller = 16,
    TokenAlreadyOnListing = 17,
    OfferExpired = 18
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}
