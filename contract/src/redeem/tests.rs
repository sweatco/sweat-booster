#![cfg(test)]

use near_contract_standards::non_fungible_token::NonFungibleTokenEnumeration;
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::PromiseOrValue;
use near_sdk::test_utils::test_env::alice;

use sweat_booster_model::api::{BalanceBoosterData, BoosterType, MintApi, RedeemApi};
use crate::common::test_data::set_test_future_success;
use crate::common::tests::{Context, oracle};
use crate::{BoosterExtra, ExtraExtractor};
use crate::mint::tests::DEPOSIT_FOR_MINTING;

#[test]
fn redeem_balance_booster_by_owner() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let reference_booster_data = BalanceBoosterData {
        media: "bafkreigdssmvjby7srbr44ivexsexqvdvjby7hhoobl3q3twim3bgxffrm".to_string(),
        media_hash: Base64VecU8::from(b"w5SZrIcflEMfcRUl5EvCa4hy2ZzscFe4bnZDNhNcpYs=".to_vec()),
        denomination: U128(2_000_000),
    };

    context.switch_account(oracle);
    context.with_deposit_yocto(
        DEPOSIT_FOR_MINTING,
        |context| {
            context.contract().mint(alice(), BoosterType::BalanceBooster(reference_booster_data.clone()));
        },
    );

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_redeem = alice_tokens.first().unwrap();

    context.switch_account(alice());
    let redeemed_balance = context.contract().redeem(token_to_redeem.token_id.clone());
    
    match redeemed_balance {
        PromiseOrValue::Promise(_) => { panic!("Expected value") }
        PromiseOrValue::Value(amount) => { assert_eq!(reference_booster_data.denomination, amount) }
    }

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(0, alice_tokens.len());
}

#[test]
#[should_panic(expected = "Account doesnt own the token")]
fn redeem_balance_booster_not_by_owner() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    let reference_booster_data = BalanceBoosterData {
        media: "bafkreigdssmvjby7sqewr4ivexsexqvdvjby7hhoobl3q3twim3bgxffrm".to_string(),
        media_hash: Base64VecU8::from(b"w5SZrIcflEMfcRUl5EvCwert2ZzscFe4bnZDNhNcpYs=".to_vec()),
        denomination: U128(2_000_000),
    };

    context.switch_account(oracle.clone());
    context.with_deposit_yocto(
        DEPOSIT_FOR_MINTING,
        |context| {
            context.contract().mint(alice(), BoosterType::BalanceBooster(reference_booster_data.clone()));
        },
    );

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

    let reference_booster_data = BalanceBoosterData {
        media: "bafkreigdssmvjby7srbr44ivexsexqvdvjby7hhoobl3q3twim3bgxffrm".to_string(),
        media_hash: Base64VecU8::from(b"w5SZrIcflEMfcRUl5EvCa4hy2ZzscFe4bnZDNhNcpYs=".to_vec()),
        denomination: U128(2_000_000),
    };

    context.switch_account(oracle);
    context.with_deposit_yocto(
        DEPOSIT_FOR_MINTING,
        |context| {
            context.contract().mint(alice(), BoosterType::BalanceBooster(reference_booster_data.clone()));
        },
    );

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_redeem = alice_tokens.first().unwrap();
    
    context.switch_account(alice());
    set_test_future_success(false);
    let redeemed_balance = context.contract().redeem(token_to_redeem.token_id.clone());
    
    match redeemed_balance {
        PromiseOrValue::Promise(_) => { panic!("Expected value") }
        PromiseOrValue::Value(amount) => { assert_eq!(0, amount.0) }
    }

    let alice_tokens = context.contract().nft_tokens_for_owner(alice(), None, None);
    assert_eq!(1, alice_tokens.len());

    let token_to_redeem = alice_tokens.first().unwrap();
    let BoosterExtra::BalanceBooster(extra) = token_to_redeem.get_extra();
    assert!(extra.is_redeemable);
}