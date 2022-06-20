use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::UnorderedMap;
use near_sdk::{AccountId, Balance};

use crate::balances::VersionedBalances;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractV0 {
    pub funders: UnorderedMap<AccountId, Balance>,
    pub hashes: HashMap<String, VersionedBalances>,
}

impl Default for ContractV0 {
    fn default() -> Self {
        Self {
            funders: UnorderedMap::new(b"f"),
            hashes: HashMap::new(),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub funders: UnorderedMap<AccountId, Balance>,
    pub nonce: u64,
    pub hashes: HashMap<String, VersionedBalances>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            funders: UnorderedMap::new(b"f"),
            nonce: 0,
            hashes: HashMap::new(),
        }
    }
}
