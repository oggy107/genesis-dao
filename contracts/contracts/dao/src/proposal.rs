/// The DAO's proposal module.
///
/// This module contains functions for managing proposals.
use crate::dao_token;
use crate::event;
use crate::member;
use crate::metadata;
use crate::types;
use crate::types::*;
use soroban_sdk::{Address, Bytes, BytesN, Env, Vec};

pub mod add_member;
pub mod general_purpose;
pub mod grant_voting_power;
pub mod remove_member;
pub mod revoke_voting_power;

pub mod vote;
pub use vote::*;

pub mod status;
pub use status::*;

/// Returns true if the quorum is reached.
fn is_quorum_reached(env: &Env, proposal_id: BytesN<32>) -> bool {
    let total_votes = read_votes(env, proposal_id.clone()).len();

    let quorum_percentage = metadata::dao::read(env).min_quorum_percentage;

    let total_members = member::total(env);

    total_votes > (total_members * quorum_percentage) / 100
}

/// Validates the proposal according to DAO metadata. Panics if the proposal is invalid.
fn validate_proposal(env: &Env, proposal_metadata: &types::proposal::Metadata) {
    if proposal_metadata.end_time < proposal_metadata.start_time {
        panic!("End time must be after start time");
    }

    if proposal_metadata.start_time < env.ledger().timestamp() {
        panic!("Start time must be in the future");
    }

    let dao_metadata = metadata::dao::read(env);
    let proposal_duration = proposal_metadata.end_time - proposal_metadata.start_time;

    if proposal_duration > dao_metadata.max_proposal_duration {
        panic!("Proposal duration exceeds max_proposal_duration");
    }

    if proposal_duration < dao_metadata.min_proposal_duration {
        panic!("Proposal duration is less than min_proposal_duration");
    }
}

/// Checks if the proposal is still valid for actions such as voting. Panics if the proposal is invalid.
fn check_proposal(
    env: &Env,
    proposal_id: BytesN<32>,
    proposal_metadata: &types::proposal::Metadata,
) {
    if proposal_metadata.end_time < env.ledger().timestamp() {
        write_status(env, proposal_id.clone(), types::proposal::Status::Ended);
        panic!("Proposal has ended");
    } else if proposal_metadata.start_time > env.ledger().timestamp() {
        write_status(
            env,
            proposal_id.clone(),
            types::proposal::Status::NotStarted,
        );
        panic!("Proposal has not started");
    }
}

fn only_member_proposal(env: &Env, caller: Address) {
    if !member::is_member(&env, caller) {
        panic!("Only members can create proposals");
    }
}

fn only_member_vote(env: &Env, caller: Address) {
    if !member::is_member(&env, caller) {
        panic!("Only members can vote");
    }
}

fn generate_unique_id(env: &Env) -> BytesN<32> {
    let mut value = [0u8; 64];
    env.prng().fill(&mut value);

    env.crypto().sha256(&Bytes::from_slice(env, &value))
}
