use near_sdk::{near, BorshStorageKey};

#[near]
#[derive(BorshStorageKey)]
pub(crate) enum StorageKey {
    OwnerById,
    TokenMetadata,
    Enumeration,
    Approval,
    ContractMetadata,
    Oracles,
}
