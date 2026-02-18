# boing.network — Website Specification

> **Domain:** boing.network  
> **Hosting:** Cloudflare Pages  
> **Backend:** Cloudflare Workers + D1 + R2 + KV

---

## 1. Site Structure

```
boing.network/
├── /                    → Landing: hero (animated), tokenomics (charts), roadmap, ecosystem, innovations, resources
├── /about               → Design philosophy, pillars, innovation overview
├── /docs/               → Single-page documentation (table of contents, anchor navigation)
│   │                    → Sections: Overview, Getting Started, Network, Tokenomics, Architecture, RPC API, Operations, Security, Governance, Resources
│   ├── /getting-started → Redirects to /docs#getting-started
│   └── /rpc-api         → Redirects to /docs#rpc-api
├── /developers/         → Developer resources
│   ├── /quickstart      → CLI, SDK, local dev
│   ├── /sdk             → boing init, dev, deploy
│   ├── /automation      → Scheduler, triggers, executor incentives
│   └── /rpc-reference   → Full RPC docs + examples
├── /network/            → Network status & explorer
│   ├── /status          → Uptime, validators, block height (D1 / API)
│   ├── /explorer        → Block/tx/account lookup (when available)
│   └── /testnet         → Testnet info, faucet, bootnodes
├── /community           → GitHub, Discord, governance, grants
└── /resources           → Tokenomics, roadmap, whitepapers, FAQs
```

---

## 2. Content Mapping (Source → Site)

| Page / Section | Source | Notes |
|----------------|--------|-------|
| Landing | BOING-BLOCKCHAIN-DESIGN-PLAN.md (Design Philosophy, Priority Pillars) | Hero, "Authentic L1" messaging |
| /about | Design Plan §1–5, Innovation table | Philosophy, unique features |
| /docs/getting-started | BUILD-ROADMAP.md Quick Start, README | `cargo build`, `cargo run -p boing-node` |
| /docs/architecture | Design Plan (Tech Stack, Full Stack Architecture) | Mermaid diagrams, layer breakdown |
| /docs/rpc-api | docs/RPC-API-SPEC.md | Full RPC spec, methods, error codes |
| /docs/runbook | RUNBOOK.md | Operator procedures |
| /docs/security | SECURITY-STANDARDS.md | DDoS, rate limits, incident response |
| /developers/quickstart | README, BUILD-ROADMAP | Crates, CLI, local dev |
| /developers/sdk | DEVELOPMENT-RECOMMENDATIONS.md | boing init, dev, deploy |
| /developers/automation | AUTOMATION-VERIFICATION.md | Verification types, incentives |
| /network/status | API (D1 or live RPC) | Block height, validator count |
| /network/testnet | To be defined | Bootnodes, faucet, config |
| /resources | BUILD-ROADMAP, NETWORK-COST-ESTIMATE | Roadmap, cost estimates |

---

## 3. Cloudflare Services Mapping

| Service | Use Case | Example |
|---------|----------|---------|
| **Cloudflare Pages** | Static site (HTML/JS/CSS) + SPA routing | Landing, docs, developer pages |
| **Cloudflare Workers** | API routes, serverless logic | `/api/status`, `/api/explorer`, redirects |
| **D1 Database** | Indexed data, analytics | Block explorer index, tx/account lookups, network stats |
| **R2 Storage** | Large assets, backups | Chain snapshots, archival data, docs PDFs |
| **KV** | Caching, session, rate-limit state | RPC response cache, rate-limit counters |

---

## 4. Key Pages — Content Outline

### Landing (/)

- **Headline:** Authentic L1 blockchain — built from first principles
- **Subhead:** Security → Scalability → Decentralization → Authenticity
- **CTA:** Get Started, Read Docs, Join Testnet
- **Features:** Native AA, Adaptive Gas, Cross-Chain Primitives, Transparent Slashing
- **Footer:** GitHub, Docs, Community, Resources

### /docs/getting-started

1. Prerequisites (Rust, system)
2. Clone & build: `cargo build`
3. Run node: `cargo run -p boing-node`
4. Optional: `--validator`, `--data-dir`, `--rpc-port`
5. First transaction (submit via RPC)

### /developers/quickstart

1. Crates overview (table from README)
2. CLI: `boing init`, `boing dev`, `boing deploy`
3. RPC base URL: `http://<host>:8545/`
4. Links to RPC reference, SDK, automation

### /network/status

- Block height (from live RPC or D1 index)
- Validator count / list
- Uptime / health (optional)

---

## 5. Tech Stack (Website)

- **Framework:** Astro (static-first, Markdown/MDX, Cloudflare adapter)
- **Styling:** Tailwind CSS or minimal custom CSS
- **Deployment:** Cloudflare Pages (GitHub → auto deploy)
- **API:** Cloudflare Workers (optional) for `/api/*` routes
- **Future:** D1 for block explorer, KV for caching

---

## 6. SEO & Meta

- **Slogan:** The DeFi that always bounces back
- Title: `Boing Network | The DeFi that always bounces back`
- Description: `The DeFi that always bounces back. Authentic L1 blockchain built from first principles.`
- Canonical URLs, Open Graph, Twitter cards, JSON-LD structured data
- OG image: `/og.png` (1200×630) — add for social sharing
- Favicon: `/favicon.svg`
- robots.txt, sitemap.xml

---

*Boing Network — Authentic. Decentralized. Optimal. Sustainable.*
