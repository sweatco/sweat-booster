use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{env, env::log_str, json_types::U128, near, serde_json};

pub const PACKAGE_NAME: &str = "sweat_booster";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[near(serializers = [json])]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum EventKind {
    Redeem(RedeemData),
    Burn(BurnData),
}

#[near(serializers = [json])]
pub struct RedeemData {
    pub token_id: TokenId,
    pub denomination: U128,
}

#[near(serializers = [json])]
pub struct BurnData {
    pub token_id: TokenId,
    pub denomination: U128,
}

#[near(serializers = [json])]
#[serde(rename_all = "snake_case")]
struct Event {
    standard: &'static str,
    version: &'static str,
    #[serde(flatten)]
    event_kind: EventKind,
}

impl From<EventKind> for Event {
    fn from(event_kind: EventKind) -> Self {
        Self {
            standard: PACKAGE_NAME,
            version: VERSION,
            event_kind,
        }
    }
}

pub fn emit(event: EventKind) {
    log_str(Event::from(event).to_json_event_string().as_str());
}

impl Event {
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| env::panic_str(&format!("Failed to serialize Event: {err}")))
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }
}
