use near_contract_standards::non_fungible_token::TokenId;
use nitka::{
    misc::ToNear,
    near_sdk::json_types::{Base64VecU8, U128},
    panic_finder::PanicFinder,
};
use sweat_booster_model::api::{BurnApiIntegration, MintApiIntegration};

use crate::common::{
    prepare::{prepare_contract, IntegrationContext},
    NFT_MINTING_DEPOSIT,
};

#[tokio::test]
async fn burn_existing_token_for_owner() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://another.li.nk/sggqrqwe".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVeo2lEMecRUl5EvCo4hy2ZzucFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(5_000_000_000);

    let minting_result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NFT_MINTING_DEPOSIT)
        .await?;

    context
        .sweat_booster()
        .burn(alice.to_near(), minting_result.token_id)
        .with_user(&manager)
        .await?;

    Ok(())
}

#[tokio::test]
async fn burn_existing_token_for_not_owner() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://some.li.nk/sggqrqwe".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVeo2lEMecRUl5EvCo4hy2ZzucFe4bnZDNhkJlYs=".to_vec());
    let denomination = U128(10_000_000_000);

    let minting_result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NFT_MINTING_DEPOSIT)
        .await?;

    let burn_result = context
        .sweat_booster()
        .burn(manager.to_near(), minting_result.token_id)
        .with_user(&manager)
        .result();

    assert!(burn_result.await.has_panic("Account doesn't own the token"));

    Ok(())
}

#[tokio::test]
async fn burn_existing_token_for_owner_by_unauthorized_account() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://some.li.nk/sggqrqwe".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVeo2lEMecRUl5EvCo4hy2ZzucFe4bnZDNhkJlYs=".to_vec());
    let denomination = U128(10_000_000_000);

    let minting_result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NFT_MINTING_DEPOSIT)
        .await?;

    let burn_result = context
        .sweat_booster()
        .burn(alice.to_near(), minting_result.token_id)
        .with_user(&alice)
        .result();

    assert!(burn_result.await.has_panic("Only oracle can call this method"));

    Ok(())
}

#[tokio::test]
async fn burn_not_existing_token() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let result = context
        .sweat_booster()
        .burn(alice.to_near(), TokenId::from("token_id"))
        .with_user(&manager)
        .result();

    assert!(result.await.has_panic("Account doesn't own the token"));

    Ok(())
}
