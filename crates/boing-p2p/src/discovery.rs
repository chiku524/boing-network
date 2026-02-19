//! Advanced decentralized peer discovery configuration.
//!
//! See DECENTRALIZATION-STRATEGY.md for full design.

use serde::{Deserialize, Serialize};

/// Peer reputation score for Sybil/eclipse resistance.
/// Higher = more trustworthy; used to prioritize connections.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerScore {
    /// Reliability: successful responses vs failures.
    pub reliability: i32,
    /// Latency: lower is better (negative weight).
    pub latency_ms: u32,
    /// Uptime: blocks or time connected.
    pub uptime_blocks: u64,
}

impl PeerScore {
    pub fn new(reliability: i32, latency_ms: u32, uptime_blocks: u64) -> Self {
        Self {
            reliability,
            latency_ms,
            uptime_blocks,
        }
    }

    /// Combined score for ranking peers (higher = prefer).
    pub fn rank(&self) -> i64 {
        (self.reliability as i64)
            .saturating_sub(self.latency_ms as i64 / 10)
            .saturating_add((self.uptime_blocks as i64).min(1000))
    }
}

/// Bootnode entry for rotation; governance or community-funded.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootnodeEntry {
    pub multiaddr: String,
    pub peer_id: Option<String>,
}

/// Discovery configuration for DHT + gossip-first.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PeerDiscoveryConfig {
    /// Use DHT-based discovery (Kademlia).
    pub use_dht: bool,
    /// Use gossip-first overlay (exchange peer lists via existing connections).
    pub use_gossip_first: bool,
    /// Bootnodes; rotated via governance when use_governance_rotation.
    pub bootnodes: Vec<BootnodeEntry>,
    /// When true, bootnodes are fetched/rotated from chain.
    pub use_governance_rotation: bool,
    /// Min peers to maintain.
    pub min_peers: u32,
    /// Max peers.
    pub max_peers: u32,
}

impl PeerDiscoveryConfig {
    pub fn default_mainnet() -> Self {
        Self {
            use_dht: true,
            use_gossip_first: true,
            bootnodes: vec![],
            use_governance_rotation: false,
            min_peers: 10,
            max_peers: 50,
        }
    }

    pub fn default_devnet() -> Self {
        Self {
            use_dht: false,
            use_gossip_first: true,
            bootnodes: vec![],
            use_governance_rotation: false,
            min_peers: 2,
            max_peers: 20,
        }
    }
}
