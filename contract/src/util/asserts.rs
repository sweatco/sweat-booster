use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, env, require};
use crate::Contract;

impl Contract {
    pub(crate) fn assert_oracle(&self) {
        if !self.oracles.contains(&env::predecessor_account_id()) {
            env::panic_str("Only oracle can call this method");
        }
    }

    pub(crate) fn assert_owner(&self, owner_id: &AccountId, token_id: &TokenId) {
        require!(self.tokens.owner_by_id.get(token_id) == Some(owner_id.clone()), "Account doesnt own the token");
    }
}