use chain_core::{hash::Hash, merkle::merkle_root, crypto::{Keypair, SigAlg}, tx::Tx};

#[test]
fn merkle_not_zero() {
    let h1 = Hash([1;32]); let h2 = Hash([2;32]);
    let r = merkle_root(vec![h1, h2]);
    assert_ne!(r, Hash::zero());
}

#[test]
fn ed25519_sign_verify() {
    let kp = Keypair::generate(SigAlg::Ed25519);
    let tx = Tx::new(kp.public(), kp.public(), 1, 1, SigAlg::Ed25519).sign(&kp);
    tx.verify().unwrap();
}
