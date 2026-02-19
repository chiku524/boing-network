//! Boing Consensus — PoS + HotStuff-style BFT
//!
//! Permissionless validator set, deterministic finality.

mod engine;

pub use engine::{ConsensusEngine, ConsensusError};
pub use boing_primitives::{Block, BlockHeader, Hash};
