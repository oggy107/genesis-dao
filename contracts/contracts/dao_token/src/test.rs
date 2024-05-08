#![cfg(test)]
#![allow(unused_imports)]

extern crate std;
use std::println;

use super::*;
use soroban_sdk::testutils::{Address as AddressTest, Logs};
use soroban_sdk::{Address, Env, String};

fn create_token<'a>(env: &'a Env, admin: &'a Address) -> TokenContractClient<'a> {
    let name = String::from_str(&env, "MyToken");
    let symbol = String::from_str(&env, "MTK");

    let contract_id = env.register_contract(None, TokenContract);
    let token = TokenContractClient::new(&env, &contract_id);
    token.initialize(admin, &name, &symbol);

    token
}

#[test]
fn initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);

    create_token(&env, &admin);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn already_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);

    let token = create_token(&env, &admin);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test"),
        &String::from_str(&env, "TES"),
    );

    // env.logs().print();
}

#[test]
fn meta_data() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);

    let token = create_token(&env, &admin);

    let name = token.name();
    let symbol = token.symbol();

    assert_eq!(name, String::from_str(&env, "MyToken"));
    assert_eq!(symbol, String::from_str(&env, "MTK"));

    println!("metadata: {:?}, {:?}", name, symbol);
}

#[test]
fn mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let token = create_token(&env, &admin);

    let user_balance = token.balance(&user);

    assert_eq!(user_balance, 0_i128);

    token.mint(&user, &100_i128);

    let user_balance = token.balance(&user);

    assert_eq!(user_balance, 100_i128);
}

#[test]
fn burn() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let from = Address::generate(&env);

    let token = create_token(&env, &admin);

    token.mint(&from, &100_i128);

    let from_balance = token.balance(&from);

    assert_eq!(from_balance, 100_i128);

    token.burn(&from, &60_i128);

    let from_balance = token.balance(&from);

    assert_eq!(from_balance, 40_i128);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn burn_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let from = Address::generate(&env);

    let token = create_token(&env, &admin);

    token.mint(&from, &100_i128);

    let from_balance = token.balance(&from);

    assert_eq!(from_balance, 100_i128);

    token.burn(&from, &300_i128);
}
