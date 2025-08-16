use anyhow::{anyhow, Result};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SigAlg {
    Ed25519,
    Secp256k1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey {
    pub alg: SigAlg,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    pub alg: SigAlg,
    pub bytes: Vec<u8>,
}

pub struct Keypair {
    pub alg: SigAlg,
    ed: Option<ed25519_dalek::SigningKey>,
    secp: Option<k256::ecdsa::SigningKey>,
}

impl Keypair {
    pub fn generate(alg: SigAlg) -> Self {
        match alg {
            SigAlg::Ed25519 => {
                let kp = ed25519_dalek::SigningKey::generate(&mut OsRng);
                Self { alg, ed: Some(kp), secp: None }
            }
            SigAlg::Secp256k1 => {
                let sk = k256::ecdsa::SigningKey::random(&mut OsRng);
                Self { alg, ed: None, secp: Some(sk) }
            }
        }
    }

    pub fn public(&self) -> PublicKey {
        match self.alg {
            SigAlg::Ed25519 => {
                let pk = self.ed.as_ref().unwrap().verifying_key().to_bytes().to_vec();
                PublicKey { alg: SigAlg::Ed25519, bytes: pk }
            }
            SigAlg::Secp256k1 => {
                let vk = k256::ecdsa::VerifyingKey::from(self.secp.as_ref().unwrap());
                PublicKey { alg: SigAlg::Secp256k1, bytes: vk.to_sec1_bytes().to_vec() }
            }
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        match self.alg {
            SigAlg::Ed25519 => {
                use ed25519_dalek::Signer as _;
                let sig = self.ed.as_ref().unwrap().sign(msg);
                Signature { alg: SigAlg::Ed25519, bytes: sig.to_bytes().to_vec() }
            }
            SigAlg::Secp256k1 => {
                use k256::ecdsa::{signature::Signer, Signature as EcdsaSig};
                let sig: EcdsaSig = self.secp.as_ref().unwrap().sign(msg);
                Signature { alg: SigAlg::Secp256k1, bytes: sig.to_vec() }
            }
        }
    }
}

pub fn verify(pk: &PublicKey, msg: &[u8], sig: &Signature) -> Result<()> {
    if pk.alg != sig.alg {
        return Err(anyhow!("signature algorithm mismatch"));
    }
    match sig.alg {
        SigAlg::Ed25519 => {
            use ed25519_dalek::{Signature as EdSig, VerifyingKey as EdPk};
            // ed25519 public keys are 32 bytes; signatures are 64 bytes
            let pk_bytes: [u8; 32] = pk
                .bytes
                .as_slice()
                .try_into()
                .map_err(|_| anyhow!("bad public key length"))?;
            let edpk = EdPk::from_bytes(&pk_bytes).map_err(|e| anyhow!(e))?;
            let sig_bytes: [u8; 64] = sig
                .bytes
                .as_slice()
                .try_into()
                .map_err(|_| anyhow!("bad sig length"))?;
            let sig = EdSig::from_bytes(&sig_bytes);
            edpk.verify_strict(msg, &sig).map_err(|e| anyhow!(e))
        }
            SigAlg::Secp256k1 => {
                use k256::ecdsa::{signature::Verifier as _, Signature as EcdsaSig, VerifyingKey};
                let vk = VerifyingKey::from_sec1_bytes(&pk.bytes).map_err(|e| anyhow!(e))?;
                let sig = EcdsaSig::from_der(&sig.bytes)
                    .or_else(|_| EcdsaSig::try_from(sig.bytes.as_slice()))
                    .map_err(|e| anyhow!(e))?;
                vk.verify(msg, &sig).map_err(|e| anyhow!(e))
            }
    }
}
