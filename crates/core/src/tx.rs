use crate::crypto::{self, Keypair, PublicKey, SigAlg, Signature};
use crate::hash::{sha256, Hash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub from: PublicKey,
    pub to: PublicKey,
    pub amount: u64,
    pub nonce: u64,
    pub alg: SigAlg,
    pub sig: Option<Signature>,
}

impl Tx {
    pub fn new(from: PublicKey, to: PublicKey, amount: u64, nonce: u64, alg: SigAlg) -> Self {
        Self { from, to, amount, nonce, alg, sig: None }
    }

    pub fn digest(&self) -> Hash {
        let buf = bincode::serialize(&( &self.from.bytes, &self.to.bytes, self.amount, self.nonce, self.alg as u8 ))
            .unwrap();
        sha256(&buf)
    }

    pub fn sign(mut self, kp: &Keypair) -> Self {
        let h = self.digest();
        let sig = kp.sign(&h.0);
        self.sig = Some(sig);
        self
    }

    pub fn verify(&self) -> anyhow::Result<()> {
        let h = self.digest();
        let sig = self.sig.as_ref().ok_or_else(|| anyhow::anyhow!("missing signature"))?;
        crypto::verify(&self.from, &h.0, sig)
    }
}
