use super::*;

/// create a proposal to remove a member
///
/// # Arguments
///
/// - `proposal` - The proposal.
pub fn write(env: &Env, proposal: types::proposal::RemoveMember) -> BytesN<32> {
    if !member::is_member(env, proposal.candidate.clone()) {
        panic!("Member does not exist");
    }

    only_member_proposal(env, proposal.metadata.proposer.clone());
    validate_proposal(env, &proposal.metadata);

    let proposal_id = generate_unique_id(&env);

    env.storage().persistent().set(
        &data_key::Proposal::RemoveMember(proposal_id.clone()),
        &proposal,
    );

    metadata::proposal::write(&env, proposal_id.clone(), proposal.metadata.clone());

    update_status(env, proposal_id.clone(), &proposal.metadata);

    proposal_id
}

/// read a remove member proposal
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
pub fn read(env: &Env, proposal_id: BytesN<32>) -> types::proposal::RemoveMember {
    if let Some(proposal) = env
        .storage()
        .persistent()
        .get(&data_key::Proposal::RemoveMember(proposal_id))
    {
        proposal
    } else {
        panic!("Proposal not found");
    }
}

/// evaluate a remove member proposal
/// if the quorum is reached, the proposal will be accepted and the member will be removed
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

        let candidate = proposal.candidate.clone();

        member::remove_member(env, candidate.clone());
        event::remove_member(env, candidate);
    } else {
        write_status(env, proposal_id.clone(), types::proposal::Status::Rejected);
    }
}
