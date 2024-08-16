#![cfg(test)]

use near_contract_standards::non_fungible_token::NonFungibleTokenEnumeration;
use near_sdk::{
    json_types::{Base64VecU8, U128},
    test_utils::test_env::alice,
    PromiseOrValue,
};
use sweat_booster_model::{
    api::{MintApi, RedeemApi},
    model::BoosterExtra,
};

use crate::{
    common::{
        test_data::set_test_future_success,
        tests::{oracle, Context},
    },
    mint::{model::ExtraExtractor, tests::DEPOSIT_FOR_MINTING},
};

#[test]
fn redeem_balance_booster_by_owner() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let media = "bafkreigdssmvjby7srbr44ivexsexqvdvjby7hhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZrIcflEMfcRUl5EvCa4hy2ZzscFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    context.switch_account(oracle);
    context.with_deposit_yocto(DEPOSIT_FOR_MINTING, |context| {
        context
            .contract()
            .mint_balance_booster(alice(), denomination, media, media_hash);
    });

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_redeem = alice_tokens.first().unwrap();

    context.switch_account(alice());
    let redeemed_balance = context.contract().redeem(token_to_redeem.token_id.clone());

    match redeemed_balance {
        PromiseOrValue::Promise(_) => {
            panic!("Expected value")
        }
        PromiseOrValue::Value(amount) => {
            assert_eq!(denomination, amount)
        }
    }

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(0, alice_tokens.len());
}

#[test]
#[should_panic(expected = "Account doesn't own the token")]
fn redeem_balance_booster_not_by_owner() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let media = "bafkreigdssmvjby7sqewr4ivexsexqvdvjby7hhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZrIcflEMfcRUl5EvCwert2ZzscFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    context.switch_account(oracle.clone());
    context.with_deposit_yocto(DEPOSIT_FOR_MINTING, |context| {
        context
            .contract()
            .mint_balance_booster(alice(), denomination, media, media_hash);
    });

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_redeem = alice_tokens.first().unwrap();

    context.switch_account(oracle);
    context.contract().redeem(token_to_redeem.token_id.clone());
}

#[test]
fn redeem_balance_booster_by_owner_with_failed_transfer() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let media = "bafkreigdssmvjby7srbr44ivexsexqvdvjby7hhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZrIcflEMfcRUl5EvCa4hy2ZzscFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    context.switch_account(oracle);
    context.with_deposit_yocto(DEPOSIT_FOR_MINTING, |context| {
        context
            .contract()
            .mint_balance_booster(alice(), denomination, media, media_hash);
    });

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_redeem = alice_tokens.first().unwrap();

    context.switch_account(alice());
    set_test_future_success(false);
    let redeemed_balance = context.contract().redeem(token_to_redeem.token_id.clone());

    match redeemed_balance {
        PromiseOrValue::Promise(_) => {
            panic!("Expected value")
        }
        PromiseOrValue::Value(amount) => {
            assert_eq!(0, amount.0)
        }
    }

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_redeem = alice_tokens.first().unwrap();
    let BoosterExtra::BalanceBooster(extra) = token_to_redeem.get_extra();
    assert!(extra.is_redeemable);
}
