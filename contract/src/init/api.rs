use near_contract_standards::non_fungible_token::{metadata::NFTContractMetadata, NonFungibleToken};
use near_sdk::{
    collections::{LazyOption, UnorderedSet},
    env, near, AccountId,
};
use sweat_booster_model::api::InitApi;

use crate::{init::model::StorageKey, Contract, ContractExt};

#[near]
impl InitApi for Contract {
    #[init]
    fn new(ft_account_id: AccountId, oracle: AccountId, base_uri: Option<String>) -> Self {
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
}
