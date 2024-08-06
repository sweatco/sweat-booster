use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{env, require, AccountId, Gas};

use crate::{common::remaining_gas, Contract};

impl Contract {
    pub(crate) fn assert_oracle(&self) {
        if !self.oracles.contains(&env::predecessor_account_id()) {
            env::panic_str("Only oracle can call this method");
        }
    }

    pub(crate) fn assert_owner(&self, owner_id: &AccountId, token_id: &TokenId) {
        require!(
            self.tokens.owner_by_id.get(token_id) == Some(owner_id.clone()),
            "Account doesnt own the token"
        );
    }
}

pub(crate) fn assert_enough_gas(required: Gas) {
    let remaining_gas = remaining_gas();
    require!(
        remaining_gas >= required,
        format!("Not enough gas for further operations: {remaining_gas} gas left when {required} required"),
    );
}
