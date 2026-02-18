# Boing Network

Authentic, decentralized L1 blockchain — built from first principles.

## Quick Start

```bash
cargo build
cargo run -p boing-node
```

## Crates

| Crate | Description |
|-------|-------------|
| `boing-primitives` | Types, hashing (BLAKE3), cryptography |
| `boing-consensus` | PoS + HotStuff BFT |
| `boing-state` | State store (Verkle tree target) |
| `boing-execution` | VM + parallel transaction scheduler |
| `boing-automation` | Scheduler, triggers, executor incentives |
| `boing-cli` | `boing init`, `boing dev`, `boing deploy` |
| `boing-p2p` | libp2p networking |
| `boing-node` | Node binary |

## Docs

- [BOING-BLOCKCHAIN-DESIGN-PLAN.md](./BOING-BLOCKCHAIN-DESIGN-PLAN.md) — Architecture, design decisions, innovations
- [RUNBOOK.md](./RUNBOOK.md) — Operational runbook for node operators
- [DECENTRALIZATION-STRATEGY.md](./DECENTRALIZATION-STRATEGY.md) — Advanced P2P, peer discovery, light clients
- [ENHANCEMENT-VISION.md](./ENHANCEMENT-VISION.md) — Intent-based execution, storage, Boing Studio
- [WEBRTC-SIGNALING.md](./WEBRTC-SIGNALING.md) — Decentralized WebRTC signaling for browser light clients
- [SECURITY-STANDARDS.md](./SECURITY-STANDARDS.md) — Protocol, network, application, and operational security
- [BUILD-ROADMAP.md](./BUILD-ROADMAP.md) — Implementation tasks and phases
- [DEVELOPMENT-RECOMMENDATIONS.md](./DEVELOPMENT-RECOMMENDATIONS.md) — SDK, automation, dApp incentive recommendations
- [AUTOMATION-VERIFICATION.md](./AUTOMATION-VERIFICATION.md) — Cryptographic verification for decentralized automation
- [NETWORK-COST-ESTIMATE.md](./NETWORK-COST-ESTIMATE.md) — Cost overview and economic parameters

## Website

The [boing.network](https://boing.network) website lives in `website/`. It's built with Astro and deploys to Cloudflare Pages. See `website/README.md` and `website/CLOUDFLARE-SETUP.md` for setup and deployment.

## Priorities

Security → Scalability → Decentralization → Authenticity