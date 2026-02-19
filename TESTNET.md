# Boing Network — Testnet Guide

> **Purpose:** Run nodes on the testnet, get testnet BOING from the faucet, and join as a validator or developer.  
> **References:** [RUNBOOK.md](./RUNBOOK.md), [BETA-READINESS.md](./BETA-READINESS.md), [docs/RPC-API-SPEC.md](./docs/RPC-API-SPEC.md)

---

## 1. Single node vs multi-node

| Mode | Use case | How to run |
|------|----------|------------|
| **Single node** | Local dev, trying the chain alone, no P2P | Run `boing-node` **without** `--p2p_listen`. The node runs in isolation: it produces blocks if `--validator`, and serves RPC. No other peers. |
| **Multi-node testnet** | Public testnet: many nodes syncing and validating together | Run `boing-node` **with** `--p2p_listen` and `--bootnodes`. Your node joins the P2P network, syncs blocks from peers, and (if `--validator`) can produce blocks when it’s the leader. |

**Summary:**  
- **Single node** = one machine, one chain, no peer discovery. Good for “run a chain on my laptop.”  
- **Multi-node** = many nodes connected via P2P; they discover each other using **bootnodes** and stay in sync.

---

## 2. What are bootnodes?

**Bootnodes** are well-known peer addresses that new nodes **dial on startup** to join the network. Without them, a node with P2P enabled would only see peers on the same LAN (via mDNS). With bootnodes, your node can connect to the public testnet even from home.

- **Format:** Multiaddr, e.g. `/ip4/1.2.3.4/tcp/4001` (IP + port where a testnet node is listening).
- **Who runs them:** Usually the team or community; they run a node with a stable IP and publish its address.
- **How you use them:** Pass `--bootnodes /ip4/.../tcp/4001,/ip4/.../tcp/4002` when starting your node (comma-separated).

Example (replace with real testnet bootnodes):

```bash
./boing-node --p2p_listen /ip4/0.0.0.0/tcp/4001 \
  --bootnodes /ip4/testnet.boing.network/tcp/4001 \
  --validator --rpc-port 8545 --data-dir ./data
```

---

## 3. Running a single node (no P2P)

```bash
cargo build --release

# Full node (no block production)
./target/release/boing-node --rpc-port 8545 --data-dir ./data

# Validator (produces blocks)
./target/release/boing-node --validator --rpc-port 8545 --data-dir ./data
```

RPC: `http://127.0.0.1:8545/`. No bootnodes needed.

---

## 4. Running on the multi-node testnet

1. **Build**  
   `cargo build --release`

2. **Start with P2P + bootnodes** (use the bootnode list from [Network / Testnet](https://boing.network/network/testnet) or below):

   ```bash
   ./target/release/boing-node \
     --p2p_listen /ip4/0.0.0.0/tcp/4001 \
     --bootnodes "<BOOTNODE_1>,<BOOTNODE_2>" \
     --validator \
     --rpc-port 8545 \
     --data-dir ./boing-data
   ```

3. **Get testnet BOING** from the [Faucet](/network/faucet) (see below).

4. **Stake** by submitting a `Bond` transaction via RPC so you can participate as a validator (validator set is derived from top stakers).

---

## 5. Faucet (testnet BOING)

Testnet nodes can expose a **faucet** so users get test BOING without mining.

### 5.1 RPC method: `boing_faucetRequest`

When the node is started with **`--faucet-enable`**, it accepts:

| Method | Params | Description |
|--------|--------|-------------|
| `boing_faucetRequest` | `[hex_account_id]` | Send **1,000** testnet BOING to the given account (32-byte hex). Rate limit: **1 request per 60 seconds per account**. |

**Example (curl):**

```bash
# Your account ID as 32-byte hex (e.g. from your wallet)
curl -s -X POST http://127.0.0.1:8545/ -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"boing_faucetRequest","params":["0xYOUR_32_BYTE_ACCOUNT_ID_HEX"]}'
```

**Response (success):**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "ok": true,
    "amount": 1000,
    "to": "hex_account_id",
    "message": "Check your wallet; tx is in the mempool."
  }
}
```

**Notes:**

- Only nodes started with `--faucet-enable` support this. **Do not use on mainnet.**
- The faucet account is funded at genesis with 10,000,000 testnet BOING; each request sends 1,000.
- If you get “Faucet cooldown”, wait 60 seconds and try again for the same account.

### 5.2 Public faucet page

The website provides a **dedicated faucet page** at [boing.network/network/faucet](https://boing.network/network/faucet) (or your deployment path). Enter your account ID (hex) and request testnet BOING; the page calls the testnet RPC’s `boing_faucetRequest` for you.

**Testnet RPC URL** for the faucet is published on the [Testnet](/network/testnet) page (e.g. `https://testnet-rpc.boing.network/` or the official testnet node URL).

---

## 6. Bootnode list (testnet)

When the testnet is live, the canonical list will be kept at:

- **Website:** [boing.network/network/testnet](/network/testnet)  
- **This repo:** Below (update before testnet launch)

| Bootnode | Multiaddr | Notes |
|----------|-----------|--------|
| *(To be filled at testnet launch)* | e.g. `/ip4/x.x.x.x/tcp/4001` | Official testnet node |

Until then, you can run a multi-node testnet locally by starting two nodes and having the second dial the first:

**Terminal 1 (first node):**

```bash
./target/release/boing-node --p2p_listen /ip4/127.0.0.1/tcp/4001 --validator --rpc-port 8545
```

**Terminal 2 (second node, dials the first):**

```bash
./target/release/boing-node --p2p_listen /ip4/127.0.0.1/tcp/4002 \
  --bootnodes /ip4/127.0.0.1/tcp/4001 \
  --rpc-port 8546
```

---

## 7. One-click mining / validator UI (VibeMiner)

For users who prefer a **desktop UI** instead of the terminal, Boing testnet can be used with **VibeMiner**, which provides one-click mining/validating. See [VIBEMINER-INTEGRATION.md](./VIBEMINER-INTEGRATION.md) for how networks (including Boing) integrate and how to run a node via VibeMiner.

---

*Boing Network — Authentic. Decentralized. Optimal. Sustainable.*
