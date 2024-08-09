#![cfg(test)]

use std::{
    borrow::Borrow,
    sync::{Arc, Mutex, MutexGuard},
};

use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId, NearToken};
use sweat_booster_model::api::InitApi;
use crate::Contract;

pub(crate) fn oracle() -> AccountId {
    account("oracle")
}

fn account(id: &str) -> AccountId {
    id.parse().unwrap()
}

pub(crate) struct Context {
    contract: Arc<Mutex<Contract>>,
    builder: VMContextBuilder,
}

impl Context {
    pub(crate) fn new(oracle: &AccountId) -> Self {
        let owner: AccountId = account("owner");
        let ft_contract_id: AccountId = account("token");

        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(owner.clone())
            .signer_account_id(owner.clone())
            .predecessor_account_id(owner.clone())
            .block_timestamp(0);

        testing_env!(builder.build());

        let contract = Contract::new(ft_contract_id.clone(), oracle.clone(), None);

        Self {
            builder,
            contract: Arc::new(Mutex::new(contract)),
        }
    }

    pub(crate) fn contract(&self) -> MutexGuard<Contract> {
        self.contract.lock().unwrap()
    }

    pub(crate) fn switch_account(&mut self, account_id: impl Borrow<AccountId>) {
        let account_id = account_id.borrow().clone();
        self.builder
            .predecessor_account_id(account_id.clone())
            .signer_account_id(account_id);
        testing_env!(self.builder.build());
    }

    pub(crate) fn with_deposit_yocto(&mut self, amount: u128, f: impl FnOnce(&mut Context)) {
        self.set_deposit_yocto(amount);

        f(self);

        self.set_deposit_yocto(0);
    }

    fn set_deposit_yocto(&mut self, amount: u128) {
        self.builder.attached_deposit(NearToken::from_yoctonear(amount));
        testing_env!(self.builder.build());
    }
}
