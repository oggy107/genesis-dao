#![cfg(test)]
#![allow(unused_imports)]

extern crate std;
use std::println;

use super::*;
use soroban_sdk::testutils::{Address as AddressTest, Ledger, Logs};
use soroban_sdk::{vec, Address, Env, IntoVal, String, Symbol, Val};

use mock::*;

mod dao_token_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/dao_token.wasm"
    );
}

fn create_dao<'a>(env: Env, members: &Vec<Address>) -> DaoContractClient<'a> {
    let contract_id = env.register_contract(None, DaoContract);
    let dao = DaoContractClient::new(&env, &contract_id);

    let wasm_hash = env
        .deployer()
        .upload_contract_wasm(dao_token_contract::WASM);

    dao.initialize(
        members,
        &wasm_hash,
        &dao::Metadata {
            min_proposal_duration: 3600_u64,   // 1day
            max_proposal_duration: 604800_u64, // 7 days
            min_quorum_percentage: 50_u32,     // 50%
        },
    );

    dao
}

fn get_initial_members(env: &Env) -> Vec<Address> {
    vec![
        env,
        Address::generate(env),
        Address::generate(env),
        Address::generate(env),
    ]
}

#[test]
fn initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let members = dao.get_members();

    assert_eq!(members, initial_members);
    // env.logs().print();
}

#[test]
fn has_voting_power() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let a = Address::generate(&env);
    let b = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    assert!(!dao.has_voting_power(&a));
    assert!(!dao.has_voting_power(&b));
    assert!(dao.has_voting_power(&initial_members.get(0).unwrap()));
}

#[test]
fn add_member_proposal() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    // env.logs().print();
}

#[test]
#[should_panic(expected = "Only members can create proposals")]
fn add_member_proposal_not_member() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &a,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );
}

#[test]
#[should_panic(expected = "End time must be after start time")]
fn add_member_proposal_invalid_time() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        Some(10_u64),
        Some(5_u64),
        types::proposal::Type::AddMember,
    );
}

#[test]
#[should_panic(expected = "Proposal duration exceeds max_proposal_duration")]
fn add_member_proposal_invalid_max_duration() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        Some(10_u64),
        Some(1000000_u64),
        types::proposal::Type::AddMember,
    );
}

#[test]
#[should_panic(expected = "Proposal duration is less than min_proposal_duration")]
fn add_member_proposal_invalid_min_duration() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        Some(10_u64),
        Some(1000_u64),
        types::proposal::Type::AddMember,
    );
}

#[test]
fn proposal_vote() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let add_member_proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    let remove_member_proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &member,
        None,
        None,
        types::proposal::Type::RemoveMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5400;
    });

    dao.vote_proposal(&member, &add_member_proposal_id);
    dao.vote_proposal(&member, &remove_member_proposal_id);

    // env.logs().print();
}

#[test]
#[should_panic(expected = "Only members can vote")]
fn proposal_vote_only_member() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    dao.vote_proposal(&a, &proposal_id);

    // env.logs().print();
}

// TODO: add test for no voting power

#[test]
#[should_panic(expected = "Voter has already voted")]
fn proposal_vote_already_voted() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    dao.vote_proposal(&initial_members.get(0).unwrap(), &proposal_id);
    dao.vote_proposal(&initial_members.get(0).unwrap(), &proposal_id);

    // env.logs().print();
}

#[test]
#[should_panic(expected = "Proposal has ended")]
fn proposal_vote_ended() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        None,
        Some(10200_u64),
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 11000;
    });

    dao.vote_proposal(&&initial_members.get(0).unwrap(), &proposal_id);

    // env.logs().print();
}

#[test]
#[should_panic(expected = "Proposal has not started")]
fn proposal_vote_not_started() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    // let proposal_id =
    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        Some(5100_u64),
        Some(12000_u64),
        types::proposal::Type::AddMember,
    );

    dao.vote_proposal(&&initial_members.get(0).unwrap(), &proposal_id);

    // env.logs().print();
}

#[test]
fn proposal_votes_get() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5300;
    });

    dao.vote_proposal(&initial_members.get(0).unwrap(), &proposal_id);

    let votes = dao.get_proposal_votes(&proposal_id);

    assert_eq!(votes, vec![&env, initial_members.get(0).unwrap()]);

    dao.vote_proposal(&initial_members.get(2).unwrap(), &proposal_id);

    let votes = dao.get_proposal_votes(&proposal_id);

    assert_eq!(
        votes,
        vec![
            &env,
            initial_members.get(0).unwrap(),
            initial_members.get(2).unwrap()
        ]
    );

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member,
        &member,
        Some(5400),
        None,
        types::proposal::Type::RemoveMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5500;
    });

    let votes = dao.get_proposal_votes(&proposal_id);

    assert_eq!(votes, Vec::new(&env));

    env.ledger().with_mut(|li| {
        li.timestamp = 5500;
    });

    dao.vote_proposal(&initial_members.get(0).unwrap(), &proposal_id);

    let votes = dao.get_proposal_votes(&proposal_id);

    assert_eq!(votes, vec![&env, initial_members.get(0).unwrap()]);

    dao.vote_proposal(&initial_members.get(1).unwrap(), &proposal_id);

    let votes = dao.get_proposal_votes(&proposal_id);

    assert_eq!(
        votes,
        vec![
            &env,
            initial_members.get(0).unwrap(),
            initial_members.get(1).unwrap()
        ]
    );
}

#[test]
fn add_member_proposal_evaluate() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 11000;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    let members = dao.get_members();
    assert_eq!(members, initial_members);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_add_member_proposal(&member_a, &proposal_id);

    let members = dao.get_members();

    let mut expected_members = initial_members.clone();
    expected_members.append(&vec![&env, a]);

    assert_eq!(members, expected_members);

    // env.logs().print();
}

#[test]
fn remove_member_proposal() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &member,
        &member,
        None,
        None,
        types::proposal::Type::RemoveMember,
    );
}

#[test]
fn remove_member_proposal_evaluate() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &member_b,
        None,
        None,
        types::proposal::Type::RemoveMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    let members = dao.get_members();
    assert_eq!(members, initial_members);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_remove_member_proposal(&member_a, &proposal_id);

    let members = dao.get_members();

    let mut expected_members = initial_members.clone();
    let index = expected_members.first_index_of(&member_b).unwrap();
    expected_members.remove(index);

    assert_eq!(members, expected_members);

    // env.logs().print();
}

#[test]
fn grant_voting_proposal() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 11000;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_add_member_proposal(&member_a, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 11000;
    });

    mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        Some(11100),
        None,
        types::proposal::Type::GrantVotingPower,
    );
}

#[test]
#[should_panic(expected = "Member does not exist")]
fn grant_voting_proposal_no_member() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &member,
        &a,
        None,
        None,
        types::proposal::Type::GrantVotingPower,
    );
}

#[test]
#[should_panic(expected = "Already has voting power")]
fn grant_voting_proposal_already_has() {
    let env = Env::default();
    env.mock_all_auths();

    let initial_members = get_initial_members(&env);
    let member = initial_members.get(0).unwrap();

    let dao = create_dao(env.clone(), &initial_members);

    mock_proposal(
        &env,
        &dao,
        &member,
        &member,
        None,
        None,
        types::proposal::Type::GrantVotingPower,
    );
}

#[test]
fn grant_voting_proposal_evaluate() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();
    let member_c = initial_members.get(2).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_add_member_proposal(&member_a, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        Some(5300),
        None,
        types::proposal::Type::GrantVotingPower,
    );

    let has_voting_power = dao.has_voting_power(&a);

    assert_eq!(has_voting_power, false);

    env.ledger().with_mut(|li| {
        li.timestamp = 5400;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);
    dao.vote_proposal(&member_c, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_grant_voting_proposal(&member_a, &proposal_id);

    let has_voting_power = dao.has_voting_power(&a);

    assert_eq!(has_voting_power, true);

    // env.logs().print();
}

#[test]
fn revoke_voting_proposal() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();
    let member_c = initial_members.get(2).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_add_member_proposal(&member_a, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        Some(5300),
        None,
        types::proposal::Type::GrantVotingPower,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5400;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);
    dao.vote_proposal(&member_c, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_grant_voting_proposal(&member_a, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 5400;
    });

    mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        Some(5500),
        None,
        types::proposal::Type::RevokeVotingPower,
    );
}

#[test]
#[should_panic(expected = "Already does not have voting power")]
fn revoke_voting_proposal_has_power() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_add_member_proposal(&member_a, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        Some(5300),
        None,
        types::proposal::Type::RevokeVotingPower,
    );
}

#[test]
fn revoke_voting_proposal_evaluate() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();
    let member_c = initial_members.get(2).unwrap();
    let a = Address::generate(&env);

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        None,
        None,
        types::proposal::Type::AddMember,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_add_member_proposal(&member_a, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        Some(5300),
        None,
        types::proposal::Type::GrantVotingPower,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5400;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);
    dao.vote_proposal(&member_c, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_grant_voting_proposal(&member_a, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 5400;
    });

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &a,
        Some(5500),
        None,
        types::proposal::Type::RevokeVotingPower,
    );

    env.ledger().with_mut(|li| {
        li.timestamp = 5600;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);
    dao.vote_proposal(&member_c, &proposal_id);

    let has_voting_power = dao.has_voting_power(&a);

    assert_eq!(has_voting_power, true);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_revoke_voting_proposal(&member_a, &proposal_id);

    let has_voting_power = dao.has_voting_power(&a);

    assert_eq!(has_voting_power, false);

    // env.logs().print();
}

#[test]
fn general_purpose_proposal() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &member_a,
        None,
        None,
        types::proposal::Type::GeneralPurpose,
    );

    let status = dao.get_proposal_status(&proposal_id);

    assert_eq!(status, types::proposal::Status::NotStarted);
}

#[test]
fn general_purpose_proposal_evaluate() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &member_a,
        None,
        None,
        types::proposal::Type::GeneralPurpose,
    );

    let status = dao.get_proposal_status(&proposal_id);

    assert_eq!(status, types::proposal::Status::NotStarted);

    env.ledger().with_mut(|li| {
        li.timestamp = 5200;
    });

    dao.vote_proposal(&member_a, &proposal_id);
    dao.vote_proposal(&member_b, &proposal_id);

    env.ledger().with_mut(|li| {
        li.timestamp = 27000;
    });

    dao.evaluate_general_proposal(&member_a, &proposal_id);

    let status = dao.get_proposal_status(&proposal_id);

    assert_eq!(status, types::proposal::Status::Accepted);
}

#[test]
fn read_proposals() {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let initial_members = get_initial_members(&env);
    let member_a = initial_members.get(0).unwrap();
    let member_b = initial_members.get(1).unwrap();

    let dao = create_dao(env.clone(), &initial_members);

    let proposal_id = mock_proposal(
        &env,
        &dao,
        &member_a,
        &member_b,
        None,
        None,
        types::proposal::Type::GeneralPurpose,
    );

    let proposal = dao.get_gneral_purpose_proposal(&proposal_id);

    assert_eq!(
        proposal,
        types::proposal::GeneralPurpose {
            metadata: mock_proposal_metadata(&env, &member_a, Some(5100), Some(26000)),
            actions: String::from_str(&env, "mock actions"),
        }
    );
}
