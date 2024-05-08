#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

mod metadata;
pub use metadata::*;

mod types;
pub use types::DataKey;

mod admin;
pub use admin::*;

mod balance;
pub use balance::*;

pub mod event;
pub use event as Event;

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Initializes the contract with `admin`.
    ///
    /// # Arguments
    ///
    /// - `admin` - The address of the admin.
    /// - `name` - The name of the token.
    /// - `symbol` - The symbol of the token.
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        if has_admin(&env) {
            panic!("Already initialized");
        }

        write_metadata(&env, Metadata { name, symbol });

        write_admin(&env, admin);
    }

    /// Returns the name for this token.
    pub fn name(env: Env) -> String {
        read_name(&env)
    }

    /// Returns the symbol for this token.
    pub fn symbol(env: Env) -> String {
        read_symbol(&env)
    }

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// - `id` - The address for which a balance is being queried. If the
    /// address has no existing balance, returns 0.
    pub fn balance(env: Env, id: Address) -> i128 {
        read_admin(&env).require_auth();

        read_balance(&env, id)
    }

    /// Mint `amount` to `to`.
    ///
    /// # Arguments
    ///
    /// - `to` - The address which will receive the minted tokens.
    /// - `amount` - The amount of tokens to be minted.
    pub fn mint(env: Env, to: Address, amount: i128) {
        check_non_negative(amount);
        read_admin(&env).require_auth();

        write_balance(&env, to.clone(), amount);
        Event::mint(&env, to, amount);
    }

    /// Burn `amount` from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens which will be
    /// burned from.
    /// - `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["burn", from: Address]`
    /// - data - `[amount: i128]`
    pub fn burn(env: Env, from: Address, amount: i128) {
        // from.require_auth();
        read_admin(&env).require_auth();

        check_non_negative(amount);

        burn_balance(&env, from.clone(), amount);
        Event::burn(&env, from, amount);
    }
}

fn check_non_negative(amount: i128) {
    if amount < 0 {
        panic!("Negative amount");
    }
}

mod test;
