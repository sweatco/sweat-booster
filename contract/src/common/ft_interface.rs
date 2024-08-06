#![cfg(not(test))]

use near_sdk::{serde_json, AccountId, Gas, NearToken, Promise};
use serde_json::json;

pub(crate) const GAS_FOR_FT_TRANSFER: Gas = Gas::from_gas(15_000_000_000_000);

pub(crate) trait FtTransfer {
    fn ft_transfer(self, receiver_id: &AccountId, amount: u128, memo: Option<String>) -> Promise;
}

impl FtTransfer for Promise {
    fn ft_transfer(self, receiver_id: &AccountId, amount: u128, memo: Option<String>) -> Promise {
        let args = serde_json::to_vec(&json!({
            "receiver_id": receiver_id,
            "amount": amount.to_string(),
            "memo": memo.unwrap_or_default(),
        })).expect("Failed to serialize arguments");

        self.function_call(
            "ft_transfer".to_string(),
            args,
            NearToken::from_yoctonear(1),
            GAS_FOR_FT_TRANSFER,
        )
    }
}