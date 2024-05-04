use soroban_sdk::{Address, Env, Symbol};

pub fn burn(env: &Env, from: Address, amount: i128) {
    let topics = (Symbol::new(env, "burn"), from);
    env.events().publish(topics, amount);
}

pub fn mint(env: &Env, to: Address, amount: i128) {
    let topics = (Symbol::new(env, "mint"), to);
    env.events().publish(topics, amount);
}
