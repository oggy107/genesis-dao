use soroban_sdk::{contracttype, Address, String};

#[contracttype]
pub enum DataKey {
    Initialized,
    Admin,
    Balance(Address),
}

#[contracttype]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
}
