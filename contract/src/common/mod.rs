use near_sdk::{env, Gas};

pub(crate) mod asserts;
pub(crate) mod ft_interface;
mod nft_interface;
pub(crate) mod test_data;
pub(crate) mod tests;

pub(crate) fn remaining_gas() -> Gas {
    env::prepaid_gas().checked_sub(env::used_gas()).expect("Out of gas")
}
