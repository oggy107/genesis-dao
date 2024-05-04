use super::*;

/// Create a new proposal to revoke voting power
///
/// # Arguments
///
/// - `proposal` - The proposal.
pub fn write(env: &Env, proposal: types::proposal::RevokeVotingPower) -> BytesN<32> {
    if !member::is_member(env, proposal.candidate.clone()) {
        panic!("Member does not exist");
    }

    only_member_proposal(env, proposal.metadata.proposer.clone());
    validate_proposal(env, &proposal.metadata);

    let proposal_id = generate_unique_id(&env);

    env.storage().persistent().set(
        &data_key::Proposal::RevokeVotingPower(proposal_id.clone()),
        &proposal,
    );

    metadata::proposal::write(&env, proposal_id.clone(), proposal.metadata.clone());

    update_status(env, proposal_id.clone(), &proposal.metadata);

    proposal_id
}

/// Read a revoke voting power proposal
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
pub fn read(env: &Env, proposal_id: BytesN<32>) -> types::proposal::RevokeVotingPower {
    if let Some(proposal) = env
        .storage()
        .persistent()
        .get(&data_key::Proposal::RevokeVotingPower(proposal_id))
    {
        proposal
    } else {
        panic!("Proposal not found");
    }
}

/// Revoke voting power
///
/// # Arguments
///
/// - `member` - The member who's voting power will be revoked.
pub fn revoke(env: &Env, member: Address) {
    dao_token::burn(env, member, 1_i128);
}

/// Evaluate a revoke voting power proposal
/// If the quorum is reached, the proposal will be accepted and the voting power will be revoked.
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
pub fn evaluate(env: &Env, proposal_id: BytesN<32>) {
    let status = read_status(env, proposal_id.clone());

    if status == types::proposal::Status::Accepted || status == types::proposal::Status::Rejected {
        panic!("Proposal already evaluated");
    }

    let proposal = read(env, proposal_id.clone());
    update_status(env, proposal_id.clone(), &proposal.metadata);

    if read_status(env, proposal_id.clone()) != types::proposal::Status::Ended {
        panic!("Proposal is not ended yet");
    }

    if is_quorum_reached(env, proposal_id.clone()) {
        write_status(env, proposal_id.clone(), types::proposal::Status::Accepted);

        revoke(env, proposal.candidate.clone());

        event::revoke_vote_power(env, proposal.candidate.clone());
    } else {
        write_status(env, proposal_id.clone(), types::proposal::Status::Rejected);
    }
}
