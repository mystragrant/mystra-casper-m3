use casper_types::{U256, U512};

use crate::helpers::{
    blockchain_helpers::{get_account_balance, get_user},
    cep47_helpers::approve_cep_47,
    cep78_helpers::approve_cep_78,
    constants::MARKETPLACE_FEE_PERCENTAGE,
    fixtures::{
        get_default_fixture, get_listing_created_fixture,
        get_listing_created_fixture_with_royalties,
    },
    marketplace_actions::{build_cancel_listing_request, create_buy_nft_request, create_listing},
};

#[test]
fn should_let_create_listing_only_after_approval() {
    let (mut builder, market_hash, cep47_hash, cep78_hash, market_package_hash) =
        get_default_fixture();

    // For CEP47
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep47_hash,
        1,
        U512::from(100),
        0,
        0,
    );
    builder.exec(req).expect_failure().commit();

    //For CEP78
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep78_hash,
        0,
        U512::from(100u64),
        0u64,
        1,
    );
    builder.exec(req).expect_failure().commit();

    let nft_owner = get_user(&mut builder, 2);

    approve_cep_47(
        &mut builder,
        nft_owner,
        cep47_hash,
        market_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(&mut builder, nft_owner, cep78_hash, market_hash.into(), 0);

    // For CEP47
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep47_hash,
        1,
        U512::from(100u64),
        0u64,
        0,
    );
    builder.exec(req).expect_success().commit();

    // For CEP78
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep78_hash,
        0,
        U512::from(100u64),
        0u64,
        1,
    );
    builder.exec(req).expect_success().commit();
}

#[test]
fn should_let_buy_unlimited_expiration_time_listing_at_any_time() {
    let (mut builder, market_hash, cep47_hash, cep78_hash, market_package_hash) =
        get_default_fixture();

    // For CEP47
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep47_hash,
        1,
        U512::from(100),
        0,
        0,
    );
    builder.exec(req).expect_failure().commit();

    //For CEP78
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep78_hash,
        0,
        U512::from(100u64),
        0u64,
        1,
    );
    builder.exec(req).expect_failure().commit();

    let nft_owner = get_user(&mut builder, 2);

    approve_cep_47(
        &mut builder,
        nft_owner,
        cep47_hash,
        market_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(&mut builder, nft_owner, cep78_hash, market_hash.into(), 0);

    // For CEP47
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep47_hash,
        1,
        U512::from(100u64),
        0u64,
        0,
    );
    builder.exec(req).expect_success().commit();

    // For CEP78
    let req = create_listing(
        get_user(&mut builder, 2),
        market_hash,
        cep78_hash,
        0,
        U512::from(100u64),
        0u64,
        1,
    );
    builder.exec(req).expect_success().commit();

    // For CEP47
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(1),
        market_hash,
        cep47_hash,
        100u64,
        0,
        99999999999u64,
    );
    builder.exec(req).expect_success().commit();

    // For CEP78
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(0),
        market_hash,
        cep78_hash,
        100u64,
        1,
        99999999999u64,
    );
    builder.exec(req).expect_success().commit();
}

#[test]
fn should_let_cancel_listing_and_create_new_afterwards() {
    let (
        mut builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        _listing_price,
        _listing_duration_minutes,
    ) = get_listing_created_fixture();

    // For CEP47
    let req = build_cancel_listing_request(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep47_hash,
        U256::from(1),
        0,
    );
    builder.exec(req).expect_success().commit();

    // For CEP78
    let req = build_cancel_listing_request(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep78_hash,
        U256::from(0),
        1,
    );
    builder.exec(req).expect_success().commit();

    let nft_owner: casper_types::account::AccountHash = get_user(&mut builder, 2);

    //Approvals
    approve_cep_47(
        &mut builder,
        nft_owner,
        cep47_hash,
        marketplace_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(
        &mut builder,
        nft_owner,
        cep78_hash,
        marketplace_hash.into(),
        0,
    );

    // Create listing for CEP47
    let req = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep47_hash,
        1,
        U512::from(100u64),
        0u64,
        0,
    );
    builder.exec(req).expect_success().commit();

    // Create listing for CEP78
    let req = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep78_hash,
        0,
        U512::from(100u64),
        0u64,
        1,
    );
    builder.exec(req).expect_success().commit();
}

#[test]
fn should_let_update_listing() {
    let (
        mut builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        _listing_price,
        _listing_duration_minutes,
    ) = get_listing_created_fixture();

    let nft_owner: casper_types::account::AccountHash = get_user(&mut builder, 2);

    //Approvals
    approve_cep_47(
        &mut builder,
        nft_owner,
        cep47_hash,
        marketplace_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(
        &mut builder,
        nft_owner,
        cep78_hash,
        marketplace_hash.into(),
        0,
    );

    // Create listing for CEP47
    let req = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep47_hash,
        1,
        U512::from(1000u64),
        15u64,
        0,
    );
    builder.exec(req).expect_success().commit();

    // Create listing for CEP78
    let req = create_listing(
        get_user(&mut builder, 2),
        marketplace_hash,
        cep78_hash,
        0,
        U512::from(1000u64),
        15u64,
        1,
    );
    builder.exec(req).expect_success().commit();
}

#[test]
fn should_revert_buy_listing_if_paid_too_few_cspr() {
    let (
        mut builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        listing_price,
        _listing_duration_minutes,
    ) = get_listing_created_fixture();

    let price = U512::from(1);
    assert_eq!(price < listing_price, true);

    // For CEP47
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(1),
        marketplace_hash,
        cep47_hash,
        price.as_u64(),
        0,
        0,
    );
    builder.exec(req).expect_failure().commit();

    // For CEP78
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(0),
        marketplace_hash,
        cep78_hash,
        price.as_u64(),
        1,
        0,
    );
    builder.exec(req).expect_failure().commit();
}

#[test]
fn should_revert_buy_listing_if_time_passed() {
    let (
        mut builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        listing_price,
        listing_duration_minutes,
    ) = get_listing_created_fixture();

    let blocktime = 16;
    assert_eq!(blocktime > listing_duration_minutes, true);

    // For CEP47
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(1),
        marketplace_hash,
        cep47_hash,
        listing_price.as_u64(),
        0,
        blocktime * 60000,
    );
    builder.exec(req).expect_failure().commit();

    // For CEP78
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(0),
        marketplace_hash,
        cep78_hash,
        listing_price.as_u64(),
        1,
        blocktime * 60000,
    );
    builder.exec(req).expect_failure().commit();
}

#[test]
fn should_buy_listing_if_time_and_price_ok() {
    let (
        mut builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        listing_price,
        listing_duration_minutes,
    ) = get_listing_created_fixture();

    let blocktime = 10;
    assert_eq!(blocktime < listing_duration_minutes, true);

    let seller = get_user(&mut builder, 2);
    let marketplace = get_user(&mut builder, 0);

    let balance_marketplace_creator_before = get_account_balance(&mut builder, marketplace);
    let balance_seller_before = get_account_balance(&mut builder, seller);

    // For CEP47
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(1),
        marketplace_hash,
        cep47_hash,
        listing_price.as_u64(),
        0,
        blocktime * 60000,
    );
    builder.exec(req).expect_success().commit();

    let balance_marketplace_creator_after = get_account_balance(&mut builder, marketplace);
    let balance_seller_after = get_account_balance(&mut builder, seller);

    let expected_marketplace_revenue =
        listing_price * U512::from(MARKETPLACE_FEE_PERCENTAGE) / U512::from(100);
    let expected_seller_revenue = listing_price - expected_marketplace_revenue;

    assert_eq!(
        expected_marketplace_revenue,
        balance_marketplace_creator_after - balance_marketplace_creator_before
    );
    assert_eq!(
        expected_seller_revenue,
        balance_seller_after - balance_seller_before
    );

    let balance_marketplace_creator_before = get_account_balance(&mut builder, marketplace);
    let balance_seller_before = get_account_balance(&mut builder, seller);

    // For CEP47
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(0),
        marketplace_hash,
        cep78_hash,
        listing_price.as_u64(),
        1,
        blocktime * 60000,
    );
    builder.exec(req).expect_success().commit();

    let balance_marketplace_creator_after = get_account_balance(&mut builder, marketplace);
    let balance_seller_after = get_account_balance(&mut builder, seller);

    let expected_marketplace_revenue =
        listing_price * U512::from(MARKETPLACE_FEE_PERCENTAGE) / U512::from(100);
    let expected_seller_revenue = listing_price - expected_marketplace_revenue;

    assert_eq!(
        expected_marketplace_revenue,
        balance_marketplace_creator_after - balance_marketplace_creator_before
    );
    assert_eq!(
        expected_seller_revenue,
        balance_seller_after - balance_seller_before
    );
}

#[test]
fn should_buy_listing_with_royalties() {
    let (
        mut builder,
        marketplace_hash,
        cep47_hash,
        cep78_hash,
        marketplace_package_hash,
        listing_price,
        listing_duration_minutes,
        royalty_cep47,
        royalty_cep78,
        royalty_creator,
    ) = get_listing_created_fixture_with_royalties();

    let blocktime = 10;
    assert_eq!(blocktime < listing_duration_minutes, true);

    let seller = get_user(&mut builder, 2);
    let marketplace = get_user(&mut builder, 0);

    let balance_marketplace_creator_before = get_account_balance(&mut builder, marketplace);
    let balance_seller_before = get_account_balance(&mut builder, seller);
    let balance_creator_before = get_account_balance(&mut builder, royalty_creator);

    // For CEP47
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(1),
        marketplace_hash,
        cep47_hash,
        listing_price.as_u64(),
        0,
        blocktime * 60000,
    );
    builder.exec(req).expect_success().commit();

    let balance_marketplace_creator_after = get_account_balance(&mut builder, marketplace);
    let balance_seller_after = get_account_balance(&mut builder, seller);
    let balance_creator_after: U512 = get_account_balance(&mut builder, royalty_creator);

    let expected_marketplace_revenue =
        listing_price * U512::from(MARKETPLACE_FEE_PERCENTAGE) / U512::from(100);
    let expected_creator_revenue = listing_price * U512::from(royalty_cep47) / U512::from(100);
  


    let balance_marketplace_creator_before = get_account_balance(&mut builder, marketplace);
    let balance_seller_before = get_account_balance(&mut builder, seller);
    let balance_creator_before = get_account_balance(&mut builder, royalty_creator);

    // For CEP47
    let req = create_buy_nft_request(
        get_user(&mut builder, 1),
        U256::from(0),
        marketplace_hash,
        cep78_hash,
        listing_price.as_u64(),
        1,
        blocktime * 60000,
    );
    builder.exec(req).expect_success().commit();

    let balance_marketplace_creator_after = get_account_balance(&mut builder, marketplace);
    let balance_seller_after = get_account_balance(&mut builder, seller);
    let balance_creator_after: U512 = get_account_balance(&mut builder, royalty_creator);

    let expected_marketplace_revenue =
        listing_price * U512::from(MARKETPLACE_FEE_PERCENTAGE) / U512::from(100);
    let expected_creator_revenue = listing_price * U512::from(royalty_cep78) / U512::from(100);
    let expected_seller_revenue =
        listing_price - expected_marketplace_revenue - expected_creator_revenue;


}
