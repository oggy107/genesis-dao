use super::*;

/// Vote on a proposal
///
/// # Arguments
///
/// - `voter` - The voter who is voting.
/// - `proposal_id` - The id of the proposal.
pub fn vote(env: &Env, voter: Address, proposal_id: BytesN<32>) {
    only_member_vote(env, voter.clone());

    if !crate::has_voting_power(&env, voter.clone()) {
        panic!("Voter does not have voting power");
    }

    let proposal_metadata = metadata::proposal::read(env, proposal_id.clone());

    check_proposal(env, proposal_id.clone(), &proposal_metadata);

    write_vote(env, proposal_id.clone(), voter.clone());
    event::vote(env, proposal_id, voter);
}

/// Writes a vote to the proposal.
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
/// - `voter` - The voter who is voting.
fn write_vote(env: &Env, proposal_id: BytesN<32>, voter: Address) {
    let mut votes = read_votes(env, proposal_id.clone());

    if votes.contains(&voter) {
        panic!("Voter has already voted");
    }

    votes.push_back(voter.clone());

    env.storage()
        .persistent()
        .set(&data_key::Proposal::Vote(proposal_id), &votes);
}

/// Returns the votes of a proposal.
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
pub fn read_votes(env: &Env, proposal_id: BytesN<32>) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&data_key::Proposal::Vote(proposal_id))
        .unwrap_or(Vec::new(env))
}
