use nitka::{
    misc::ToNear,
    near_sdk::json_types::{Base64VecU8, U128},
};
use sweat_booster_model::api::{BurnApiIntegration, MintApiIntegration};

use crate::common::{
    prepare::{prepare_contract, IntegrationContext},
    NFT_MINTING_DEPOSIT,
};

#[tokio::test]
async fn burn() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://another.li.nk/sggqrqwe".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVIcflEMecRUl5EvCo4hy2ZzucFe4bnZDNhNcpYs=".to_vec());
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
