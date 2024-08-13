use near_sdk::near;
use near_sdk::json_types::{Base64VecU8, U128};

#[near(serializers = [json])]
pub enum BoosterType {
    BalanceBooster(BalanceBoosterData),
}

/// Struct representing the data required to create a balance booster token.
#[near(serializers = [json])]
#[derive(Clone)]
pub struct BalanceBoosterData {
    /// A string representing the media associated with the balance booster.
    /// This can be either a full URL or a CID if a base URL is specified in the contract.
    pub media: String,

    /// SHA256 hash of content referenced by the `media` field.
    pub media_hash: Base64VecU8,

    /// The denomination of the balance booster.
    pub denomination: U128,
}
