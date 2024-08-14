use near_contract_standards::non_fungible_token::Token;
use near_sdk::{serde_json, serde_json::json};
use nitka::{
    misc::ToNear,
    near_sdk::json_types::{Base64VecU8, U128},
    panic_finder::PanicFinder,
};
use sweat_booster_model::{
    api::{MintApiIntegration, RedeemApiIntegration},
    model::BoosterExtra,
};

use crate::common::{
    prepare::{prepare_contract, prepare_contract_with_balances, ContextHelpers, IntegrationContext},
    NFT_MINTING_DEPOSIT,
};

#[tokio::test]
async fn redeem_by_authorized_account() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://another.li.nk/sggqrqwe".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVIcflEMqiRUo0EvCo4hu8EzucFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    let minting_result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NFT_MINTING_DEPOSIT)
        .await?;

    let alice_balance_before_redeem = context.account_balance(&alice).await?;

    let redeem_result = context
        .sweat_booster()
        .redeem(minting_result.token_id.clone())
        .with_user(&alice)
        .await?;
    assert_eq!(redeem_result.0, denomination.0);

    let alice_balance_after_redeem = context.account_balance(&alice).await?;
    assert_eq!(denomination.0, alice_balance_after_redeem - alice_balance_before_redeem);

    let redeemed_token: Option<Token> = context
        .sweat_booster()
        .contract
        .call("nft_token")
        .args_json(json!({
            "token_id": minting_result.token_id,
        }))
        .view()
        .await?
        .json()?;

    assert!(redeemed_token.is_none());

    Ok(())
}

#[tokio::test]
async fn redeem_by_unauthorized_account() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://what.ever/token".to_string();
    let media_hash = Base64VecU8::from(b"oeu9VIcflEMqiRUo0EvCo4hu8EzucFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(10_000_000_000_000);

    let result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NFT_MINTING_DEPOSIT)
        .await?;

    let result = context
        .sweat_booster()
        .redeem(result.token_id)
        .with_user(&manager)
        .result()
        .await;

    assert!(result.has_panic("Account doesn't own the token"));

    Ok(())
}

#[tokio::test]
async fn redeem_by_authorized_account_with_failed_ft_transfer() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract_with_balances(0, 0, 0).await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://lets.fail/id".to_string();
    let media_hash = Base64VecU8::from(b"oeu9VIcflEMqiRUo0Oiko4hu8EzucFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(10_000_000_000_000);

    let minting_result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NFT_MINTING_DEPOSIT)
        .await?;

    let redeem_result = context
        .sweat_booster()
        .redeem(minting_result.token_id.clone())
        .with_user(&alice)
        .await?;

    assert_eq!(0, redeem_result.0);

    let minted_token: Token = context
        .sweat_booster()
        .contract
        .call("nft_token")
        .args_json(json!({
            "token_id": minting_result.token_id,
        }))
        .view()
        .await?
        .json()?;

    assert!(minted_token.metadata.is_some());
    let metadata = minted_token.metadata.unwrap();

    assert_eq!(media, metadata.media.unwrap());
    assert_eq!(media_hash.0, metadata.media_hash.unwrap().0);

    assert!(metadata.extra.is_some());
    let BoosterExtra::BalanceBooster(extra) = serde_json::from_str(metadata.extra.unwrap().as_str())?;

    assert_eq!(denomination.0, extra.denomination);
    assert!(extra.is_redeemable);

    Ok(())
}
