use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::{account::AccountHash, runtime_args, ContractHash, RuntimeArgs, U256, U512};

use super::{
    blockchain_helpers::{get_contract_hash_from_account, get_user},
    cep47_helpers::{approve_cep_47, deploy_cep_47, mint_cep_47},
    cep78_helpers::{approve_cep_78, deploy_cep_78, mint_cep_78},
    constants::{CONTRACT_KEY, MARKETPLACE_WASM},
    marketplace_actions::{build_set_royalties_request, create_listing, create_make_offer_request},
};

pub fn get_default_fixture() -> (
    InMemoryWasmTestBuilder,
    ContractHash,
    ContractHash,
    ContractHash,
    ContractHash,
) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder
        .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
        .commit();

    // Deploy contracts
    let marketplace_hash = deploy_marketplace(&mut builder, MARKETPLACE_WASM);
    let nft_hash = deploy_cep_47(&mut builder);
    let nft_hash_78: ContractHash = deploy_cep_78(&mut builder);

    let nft_owner = get_user(&mut builder, 2);

    mint_cep_78(
        &mut builder,
        nft_hash_78,
        nft_owner.into(),
        vec![U256::from(1)],
    );

    mint_cep_47(
        &mut builder,
        nft_hash,
        nft_owner.into(),
        vec![U256::from(1)],
    );

    let marketplace_contract_package_hash = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get("mystra_marketplace_package_name")
        .expect("must have contract hash key")
        .into_hash()
        .map(ContractHash::new)
        .expect("must get contract hash");

    (
        builder,
        marketplace_hash,
        nft_hash,
        nft_hash_78,
        marketplace_contract_package_hash,
    )
}

pub fn get_listing_created_fixture() -> (
    InMemoryWasmTestBuilder,
    ContractHash,
    ContractHash,
    ContractHash,
    ContractHash,
    U512,
    u64,
) {
    let (mut builder, marketplace_hash, cep47_hash, cep78_hash, marketplace_package_hash) =
        get_default_fixture();

    //Setup data
    let listing_price = U512::from(1_000_000_000_000u64);
    let listing_duration = 15;

    let nft_owner = get_user(&mut builder, 2);

    //Setup approvals
    approve_cep_47(
        &mut builder,
        nft_owner.into(),
        cep47_hash,
        marketplace_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(
        &mut builder,
        nft_owner.into(),
        cep78_hash,
        marketplace_hash.into(),
        0,
    );

    // Create CEP47 listing
    let request = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep47_hash,
        1,
        listing_price,
        listing_duration,
        0,
    );
    builder.exec(request).expect_success().commit();

    // Create CEP78 listing
    let request = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep78_hash,
        0,
        listing_price,
        listing_duration,
        1,
    );
    builder.exec(request).expect_success().commit();

    (
        builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        listing_price,
        listing_duration,
    )
}




pub fn get_listing_created_fixture_with_royalties() -> (
    InMemoryWasmTestBuilder,
    ContractHash,
    ContractHash,
    ContractHash,
    ContractHash,
    U512,
    u64,
    u64,
    u64,
    AccountHash
) {
    let (mut builder, marketplace_hash, cep47_hash, cep78_hash, marketplace_package_hash) =
        get_default_fixture();

    //Setup data
    let listing_price = U512::from(1_000_000_000_000u64);
    let listing_duration = 15;

    let nft_owner = get_user(&mut builder, 2);

    let royalty_percentage_cep78 = 5;
    let royalty_percentage_cep47 = 15;

    // Set royalties
    let req = build_set_royalties_request(
        get_user(&mut builder, 0),
        get_user(&mut builder, 3),
        marketplace_hash,
        cep47_hash,
        royalty_percentage_cep47,
    );
    builder.exec(req).expect_success().commit();

    // Set royalties
    let req = build_set_royalties_request(
        get_user(&mut builder, 0),
        get_user(&mut builder, 3),
        marketplace_hash,
        cep78_hash,
        royalty_percentage_cep78,
    );
    builder.exec(req).expect_success().commit();

    //Setup approvals
    approve_cep_47(
        &mut builder,
        nft_owner.into(),
        cep47_hash,
        marketplace_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(
        &mut builder,
        nft_owner.into(),
        cep78_hash,
        marketplace_hash.into(),
        0,
    );

    // Create CEP47 listing
    let request = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep47_hash,
        1,
        listing_price,
        listing_duration,
        0,
    );
    builder.exec(request).expect_success().commit();

    // Create CEP78 listing
    let request = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep78_hash,
        0,
        listing_price,
        listing_duration,
        1,
    );
    builder.exec(request).expect_success().commit();

    let creator =  get_user(&mut builder, 3);

    (
        builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        listing_price,
        listing_duration,
        royalty_percentage_cep47,
        royalty_percentage_cep78,
 creator      
    )
}

/// Deploys a contract version to the InMemoryWasmTestBuilder
fn deploy_marketplace(builder: &mut InMemoryWasmTestBuilder, wasm_code: &str) -> ContractHash {
    let request =
        ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, wasm_code, runtime_args! {}).build();
    builder.exec(request).expect_success().commit();
    get_contract_hash_from_account(builder, CONTRACT_KEY)
}


pub fn get_offers_created_fixture() -> (
    InMemoryWasmTestBuilder,
    ContractHash,
    ContractHash,
    ContractHash,
    ContractHash,
    U512,
    u64,
    AccountHash
) {
    let (mut builder, marketplace_hash, cep47_hash, cep78_hash, marketplace_package_hash) =
        get_default_fixture();

        let offer_amount = 1000_000_000_000;
        let offer_duration = 15;

        let offerer: AccountHash = get_user(&mut builder, 3);

        let req = create_make_offer_request(offerer, U256::from(1), marketplace_hash, cep47_hash, offer_amount, offer_duration);
        builder.exec(req).expect_success().commit();
    
        let req = create_make_offer_request(offerer, U256::from(0), marketplace_hash, cep78_hash, offer_amount, offer_duration);
        builder.exec(req).expect_success().commit();
    (
        builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        U512::from(offer_amount),
        offer_duration,
        offerer
    )
}

