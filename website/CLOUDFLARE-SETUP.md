# Boing Network — Cloudflare Project Setup

> **Domain:** boing.network  
> **Purpose:** Host boing.network website and backend services (D1, R2, KV)

---

## Overview

Create a Cloudflare project that includes:

| Resource | Purpose |
|----------|---------|
| **Cloudflare Pages** | Website (boing.network) — static + optional API routes |
| **D1 Database** | Indexed chain data for block explorer, stats, API |
| **R2 Storage** | Backups, archival data, large assets |
| **KV Namespace** | Caching, rate limiting, session data |

---

## Step 1: Add Domain to Cloudflare

1. Log in to [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. **Add a site** → Enter `boing.network`
3. Select a plan (Free is fine for Pages + Workers)
4. Update nameservers at your registrar to Cloudflare’s

---

## Step 2: Create D1 Database

```bash
# Install Wrangler CLI (if not already)
npm install -g wrangler

# Create D1 database
wrangler d1 create boing-network-db
```

Use the output `database_id` in `wrangler.toml`:

```toml
[[d1_databases]]
binding = "DB"
database_name = "boing-network-db"
database_id = "<YOUR_DATABASE_ID>"
```

### Example schema (for block explorer / status)

```sql
-- Blocks index (populated by indexer or API)
CREATE TABLE IF NOT EXISTS blocks (
  height INTEGER PRIMARY KEY,
  hash TEXT NOT NULL,
  parent_hash TEXT,
  proposer TEXT,
  tx_count INTEGER,
  created_at TEXT
);

-- Accounts cache
CREATE TABLE IF NOT EXISTS accounts (
  id TEXT PRIMARY KEY,
  balance TEXT,
  nonce INTEGER,
  updated_at TEXT
);

-- Network stats (updated periodically)
CREATE TABLE IF NOT EXISTS network_stats (
  id TEXT PRIMARY KEY DEFAULT 'latest',
  block_height INTEGER,
  validator_count INTEGER,
  updated_at TEXT
);
```

Apply schema:

```bash
wrangler d1 execute boing-network-db --file=./schema.sql
```

---

## Step 3: Create R2 Bucket

```bash
# Create R2 bucket
wrangler r2 bucket create boing-network-assets
```

Add to `wrangler.toml`:

```toml
[[r2_buckets]]
binding = "ASSETS"
bucket_name = "boing-network-assets"
```

**Typical uses:**
- Chain snapshots / backups
- Archived block data
- Large docs (PDFs)
- Static assets if desired

---

## Step 4: Create KV Namespace

```bash
# Production KV
wrangler kv:namespace create "BOING_CACHE"

# Preview KV (for local/dev)
wrangler kv:namespace create "BOING_CACHE" --preview
```

Add to `wrangler.toml`:

```toml
[[kv_namespaces]]
binding = "CACHE"
id = "<PRODUCTION_KV_ID>"

[[kv_namespaces]]
binding = "CACHE"
id = "<PREVIEW_KV_ID>"
preview = true
```

**Typical uses:**
- RPC response caching (e.g. `boing_chainHeight`, block by height)
- Rate-limit counters for public API
- Feature flags, session tokens

---

## Step 5: Cloudflare Pages Project

### Option A: GitHub integration (recommended)

1. Go to **Workers & Pages** → **Create** → **Pages** → **Connect to Git**
2. Select the `boing-network` repo
3. Configure:
   - **Build command:** `cd website && npm run build`
   - **Build output:** `website/dist`
   - **Root directory:** (leave blank or `/` if website is at repo root)
4. Add environment variables if needed (e.g. `CF_ACCOUNT_ID`)

### Option B: Wrangler

```bash
cd website
wrangler pages project create boing-network --production-branch main
wrangler pages deploy dist --project-name=boing-network
```

`wrangler.toml` is kept minimal (no D1/R2/KV) for static Pages deploy. For Workers/API with bindings use `wrangler.worker.toml`.

---

## Step 6: Custom Domain

1. In **Workers & Pages** → **boing-network** → **Custom domains**
2. Add `boing.network` and `www.boing.network`
3. Cloudflare will provision SSL

---

## Step 7: Workers with D1 + R2 + KV (optional API)

If you want API routes (e.g. `/api/status`, `/api/blocks/:height`), create a Worker:

```toml
# wrangler.toml (in website/ or api/)
name = "boing-network-api"
main = "src/worker.ts"
compatibility_date = "2024-01-01"

[[d1_databases]]
binding = "DB"
database_name = "boing-network-db"
database_id = "<YOUR_DATABASE_ID>"

[[r2_buckets]]
binding = "ASSETS"
bucket_name = "boing-network-assets"

[[kv_namespaces]]
binding = "CACHE"
id = "<YOUR_KV_ID>"
```

Route the Worker to `api.boing.network` or `boing.network/api/*` via **Workers Routes**.

---

## Step 8: Wrangler Config Layout

Suggested layout for the boing-network repo:

```
boing-network/
├── website/              # Astro site
│   ├── wrangler.toml     # Pages or Pages + Worker
│   ├── schema.sql        # D1 schema
│   └── ...
├── CLOUDFLARE-SETUP.md   # This file
└── ...
```

Example minimal `website/wrangler.toml` for Pages only:

```toml
name = "boing-network"
pages_build_output_dir = "dist"
compatibility_date = "2024-01-01"
```

For Pages + Functions (with D1/KV/R2), use the [Cloudflare adapter for Astro](https://docs.astro.build/en/guides/deploy/cloudflare/) and add bindings there.

---

## Checklist

- [ ] Domain `boing.network` added to Cloudflare
- [ ] D1 database `boing-network-db` created
- [ ] R2 bucket `boing-network-assets` created
- [ ] KV namespace `BOING_CACHE` created
- [ ] Pages project linked to GitHub
- [ ] Custom domain `boing.network` on Pages
- [ ] (Optional) Worker with D1/R2/KV bindings for API
- [ ] Environment variables set in Cloudflare dashboard if needed

---

*Boing Network — Authentic. Decentralized. Optimal. Sustainable.*
