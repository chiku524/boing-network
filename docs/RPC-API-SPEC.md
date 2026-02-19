# Boing Network — JSON-RPC API Specification

> **Version:** 0.1  
> **Transport:** HTTP POST  
> **Encoding:** JSON-RPC 2.0  
> **References:** [RUNBOOK.md](RUNBOOK.md)

---

## Overview

Boing nodes expose a JSON-RPC HTTP interface for submitting transactions, querying chain state, and simulation. Rate limiting applies per `RateLimitConfig` (see [SECURITY-STANDARDS.md](SECURITY-STANDARDS.md)).

### Base URL

```
http://<host>:<rpc_port>/
```

Default RPC port: `8545`.

### Request Format

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "boing_chainHeight",
  "params": []
}
```

### Response Format

**Success:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": 42
}
```

**Error:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid params: expected [hex_signed_tx]"
  }
}
```

---

## Methods

### boing_submitTransaction

Submit a signed transaction to the mempool.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_signed_tx]` | Hex-encoded bincode-serialized SignedTransaction |

**Example:**
```json
{"jsonrpc":"2.0","id":1,"method":"boing_submitTransaction","params":["0x..."]}
```

---

### boing_chainHeight

Return the current chain height (tip block number).

| Field | Type | Description |
|-------|------|-------------|
| Params | `[]` | None |

**Result:** `u64`

---

### boing_getBlockByHeight

Get a block by height.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[height]` | Block height (u64) |

**Result:** Block object or `null` if not found.

---

### boing_getBlockByHash

Get a block by hash.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_block_hash]` | 32-byte block hash (hex) |

**Result:** Block object or `null` if not found.

---

### boing_getAccountProof

Get a Merkle proof for an account.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_account_id]` | 32-byte AccountId (hex) |

**Result:** `{ proof: string, root: string, value_hash: string }`

---

### boing_verifyAccountProof

Verify an account Merkle proof.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_proof, hex_state_root]` | Proof and expected root (hex) |

**Result:** `{ valid: boolean }`

---

### boing_simulateTransaction

Simulate a transaction without applying it.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_signed_tx]` | Hex-encoded SignedTransaction |

**Result:** `{ gas_used: number, success: boolean, error?: string }`

---

### boing_registerDappMetrics

Register a dApp for incentive tracking.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_contract, hex_owner]` | Contract and owner AccountIds (hex) |

**Result:** `{ registered: true, contract: string, owner: string }`

---

### boing_submitIntent

Submit a signed intent for solver fulfillment.

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_signed_intent]` | Hex-encoded SignedIntent |

**Result:** `{ intent_id: string }`

---

### boing_faucetRequest (testnet only)

Request testnet BOING for an account. Only available when the node is started with `--faucet-enable`. **Do not enable on mainnet.**

| Field | Type | Description |
|-------|------|-------------|
| Params | `[hex_account_id]` | 32-byte account ID (hex). Recipient of the faucet transfer. |

**Result:** `{ ok: true, amount: number, to: string, message: string }`

**Rate limit:** 1 request per 60 seconds per account ID. Returns `-32016` with message "Faucet cooldown" if called too soon.

**Errors:** `-32601` Faucet not enabled; `-32000` Faucet account not initialized or balance too low.

---

## Error Codes

| Code | Meaning |
|------|---------|
| -32600 | Invalid Request |
| -32601 | Method not found |
| -32602 | Invalid params |
| -32000 | Server error |
| -32016 | Rate limit exceeded |

---

*Boing Network — Authentic. Decentralized. Optimal. Sustainable.*
