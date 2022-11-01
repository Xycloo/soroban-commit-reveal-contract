#![cfg(test)]

use crate::testutils::{register_test_contract, CommitRevealContract};
//use crate::token::{self, TokenMetadata};
use rand::{thread_rng, RngCore};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{
    serde::Serialize, testutils::Accounts, AccountId, Address, BigInt, Bytes, BytesN, Env, IntoVal,
};

fn generate_contract_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}
/*
fn create_token_contract(e: &Env, admin: &AccountId) -> ([u8; 32], token::Client) {
    let id = e.register_contract_token(None);
    let token = token::Client::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.init(
        &Identifier::Account(admin.clone()),
        &TokenMetadata {
            name: "USD coin".into_val(e),
            symbol: "USDC".into_val(e),
            decimals: 7,
        },
    );
    (id.into(), token)
}
*/

fn create_contract(
    e: &Env,
    admin: &AccountId,
    hash: BytesN<32>,
) -> ([u8; 32], CommitRevealContract) {
    let id = generate_contract_id();
    register_test_contract(&e, &id);
    let contract = CommitRevealContract::new(e, &id);
    /*

    contract initialization

     */

    contract.initialize(&hash);

    (id, contract)
}

#[test]
fn test() {
    let e: Env = Default::default();
    let admin = e.accounts().generate(); // token admin

    // two users for testing
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());

    //    let (contract1, usdc_token) = create_token_contract(&e, &admin1); // registered and initialized the usdc token contract

    let image = Bytes::from_slice(&e, "soroban is awesome".as_bytes());
    let hash = e.compute_hash_sha256(&image);

    let (contract_arr_id, contract) = create_contract(&e, &user1, hash);

    let contract_id = Identifier::Contract(BytesN::from_array(&e, &contract_arr_id));
    // the id of the contract

    let user1_address = Address::Account(user1.clone());
    let mut commit_image = Bytes::new(&e);
    match user1_address {
        Address::Account(a) => commit_image.append(&a.serialize(&e)),
        Address::Contract(a) => commit_image.append(&a.into()), // why not support contracts that play the game :-)
    }

    commit_image.append(&Bytes::from_slice(&e, "soroban is awesome".as_bytes()));
    commit_image.append(&Bytes::from_slice(&e, "mysecret".as_bytes()));

    let val = e.compute_hash_sha256(&commit_image);

    contract.commit(user1.clone(), &val);
    contract.check(
        user1,
        &Bytes::from_slice(&e, "soroban is awesome".as_bytes()),
        &Bytes::from_slice(&e, "mysecret".as_bytes()),
    )
}

#[test]
#[should_panic]
fn test_wrong_solution() {
    let e: Env = Default::default();
    let admin = e.accounts().generate(); // token admin

    // two users for testing
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());

    //    let (contract1, usdc_token) = create_token_contract(&e, &admin1); // registered and initialized the usdc token contract

    let image = Bytes::from_slice(&e, "soroban is awesome".as_bytes());
    let hash = e.compute_hash_sha256(&image);

    let (contract_arr_id, contract) = create_contract(&e, &user1, hash);

    let contract_id = Identifier::Contract(BytesN::from_array(&e, &contract_arr_id));
    // the id of the contract

    let user1_address = Address::Account(user1.clone());
    let mut commit_image = Bytes::new(&e);
    match user1_address {
        Address::Account(a) => commit_image.append(&a.serialize(&e)),
        Address::Contract(a) => commit_image.append(&a.into()), // why not support contracts that play the game :-)
    }

    commit_image.append(&Bytes::from_slice(&e, "soroban is not awesome".as_bytes()));
    commit_image.append(&Bytes::from_slice(&e, "mysecret".as_bytes()));

    let val = e.compute_hash_sha256(&commit_image);

    contract.commit(user1.clone(), &val);
    contract.check(
        user1,
        &Bytes::from_slice(&e, "soroban is not awesome".as_bytes()),
        &Bytes::from_slice(&e, "mysecret".as_bytes()),
    )
}

#[test]
#[should_panic]
fn test_front_run() {
    let e: Env = Default::default();
    let admin = e.accounts().generate(); // token admin

    // two users for testing
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());

    //    let (contract1, usdc_token) = create_token_contract(&e, &admin1); // registered and initialized the usdc token contract

    let image = Bytes::from_slice(&e, "soroban is awesome".as_bytes());
    let hash = e.compute_hash_sha256(&image);

    let (contract_arr_id, contract) = create_contract(&e, &user1, hash);

    let contract_id = Identifier::Contract(BytesN::from_array(&e, &contract_arr_id));
    // the id of the contract

    let user1_address = Address::Account(user1.clone());
    let mut commit_image = Bytes::new(&e);
    match user1_address {
        Address::Account(a) => commit_image.append(&a.serialize(&e)),
        Address::Contract(a) => commit_image.append(&a.into()), // why not support contracts that play the game :-)
    }

    commit_image.append(&Bytes::from_slice(&e, "soroban is not awesome".as_bytes()));
    commit_image.append(&Bytes::from_slice(&e, "mysecret".as_bytes()));

    let val = e.compute_hash_sha256(&commit_image);

    contract.commit(user1, &val);
    contract.check(
        user2,
        &Bytes::from_slice(&e, "soroban is not awesome".as_bytes()),
        &Bytes::from_slice(&e, "mysecret".as_bytes()),
    )
}
