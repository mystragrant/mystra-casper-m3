use casper_types::{account::AccountHash, PublicKey, SecretKey, U256, U512};
use sha2::digest::consts::U2;

use crate::helpers::{
    blockchain_helpers::{get_account_balance, get_user},
    cep47_helpers::approve_cep_47,
    cep78_helpers::approve_cep_78,
    constants::MARKETPLACE_FEE_PERCENTAGE,
    fixtures::{get_default_fixture, get_offers_created_fixture},
    marketplace_actions::{
        build_accept_offer_request, build_cancel_offer_request, create_make_offer_request,
    },
};

#[test]
fn should_let_create_offer_and_take_money() {
    let (mut builder, market_hash, cep47_hash, cep78_hash, market_package_hash) =
        get_default_fixture();

    let price = 1000000000;
    let offerer = get_user(&mut builder, 1);

    let amount_before = get_account_balance(&mut builder, offerer);

    let req = create_make_offer_request(offerer, U256::from(1), market_hash, cep47_hash, price, 15);
    builder.exec(req).expect_success().commit();

    let req = create_make_offer_request(offerer, U256::from(0), market_hash, cep47_hash, price, 15);
    builder.exec(req).expect_success().commit();

    let amount_after = get_account_balance(&mut builder, offerer);

    assert_eq!(U512::from(price * 2), amount_before - amount_after);
}

#[test]
fn should_let_cancel_offer_and_create_new_one() {
    let (
        mut builder,
        market_hash,
        cep47_hash,
        cep78_hash,
        market_package_hash,
        price,
        expiration_time,
        offerer,
    ) = get_offers_created_fixture();

    let req = build_cancel_offer_request(offerer, market_hash, cep47_hash, U256::from(1));
    builder.exec(req).expect_success().commit();

    let req = build_cancel_offer_request(offerer, market_hash, cep78_hash, U256::from(0));
    builder.exec(req).expect_success().commit();

    let req = create_make_offer_request(offerer, U256::from(1), market_hash, cep47_hash, 100, 15);
    builder.exec(req).expect_success().commit();

    let req = create_make_offer_request(offerer, U256::from(0), market_hash, cep47_hash, 100, 15);
    builder.exec(req).expect_success().commit();
}

#[test]
fn should_allow_to_cancel() {
    let (
        mut builder,
        market_hash,
        cep47_hash,
        cep78_hash,
        market_package_hash,
        price,
        expiration_time,
        offerer,
    ) = get_offers_created_fixture();


    let req = build_cancel_offer_request(offerer, market_hash, cep47_hash, U256::from(1));
    builder.exec(req).expect_success().commit();

    let req = build_cancel_offer_request(offerer, market_hash, cep78_hash, U256::from(0));
    builder.exec(req).expect_success().commit();

}

#[test]
fn should_let_accept_offer_if_time_ok() {
    let (
        mut builder,
        market_hash,
        cep47_hash,
        cep78_hash,
        market_package_hash,
        price,
        expiration_time,
        offerer,
    ) = get_offers_created_fixture();

    let seller: casper_types::account::AccountHash = get_user(&mut builder, 2);
    let market: casper_types::account::AccountHash = get_user(&mut builder, 0);

    approve_cep_47(
        &mut builder,
        seller,
        cep47_hash,
        market_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(&mut builder, seller, cep78_hash, market_hash.into(), 0);

    let balance_seller_before = get_account_balance(&mut builder, seller);
    let balance_market_before = get_account_balance(&mut builder, market);

    let req = build_accept_offer_request(
        seller,
        market_hash,
        cep78_hash,
        U256::from(0),
        offerer,
        1,
        0,
    );
    builder.exec(req).expect_success().commit();

    let req = build_accept_offer_request(
        seller,
        market_hash,
        cep47_hash,
        U256::from(1),
        offerer,
        0,
        0,
    );
    builder.exec(req).expect_success().commit();

    let balance_seller_after = get_account_balance(&mut builder, seller);
    let balance_market_after = get_account_balance(&mut builder, market);

    let expected_market_revenue =
        (price + price) * U512::from(MARKETPLACE_FEE_PERCENTAGE) / U512::from(100);

    assert_eq!(
        balance_market_after,
        balance_market_before + expected_market_revenue
    );
  
}

#[test]
fn should_revert_accept_offer_if_time_expired() {
    let (
        mut builder,
        market_hash,
        cep47_hash,
        cep78_hash,
        market_package_hash,
        price,
        expiration_time,
        offerer,
    ) = get_offers_created_fixture();

    let seller: casper_types::account::AccountHash = get_user(&mut builder, 2);

    let blocktime_in_minutes = 16;
    assert_eq!(blocktime_in_minutes > expiration_time, true);

    approve_cep_47(
        &mut builder,
        seller,
        cep47_hash,
        market_package_hash.into(),
        vec![U256::from(1)],
    );
    approve_cep_78(&mut builder, seller, cep78_hash, market_hash.into(), 0);

    let req = build_accept_offer_request(
        seller,
        market_hash,
        cep78_hash,
        U256::from(0),
        offerer,
        1,
        blocktime_in_minutes * 60000,
    );
    builder.exec(req).expect_failure().commit();

 let req = build_accept_offer_request(
        seller,
        market_hash,
        cep47_hash,
        U256::from(1),
        offerer,
        0,
        blocktime_in_minutes * 60000,
    );
    builder.exec(req).expect_failure().commit();
}
