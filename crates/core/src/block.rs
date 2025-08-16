use crate::hash::{sha256, Hash};
use crate::merkle::merkle_root;
use crate::tx::Tx;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub prev: Hash,
    pub merkle: Hash,
    pub timestamp: u64,
    pub nonce: u64,
    pub difficulty: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Tx>,
}

impl Block {
    pub fn new(prev: Hash, txs: Vec<Tx>, difficulty: u32) -> Self {
        let merkle = merkle_root(txs.iter().map(|t| t.digest()).collect());
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            header: BlockHeader { prev, merkle, timestamp: ts, nonce: 0, difficulty },
            txs,
        }
    }

    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(&self.header).unwrap();
        sha256(&bytes)
    }
}
