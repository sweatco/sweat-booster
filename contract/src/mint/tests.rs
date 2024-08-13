#![cfg(test)]

use std::panic::catch_unwind;

use near_contract_standards::non_fungible_token::NonFungibleTokenEnumeration;
use near_sdk::{
    json_types::{Base64VecU8, U128},
    test_utils::test_env::alice,
};
use regex::Regex;
use sweat_booster_model::{api::MintApi, model::BoosterExtra};

use crate::{
    common::tests::{oracle, Context},
    mint::model::ExtraExtractor,
};

pub(crate) const DEPOSIT_FOR_MINTING: u128 = 8_000_000_000_000_000_000_000;

#[test]
#[should_panic(expected = "Only oracle can call this method")]
fn mint_by_unauthorized_account() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(alice());
    context
        .contract()
        .mint_balance_booster(alice(), U128(1_000_000), "".to_string(), Base64VecU8::from(vec![]));
}

#[test]
fn mint_by_authorized_account_without_deposit() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(oracle);

    let result = catch_unwind(|| {
        context
            .contract()
            .mint_balance_booster(alice(), U128(1_000_000), "".to_string(), Base64VecU8::from(vec![]))
    });

    assert!(result.is_err());

    if let Err(error) = result {
        let panic_message = error.downcast_ref::<String>().unwrap();
        let regex = Regex::new(r"Must attach \d+\.\d+ NEAR yoctoNEAR").unwrap();
        assert!(regex.is_match(panic_message));
    }
}

#[test]
fn mint_by_authorized_account() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let media = "bafkreigdssmvjby7srbr44ivexsexqvdrbznthhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVIcflEMecRUl5EvCo4hy2ZzucFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(1_000_000);

    context.switch_account(oracle);
    context.with_deposit_yocto(DEPOSIT_FOR_MINTING, |context| {
        context
            .contract()
            .mint_balance_booster(alice(), denomination, media, media_hash);
    });

    assert_eq!(1, context.contract().nft_supply_for_owner(alice()).0);

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    let BoosterExtra::BalanceBooster(target_booster) = alice_tokens.first().unwrap().get_extra();

    assert_eq!(target_booster.denomination, denomination.0);
}
