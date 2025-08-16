use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    pub fn zero() -> Self { Self([0u8; 32]) }
    pub fn to_hex(&self) -> String { hex::encode(self.0) }
}

pub fn sha256(data: &[u8]) -> Hash {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut h = [0u8; 32];
    h.copy_from_slice(&out);
    Hash(h)
}

pub fn blake3_hash(data: &[u8]) -> Hash {
    let out = blake3::hash(data);
    Hash(*out.as_bytes())
}
