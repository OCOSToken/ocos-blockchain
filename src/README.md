# 🧠 OCOS Blockchain Core – `src/` Directory

Welcome to the **core engine** of the OCOS Blockchain. This folder contains all essential modules required to run, extend, secure, and operate the OCOS decentralized infrastructure — including blockchain logic, consensus, cryptography, wallets, networking, and smart governance tools.

Every submodule is structured to ensure **modularity**, **security**, and **scalability**, inspired by production-grade systems such as Bitcoin Core, Polkadot/Substrate, Solana, and Cosmos SDK.

---

## 🧩 Overview of Modules

| Module         | Purpose                                                                 |
|----------------|-------------------------------------------------------------------------|
| `api/`         | Exposes REST, RPC, or WebSocket endpoints for node interaction          |
| `blockchain/`  | Core block and chain logic (validation, Merkle root, genesis, etc.)     |
| `consensus/`   | Implements consensus logic: PoW, PoS, DAO voting, etc.                  |
| `crypto/`      | Cryptographic primitives: hashing, signing, keypairs                    |
| `transaction/` | Transaction structs, mempool, validation, and verification              |
| `network/`     | Peer discovery, P2P networking, gossip protocol                         |
| `node/`        | Node configuration, lifecycle, and daemon orchestration                 |
| `storage/`     | State and block storage (DB abstraction, snapshots, migration)          |
| `wallet/`      | Wallet features: key management, mnemonics, HD derivation, multisig     |
| `utils/`       | Logger, time management, configuration parser, helpers                  |

Each directory is an independent Rust module and tested independently for maximum reliability and reusability.

---

## 📂 Detailed Directory Breakdown

### `api/`
- **Description:** Interface layer between OCOS node and external clients.
- **Includes:** REST API handlers, HTTP routes, JSON formatters, (future: gRPC & WebSocket)
- **Usage:** Ideal for explorer integration, user wallets, staking portals.

---

### `blockchain/`
- **Description:** The heart of the chain.
- **Includes:**
  - `block.rs` – Block structure & hashing
  - `chain.rs` – Chain state & validation logic
  - `genesis.rs` – Creation of the initial block with embedded timestamp/message
- **Implements:** Merkle tree, double-SHA256, chain validation rules.

---

### `consensus/`
- **Description:** Mechanisms to reach agreement across nodes.
- **PoW:** Optional mining support  
- **PoS:** Placeholder for staking  
- **DAO:** Governance-based consensus voting (planned)

---

### `crypto/`
- **Description:** Cryptographic foundations for all operations.
- **Includes:**
  - `hash.rs` – SHA-256, Blake2b, Merkle root
  - `keypair.rs` – secp256k1 key generation
  - `signature.rs` – ECDSA message signing & verification
- **Security Level:** Complies with modern cryptographic standards via `k256`, `sha2`, and `blake2`.

---

### `transaction/`
- **Description:** Full transaction lifecycle logic.
- **Includes:**
  - `tx.rs` – Transaction struct & serialization
  - `mempool.rs` – In-memory tx queue
  - `validator.rs` – Ensures tx validity and nonce/order

---

### `network/`
- **Description:** Manages OCOS node connectivity.
- **Features:**
  - Peer list management
  - Gossip protocol
  - Future: DHT and NAT traversal

---

### `node/`
- **Description:** Controls the lifecycle of a single OCOS node.
- **Responsibilities:**
  - Load configuration
  - Start networking
  - Sync chain
  - Manage modules (e.g., API, consensus, wallet)

---

### `storage/`
- **Description:** Persists chain state and blocks.
- **Backends:** RocksDB planned; InMemory for testnet/devnet.
- **Features:**
  - Write-ahead logging (WAL)
  - Snapshot export/restore
  - Migration-aware design

---

### `wallet/`
- **Description:** Manages user accounts and signing capabilities.
- **Includes:**
  - `wallet.rs` – Core wallet with keypair and OCOS address
  - `mnemonic.rs` – BIP39 deterministic wallet recovery
  - `bip44.rs` – Hierarchical HD wallets (m/44'/4747'/0'/0/index)
  - `multisig.rs` – M-of-N multi-signature support for DAO & treasury

---

### `utils/`
- **Description:** Contains shared utility functions used across all modules.
- **Examples:**
  - Logging wrappers
  - UNIX timestamp formatting
  - Config parser (`.toml`, `.env`)
  - Hex helpers, base58 (planned)

---

## 🔒 Security Philosophy

OCOS follows **secure-by-default** and **minimal attack surface** principles:
- Memory-safe Rust abstractions
- No unsafe code in cryptographic modules
- Signature verification and address recovery based on deterministic standards
- Future: Hardware Wallet and WebAuthn support via modular injection

---

## ✅ How to Use These Modules

In `lib.rs`:

```rust
pub mod api;
pub mod blockchain;
pub mod consensus;
pub mod crypto;
pub mod transaction;
pub mod network;
pub mod node;
pub mod storage;
pub mod wallet;
pub mod utils;
