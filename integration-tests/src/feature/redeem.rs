use nitka::{
    misc::ToNear,
    near_sdk::json_types::{Base64VecU8, U128},
};
use sweat_booster_model::api::{MintApiIntegration, RedeemApiIntegration};

use crate::common::{
    prepare::{prepare_contract, ContextHelpers, IntegrationContext},
    NFT_MINTING_DEPOSIT,
};

#[tokio::test]
async fn redeem() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = prepare_contract().await?;
    let alice = context.alice().await?;
    let manager = context.manager().await?;

    let media = "https://another.li.nk/sggqrqwe".to_string();
    let media_hash = Base64VecU8::from(b"w5SZVIcflEMecRUl5EvCo4hy2ZzucFe4bnZDNhNcpYs=".to_vec());
    let denomination = U128(2_000_000);

    let result = context
        .sweat_booster()
        .mint_balance_booster(alice.to_near(), denomination, media.clone(), media_hash.clone())
        .with_user(&manager)
        .deposit(NFT_MINTING_DEPOSIT)
        .await?;

    let alice_balance_before_redeem = context.account_balance(&alice).await?;

    let result = context
        .sweat_booster()
        .redeem(result.token_id)
        .with_user(&alice)
        .await?;
    assert_eq!(result.0, denomination.0);

    let alice_balance_after_redeem = context.account_balance(&alice).await?;
    assert_eq!(denomination.0, alice_balance_after_redeem - alice_balance_before_redeem);

    Ok(())
}
