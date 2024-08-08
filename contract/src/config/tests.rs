#![cfg(test)]

use near_contract_standards::non_fungible_token::metadata::NonFungibleTokenMetadataProvider;
use near_sdk::test_utils::test_env::alice;
use sweat_booster_model::api::ConfigApi;

use crate::common::tests::{oracle, Context};

#[test]
fn set_base_url_by_oracle() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let base_uri = context.contract().nft_metadata().base_uri;
    assert_eq!(None, base_uri);

    let reference_base_uri = "http://base.uri".to_string();

    context.switch_account(oracle);
    context.contract().set_base_uri(reference_base_uri.clone());

    let base_uri = context.contract().nft_metadata().base_uri;
    assert_eq!(Some(reference_base_uri), base_uri);
}

#[test]
#[should_panic(expected = "Only oracle can call this method")]
fn set_base_url_not_by_oracle() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(alice());
    context.contract().set_base_uri("http://some.link".to_string());
}
