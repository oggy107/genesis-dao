use crate::types::DataKey;
use soroban_sdk::{Address, Env};

pub fn write_balance(env: &Env, to: Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Balance(to), &amount);
}

pub fn read_balance(env: &Env, from: Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Balance(from))
        .unwrap_or(0)
}

pub fn burn_balance(env: &Env, from: Address, amount: i128) {
    let balance = read_balance(env, from.clone()) - amount;

    if balance < 0 {
        panic!("insufficient balance");
    }

    write_balance(env, from.clone(), balance);
}
