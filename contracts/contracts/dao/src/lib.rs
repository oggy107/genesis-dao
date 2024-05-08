#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, IntoVal, String, Symbol, Vec};

pub mod types;
pub use types::*;

pub mod metadata;

pub mod member;
pub use member::*;

mod mock;

mod event;

pub mod proposal;

mod dao_token;

#[contract]
pub struct DaoContract;

#[contractimpl]
impl DaoContract {
    /// Initializes the DAO.
    ///
    /// This function should only be called once.
    ///
    /// # Arguments
    ///
    /// - `initial_members` - The initial members of the DAO. Must be atleast 3.
    /// - `dao_token_wasm_hash` - The WASM hash of the DAO token contract.
    /// - `metadata` - The metadata for the DAO.
    pub fn initialize(
        env: Env,
        initial_members: Vec<Address>,
        dao_token_wasm_hash: BytesN<32>,
        metadata: dao::Metadata,
    ) {
        if initial_members.len() < 3 {
            panic!("Initial members must be atleast 3");
        }

        let dao_token_contract_id = deploy_dao_token_contract(&env, dao_token_wasm_hash);

        env.storage()
            .persistent()
            .set(&data_key::Dao::TokenContractId, &dao_token_contract_id);

        metadata::dao::write(&env, metadata);

        add_members(&env, initial_members.clone());

        for member in initial_members {
            proposal::grant_voting_power::grant(&env, member.clone());
            event::add_member(&env, member);
        }
    }

    /// Returns the members of the DAO.
    pub fn get_members(env: Env) -> Vec<Address> {
        read_members(&env)
    }

    /// Checks if a member has voting power.
    ///
    /// # Arguments
    ///
    /// - `member` - The member to check.
    pub fn has_voting_power(env: &Env, member: Address) -> bool {
        has_voting_power(env, member)
    }

    /// Returns the status of a proposal.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn get_proposal_status(env: Env, proposal_id: BytesN<32>) -> types::proposal::Status {
        proposal::read_status(&env, proposal_id)
    }

    /// Votes on a proposal.
    ///
    /// # Arguments
    ///
    /// - `voter` - The voter to vote on the proposal.
    /// - `proposal_id` - The id of the proposal.
    pub fn vote_proposal(env: Env, voter: Address, proposal_id: BytesN<32>) {
        voter.require_auth();

        proposal::vote(&env, voter, proposal_id);
    }

    /// Returns the votes of a proposal.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn get_proposal_votes(env: Env, proposal_id: BytesN<32>) -> Vec<Address> {
        proposal::read_votes(&env, proposal_id)
    }

    /// Proposal to add a member to the DAO.
    ///
    /// # Arguments
    ///
    /// - `proposal` - The proposal.
    pub fn add_member_proposal(env: Env, proposal: types::proposal::AddMember) -> BytesN<32> {
        proposal.metadata.proposer.require_auth();

        proposal::add_member::write(&env, proposal.clone())
    }

    /// Returns the proposal to add a member to the DAO.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn get_add_member_proposal(
        env: Env,
        proposal_id: BytesN<32>,
    ) -> types::proposal::AddMember {
        proposal::add_member::read(&env, proposal_id)
    }

    /// Evaluates the proposal to add a member to the DAO.
    /// if `quorum` has been reached then the member is added to the DAO.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn evaluate_add_member_proposal(env: Env, evaluator: Address, proposal_id: BytesN<32>) {
        if !is_member(&env, evaluator.clone()) {
            panic!("Evaluator is not a member");
        }

        event::evaluate(&env, proposal_id.clone(), evaluator);

        proposal::add_member::evaluate(&env, proposal_id);
    }

    /// Proposal to remove a member from the DAO.
    ///
    /// # Arguments
    ///
    /// - `proposal` - The proposal.
    pub fn remove_member_proposal(env: Env, proposal: types::proposal::RemoveMember) -> BytesN<32> {
        proposal.metadata.proposer.require_auth();

        proposal::remove_member::write(&env, proposal.clone())
    }

    /// Returns the proposal to remove a member from the DAO.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn get_remove_member_proposal(
        env: Env,
        proposal_id: BytesN<32>,
    ) -> types::proposal::RemoveMember {
        proposal::remove_member::read(&env, proposal_id)
    }

    /// Evaluates the proposal to remove a member from the DAO.
    /// if `quorum` has been reached then the member is removed from the DAO.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn evaluate_remove_member_proposal(env: Env, evaluator: Address, proposal_id: BytesN<32>) {
        if !is_member(&env, evaluator.clone()) {
            panic!("Evaluator is not a member");
        }

        event::evaluate(&env, proposal_id.clone(), evaluator);

        proposal::remove_member::evaluate(&env, proposal_id);
    }

    /// Proposal to grant voting power to a member.
    ///
    /// # Arguments
    ///
    /// - `proposal` - The proposal.
    pub fn grant_voting_proposal(
        env: Env,
        proposal: types::proposal::GrantVotingPower,
    ) -> BytesN<32> {
        proposal.metadata.proposer.require_auth();

        if has_voting_power(&env, proposal.candidate.clone()) {
            panic!("Already has voting power");
        }

        proposal::grant_voting_power::write(&env, proposal.clone())
    }

    /// Returns the proposal to grant voting power to a member.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn get_grant_voting_proposal(
        env: Env,
        proposal_id: BytesN<32>,
    ) -> types::proposal::GrantVotingPower {
        proposal::grant_voting_power::read(&env, proposal_id)
    }

    /// Evaluates the proposal to grant voting power to a member.
    /// if `quorum` has been reached then the member is granted voting power.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn evaluate_grant_voting_proposal(env: Env, evaluator: Address, proposal_id: BytesN<32>) {
        if !is_member(&env, evaluator.clone()) {
            panic!("Evaluator is not a member");
        }

        event::evaluate(&env, proposal_id.clone(), evaluator);

        proposal::grant_voting_power::evaluate(&env, proposal_id);
    }

    /// Proposal to revoke voting power to a member.
    ///
    /// # Arguments
    ///
    /// - `proposal` - The proposal.
    pub fn revoke_voting_proposal(
        env: Env,
        proposal: types::proposal::RevokeVotingPower,
    ) -> BytesN<32> {
        proposal.metadata.proposer.require_auth();

        if !has_voting_power(&env, proposal.candidate.clone()) {
            panic!("Already does not have voting power");
        }

        proposal::revoke_voting_power::write(&env, proposal.clone())
    }

    /// Returns the proposal to revoke voting power to a member.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn get_revoke_voting_proposal(
        env: Env,
        proposal_id: BytesN<32>,
    ) -> types::proposal::RevokeVotingPower {
        proposal::revoke_voting_power::read(&env, proposal_id)
    }

    /// Evaluates the proposal to revoke voting power to a member.
    /// if `quorum` has been reached then the member is revoked voting power.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn evaluate_revoke_voting_proposal(env: Env, evaluator: Address, proposal_id: BytesN<32>) {
        if !is_member(&env, evaluator.clone()) {
            panic!("Evaluator is not a member");
        }

        event::evaluate(&env, proposal_id.clone(), evaluator);

        proposal::revoke_voting_power::evaluate(&env, proposal_id);
    }

    /// Proposal to create a general purpose proposal.
    ///
    /// # Arguments
    ///
    /// - `proposal` - The proposal.
    pub fn general_purpose_proposal(
        env: Env,
        proposal: types::proposal::GeneralPurpose,
    ) -> BytesN<32> {
        proposal.metadata.proposer.require_auth();

        proposal::general_purpose::write(&env, proposal.clone())
    }

    /// Returns the proposal to create a general purpose proposal.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn get_gneral_purpose_proposal(
        env: Env,
        proposal_id: BytesN<32>,
    ) -> types::proposal::GeneralPurpose {
        proposal::general_purpose::read(&env, proposal_id)
    }

    /// Evaluates the proposal to create a general purpose proposal.
    ///
    /// # Arguments
    ///
    /// - `proposal_id` - The id of the proposal.
    pub fn evaluate_general_proposal(env: Env, evaluator: Address, proposal_id: BytesN<32>) {
        if !is_member(&env, evaluator.clone()) {
            panic!("Evaluator is not a member");
        }

        event::evaluate(&env, proposal_id.clone(), evaluator);

        proposal::general_purpose::evaluate(&env, proposal_id);
    }
}

fn has_voting_power(env: &Env, member: Address) -> bool {
    dao_token::balance(env, member) > 0
}

fn deploy_dao_token_contract(env: &Env, wasm_hash: BytesN<32>) -> Address {
    let deployer = env.current_contract_address();

    let name = String::from_str(&env, "VoteToken");
    let symbol = String::from_str(&env, "VTK");

    let deployed_address = env
        .deployer()
        .with_address(deployer.clone(), BytesN::from_array(&env, &[0_u8; 32]))
        .deploy(wasm_hash);

    env.invoke_contract::<()>(
        &deployed_address,
        &Symbol::new(&env, "initialize"),
        // args.init_fn_args.clone(),
        (deployer, name, symbol).into_val(env),
    );

    deployed_address
}

mod test;
