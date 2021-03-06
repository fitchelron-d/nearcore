use std::convert::TryFrom;
use std::sync::Arc;

use borsh::BorshDeserialize;

use near_chain_configs::GenesisConfig;
use near_primitives::borsh;
use near_primitives::hash::CryptoHash;
use near_primitives::sharding::ChunkHash;
use near_store::{DBCol, ShardTries, Store};

mod validate;

#[derive(Debug)]
pub struct ErrorMessage {
    pub col: Option<DBCol>,
    pub key: Option<String>,
    pub func: String,
    pub reason: String,
}

impl ErrorMessage {
    fn new(func: String, reason: String) -> Self {
        Self { col: None, key: None, func, reason }
    }
}

pub struct StoreValidator {
    config: GenesisConfig,
    shard_tries: ShardTries,
    store: Arc<Store>,

    pub errors: Vec<ErrorMessage>,
    tests: u64,
}

impl StoreValidator {
    pub fn new(config: GenesisConfig, shard_tries: ShardTries, store: Arc<Store>) -> Self {
        StoreValidator {
            config,
            shard_tries: shard_tries.clone(),
            store: store.clone(),
            errors: vec![],
            tests: 0,
        }
    }
    pub fn is_failed(&self) -> bool {
        self.tests == 0 || self.errors.len() > 0
    }
    pub fn num_failed(&self) -> u64 {
        self.errors.len() as u64
    }
    pub fn tests_done(&self) -> u64 {
        self.tests
    }
    fn col_to_key(col: DBCol, key: &[u8]) -> String {
        match col {
            DBCol::ColBlockHeader | DBCol::ColBlock => {
                format!("{:?}", CryptoHash::try_from(key.as_ref()))
            }
            DBCol::ColChunks => format!("{:?}", ChunkHash::try_from_slice(key.as_ref())),
            _ => format!("{:?}", key),
        }
    }
    pub fn validate(&mut self) {
        self.check(&validate::nothing, &[0], &[0], DBCol::ColBlockMisc);
        for (key, value) in self.store.clone().iter(DBCol::ColBlockHeader) {
            // Block Header Hash is valid
            self.check(&validate::block_header_validity, &key, &value, DBCol::ColBlockHeader);
        }
        for (key, value) in self.store.clone().iter(DBCol::ColBlock) {
            // Block Hash is valid
            self.check(&validate::block_hash_validity, &key, &value, DBCol::ColBlock);
            // Block Header for current Block exists
            self.check(&validate::block_header_exists, &key, &value, DBCol::ColBlock);
            // Block Height is greater or equal to tail, or to Genesis Height
            self.check(&validate::block_height_cmp_tail, &key, &value, DBCol::ColBlock);
        }
        for (key, value) in self.store.clone().iter(DBCol::ColChunks) {
            // Chunk Hash is valid
            self.check(&validate::chunk_hash_validity, &key, &value, DBCol::ColChunks);
            // Block for current Chunk exists
            self.check(&validate::block_of_chunk_exists, &key, &value, DBCol::ColChunks);
            // There is a State Root in the Trie
            self.check(&validate::chunks_state_roots_in_trie, &key, &value, DBCol::ColChunks);
        }
    }

    fn check(
        &mut self,
        f: &dyn Fn(&StoreValidator, &[u8], &[u8]) -> Result<(), ErrorMessage>,
        key: &[u8],
        value: &[u8],
        col: DBCol,
    ) {
        let result = f(self, key, value);
        self.tests += 1;
        match result {
            Ok(_) => {}
            Err(e) => {
                let mut e = e;
                e.col = Some(col);
                e.key = Some(Self::col_to_key(col, key));
                self.errors.push(e)
            }
        }
    }
}
