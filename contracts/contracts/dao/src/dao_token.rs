use crate::types::data_key;
use soroban_sdk::{Address, Env, IntoVal, Symbol};

fn get_dao_token_contract_id(env: &Env) -> Address {
    if let Some(dao_token_contract_id) = env
        .storage()
        .persistent()
        .get::<_, Address>(&data_key::Dao::TokenContractId)
    {
        dao_token_contract_id
    } else {
        panic!("dao_token_contract_id not found");
    }
}

/// Mints tokens for the DAO to grant voting power.
///
/// # Arguments
///
/// - `to` - The address to mint tokens to.
/// - `amount` - The amount of tokens to mint.
pub fn mint(env: &Env, to: Address, amount: i128) {
    let dao_token_contract_id = get_dao_token_contract_id(env);

    env.invoke_contract::<()>(
        &dao_token_contract_id,
        &Symbol::new(env, "mint"),
        (&to, amount).into_val(env),
    );
}

/// Burns tokens from the DAO to revoke voting power.
///
/// # Arguments
///
/// - `from` - The address to burn tokens from.
/// - `amount` - The amount of tokens to burn.
pub fn burn(env: &Env, from: Address, amount: i128) {
    let dao_token_contract_id = get_dao_token_contract_id(env);

    env.invoke_contract::<()>(
        &dao_token_contract_id,
        &Symbol::new(env, "burn"),
        (&from, amount).into_val(env),
    );
}

/// Returns the balance of the DAO token contract which grants voting power.
///
/// # Arguments
///
/// - `account` - The address to check the balance of.
pub fn balance(env: &Env, account: Address) -> i128 {
    let dao_token_contract_id = get_dao_token_contract_id(env);

    env.invoke_contract::<i128>(
        &dao_token_contract_id,
        &Symbol::new(env, "balance"),
        (&account,).into_val(env),
    )
}
