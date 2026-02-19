# Boing Network — Decentralized WebRTC Signaling

> **Purpose:** Enable browser-based light clients via WebRTC without centralized signaling servers.  
> **References:** [DECENTRALIZATION-STRATEGY.md](./DECENTRALIZATION-STRATEGY.md), [ENHANCEMENT-VISION.md](./ENHANCEMENT-VISION.md)

---


## Table of Contents

1. [Overview](#1-overview)
2. [Boing Mainnet as Signaling Channel](#2-boing-mainnet-as-signaling-channel)
3. [Preventing Offer/Answer Spam](#3-preventing-offeranswer-spam)
4. [Decentralized Storage for Large Payloads](#4-decentralized-storage-for-large-payloads)
5. [DHT-Enhanced Peer Discovery](#5-dht-enhanced-peer-discovery)
6. [Incentivized STUN/TURN Servers](#6-incentivized-stunturn-servers)
7. [STUN/TURN Reputation System](#7-stunturn-reputation-system)
8. [End-to-End Signaling Flow](#8-end-to-end-signaling-flow)

---

## 1. Overview

WebRTC requires a **signaling** process to exchange network information (IP addresses, ports, SDPs) between peers before a direct P2P connection can be established. Centralized signaling servers are single points of failure and censorship.

**Goal:** Eliminate centralized signaling; leverage Boing mainnet, decentralized storage, DHT, and incentivized NAT traversal.

---

## 2. Boing Mainnet as Signaling Channel

### Offer/Answer Smart Contract

| Component | Description |
|-----------|-------------|
| **Contract** | Dedicated signaling contract on Boing for offer/answer exchange. |
| **Offer Flow** | Peer A encrypts WebRTC offer (SDP) with Peer B's public key; posts to contract or stores CID (see §4). |
| **Answer Flow** | Peer B fetches offer, decrypts, generates answer, encrypts, posts back to contract. |
| **Event Logging** | Contract emits events on offer/answer post; peers listen via Boing SDK. |

### Benefits

- **Native decentralization** — Inherits censorship resistance of Boing mainnet.
- **Verifiable history** — Signaling exchanges recorded on-chain.
- **Considerations** — Minimize tx cost and latency via efficient contract design; use off-chain storage for large SDPs.

---

## 3. Preventing Offer/Answer Spam

To prevent malicious actors from flooding the signaling contract with offers/answers:

| Mechanism | Description |
|-----------|-------------|
| **Transaction Fees (Gas)** | Every contract interaction incurs gas. Adaptive gas model and predictable pricing make high-volume spam prohibitively expensive. |
| **On-Chain Rate Limiting** | Contract tracks `last_sent_timestamp` and `message_count` per address. E.g. max 5 offers per minute per address. Directly caps volume from any single identity. |
| **Staking or Deposit** | Initiating an offer requires a small BOING deposit held by the contract. Returned on successful connection or after expiration; forfeited or partially returned if rejected/expired. Raises economic barrier for spam. |
| **Identity and Reputation** | Optional: require minimum reputation score or Soulbound credentials to send offers. Ties signaling privileges to established network participation. |
| **Challenge-Response** | For sensitive offers: recipient can request a lightweight proof (e.g. simple puzzle) before processing. Adds computational cost to illegitimate offers without burdening legitimate users. |

---

## 4. Decentralized Storage for Large Payloads

| Aspect | Description |
|--------|-------------|
| **Small Messages** | ICE candidates and small SDPs can go directly on-chain. |
| **Large SDPs** | Store encrypted SDPs on IPFS/Filecoin/Arweave. |
| **On-Chain Pointers** | Post only CID (Content Identifier) or hash + recipient public key to Boing contract. |
| **Retrieval** | Recipient fetches CID from chain, downloads from decentralized storage, decrypts. |

**Benefits:** Reduces on-chain load and costs; leverages specialized decentralized storage infrastructure.

---

## 5. DHT-Enhanced Peer Discovery

| Component | Description |
|-----------|-------------|
| **DHT Announcement** | Peers announce WebRTC capability and public key/Boing address in Kademlia DHT. |
| **Recipient Lookup** | Before initiating signaling, peer uses DHT to confirm recipient is online and fetch metadata. |
| **Integration** | Complements libp2p discovery in DECENTRALIZATION-STRATEGY. |

**Benefits:** Resilient, dynamic discovery; no central directory.

---

## 6. Incentivized STUN/TURN Servers

| Aspect | Description |
|--------|-------------|
| **Role** | STUN (Session Traversal Utilities for NAT) and TURN (Traversal Using Relays around NAT) handle NAT traversal. |
| **Decentralization** | Community members run STUN/TURN nodes. |
| **Protocol Incentives** | Reward reliable STUN/TURN providers (similar to validators or storage providers). |
| **Distribution** | Ensures geographically distributed relay capacity. |

---

## 7. STUN/TURN Reputation System

To establish trust and reliability for community-run STUN/TURN servers:

| Component | Description |
|-----------|-------------|
| **On-Chain Registry** | Smart contract for STUN/TURN server registration. Requires minimum BOING stake. Stores network address and metadata. |
| **Performance Metrics** | Nodes/dApps monitor and report: **uptime**, **latency**, **success rate**, **bandwidth/throughput**. Data submitted to registry; optionally aggregated via decentralized oracle. |
| **Reputation Score** | Contract maintains dynamic score per server from aggregated metrics. Score decays over time to prioritize recent performance. |
| **Selection** | SDK allows dApps to query registry and select servers by reputation, geography, or other criteria. Drives competition among providers. |
| **Slashing** | Servers that underperform, act maliciously, or fail minimum uptime face stake slashing. Proportional to severity and duration. Aligns with Boing's slashing and appeal mechanisms. |

---

## 8. End-to-End Signaling Flow

1. **Discovery:** Peers use DHT to find each other and confirm WebRTC readiness.
2. **Offer:** Peer A encrypts SDP with Peer B's public key; uploads to IPFS or posts on-chain; posts CID to Boing contract.
3. **Answer:** Peer B retrieves offer, decrypts, posts encrypted answer (or CID) to contract.
4. **ICE Candidates:** Both exchange ICE candidates via contract or low-latency gossip.
5. **Connection:** Direct WebRTC connection; use STUN/TURN for NAT traversal as needed.

---

*Boing Network — Authentic. Decentralized. Optimal. Sustainable.*
