/// Module for modifying DAO members.
use soroban_sdk::{vec, Address, Env, Vec};

use crate::types::data_key;

/// Adds members to the DAO.
///
/// # Arguments
///
/// - `members` - The members to add.
pub fn add_members(env: &Env, members: Vec<Address>) {
    for member in members.clone() {
        if is_member(env, member) {
            panic!("Member already exists");
        }
    }

    let mut previous_members = read_members(env);

    previous_members.append(&members);

    env.storage()
        .persistent()
        .set(&data_key::Dao::Members, &previous_members);
}

/// Adds a single member to the DAO.
///
/// # Arguments
///
/// - `member` - The member to add.
pub fn add_member(env: &Env, member: Address) {
    add_members(env, vec![env, member]);
}

/// Removes members from the DAO.
///
/// # Arguments
///
/// - `members` - The members to remove.
pub fn remove_member(env: &Env, member: Address) {
    if !is_member(env, member.clone()) {
        panic!("Member does not exist");
    }

    let mut previous_members = read_members(env);

    let index = previous_members.first_index_of(&member).unwrap();

    previous_members.remove(index);

    env.storage()
        .persistent()
        .set(&data_key::Dao::Members, &previous_members);
}

/// Reads the members of the DAO.
pub fn read_members(env: &Env) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&data_key::Dao::Members)
        .unwrap_or(Vec::new(env))
}

/// Checks if a member exists in the DAO.
pub fn is_member(env: &Env, member: Address) -> bool {
    read_members(env).contains(&member)
}

/// Returns the total number of members in the DAO.
pub fn total(env: &Env) -> u32 {
    read_members(env).len()
}
