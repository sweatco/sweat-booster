use near_contract_standards::non_fungible_token::{metadata::NFTContractMetadata, NonFungibleToken};
use near_sdk::{
    collections::{LazyOption, UnorderedSet},
    env, near, AccountId, BorshStorageKey, PanicOnDefault,
};

pub mod auth;
pub mod burn;
pub mod common;
pub mod mint;
pub mod redeem;
pub mod init;
pub mod config;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    ft_account_id: AccountId,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    last_id: u128,
    oracles: UnorderedSet<AccountId>,
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
