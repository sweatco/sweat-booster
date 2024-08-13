use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::{json_types::{Base64VecU8, U128}, serde::{Deserialize, Serialize}, AccountId, NearSchema};
#[cfg(feature = "integration-api")]
use nitka::near_sdk;
use nitka_proc::make_integration_version;

#[cfg(feature = "integration-test")]
pub struct SweatBoosterContract<'a> {
    pub contract: &'a near_workspaces::Contract,
}

/// An API for initializing the contract.
#[make_integration_version]
pub trait InitApi {
    /// A method to initialize the contract.
    ///
    /// # Arguments
    ///
    /// * `ft_account_id` - An `AccountId` representing the account address of the fungible
    ///    token contract that this smart contract will interact with.
    /// * `oracle` - An `AccountId` representing the account that can perform sensitive operations.
    /// * `base_uri` - An `Option<String>` representing the URI for the gateway providing access to tokens media.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the contract.
    fn new(ft_account_id: AccountId, oracle: AccountId, base_uri: Option<String>) -> Self;
}
/// An API for configuring the contract.
#[make_integration_version]
pub trait ConfigApi {
    /// Sets the URI for the gateway providing access to tokens media.
    ///
    /// # Arguments
    ///
    /// * `base_uri` - A `String` representing the new URI to be set.
    ///
    /// # Panics
    ///
    /// Panics if called by someone other than the oracle.
    fn set_base_uri(&mut self, base_uri: String);
}

/// An API for managing authorization of oracles for sensitive operations in the smart contract.
///
/// This API allows managing of oracles, which are accounts authorized to perform
/// sensitive operations.
#[make_integration_version]
pub trait AuthApi {
    /// Adds an oracle to the smart contract.
    ///
    /// Registers an oracle identified by `account_id`, authorizing them for sensitive operations.
    /// This method is private and can only be called by the account where the contract is deployed.
    /// It will panic if an attempt is made to register the same oracle twice.
    ///
    /// # Arguments
    ///
    /// * `account_id` - An `AccountId` representing the oracle to be added.
    ///
    /// # Panics
    ///
    /// Panics if the oracle is already registered.
    fn add_oracle(&mut self, account_id: AccountId);

    /// Removes an oracle from the smart contract.
    ///
    /// Revokes authorization from an oracle identified by `account_id`. This method is private
    /// and can only be called by the account where the contract is deployed. It will panic
    /// if there is no registered oracle with the specified `account_id`.
    ///
    /// # Arguments
    ///
    /// * `account_id` - An `AccountId` representing the oracle to be removed.
    ///
    /// # Panics
    ///
    /// Panics if no oracle with the specified `account_id` is registered.
    /// Also panics on attempt to remove the only oracle.
    fn remove_oracle(&mut self, account_id: AccountId);

    /// Retrieves the list of registered oracles.
    ///
    /// Returns a vector of `AccountId`s representing the oracles currently authorized
    /// for sensitive operations.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<AccountId>` containing the account IDs of the registered oracles.
    fn get_oracles(&self) -> Vec<AccountId>;
}

/// An API for minting new booster tokens.
#[make_integration_version]
pub trait MintApi {
    /// Mints a new booster token.
    ///
    /// This method creates a new booster token and assigns it to the specified receiver.
    ///
    /// # Arguments
    ///
    /// * `receiver_id` - An `AccountId` representing the receiver of the new token.
    /// * `booster_type` - A `BoosterType` specifying the type of booster.
    ///
    /// # Returns
    ///
    /// Returns a `Token` representing the newly minted booster token.
    fn mint(&mut self, receiver_id: AccountId, booster_type: BoosterType) -> Token;
}

/// An API for redeeming booster tokens.
///
/// This trait provides a method to redeem booster tokens, i.e. get the effect of the booster.
#[make_integration_version]
pub trait RedeemApi {
    /// Redeems a booster token.
    ///
    /// This method redeems a booster token identified by the specified token ID.
    /// Once the token is redeemed, it's burned.
    ///
    /// # Arguments
    ///
    /// * `token_id` - A `TokenId` representing the booster token to be redeemed.
    ///
    /// # Returns
    ///
    /// Returns a `PromiseOrValue<U128>` representing the amount of redeemed tokens.
    /// If the redeem operation fails on `ft_transfer`, it returns 0.
    fn redeem(&mut self, token_id: TokenId) -> ::near_sdk::PromiseOrValue<U128>;
}

/// An API for burning booster tokens, i.e. moving it out of circulation.
#[make_integration_version]
pub trait BurnApi {
    /// Burns a booster token.
    ///
    /// This method burns a booster token identified by the specified token ID, belonging to the specified owner.
    /// It will panic if the token does not exist or if it does not belong to the owner.
    /// It removes all data associated with the token from the contract state.
    ///
    /// # Arguments
    ///
    /// * `owner_id` - An `AccountId` representing the owner of the booster token to be burned.
    /// * `token_id` - A `TokenId` representing the booster token to be burned.
    ///
    /// # Panics
    ///
    /// Panics if the token does not exist or if it does not belong to the owner.
    fn burn(&mut self, owner_id: AccountId, token_id: TokenId);
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, NearSchema)]
#[serde(crate = "near_sdk::serde")]
pub enum BoosterType {
    BalanceBooster(BalanceBoosterData),
}

/// Struct representing the data required to create a balance booster token.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, NearSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct BalanceBoosterData {
    /// A string representing the media associated with the balance booster.
    /// This can be either a full URL or a CID if a base URL is specified in the contract.
    pub media: String,

    /// SHA256 hash of content referenced by the `media` field.
    pub media_hash: Base64VecU8,

    /// The denomination of the balance booster.
    pub denomination: U128,
}
