use std::collections::HashMap;

use near_contract_standards::non_fungible_token::{NonFungibleToken, NonFungibleTokenEnumeration, NonFungibleTokenResolver, Token, TokenId};
use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata};
use near_sdk::{AccountId, BorshStorageKey, env, ext_contract, Gas, near, NearToken, PanicOnDefault, Promise, PromiseError, PromiseOrValue, require, serde_json};
use near_sdk::collections::{LazyOption, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use serde_json::json;

use sweat_booster_model::api::{AuthApi, BoosterType, BurnApi, MintApi, RedeemApi};

use crate::BoosterExtra::BalanceBooster;

pub mod auth;
mod common;
pub mod mint;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    ft_account_id: AccountId,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    last_id: u128,
    oracles: UnorderedSet<AccountId>,
}

#[near(serializers = [borsh, json])]
pub enum BoosterExtra {
    BalanceBooster(BalanceBoosterExtra),
}

#[near]
impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> U128 {
        self.tokens.nft_total_supply()
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        self.tokens.nft_tokens(from_index, limit)
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        self.tokens.nft_supply_for_owner(account_id)
    }

    fn nft_tokens_for_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        self.tokens.nft_tokens_for_owner(account_id, from_index, limit)
    }
}

#[near]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().expect("No metadata found")
    }
}

#[near]
impl NonFungibleTokenResolver for Contract {
    fn nft_resolve_transfer(&mut self, previous_owner_id: AccountId, receiver_id: AccountId, token_id: TokenId, approved_account_ids: Option<HashMap<AccountId, u64>>) -> bool {
        self.tokens.nft_resolve_transfer(previous_owner_id, receiver_id, token_id, approved_account_ids)
    }
}

#[near]
impl NonFungibleTokenCore for Contract {
    #[payable]
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>) {
        self.tokens.nft_transfer(receiver_id, token_id, approval_id, memo)
    }

    #[payable]
    fn nft_transfer_call(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>, msg: String) -> PromiseOrValue<bool> {
        self.tokens.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        self.tokens.nft_token(token_id)
    }
}

#[near]
impl RedeemApi for Contract {
    fn redeem(&mut self, token_id: TokenId) -> PromiseOrValue<U128> {
        let account_id = env::predecessor_account_id();

        self.assert_owner(&account_id, &token_id);

        let token = self.tokens.nft_token(token_id.clone()).expect("Token not found");

        let BalanceBooster(mut extra) = token.get_extra();
        require!(extra.is_redeemable, "Redeem is in progress");

        extra.is_redeemable = false;
        let amount = extra.denomination;

        self.update_extra(token, BalanceBooster(extra));

        Promise::new(self.ft_account_id.clone()).ft_transfer(&account_id, amount, None).then(
            ext_self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(10))
                .on_redeem_transfer(token_id)
        ).into()
    }
}

#[near]
impl BurnApi for Contract {
    fn burn(&mut self, owner_id: AccountId, token_id: TokenId) {
        self.assert_oracle();
        self.assert_owner(&owner_id, &token_id);

        self.tokens.burn(token_id);
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new(ft_account_id: AccountId, oracle: AccountId, base_uri: Option<String>) -> Self {
        let contract_metadata = NFTContractMetadata {
            spec: "nft-2.0.0".to_string(),
            name: "Booster".to_string(),
            symbol: "BSTR".to_string(),
            icon: None,
            base_uri,
            reference: None,
            reference_hash: None,
        };
        let mut oracles = UnorderedSet::new(StorageKey::Oracles);
        oracles.insert(&oracle);

        Self {
            ft_account_id,
            tokens: NonFungibleToken::new(
                StorageKey::OwnerById,
                env::predecessor_account_id(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::ContractMetadata, Some(&contract_metadata)),
            last_id: 0,
            oracles,
        }
    }

    pub fn set_base_uri(&mut self, base_uri: String) {
        let mut metadata = self.metadata.get().expect("No metadata found");
        metadata.base_uri = Some(base_uri);
        self.metadata.replace(&metadata);
    }
}

#[near]
#[derive(BorshStorageKey)]
enum StorageKey {
    OwnerById,
    TokenMetadata,
    Enumeration,
    Approval,
    ContractMetadata,
    Oracles,
}

#[near(serializers = [borsh, json])]
pub struct BalanceBoosterExtra {
    denomination: u128,
    is_redeemable: bool,
}

const GAS_FOR_FT_TRANSFER: Gas = Gas::from_gas(15_000_000_000_000);

trait FtTransferPromise {
    fn ft_transfer(self, receiver_id: &AccountId, amount: u128, memo: Option<String>) -> Promise;
}

impl FtTransferPromise for Promise {
    fn ft_transfer(self, receiver_id: &AccountId, amount: u128, memo: Option<String>) -> Promise {
        let args = serde_json::to_vec(&json!({
            "receiver_id": receiver_id,
            "amount": amount.to_string(),
            "memo": memo.unwrap_or_default(),
        }))
            .expect("Failed to serialize arguments");

        self.function_call(
            "ft_transfer".to_string(),
            args,
            NearToken::from_yoctonear(1),
            GAS_FOR_FT_TRANSFER,
        )
    }
}

#[ext_contract(ext_self)]
trait Callbacks {
    fn on_redeem_transfer(&mut self, #[callback_result] result: Result<(), PromiseError>, token_id: TokenId) -> PromiseOrValue<U128>;
}

#[near]
impl Callbacks for Contract {
    #[private]
    fn on_redeem_transfer(&mut self, #[callback_result] result: Result<(), PromiseError>, token_id: TokenId) -> PromiseOrValue<U128> {
        if result.is_ok() {
            let metadata = self.tokens.token_metadata_by_id
                .as_mut()
                .and_then(|by_id| by_id.remove(&token_id)).unwrap();
            self.tokens.owner_by_id.remove(&token_id);

            let extra = metadata.extra.expect("Metadata doesn't contain extra");
            let extra = serde_json::from_str::<BalanceBoosterExtra>(extra.as_str()).expect("Failed to parse extra");

            return PromiseOrValue::Value(extra.denomination.into());
        }

        let token = self.tokens.nft_token(token_id.clone()).expect("Token not found");

        let BalanceBooster(mut extra) = token.get_extra();
        extra.is_redeemable = true;

        self.update_extra(token, BalanceBooster(extra));

        PromiseOrValue::Value(0.into())
    }
}

trait NonFungibleTokenBurn {
    fn burn(&mut self, token_id: TokenId);
}

impl NonFungibleTokenBurn for NonFungibleToken {
    fn burn(&mut self, token_id: TokenId) {
        let owner_id = self.owner_by_id.remove(&token_id).expect("Owner not found");

        if let Some(approvals_by_id) = &mut self.approvals_by_id {
            approvals_by_id.remove(&token_id);
        }
        if let Some(tokens_per_owner) = &mut self.tokens_per_owner {
            let mut u = tokens_per_owner.remove(&owner_id).unwrap();
            u.remove(&token_id);
        }
        if let Some(token_metadata_by_id) = &mut self.token_metadata_by_id {
            token_metadata_by_id.remove(&token_id);
        }
    }
}

impl Contract {
    fn to_balance_booster_token(&self, booster_type: BoosterType) -> TokenMetadata {
        let BoosterType::BalanceBooster(data) = booster_type;

        let issued_at = env::block_timestamp_ms();
        let denomination_quot = data.denomination.0 / u128::pow(10, 18);
        let denomination_rem = data.denomination.0 % u128::pow(10, 18);

        TokenMetadata {
            title: Some(format!("Voucher #{}", self.last_id)),
            description: Some(format!("{denomination_quot}.{denomination_rem} $SWEAT voucher").to_string()),
            media: Some(data.media),
            media_hash: Some(data.media_hash),
            copies: Some(1),
            issued_at: Some(issued_at.to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: Some(
                serde_json::to_string(&BalanceBooster(BalanceBoosterExtra {
                    denomination: data.denomination.0,
                    is_redeemable: true,
                })).unwrap()
            ),
            reference: None,
            reference_hash: None,
        }
    }

    fn update_extra(&mut self, token: Token, extra: BoosterExtra) {
        let mut metadata = token.metadata.expect("Token doesn't contain metadata");
        metadata.extra = Some(serde_json::to_string(&extra).expect("Failed to serialize extra"));
        self.tokens.token_metadata_by_id.as_mut().unwrap().insert(&token.token_id, &metadata);
    }
}

trait ExtraExtractor {
    fn get_extra(&self) -> BoosterExtra;
}

impl ExtraExtractor for Token {
    fn get_extra(&self) -> BoosterExtra {
        let metadata = self.metadata.as_ref().expect("Token doesn't contain metadata");
        let extra = metadata.extra.as_ref().expect("Metadata doesn't contain extra");

        serde_json::from_str::<BoosterExtra>(extra.as_str()).expect("Failed to parse extra")
    }
}
