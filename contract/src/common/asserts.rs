use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{env, require, AccountId, Gas};
use sweat_booster_model::model::BoosterExtra;
use crate::{common::remaining_gas, Contract};
use crate::mint::model::ExtraExtractor;

impl Contract {
    pub(crate) fn assert_oracle(&self) {
        if !self.oracles.contains(&env::predecessor_account_id()) {
            env::panic_str("Only oracle can call this method");
        }
    }

    pub(crate) fn assert_owner(&self, owner_id: &AccountId, token_id: &TokenId) {
        require!(
            self.tokens.owner_by_id.get(token_id) == Some(owner_id.clone()),
            "Account doesn't own the token"
        );
    }

    pub(crate) fn assert_is_redeemable(&self, token_id: &TokenId) {
        let token = self.nft_token(token_id.clone()).expect("Token doesn't exist");
        let BoosterExtra::BalanceBooster(data) = token.get_extra();
        
        require!(data.is_redeemable, "Token is not redeemable");
    }
}

pub(crate) fn assert_enough_gas(required: Gas) {
    let remaining_gas = remaining_gas();
    require!(
        remaining_gas >= required,
        format!("Not enough gas for further operations: {remaining_gas} gas left when {required} required"),
    );
}
