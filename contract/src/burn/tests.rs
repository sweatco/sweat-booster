#![cfg(test)]

use near_contract_standards::non_fungible_token::{core::NonFungibleTokenCore, NonFungibleTokenEnumeration};
use near_sdk::{
    json_types::{Base64VecU8, U128},
    test_utils::test_env::{alice, bob},
};
use sweat_booster_model::api::{BurnApi, MintApi};

use crate::{
    common::tests::{oracle, Context},
    mint::tests::DEPOSIT_FOR_MINTING,
};

#[test]
fn burn_token_by_oracle_for_valid_owner() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let media = "bafkiufjkssmvjby7srbr44ivexsexqvdvjby7hhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZrIcpoakfcRUl5EvCa4hy2ZzscFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    context.switch_account(oracle);
    context.with_deposit_yocto(DEPOSIT_FOR_MINTING, |context| {
        context
            .contract()
            .mint_balance_booster(alice(), denomination, media, media_hash);
    });

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_burn = alice_tokens.first().unwrap();

    context.contract().burn(alice(), token_to_burn.token_id.clone());

    let burnt_token = context.contract().nft_token(token_to_burn.token_id.clone());
    assert!(burnt_token.is_none());

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(0, alice_tokens.len());
}

#[test]
#[should_panic(expected = "Account doesn't own the token")]
fn burn_token_by_oracle_for_invalid_owner() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let media = "bafkiufjkssmvjby7srbr44ivexs74937vjby7hhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZrIcpoakfcRUl5EvCqerjZzscFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    context.switch_account(oracle);
    context.with_deposit_yocto(DEPOSIT_FOR_MINTING, |context| {
        context
            .contract()
            .mint_balance_booster(alice(), denomination, media, media_hash);
    });

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_burn = alice_tokens.first().unwrap();

    context.contract().burn(bob(), token_to_burn.token_id.clone());
}

#[test]
#[should_panic(expected = "Only oracle can call this method")]
fn burn_token_not_by_oracle() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let media = "bafkiufjkssmvjby7srbr44iv84ij4qvdvjby7hhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZrIcpoakfcRUl5EvCaoi8fuzscFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    context.switch_account(oracle);
    context.with_deposit_yocto(DEPOSIT_FOR_MINTING, |context| {
        context
            .contract()
            .mint_balance_booster(alice(), denomination, media, media_hash);
    });

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    let token_to_burn = alice_tokens.first().unwrap();

    context.switch_account(alice());
    context.contract().burn(alice(), token_to_burn.token_id.clone());
}
