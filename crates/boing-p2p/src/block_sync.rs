//! Block sync protocol — request/response for fetching blocks by hash or height.

use serde::{Deserialize, Serialize};

use boing_primitives::{Block, Hash};

/// Request for a block (by hash or height).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlockRequest {
    ByHash([u8; 32]),
    ByHeight(u64),
}

/// Response: block if found, or unit meaning not found.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockResponse(pub Option<Block>);

impl BlockRequest {
    pub fn by_hash(h: &Hash) -> Self {
        BlockRequest::ByHash(h.0)
    }
}
