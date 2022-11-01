#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod test;
pub mod testutils;

use soroban_auth::Identifier;
use soroban_sdk::{
    bigint, contractimpl, contracttype, serde::Serialize, Address, BigInt, Bytes, BytesN, Env,
};

mod token {
    soroban_sdk::contractimport!(file = "./soroban_token_spec.wasm");
}

fn put_started(e: &Env, started: bool) {
    let key = DataKey::Started;
    e.data().set(key, started);
}

fn game_started(e: &Env) -> bool {
    let key = DataKey::Started;
    e.data().get(key).unwrap_or(Ok(false)).unwrap()
}

fn put_hash(e: &Env, hash: BytesN<32>) {
    let key = DataKey::Hash;
    e.data().set(key, hash);
}

fn get_hash(e: &Env) -> BytesN<32> {
    let key = DataKey::Hash;
    e.data().get(key).unwrap().unwrap()
}

fn store_commit(e: &Env, id: Address, val: BytesN<32>) {
    let key = DataKey::Commit(id);
    e.data().set(key, val);
}

fn get_commit(e: &Env, id: Address) -> BytesN<32> {
    let key = DataKey::Commit(id);
    e.data().get(key).unwrap().unwrap()
}

fn send_reward(e: &Env, to: Identifier) {
    let client = token::Client::new(
        e,
        BytesN::from_array(
            e,
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        ),
    );
    client.xfer(
        &soroban_auth::Signature::Invoker,
        &BigInt::zero(e),
        &to,
        &bigint!(e, 100),
    )
}

#[contracttype]
#[derive(Clone)]
/// Contract data keys
pub enum DataKey {
    Started,
    Hash,
    Commit(Address),
}

/// Contract trait
pub trait CommitRevealContractTrait {
    fn initialize(e: Env, hash: BytesN<32>);

    fn commit(e: Env, value: BytesN<32>);

    fn check(e: Env, guess: Bytes, secret: Bytes);
}

pub struct CommitRevealContract;

#[contractimpl]
impl CommitRevealContractTrait for CommitRevealContract {
    fn initialize(e: Env, hash: BytesN<32>) {
        if game_started(&e) {
            panic!("game already started")
        }

        put_hash(&e, hash);
        put_started(&e, true);
    }

    fn commit(e: Env, val: BytesN<32>) {
        if !game_started(&e) {
            panic!("game started yet")
        }

        store_commit(&e, e.invoker(), val);
    }

    fn check(e: Env, guess: Bytes, secret: Bytes) {
        let invoker = e.invoker();
        let invoker_id: Identifier;
        let commit = get_commit(&e, invoker.clone());

        let mut rhs = Bytes::new(&e);
        match invoker {
            Address::Account(a) => {
                rhs.append(&a.clone().serialize(&e));
                invoker_id = Identifier::Account(a)
            }
            Address::Contract(a) => {
                rhs.append(&a.clone().into());
                invoker_id = Identifier::Contract(a)
            } // why not support contracts that play the game :-)
        }

        rhs.append(&guess);
        rhs.append(&secret);
        let rhs_commit = e.compute_hash_sha256(&rhs);

        if commit != rhs_commit {
            panic!("params don't match the commitment")
        }

        if e.compute_hash_sha256(&guess) != get_hash(&e) {
            panic!("wrong solution")
        }

        send_reward(&e, invoker_id);
    }
}
