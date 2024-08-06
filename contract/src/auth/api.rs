use near_sdk::{near, require, AccountId};
use sweat_booster_model::api::AuthApi;

use crate::{Contract, ContractExt};

#[near]
impl AuthApi for Contract {
    fn add_oracle(&mut self, account_id: AccountId) {
        self.assert_oracle();

        require!(self.oracles.insert(&account_id), "Oracle already registered");
    }

    fn remove_oracle(&mut self, account_id: AccountId) {
        self.assert_oracle();

        require!(
            self.oracles.len() > 1,
            "There must be at least one oracle left after removing an oracle"
        );
        require!(self.oracles.remove(&account_id), "Oracle is not registered");
    }

    fn get_oracles(&self) -> Vec<AccountId> {
        self.oracles.to_vec()
    }
}
