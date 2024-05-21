use casper_engine_test_support::DEFAULT_ACCOUNTS;
use casper_types::account::AccountHash;

    // Contract Wasm File Paths (Constants)
    pub const MARKETPLACE_WASM: &str = "contract.wasm";
    pub const CEP47_WASM: &str = "cep47-token.wasm";
    pub const CEP78_WASM: &str = "cep78-token.wasm";

    pub const PAYMENT_WASM: &str = "payment-call.wasm";
    pub const OFFER_WASM: &str = "make-offer-call.wasm";
    pub const BID_WASM: &str = "bid-call.wasm";

    // Contract Storage Keys (Constants)
    pub const CONTRACT_KEY: &str = "mystra_marketplace";

    // Contract Entry Points (Constants)
    pub const ENTRY_POINT_CREATE_LISTING: &str = "create_listing";
    pub const ENTRY_POINT_ACCEPT_OFFER: &str = "accept_offer";
    pub const ENTRY_POINT_CANCEL_OFFER: &str = "cancel_offer";

    pub const MARKETPLACE_FEE_PERCENTAGE : u64 = 10;