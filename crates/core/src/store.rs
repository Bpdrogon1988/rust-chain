use crate::block::Block;
use crate::hash::Hash;
use anyhow::{anyhow, Result};

#[derive(Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self { Self { chain: Vec::new() } }

    pub fn tip(&self) -> Hash {
        self.chain.last().map(|b| b.hash()).unwrap_or_else(|| Hash::zero())
    }

    pub fn push(&mut self, b: Block) -> Result<()> {
        if self.chain.is_empty() {
            // accept any genesis
            self.chain.push(b);
            return Ok(());
        }
        let prev = self.tip();
        if b.header.prev != prev {
            return Err(anyhow!("prev hash mismatch"));
        }
        // (week 1) minimal: accept without PoW; merkle checked by constructing Block
        self.chain.push(b);
        Ok(())
    }
}
