use near_contract_standards::non_fungible_token::{metadata::TokenMetadata, NonFungibleToken, TokenId};
use near_sdk::{near, serde_json, AccountId};
use sweat_booster_model::{
    api::BurnApi,
    event::{emit, BurnData, EventKind},
    model::BoosterExtra,
};

use crate::{Contract, ContractExt};

#[near]
impl BurnApi for Contract {
    fn burn(&mut self, owner_id: AccountId, token_id: TokenId) {
        self.assert_oracle();
        self.assert_owner(&owner_id, &token_id);

        let metadata = self.tokens.burn(token_id.clone());
        let BoosterExtra::BalanceBooster(extra) =
            serde_json::from_str::<BoosterExtra>(metadata.extra.expect("Token has no extra").as_str())
                .expect("Failed to parse extra");

        emit(EventKind::Burn(BurnData {
            token_id,
            denomination: extra.denomination.into(),
        }));
    }
}

pub(crate) trait NonFungibleTokenBurn {
    fn burn(&mut self, token_id: TokenId) -> TokenMetadata;
}

impl NonFungibleTokenBurn for NonFungibleToken {
    fn burn(&mut self, token_id: TokenId) -> TokenMetadata {
        let owner_id = self.owner_by_id.remove(&token_id).expect("Owner not found");

        if let Some(approvals_by_id) = &mut self.approvals_by_id {
            approvals_by_id.remove(&token_id);
        }

        if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
            let mut owner_tokens = tokens_per_owner
                .get(&owner_id)
                .expect("Unable to access tokens per owner.");
            owner_tokens.remove(&token_id);
            if owner_tokens.is_empty() {
                tokens_per_owner.remove(&owner_id);
            } else {
                tokens_per_owner.insert(&owner_id, &owner_tokens);
            }
        }

        self.token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.remove(&token_id))
            .expect("Token metadata not found")
    }
}
