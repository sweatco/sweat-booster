use near_sdk::near;

#[near(serializers = [borsh, json])]
pub enum BoosterExtra {
    BalanceBooster(BalanceBoosterData),
}

#[near(serializers = [borsh, json])]
pub struct BalanceBoosterData {
    pub denomination: u128,
    pub is_redeemable: bool,
}
