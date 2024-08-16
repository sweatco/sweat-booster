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
            let mut u = tokens_per_owner.remove(&owner_id).unwrap();
            u.remove(&token_id);
        }

        self.token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.remove(&token_id))
            .expect("Token metadata not found")
    }
}
