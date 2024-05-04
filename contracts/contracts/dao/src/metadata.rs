use crate::types;
use crate::types::*;
use soroban_sdk::{BytesN, Env};

/// Module for DAO metadata.
pub mod dao {
    use super::*;

    /// Writes the DAO metadata.
    ///
    /// # Arguments
    ///
    /// - `metadata` - The metadata to write.
    pub fn write(env: &Env, metadata: types::dao::Metadata) {
        env.storage()
            .persistent()
            .set(&data_key::Dao::Metadata, &metadata);
    }

    /// Reads the DAO metadata.
    pub fn read(env: &Env) -> types::dao::Metadata {
        env.storage()
            .persistent()
            .get(&data_key::Dao::Metadata)
            .unwrap()
    }
}

/// Module for proposal metadata.
pub mod proposal {
    use super::*;

    /// Writes the proposal metadata.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    /// - `metadata` - The metadata to write.
    pub fn write(env: &Env, proposal_id: BytesN<32>, metadata: types::proposal::Metadata) {
        env.storage()
            .persistent()
            .set(&data_key::Proposal::Metadata(proposal_id), &metadata);
    }

    /// Reads the proposal metadata.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn read(env: &Env, proposal_id: BytesN<32>) -> types::proposal::Metadata {
        env.storage()
            .persistent()
            .get(&data_key::Proposal::Metadata(proposal_id))
            .unwrap()
    }
}
