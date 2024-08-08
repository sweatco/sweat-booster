use near_contract_standards::non_fungible_token::{metadata::NFTContractMetadata, NonFungibleToken};
use near_sdk::{
    collections::{LazyOption, UnorderedSet},
    env, near, AccountId, BorshStorageKey, PanicOnDefault,
};

pub mod auth;
pub mod burn;
pub mod common;
pub mod config;
pub mod init;
pub mod mint;
pub mod redeem;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    /// An `AccountId` representing the account address of the fungible
    /// token contract that this smart contract will interact with.
    ft_account_id: AccountId,

    /// Implementation of the non-fungible token standard.
    tokens: NonFungibleToken,

    /// Metadata for the NFT contract.
    metadata: LazyOption<NFTContractMetadata>,

    /// A counter providing incremental IDs for NFTs.
    last_id: u128,

    /// IDs of accounts authorized to perform sensitive operations on the contract.
    oracles: UnorderedSet<AccountId>,
}
