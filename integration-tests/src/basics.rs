use near_contract_standards::non_fungible_token::Token;
use near_sdk::{serde_json, NearToken};
use nitka::{
    json,
    misc::ToNear,
    near_sdk::json_types::{Base64VecU8, U128},
};
use sweat_booster_model::{
    api::{MintApiIntegration, RedeemApiIntegration},
    model::BoosterExtra,
};
use sweat_booster_model::api::{BurnApiIntegration, SweatBoosterContract};
use crate::common::{prepare_contract, ContextHelpers, IntegrationContext};


