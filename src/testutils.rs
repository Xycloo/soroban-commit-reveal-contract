#![cfg(any(test, feature = "testutils"))]

use crate::CommitRevealContractClient;

use soroban_sdk::{AccountId, Bytes, BytesN, Env};

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    e.register_contract(&contract_id, crate::CommitRevealContract {});
}

pub struct CommitRevealContract {
    env: Env,
    contract_id: BytesN<32>,
}

impl CommitRevealContract {
    fn client(&self) -> CommitRevealContractClient {
        CommitRevealContractClient::new(&self.env, &self.contract_id)
    }

    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn initialize(&self, hash: &BytesN<32>) {
        self.client().initialize(hash)
    }

    pub fn commit(&self, user: AccountId, val: &BytesN<32>) {
        self.env.set_source_account(&user);
        self.client().commit(val)
    }

    pub fn check(&self, user: AccountId, guess: &Bytes, secret: &Bytes) {
        self.env.set_source_account(&user);
        self.client().check(guess, secret)
    }
}
