# Boing Network — Cryptographic Verification for Decentralized Automation

> **Purpose:** Define how automated tasks are cryptographically verified for trustless, reliable execution.  
> **References:** [DEVELOPMENT-RECOMMENDATIONS.md](./DEVELOPMENT-RECOMMENDATIONS.md), [BOING-BLOCKCHAIN-DESIGN-PLAN.md](./BOING-BLOCKCHAIN-DESIGN-PLAN.md)

---

## Overview

The Boing Network's native decentralized automation layer requires robust cryptographic verification to ensure trustless execution for developers, validators, and users. The verification method depends on whether the task is **on-chain** or involves **off-chain** elements.

---

## 1. On-Chain Automation

For automation that resides entirely within the Boing blockchain (scheduled smart contract calls, native cron, auto-compounding), verification is inherent:

### 1.1 Cryptographically Signed Transactions

- Every automated action generates a **signed transaction**.
- Transactions are signed by the Decentralized Executor or protocol-level automation contract.
- All network nodes **mathematically verify** digital signatures; invalid signatures cause rejection.
- **Implementation:** Uses Ed25519 via `SignedTransaction`; verification in consensus and execution layers.

### 1.2 Deterministic Execution

- Smart contract code is **deterministic**.
- Given the same starting state and inputs, every honest node produces the same final state.
- Any deviation indicates a fault.
- **Implementation:** Boing VM (interpreter, bytecode spec) is deterministic; parallel execution preserves determinism via access-list batching.

### 1.3 Consensus Mechanism

- PoS + BFT finality ensures a **supermajority** of validators agree on block order and validity.
- Once a block is finalized, execution is **immutable** and verified by the network.
- **Implementation:** HotStuff consensus; 2f+1 quorum; equivocation detection and slashing.

---

## 2. Off-Chain Automation & External Data

When tasks require off-chain computation or external data (e.g. "When token X hits price Y, execute Z"), additional cryptographic techniques apply:

### 2.1 Zero-Knowledge Proofs (ZKPs)

| Aspect | Description |
|--------|-------------|
| **Mechanism** | An Executor proves a computation was performed correctly *without revealing inputs or computation details*. |
| **Boing Application** | dApps perform complex calculations off-chain (e.g. risk assessment), submit a concise ZKP to the chain, and an on-chain contract validates it before triggering automation. |
| **Status** | Design target; SDK to provide ZKP generation helpers. |

### 2.2 Optimistic Rollups / Fraud Proofs

| Aspect | Description |
|--------|-------------|
| **Mechanism** | Results are optimistically assumed correct. During a challenge period, anyone can submit a **Fraud Proof** (cryptographic evidence of incorrect execution). If fraud is proven, the Executor is slashed. |
| **Boing Application** | Cost-effective for tasks where a challenge delay is acceptable; scalable off-chain verification. |
| **Status** | `FraudProof` type in automation crate; integration TBD. |

### 2.3 Decentralized Oracle Networks

| Aspect | Description |
|--------|-------------|
| **Mechanism** | Multiple oracle nodes aggregate external data, cryptographically sign it, and provide attestations. |
| **Boing Application** | Native automation integrates oracle data so conditions like "token X price = Y" are verifiable via oracle attestations. |
| **Status** | `OracleAttestation` design; oracle network integration TBD. |

### 2.4 Attestations by Decentralized Executors

| Aspect | Description |
|--------|-------------|
| **Mechanism** | Executors cryptographically sign execution reports. Stake provides economic incentive for honesty; slashing punishes incorrect or malicious signing. |
| **Boing Application** | Executors sign messages confirming action and parameters; protocol verifies signatures and applies slashing. |
| **Status** | `ExecutorAttestation` implemented; signed execution reports. |

---

## 3. Protocol-Level Integration

### 3.1 Native Scheduler & Trigger Verification

- Scheduler and trigger components **demand and verify** appropriate proofs or attestations per task type.
- On-chain tasks: standard tx verification.
- Off-chain tasks: ZKP, Fraud Proof, or Executor Attestation.

### 3.2 Transparent Incentives & Slashing

- **Rewards:** Executors earn BOING for correct, timely execution.
- **Slashing:** Incorrect execution, missed tasks, or fraud proofs trigger stake slashing.
- **Implementation:** `ExecutorIncentive`, `ExecutorRegistration`; slashing wired to consensus equivocation and automation verification outcomes.

### 3.3 Boing SDK Support

- SDK provides simplified interfaces for:
  - ZKP generation for off-chain dApp logic.
  - Interaction with the native oracle layer.
  - Submitting and verifying Executor attestations.

---

## Summary: Verification by Task Type

| Task Type | Verification Method |
|-----------|---------------------|
| On-chain scheduled call | Signed tx + deterministic execution + consensus |
| On-chain cron/trigger | Same as above |
| Off-chain compute (ZKP) | ZKP validated on-chain |
| Off-chain compute (optimistic) | Fraud Proof during challenge period |
| External data condition | Oracle attestations |
| Executor-initiated action | Executor attestation (signed report) + slashing |

---

---

## Implementation

| Type | Location | Description |
|------|----------|-------------|
| `ExecutorAttestation` | `boing-automation::verification` | Signed execution report; `new()`, `verify()` |
| `ExecutionProof` | `boing-automation::verification` | Enum: Attestation, Zkp, FraudProof |
| `ZkpProof` | `boing-automation::verification` | Placeholder for ZKP bytes |
| `FraudProof` | `boing-automation::verification` | Evidence of incorrect execution |
| `OracleAttestation` | `boing-automation::verification` | Oracle data + quorum signatures |

See [boing-automation](crates/boing-automation/) crate.
