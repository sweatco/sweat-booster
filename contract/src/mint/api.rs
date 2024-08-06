use near_contract_standards::non_fungible_token::Token;
use near_sdk::{near, AccountId};
use sweat_booster_model::api::{BoosterType, MintApi};

use crate::{Contract, ContractExt};

#[near]
impl MintApi for Contract {
    fn mint(&mut self, receiver_id: AccountId, booster_type: BoosterType) -> Token {
        self.assert_oracle();

        let metadata = self.to_balance_booster_token(booster_type);

        let result = self
            .tokens
            .internal_mint(self.last_id.to_string(), receiver_id, Some(metadata));

        self.last_id += 1;

        result
    }
}
