use super::*;

/// Create a new proposal for general purposes
///
/// # Arguments
///
/// - `proposal` - The proposal.
pub fn write(env: &Env, proposal: types::proposal::GeneralPurpose) -> BytesN<32> {
    only_member_proposal(env, proposal.metadata.proposer.clone());
    validate_proposal(env, &proposal.metadata);

    let proposal_id = generate_unique_id(&env);

    env.storage().persistent().set(
        &data_key::Proposal::GeneralPurpose(proposal_id.clone()),
        &proposal,
    );

    metadata::proposal::write(&env, proposal_id.clone(), proposal.metadata.clone());

    update_status(env, proposal_id.clone(), &proposal.metadata);

    proposal_id
}

/// Read a general purpose proposal
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
pub fn read(env: &Env, proposal_id: BytesN<32>) -> types::proposal::GeneralPurpose {
    if let Some(proposal) = env
        .storage()
        .persistent()
        .get(&data_key::Proposal::GeneralPurpose(proposal_id))
    {
        proposal
    } else {
        panic!("Proposal not found");
    }
}

/// Evaluate a general purpose proposal
/// Evaluation here is just setting the status.
/// Actions mentioned in the proposal are not executed on-chain rather the community is responsible for that.
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
    } else {
        write_status(env, proposal_id.clone(), types::proposal::Status::Rejected);
    }
}
