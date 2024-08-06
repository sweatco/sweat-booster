use near_sdk::{env, Gas};

pub(crate) mod tests;
pub(crate) mod asserts;
pub(crate) mod ft_interface;
pub(crate) mod test_data;

pub(crate) fn remaining_gas() -> Gas {
    env::prepaid_gas().checked_sub(env::used_gas()).expect("Out of gas")
}