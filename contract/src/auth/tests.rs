#![cfg(test)]

use near_sdk::test_utils::test_env::{alice, bob};
use sweat_booster_model::api::AuthApi;
use crate::common::tests::{Context, oracle};

#[test]
fn add_oracle_by_authorized_account() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(oracle.clone());
    context.contract().add_oracle(alice());

    let oracles = context.contract().get_oracles();
    assert_eq!(oracles.len(), 2);
    assert!(oracles.contains(&oracle));
    assert!(oracles.contains(&alice()));
}

#[test]
#[should_panic(expected = "Oracle already registered")]
fn add_oracle_by_authorized_account_twice() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(oracle.clone());
    context.contract().add_oracle(alice());
    context.contract().add_oracle(alice());
}

#[test]
#[should_panic(expected = "Only oracle can call this method")]
fn add_oracle_by_not_authorized_account() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(alice());
    context.contract().add_oracle(alice());
}

#[test]
fn remove_oracle_by_authorized_account() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(oracle.clone());
    context.contract().add_oracle(alice());

    let oracles = context.contract().get_oracles();
    assert!(oracles.contains(&alice()));

    context.contract().remove_oracle(alice());

    let oracles = context.contract().get_oracles();
    assert!(!oracles.contains(&alice()));
}

#[test]
#[should_panic(expected = "Only oracle can call this method")]
fn remove_oracle_by_not_authorized_account() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(alice());
    context.contract().remove_oracle(oracle);
}

#[test]
#[should_panic(expected = "There must be at least one oracle")]
fn remove_the_only_oracle() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(oracle.clone());
    context.contract().remove_oracle(oracle);
}

#[test]
#[should_panic(expected = "Oracle is not registered")]
fn remove_not_registered_oracle() {
    let oracle = oracle();
    let mut context = Context::new(&oracle);

    context.switch_account(oracle.clone());
    context.contract().add_oracle(alice());
    context.contract().remove_oracle(bob());
}
