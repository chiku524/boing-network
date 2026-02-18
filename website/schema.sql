-- D1 schema for boing.network (block explorer / network stats)
-- Apply with: wrangler d1 execute boing-network-db --file=./schema.sql

CREATE TABLE IF NOT EXISTS blocks (
  height INTEGER PRIMARY KEY,
  hash TEXT NOT NULL,
  parent_hash TEXT,
  proposer TEXT,
  tx_count INTEGER,
  created_at TEXT
);

CREATE TABLE IF NOT EXISTS accounts (
  id TEXT PRIMARY KEY,
  balance TEXT,
  nonce INTEGER,
  updated_at TEXT
);

CREATE TABLE IF NOT EXISTS network_stats (
  id TEXT PRIMARY KEY DEFAULT 'latest',
  block_height INTEGER,
  validator_count INTEGER,
  updated_at TEXT
);
