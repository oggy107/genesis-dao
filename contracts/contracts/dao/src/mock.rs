use super::*;

use soroban_sdk::{Address, Env, String};

pub fn mock_proposal_metadata(
    env: &Env,
    proposer: &Address,
    start_time: Option<u64>,
    end_time: Option<u64>,
) -> types::proposal::Metadata {
    types::proposal::Metadata {
        name: String::from_str(&env, "mock name"),
        description: String::from_str(&env, "mock description"),
        proposer: proposer.clone(),
        start_time: start_time.unwrap_or(5100_u64),
        end_time: end_time.unwrap_or(26000_u64),
    }
}

pub fn mock_proposal(
    env: &Env,
    dao: &DaoContractClient,
    proposer: &Address,
    candidate: &Address,
    start_time: Option<u64>,
    end_time: Option<u64>,
    purposal_type: types::proposal::Type,
) -> BytesN<32> {
    match purposal_type {
        types::proposal::Type::AddMember => dao.add_member_proposal(&types::proposal::AddMember {
            metadata: mock_proposal_metadata(env, proposer, start_time, end_time),
            candidate: candidate.clone(),
        }),
        types::proposal::Type::RemoveMember => {
            dao.remove_member_proposal(&types::proposal::RemoveMember {
                metadata: mock_proposal_metadata(env, proposer, start_time, end_time),
                candidate: candidate.clone(),
            })
        }
        types::proposal::Type::GrantVotingPower => {
            dao.grant_voting_proposal(&types::proposal::GrantVotingPower {
                metadata: mock_proposal_metadata(env, proposer, start_time, end_time),
                candidate: candidate.clone(),
            })
        }
        types::proposal::Type::RevokeVotingPower => {
            dao.revoke_voting_proposal(&types::proposal::RevokeVotingPower {
                metadata: mock_proposal_metadata(env, proposer, start_time, end_time),
                candidate: candidate.clone(),
            })
        }
        types::proposal::Type::GeneralPurpose => {
            dao.general_purpose_proposal(&types::proposal::GeneralPurpose {
                metadata: mock_proposal_metadata(env, proposer, start_time, end_time),
                actions: String::from_str(&env, "mock actions"),
            })
        }
    }
}
