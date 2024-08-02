use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
};
use std::sync::MutexGuard;
use std::time::Duration;
use near_sdk::{AccountId, NearToken, testing_env};
use near_sdk::test_utils::VMContextBuilder;

use crate::Contract;

pub const MS_IN_SECOND: u64 = 1000;
pub const MS_IN_MINUTE: u64 = MS_IN_SECOND * 60;
pub const MS_IN_HOUR: u64 = MS_IN_MINUTE * 60;
pub const MS_IN_DAY: u64 = MS_IN_HOUR * 24;
pub const MS_IN_YEAR: u64 = MS_IN_DAY * 365;

pub(crate) fn oracle() -> AccountId {
    account("oracle")
}

fn account(id: &str) -> AccountId {
    id.parse().unwrap()
}

pub(crate) struct Context {
    contract: Arc<Mutex<Contract>>,
    pub owner: AccountId,
    ft_contract_id: AccountId,
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
            owner,
            ft_contract_id,
            builder,
            contract: Arc::new(Mutex::new(contract)),
        }
    }

    pub(crate) fn contract(&self) -> MutexGuard<Contract> {
        self.contract.lock().unwrap()
    }

    pub(crate) fn set_block_timestamp_in_days(&mut self, days: u64) {
        self.set_block_timestamp(Duration::from_millis(days * MS_IN_DAY));
    }

    pub(crate) fn set_block_timestamp_in_minutes(&mut self, minutes: u64) {
        self.set_block_timestamp(Duration::from_millis(minutes * MS_IN_MINUTE));
    }

    pub(crate) fn set_block_timestamp_in_ms(&mut self, ms: u64) {
        self.set_block_timestamp(Duration::from_millis(ms));
    }

    pub(crate) fn set_block_timestamp(&mut self, duration: Duration) {
        self.builder.block_timestamp(duration.as_nanos() as u64);
        testing_env!(self.builder.build());
    }

    pub(crate) fn switch_account(&mut self, account_id: impl Borrow<AccountId>) {
        let account_id = account_id.borrow().clone();
        self.builder
            .predecessor_account_id(account_id.clone())
            .signer_account_id(account_id);
        testing_env!(self.builder.build());
    }

    pub(crate) fn switch_account_to_ft_contract_account(&mut self) {
        self.switch_account(&self.ft_contract_id.clone());
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
