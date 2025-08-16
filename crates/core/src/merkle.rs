use crate::hash::{blake3_hash, Hash};

/// compute merkle root. if odd count, duplicate last.
pub fn merkle_root(mut leaves: Vec<Hash>) -> Hash {
    if leaves.is_empty() {
        return Hash::zero();
    }
    while leaves.len() > 1 {
        let mut next = Vec::with_capacity((leaves.len() + 1) / 2);
        for pair in leaves.chunks(2) {
            let a = pair[0];
            let b = *pair.get(1).unwrap_or(&pair[0]);
            let mut buf = Vec::with_capacity(64);
            buf.extend_from_slice(&a.0);
            buf.extend_from_slice(&b.0);
            next.push(blake3_hash(&buf));
        }
        leaves = next;
    }
    leaves[0]
}
