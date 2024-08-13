use near_contract_standards::non_fungible_token::Token;
use near_sdk::{
    json_types::{Base64VecU8, U128},
    near, AccountId,
};
use sweat_booster_model::api::MintApi;

use crate::{Contract, ContractExt};

#[near]
impl MintApi for Contract {
    #[payable]
    fn mint_balance_booster(
        &mut self,
        receiver_id: AccountId,
        denomination: U128,
        media: String,
        media_hash: Base64VecU8,
    ) -> Token {
        self.assert_oracle();

        let metadata = self.to_balance_booster_token(denomination.0, media, media_hash);

        let result = self
            .tokens
            .internal_mint(self.last_id.to_string(), receiver_id, Some(metadata));

        self.last_id += 1;

        result
    }
}
