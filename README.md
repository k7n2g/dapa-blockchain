<div align="center">
<div><h1> </h1></div>
<img src="assets/logo.png" alt="DAPA Logo" width="200"/>

# DAPA

DAPA with **BlockDAG** Total **Privacy**, **Speed**, **Scalability** and **Smart Contracts**.

</div>

## Features

The main features of DAPA are the following:

- **BlockDAG**: enabled to improve the scalability and the security of the network by reducing orphaned blocks rate.
- **Egalitarian PoW**: Unique PoW algorithm built to allow any CPU or GPU to mine DAPA easily using [xelis-hash](https://github.com/xelis-project/xelis-hash).
- **Kalman Filter**: Difficulty adjustment algorithm using Kalman Filter to adjust the difficulty at each block instantly and smoothly to prevent any stuck-chain or dishonest miners mining at lower difficulty.
- **Privacy**: Homomorphic Encryption allows to have encrypted balances and encrypted transferred amounts.
- **Smart Contracts**: allows to create unstoppable decentralized applications by deploying programs on the network.
- **Confidential Asset**: Any asset deployed on DAPA network will have the same privacy and functionality like DAPA in any wallet.
- **Event system**: every event happening on the network (daemon or wallet) can be detected and notified easily.
- **Instant Synchronization**: Your wallet balances and history is synced in few seconds. No need to sync the whole chain to use your wallet.
- **Pruning Mode**: Reduce the blockchain size by deleting blocks, transactions and versioned balances.
- **Extra Data**: Send extra data in a transaction to transfer easily data to a wallet. This is secure and encrypted, readable only by the parties of a transaction.
- **Integrated addresses**: Auto-integrate extra-data in a transaction when using an integrated address.
- **Easy to use**: We aim to provide the easiest platform to build and use daily.

For more, see the [full documentation](https://docs.dapahe.com).

## Network

- **Mainnet**: Released Jan, 2026.

## Acknowledgments

[@cchudant](https://github.com/cchudant):
- Optimized decoding RistrettoPoint implementation (ECDLP).
- Twisted ElGamal implementation along ZK-Proofs integration for Confidential Transactions.
- To read more, please see [DAPA-HE](https://github.com/xelis-project/xelis-he) framework created by him.

[@deroholic](https://github.com/deroholic):
- Difficulty adjustment algorithm using Kalman-Filter.

Thank you to every person testing actively the code base, honest miners and every future contributor!

## How to build

Building this project requires a working [Rust](https://rustup.rs) (stable) toolchain and `clang`, `cmake` packages installed (required by `rustls` and its crypto provider `aws-lc-rs`).

It's expected to be cross-platform and guaranteed to work on Linux, Windows, MacOS platforms.

### Build from sub project

Go to one of the following folders you want to build from source: `dapa_daemon`, `dapa_miner` or `dapa_wallet`.  
To build a release (optimized) version:

```bash
cargo build --release
```

### Build from workspace

To build a specific binary from workspace (parent folder) directly, use the option `--bin` with `dapa_daemon`, `dapa_miner` or `dapa_wallet` as value.

```bash
# Example: build the miner
cargo build --release --bin dapa_miner
```

To build all at once:

```bash
cargo build --release
```

You can also build a debug version (remove `--release`) or run directly from cargo:

```bash
cargo run
```

### Build from Docker

To build using Docker, use the following command with the `app` build argument to choose which project to build:

```bash
docker build -t dapa-daemon:latest --build-arg app=dapa_daemon .
```

## Funding

DAPA project is part of MgenPower.  
To help the development, the success and provide better support of DAPA, we set a dev fee percentage starting at 10% on block reward.

Current dev fee curve:

- **10%** from block 0 to 14,000.
- **3%** from block 14,001 until the project is stable.

## Config

### Network

| Parameter | Value |
|---|---|
| Expected Block Time | 5 seconds |
| Address prefix | `dap:` on mainnet · `dat:` for testnet/devnet |
| Transaction fee | 0.0001 DAPA per KB |
| Account creation fee | 0.001 DAPA |
| Per-transfer fee | 0.00005 DAPA |
| Decimals | 8 |
| Maximum supply | 800,000,000 DAPA |
| Maximum block size | 1.25 MB |
| Difficulty adjustment | Retarget at every block |
| Block reward emission | Retarget at every block (Smooth decrease) |

### Seed Nodes (Mainnet)

```
185.198.27.165:20100   # France
45.84.138.159:20100    # Germany
194.163.189.149:20100  # Singapore
```

### Daemon

| Service | Default Port |
|---|---|
| P2P Server | `20100` |
| RPC Server | `20101` |
| GetWork (miners) | `20101` |

### Wallet

| Service | Default Port |
|---|---|
| Wallet RPC Server | `20102` |

## Running a Node

### Quick Start

The fastest way to get a node running is with fast sync, which downloads only the necessary chain state rather than replaying every block from genesis:

```bash
./dapa_daemon \
  --p2p-bind-address 0.0.0.0:20100 \
  --rpc-bind-address 0.0.0.0:20101 \
  --priority-nodes 194.163.189.149:20100 \
  --allow-fast-sync \
  --skip-block-template-txs-verification
```

> ⚠️ **IMPORTANT — Read before syncing:**
>
> - ✅ Always use `--allow-fast-sync` for initial sync. This is safe and reliable.
> - ✅ Use `--skip-block-template-txs-verification` to skip verification of historical block transactions during sync.
> - ❌ **Never use `--allow-boost-sync`** — this causes **database corruption** and will break your node. Do not use it under any circumstances.

### Recommended Production Setup

```bash
./dapa_daemon \
  --network mainnet \
  --p2p-bind-address 0.0.0.0:20100 \
  --rpc-bind-address 0.0.0.0:20101 \
  --priority-nodes 194.163.189.149:20100 \
  --allow-fast-sync \
  --skip-block-template-txs-verification
```

- Open port `20100` for P2P so peers can connect to your node.
- Open port `20101` for RPC so wallets and explorers can query your node.

### Connecting to a Specific Peer

```bash
./dapa_daemon --add-peer 194.163.189.149:20100
```

### Firewall (UFW)

```bash
sudo ufw allow 20100/tcp   # P2P
sudo ufw allow 20101/tcp   # RPC / API
sudo ufw deny  20102/tcp   # Wallet RPC — keep closed unless intentionally exposed
```

### Run as a systemd Service

Create `/etc/systemd/system/dapa-node.service`:

```ini
[Unit]
Description=DAPA Blockchain Node
After=network.target

[Service]
User=dapa
WorkingDirectory=/opt/dapa
ExecStart=/opt/dapa/dapa_daemon \
  --network mainnet \
  --p2p-bind-address 0.0.0.0:20100 \
  --rpc-bind-address 0.0.0.0:20101 \
  --priority-nodes 194.163.189.149:20100 \
  --allow-fast-sync \
  --skip-block-template-txs-verification
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now dapa-node
sudo journalctl -fu dapa-node
```

### Mining — Quick Start

The fastest way to start mining is to use the config generator at [mine.dapahe.com](https://mine.dapahe.com) — enter your wallet address and get your launch command in 60 seconds.

Or manually:

```bash
./dapa_miner \
  --daemon-address 194.163.189.149:20101 \
  --mining-address dap:YOUR_WALLET_ADDRESS
```

### Network Links

| Resource | URL |
|---|---|
| Explorer | [explorer.dapahe.com](https://explorer.dapahe.com) |
| Web Wallet | [webwallet.dapahe.com](https://webwallet.dapahe.com) |
| Mining Page | [mine.dapahe.com](https://mine.dapahe.com) |
| Documentation | [docs.dapahe.com](https://docs.dapahe.com) |

## BlockDAG

DAPA uses a BlockDAG with the following rules:

- A block is considered a **Sync Block** when the block height is less than `TOP_HEIGHT - STABLE_LIMIT` and it's the unique block at a specific height (or the only ordered block at its height and does not have lower cumulative difficulty than previous blocks).
- A block is considered a **Side Block** when its block height is less than or equal to the height of the past 8 topological blocks.
- A block is considered **Orphaned** when the block is not ordered in the DAG (no topological height assigned).
- Height is not unique — **Topo height** is unique for each block, but can change when the DAG is re-ordered up to `TOP_HEIGHT - STABLE_LIMIT`.
- You can have up to **3 previous blocks** in a block.
- For mining, you must mine on one of the **3 heaviest tips**.
- Maximum **9% cumulative difficulty** difference between tips selected in the same block.
- **Side Blocks** receive only **30% of the block reward**.
- Supply is re-calculated each time a block is re-ordered.
- Transactions and miner rewards are re-computed when a new block is added and the linked block is not yet at stable topo height.
- The same transaction can be added to more than one block if they are not in the same tip branch. The client protocol will execute it only one time.

Topoheight represents how many unique blocks exist in the blockchain ordered by DAG. The longest chain is selected by nodes; for tip branch conflicts, cumulative difficulty is used to select the main chain.

## Homomorphic Encryption

ElGamal cryptosystem was chosen because it is a well-known and studied encryption algorithm with homomorphism features. We use a variant named **Twisted ElGamal** which gives full integration with Pedersen commitments, useful for Bulletproofs compatibility and saves space and time by avoiding an intermediate proof.

Twisted ElGamal is fast and is used with the Ristretto group over the elliptic curve **Curve25519** to provide ~128 bits of security.

Homomorphic operations available: addition/subtraction between ciphertexts and/or plaintext, and multiplication against a plaintext value.

All balances and transaction asset values are in encrypted form. Nobody can determine the real value except the involved parties.

## Mining

Each job sent to a miner is a `MinerWork` instance in hex format.

**MinerWork format (120 bytes total):**

| Field | Size |
|---|---|
| Header work hash | 32 bytes |
| Timestamp (u64, milliseconds) | 8 bytes (BigEndian) |
| Nonce (u64) | 8 bytes (BigEndian) |
| Extra nonce | 32 bytes |
| Miner public key | 32 bytes |

The **header work hash** is an immutable hash calculated using `Blake3` over: block version (1 byte), block height (8 bytes), hash of tips (32 bytes), hash of transaction hashes (32 bytes) — total 73 bytes input.

> **Note:** The miner key is **not** included in the header work hash. This allows a block template to be shared generically without regenerating it per miner.  
> **Pool operators must verify** that the miner public key in a received share is the pool's own key, as it can be updated by the miner.

All hashes use `Blake3` except the Proof-of-Work hash which uses [xelis-hash](https://github.com/xelis-project/xelis-hash).

It is recommended to use the **GetWork WebSocket server** to receive new block work and submit completed work. Miners should update the block timestamp at least every 500ms for best difficulty calculation.

## Client Protocol

DAPA integrates a mechanism alongside BlockDAG to accept the same TX in multiple blocks but execute it only once. Instead of rejecting a whole block due to a TX collision across branches, the TX is simply not re-executed while its hash is retained.

The same TX can appear in multiple blocks only if:

- The TX has not been executed at stable height.
- The TX is not included in the block's direct tips (previous blocks).

For security, a user account should only broadcast TXs on the same chain/tip to prevent orphaned transactions.

## Transaction

Transaction types supported:

- **Transfer**: send many assets to many addresses in a single TX (up to 255 outputs).
- **Burn**: publicly burn an amount of a specific asset; proves coins are deleted from circulation.
- **Call Contract**: call a Smart Contract with specific parameters and a list of assets to deposit.
- **Deploy Contract**: deploy a new valid Smart Contract on chain.

| Field | Type | Comment |
|---|---|---|
| source | PublicKey | Signer of this transaction |
| data | TransactionType | Type with data of this transaction |
| fee | Integer | Fees paid by the owner for inclusion |
| nonce | Integer | Matching nonce of balance; prevents replay attacks |
| signature | Signature | Proves the owner authorised this TX |

## Integrated Address

Integrated addresses are base addresses with custom data embedded. Maximum data allowed: **1 KB** (same as a transfer payload). Helpful for linking a transaction to an account or order on the service side.

## P2P (Encrypted Network)

All transferred data uses a custom Serializer/Deserializer to transform struct representations into raw bytes. Before sending, each packet is encrypted using **ChaCha20-Poly1305** to prevent traffic analysis and authenticate transferred data.

Key highlights:

- New peer connections are handled through a single tokio task to prevent DoS attacks.
- Each verified peer gets two separate tasks — one for reading, one for writing — to prevent blocking communication in either direction.
- A per-peer transaction cache prevents sending the same TX data twice during propagation.
- Encryption keys are rotated every **1 GB** of data sent.

### Pruning Mode

Anyone wanting a light node can enable pruning to delete old blocks, transactions, and versioned balances. The pruned topoheight can only be set at a `Sync Block` at least `PRUNE_SAFETY_LIMIT` blocks behind the top topoheight.

### Fast Sync

Fast sync allows you to join the network quickly by requesting a peer's chain state at a stable point (all account nonces, assets, balances, and top blocks). Use only with a **trusted peer**. Always preferred over boost sync.

### Boost Sync

> ❌ **Do not use.** `--allow-boost-sync` causes database corruption. It is disabled by default and should remain so.

## Storage

All data is saved using the **Sled** database engine.

| Tree | Key Type | Value Type | Comment |
|---|---|---|---|
| transactions | Hash | Transaction | Full transaction by hash |
| blocks | Hash | Block Header | Block header by hash |
| blocks_at_height | Integer | Array of Hash | All block hashes at a height |
| extra | Bytes | Various | Highest topo height, pruned topoheight, TIPS |
| topo_by_hash | Hash | Integer | Topo height for a block hash |
| hash_by_topo | Integer | Hash | Block hash for a topo height |
| cumulative_difficulty | Hash | Integer | Cumulative difficulty per block |
| difficulty | Hash | Integer | Difficulty per block |
| rewards | Integer | Integer | Block reward per topoheight |
| supply | Integer | Integer | Emitted supply per topoheight |
| tx_blocks | Hash | Array of Hash | All blocks containing a TX hash |
| balances | Custom | Integer | Last topoheight of versioned balance |
| nonces | Public Key | Integer | Highest topoheight of versioned nonce |
| versioned_balances | Custom | Versioned Balance | Key: topoheight + asset + public key |
| versioned_nonces | Custom | Versioned Nonce | Key: topoheight + public key |

## Wallet

The wallet tracks all your transactions and assets. When creating a new wallet, a random **master key** is generated and encrypted with a hashed password. This master key encrypts/decrypts the entire wallet storage and can be re-encrypted to change your password without re-encrypting all data.

Password hashing algorithm: **Argon2id** with 15 MB memory and 16 iterations.

### Wallet Storage

- Tree names are hashed with a generated salt.
- Keys are hashed with a generated salt.
- Values are encrypted using **XChaCha20Poly1305** with a random nonce generated each save.

Hash algorithm: **Blake3**. Salt size: 64 bytes.

## API

The HTTP server runs using the **Actix** framework and serves the JSON-RPC API and WebSocket.

### JSON-RPC

Available at `/json_rpc` on the RPC server address (default port `20101`).

```bash
# Example: get node info
curl -s -X POST http://127.0.0.1:20101/json_rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"get_info","id":1,"params":{}}'
```

Full API reference: [API.md](API.md)

### WebSocket

The WebSocket runs on the same `/json_rpc` route and allows subscribing to events.

**Subscribe to an event:**

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "subscribe",
    "params": {
        "notify": "new_block"
    }
}
```

**Daemon events:** `block_ordered`, `block_orphaned`, `stable_height_changed`, `new_block`, `transaction_added_in_mempool`, `transaction_executed`, `transaction_sc_result`, `new_asset`, `peer_connected`, `peer_disconnected`, `peer_peer_list_updated`, `peer_state_updated`, `peer_peer_disconnected`

**Wallet events:** `new_topoheight`, `new_asset`, `new_transaction`, `balance_changed`, `rescan`, `online`, `offline`

### XSWD

XSWD (DAPA Secure WebSocket DApp) Protocol runs on port **`44325`** at path **`/xswd`**. It provides a secure channel from a desktop/CLI wallet to any dApp. Instead of username/password authentication, XSWD requires the user to manually accept each dApp connection and can set per-method permissions.

The dApp must send an identification message as its first JSON payload:

```json
{
    "id": "0000006b2aec4651b82111816ed599d1b72176c425128c66b2ab945552437dc9",
    "name": "DAPA Example",
    "description": "Description example of up to 255 characters",
    "url": "https://dapahe.com",
    "permissions": [
        "get_balance"
    ]
}
```

Use the `node.` prefix to proxy requests to the daemon (no user approval needed). Use the `wallet.` prefix for wallet methods (subject to user-configured permissions).

---

<div align="center">
<sub>DAPA is open source, has no insider allocation.</sub>
</div>
