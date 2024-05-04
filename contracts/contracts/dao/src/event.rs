/// Module for emitting events
use soroban_sdk::{Address, BytesN, Env, Symbol};

pub fn add_member(env: &Env, who: Address) {
    let topics = (Symbol::new(env, "add_member"), who.clone());
    env.events().publish(topics, who);
}

pub fn remove_member(env: &Env, who: Address) {
    let topics = (Symbol::new(env, "remove_member"), who.clone());
    env.events().publish(topics, who);
}

pub fn grant_vote_power(env: &Env, who: Address) {
    let topics = (Symbol::new(env, "grant_vote_power"), who.clone());
    env.events().publish(topics, who);
}

pub fn revoke_vote_power(env: &Env, who: Address) {
    let topics = (Symbol::new(env, "revoke_vote_power"), who.clone());
    env.events().publish(topics, who);
}

pub fn vote(env: &Env, proposal_id: BytesN<32>, voter: Address) {
    let topics = (Symbol::new(env, "vote"), proposal_id.clone());
    env.events().publish(topics, voter);
}

pub fn evaluate(env: &Env, proposal_id: BytesN<32>, evaluator: Address) {
    let topics = (Symbol::new(env, "evaluate"), evaluator);
    env.events().publish(topics, proposal_id);
}
