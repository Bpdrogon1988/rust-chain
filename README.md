# Rust Blockchain Implementation

A lightweight blockchain implementation in Rust featuring cryptographic signatures, Merkle trees, and transaction validation.

## Features

- 🔐 **Cryptographic Signatures**: Ed25519 and ECDSA (secp256k1) support
- 🌳 **Merkle Trees**: Efficient transaction verification
- 📦 **Block Management**: Create, validate, and chain blocks
- 💰 **Transaction System**: Signed transactions with verification
- 🗄️ **Blockchain Storage**: In-memory blockchain with proper linking
- 🔄 **Serialization**: Binary serialization using bincode
- 🏗️ **Modular Architecture**: Clean separation of concerns

## Project Structure

```
rust-chain/
├── crates/
│   └── core/           # Core blockchain library
│       ├── block.rs    # Block structure and validation
│       ├── crypto.rs   # Cryptographic operations
│       ├── hash.rs     # Hashing utilities
│       ├── merkle.rs   # Merkle tree implementation
│       ├── ser.rs      # Serialization/deserialization
│       ├── store.rs    # Blockchain storage
│       └── tx.rs       # Transaction handling
└── apps/
    └── demo/           # Demo application
```

## Quick Start

### Prerequisites

- Rust 1.70+ and Cargo
- Git

### Building

```bash
# Clone the repository
git clone <your-repo-url>
cd rust-chain

# Build the project
cargo build

# Run the demo
cargo run --bin demo
```

### Demo Output

The demo creates a simple blockchain with:
- Two wallets (Alice and Bob)
- A signed transaction from Alice to Bob
- A genesis block containing the transaction

```
height: 1
tip: 0cfe447627352490954621581b6f07cdb37095cab825695462a8188f5794b1fa
```

## Core Components

### Blocks
Blocks contain transactions and link to previous blocks via cryptographic hashes.

### Transactions
Signed transactions with sender, recipient, amount, and nonce.

### Cryptography
- **Ed25519**: Fast, secure digital signatures
- **ECDSA (secp256k1)**: Bitcoin-compatible signatures
- **Blake3**: Fast hashing for block headers
- **SHA2**: Additional hashing support

### Merkle Trees
Efficient verification of transaction inclusion in blocks.

## Development

This is a learning project demonstrating blockchain fundamentals. The implementation focuses on:
- Clean, readable Rust code
- Proper error handling
- Modular design
- Cryptographic security

## License

MIT License - feel free to use this code for learning and experimentation.

## Contributing

This is primarily a learning project, but suggestions and improvements are welcome!
