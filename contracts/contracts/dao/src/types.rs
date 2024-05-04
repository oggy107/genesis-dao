use soroban_sdk::{contracttype, Address, BytesN, String, Symbol, Val, Vec};

pub mod data_key {
    use super::*;

    #[contracttype]
    pub enum Dao {
        Members,
        TokenContractId,
        Metadata,
    }

    #[contracttype]
    pub enum Proposal {
        Vote(BytesN<32>),
        AddMember(BytesN<32>),
        RemoveMember(BytesN<32>),
        GrantVotingPower(BytesN<32>),
        RevokeVotingPower(BytesN<32>),
        GeneralPurpose(BytesN<32>),
        Metadata(BytesN<32>),
        Status(BytesN<32>),
    }
}

pub mod dao {
    use super::*;

    #[contracttype]
    pub struct Metadata {
        pub min_proposal_duration: u64,
        pub max_proposal_duration: u64,
        pub min_quorum_percentage: u32,
    }

    #[contracttype]
    pub struct TokenContractArgs {
        pub salt: BytesN<32>,
        pub wasm_hash: BytesN<32>,
        pub init_fn: Symbol,
        pub init_fn_args: Vec<Val>,
    }
}

pub mod proposal {
    use super::*;

    pub enum Type {
        AddMember,
        RemoveMember,
        GrantVotingPower,
        RevokeVotingPower,
        GeneralPurpose,
    }

    #[contracttype]
    #[derive(Debug, PartialEq)]
    pub enum Status {
        Active,
        Ended,
        Accepted,
        Rejected,
        NotStarted,
    }

    #[contracttype]
    #[derive(Clone, Debug, PartialEq)]
    pub struct Metadata {
        pub name: String,
        pub description: String,
        pub proposer: Address,
        pub start_time: u64,
        pub end_time: u64,
    }

    #[contracttype]
    #[derive(Clone)]
    pub struct AddMember {
        pub metadata: Metadata,
        pub candidate: Address,
    }

    #[contracttype]
    #[derive(Clone)]
    pub struct RemoveMember {
        pub metadata: Metadata,
        pub candidate: Address,
    }

    #[contracttype]
    #[derive(Clone)]
    pub struct GrantVotingPower {
        pub metadata: Metadata,
        pub candidate: Address,
    }

    #[contracttype]
    #[derive(Clone)]
    pub struct RevokeVotingPower {
        pub metadata: Metadata,
        pub candidate: Address,
    }

    #[contracttype]
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeneralPurpose {
        pub metadata: Metadata,
        pub actions: String,
    }
}
