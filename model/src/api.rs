use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{AccountId, near, PromiseOrValue};
use near_sdk::json_types::{Base64VecU8, U128};
#[cfg(feature = "integration-api")]
use nitka::near_sdk;
use nitka_proc::make_integration_version;

#[cfg(feature = "integration-test")]
pub struct SweatBoosterContract<'a> {
    pub contract: &'a near_workspaces::Contract,
}

#[make_integration_version]
pub trait AuthApi {
    fn add_oracle(&mut self, account_id: AccountId);
}

#[make_integration_version]
pub trait MintApi {
    fn mint(&mut self, receiver_id: AccountId, booster_type: BoosterType) -> Token;
}

#[make_integration_version]
pub trait RedeemApi {
    fn redeem(&mut self, token_id: TokenId) -> PromiseOrValue<U128>;
}

#[make_integration_version]
pub trait BurnApi {
    fn burn(&mut self, owner_id: AccountId, token_id: TokenId);
}

#[near(serializers = [json])]
pub enum BoosterType {
    BalanceBooster(BalanceBoosterData)
}

#[near(serializers = [json])]
pub struct BalanceBoosterData {
    pub media_cid: String,
    pub media_hash: Base64VecU8,
    pub denomination: U128,
}