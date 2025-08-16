use anyhow::Result;
use chain_core::{
    block::Block,
    crypto::{Keypair, SigAlg},
    store::Blockchain,
    tx::Tx,
};

fn main() -> Result<()> {
    // wallets
    let alice = Keypair::generate(SigAlg::Ed25519);
    let bob   = Keypair::generate(SigAlg::Ed25519);

    // tx: alice -> bob
    let tx = Tx::new(alice.public(), bob.public(), 42, 1, SigAlg::Ed25519).sign(&alice);
    tx.verify().expect("valid tx");

    // genesis block
    let mut chain = Blockchain::new();
    let genesis = Block::new(chain.tip(), vec![tx], 0);
    chain.push(genesis)?;

    println!("height: {}", chain.chain.len());
    println!("tip: {}", chain.tip().to_hex());
    Ok(())
}
