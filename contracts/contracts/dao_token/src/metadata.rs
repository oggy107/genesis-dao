use soroban_sdk::{symbol_short, Env, String, Symbol};

pub use crate::types::Metadata;

const METADATA: Symbol = symbol_short!("METADATA");

pub fn read_name(env: &Env) -> String {
    env.storage()
        .instance()
        .get::<Symbol, Metadata>(&METADATA)
        .unwrap()
        .name
}

pub fn read_symbol(env: &Env) -> String {
    env.storage()
        .instance()
        .get::<Symbol, Metadata>(&METADATA)
        .unwrap()
        .symbol
}

pub fn write_metadata(env: &Env, metadata: Metadata) {
    env.storage().instance().set(&METADATA, &metadata);
}
