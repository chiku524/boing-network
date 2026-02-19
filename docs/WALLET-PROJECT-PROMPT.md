# Boing Wallet Project — Creation Prompt

Use this prompt to bootstrap a **Boing-Network–friendly wallet** as a separate project. The wallet is **primarily for Boing Network** but should be **architected so more networks can be added later**. The project will be a **Cloudflare-based web app** under the domain **boing.express**.

---

## Copy-paste prompt (for Cursor / AI or human devs)

```
Create a new project for a crypto wallet with the following specs.

**Project name & domain**
- Product name: Boing Wallet (or "Boing Express" for the brand)
- Domain: boing.express (already purchased)
- Hosting: Cloudflare — use Cloudflare Pages for the main wallet app. Optionally use Cloudflare Workers for API routes (e.g. RPC proxy, rate limiting) if needed. Set up the project so it can be deployed to Cloudflare and the custom domain boing.express can be attached.

**Primary chain: Boing Network**
- The wallet must support Boing Network first: display balance, send/receive BOING, request testnet BOING from the faucet, and (later) staking (bond/unbond).
- Boing is not EVM-compatible. It uses:
  - **Address / AccountId:** 32 bytes, derived from the **Ed25519 public key** (verifying key). Display and accept as 64-character hex, with or without "0x" prefix. This is the "address" users paste into the faucet and share for receives.
  - **Signing:** Ed25519. Transactions are serialized in a specific way, hashed with BLAKE3, then signed. See "Boing signing spec" below.
  - **RPC:** JSON-RPC over HTTP. Key methods: **boing_getBalance([hex_account_id])** and **boing_getAccount([hex_account_id])** for balance/nonce (wallet UI); boing_submitTransaction([hex_signed_tx]), boing_chainHeight([]), boing_getBlockByHeight([height]), boing_simulateTransaction([hex_signed_tx]), boing_faucetRequest([hex_account_id]) for testnet. Params and results are JSON-RPC 2.0. Reference: boing-network repo docs/RPC-API-SPEC.md.
- **Transaction format (Boing):** A transaction has: nonce (u64), sender (AccountId), payload (Transfer | Bond | Unbond | ContractCall | ContractDeploy), access_list (read/write account IDs). The signed payload submitted to RPC is: hex(bincode(SignedTransaction)), where SignedTransaction = { tx: Transaction, signature: Signature } and Signature is 64-byte Ed25519. Serialization uses bincode (Rust-compatible). The wallet must implement (or use a small library for) Boing transaction construction and signing so it can produce valid hex_signed_tx for boing_submitTransaction.

**Boing signing spec (must match boing-network)**
- Signable message = BLAKE3 hash of the following concatenated:
  - nonce (little-endian u64, 8 bytes)
  - sender (32 bytes)
  - bincode(payload)
  - bincode(access_list)
- Signature = Ed25519 sign(signable_message) → 64 bytes.
- SignedTransaction = { tx, signature }; submit hex(bincode(SignedTransaction)) to boing_submitTransaction.
- Payload types: Transfer { to: AccountId, amount: u128 }, Bond { amount: u128 }, Unbond { amount: u128 }, ContractCall, ContractDeploy. AccessList has read and write arrays of AccountId. Use the same bincode layout as the Rust crate boing-primitives so the node accepts the tx.

**Multi-chain readiness**
- Design the app so that "networks" or "chains" are pluggable: e.g. a network adapter interface (get balance, build/sign tx, submit tx, get nonce, etc.) and a list of supported networks. Boing Network is the first and default. The UI should allow switching networks (or adding more later) without a full rewrite. Do not implement other chains now; just make the architecture ready (e.g. config-driven RPC URLs, chain id, and a Boing adapter).

**Security & key management**
- Keys must be generated and stored on the client (browser). Use a secure method (e.g. Web Crypto or a well-audited lib) for Ed25519 key generation. Store the encrypted private key in localStorage/sessionStorage or IndexedDB, with a user-chosen password or PIN for encryption/decryption. Never send private keys to any server. The Cloudflare backend (if any) should only proxy RPC or serve static assets; it must never see keys.

**UX (minimum for Boing)**
- Create wallet (generate Ed25519 keypair, show backup phrase or export instructions).
- Import wallet (from seed/phrase or hex private key, if you support it).
- View Boing address (32-byte hex) with copy button — this is what users need for the faucet and for receiving.
- Balance view: call **boing_getBalance([hex_account_id])** or **boing_getAccount([hex_account_id])** for balance (and nonce for building the next tx). Results use decimal strings for u128 to avoid JS precision issues.
- Send BOING: form (to address, amount), build Transfer tx, sign, submit via boing_submitTransaction.
- Testnet faucet: button or link that either (a) calls boing_faucetRequest with the current account address (if RPC supports it) or (b) opens the official faucet page (e.g. boing.network/network/faucet) with the address pre-filled.
- Network selector: Boing Mainnet / Boing Testnet (different RPC URLs). Default to Testnet for now if appropriate.

**Tech stack**
- Use a modern front-end framework (e.g. React, Vue, Svelte) and TypeScript. Build output should be static (or static + optional Workers) so it runs on Cloudflare Pages. Use environment variables or a small config for RPC URLs (e.g. testnet RPC, mainnet RPC) so they can be set at build or runtime.

**Deliverables**
- A Cloudflare-oriented project (e.g. npm/pnpm scripts, wrangler or Pages config if needed, README with Cloudflare deployment and boing.express domain setup).
- Boing Network integration: address display, send, faucet, and correct signing so transactions are accepted by a Boing node.
- Clean separation so another chain adapter can be added later without rewriting the wallet core.
```

---

## Reference: Boing Network repo

When implementing the wallet, use the **boing-network** repo as the source of truth:

| What | Where |
|------|--------|
| RPC methods, params, errors | `docs/RPC-API-SPEC.md` (incl. boing_getBalance, boing_getAccount for wallet) |
| Address = 32-byte AccountId (Ed25519 pubkey) | `crates/boing-primitives/src/types.rs` (AccountId) |
| Transaction, payload, AccessList | `crates/boing-primitives/src/types.rs` |
| Signable hash (BLAKE3 of nonce, sender, bincode payload, bincode access_list) | `crates/boing-primitives/src/signature.rs` (`signable_hash`) |
| SignedTransaction layout | `crates/boing-primitives/src/signature.rs` |
| Faucet (testnet) | RPC `boing_faucetRequest([hex_account_id])`; or link to boing.network/network/faucet |

For bincode compatibility, replicate the Rust struct/enum layout (field order and types) in your TypeScript/JS types and serialization.

---

## Cloudflare setup checklist (boing.express)

- [ ] Create a Cloudflare Pages project connected to the wallet repo.
- [ ] Build command and output directory set (e.g. `dist` or `build`).
- [ ] Add custom domain **boing.express** in Cloudflare (Pages → Custom domains).
- [ ] If using Workers for API: create a Worker, bind to a route (e.g. `boing.express/api/*`) if needed.
- [ ] Env vars for RPC URLs (e.g. `VITE_BOING_TESTNET_RPC`, `VITE_BOING_MAINNET_RPC`) in Pages build settings.

---

*This prompt lives in the boing-network repo so wallet implementers can reference it and the chain spec in one place.*
