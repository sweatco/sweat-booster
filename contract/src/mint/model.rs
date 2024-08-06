use near_contract_standards::non_fungible_token::{metadata::TokenMetadata, Token};
use near_sdk::{env, near, serde_json};
use sweat_booster_model::api::BoosterType;

use crate::{mint::model::BoosterExtra::BalanceBooster, Contract};

#[near(serializers = [borsh, json])]
pub enum BoosterExtra {
    BalanceBooster(BalanceBoosterData),
}

#[near(serializers = [borsh, json])]
pub struct BalanceBoosterData {
    pub denomination: u128,
    pub is_redeemable: bool,
}

impl Contract {
    pub(crate) fn to_balance_booster_token(&self, booster_type: BoosterType) -> TokenMetadata {
        let BoosterType::BalanceBooster(data) = booster_type;

        let issued_at = env::block_timestamp_ms();
        let denomination_quot = data.denomination.0 / u128::pow(10, 18);
        let denomination_rem = data.denomination.0 % u128::pow(10, 18);

        TokenMetadata {
            title: Some(format!("Voucher #{}", self.last_id)),
            description: Some(format!("{denomination_quot}.{denomination_rem} $SWEAT voucher").to_string()),
            media: Some(data.media),
            media_hash: Some(data.media_hash),
            copies: Some(1),
            issued_at: Some(issued_at.to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: Some(
                serde_json::to_string(&BalanceBooster(BalanceBoosterData {
                    denomination: data.denomination.0,
                    is_redeemable: true,
                }))
                .unwrap(),
            ),
            reference: None,
            reference_hash: None,
        }
    }

    pub(crate) fn update_extra(&mut self, token: Token, extra: BoosterExtra) {
        let mut metadata = token.metadata.expect("Token doesn't contain metadata");
        metadata.extra = Some(serde_json::to_string(&extra).expect("Failed to serialize extra"));
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .unwrap()
            .insert(&token.token_id, &metadata);
    }
}

pub(crate) trait ExtraExtractor {
    fn get_extra(&self) -> BoosterExtra;
}

impl ExtraExtractor for Token {
    fn get_extra(&self) -> BoosterExtra {
        let metadata = self.metadata.as_ref().expect("Token doesn't contain metadata");
        let extra = metadata.extra.as_ref().expect("Metadata doesn't contain extra");

        serde_json::from_str::<BoosterExtra>(extra.as_str()).expect("Failed to parse extra")
    }
}
