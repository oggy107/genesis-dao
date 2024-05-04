use super::*;

/// Write the status of a proposal
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
/// - `status` - The status of the proposal to write.
pub fn write_status(env: &Env, proposal_id: BytesN<32>, status: types::proposal::Status) {
    env.storage()
        .persistent()
        .set(&data_key::Proposal::Status(proposal_id), &status)
}

/// Read the status of a proposal
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
pub fn read_status(env: &Env, proposal_id: BytesN<32>) -> types::proposal::Status {
    env.storage()
        .persistent()
        .get(&data_key::Proposal::Status(proposal_id))
        .unwrap()
}

/// Update the status of a proposal according to the current time
///
/// # Arguments
///
/// - `proposal_id` - The id of the proposal.
/// - `proposal_metadata` - The metadata of the proposal.
pub fn update_status(
    env: &Env,
    proposal_id: BytesN<32>,
    proposal_metadata: &types::proposal::Metadata,
) {
    if proposal_metadata.end_time < env.ledger().timestamp() {
        write_status(env, proposal_id.clone(), types::proposal::Status::Ended);
    } else if proposal_metadata.start_time > env.ledger().timestamp() {
        write_status(
            env,
            proposal_id.clone(),
            types::proposal::Status::NotStarted,
        );
    } else {
        write_status(env, proposal_id.clone(), types::proposal::Status::Active);
    }
}
