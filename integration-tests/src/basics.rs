use near_contract_standards::non_fungible_token::Token;
use near_sdk::{serde_json, NearToken};
use nitka::{
    json,
    misc::ToNear,
    near_sdk::json_types::{Base64VecU8, U128},
};
use sweat_booster_model::{api::MintApiIntegration, model::BoosterExtra};

use crate::prepare::{prepare_contract, IntegrationContext};

#[tokio::test]
async fn mint() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "bafkreigdssmvjby7srbr44ivexsexqvdrbznthhoobl3q3twim3bgxffrm".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVIcflEMecRUl5EvCo4hy2ZzucFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(1_000_000);

    let result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NearToken::from_yoctonear(8_000_000_000_000_000_000_000))
        .await?;

    assert!(result.metadata.is_some());
    let metadata = result.metadata.unwrap();

    assert_eq!(media, metadata.media.unwrap());
    assert_eq!(media_hash.0, metadata.media_hash.unwrap().0);

    assert!(metadata.extra.is_some());
    let BoosterExtra::BalanceBooster(extra) = serde_json::from_str(metadata.extra.unwrap().as_str())?;

    assert_eq!(denomination.0, extra.denomination);

    let minted_token: Token = context
        .sweat_booster()
        .contract
        .call("nft_token")
        .args_json(json!({
            "token_id": result.token_id,
        }))
        .view()
        .await?
        .json()?;

    assert_eq!(minted_token.owner_id, alice.to_near());
    
    assert!(minted_token.metadata.is_some());
    let metadata = minted_token.metadata.unwrap();

    assert_eq!(media, metadata.media.unwrap());
    assert_eq!(media_hash.0, metadata.media_hash.unwrap().0);

    assert!(metadata.extra.is_some());
    let BoosterExtra::BalanceBooster(extra) = serde_json::from_str(metadata.extra.unwrap().as_str())?;

    assert_eq!(denomination.0, extra.denomination);

    Ok(())
}
