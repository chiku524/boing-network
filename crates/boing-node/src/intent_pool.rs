//! Intent pool — stores signed intents for solver/executor fulfillment.
//! Stub implementation; production would persist and integrate with solver network.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use boing_primitives::{Hash, SignedIntent};

/// In-memory intent pool.
#[derive(Clone, Debug, Default)]
pub struct IntentPool {
    inner: Arc<RwLock<IntentPoolInner>>,
}

#[derive(Debug, Default)]
struct IntentPoolInner {
    intents: HashMap<[u8; 32], SignedIntent>,
}

impl IntentPool {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(IntentPoolInner::default())),
        }
    }

    /// Submit a signed intent. Returns intent ID (hash) if accepted.
    pub fn submit(&self, signed: SignedIntent) -> Result<Hash, IntentPoolError> {
        if signed.verify().is_err() {
            return Err(IntentPoolError::InvalidSignature);
        }
        let id = signed.intent.id();
        let mut inner = self.inner.write().unwrap();
        inner.intents.insert(id.0, signed);
        Ok(id)
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().intents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IntentPoolError {
    #[error("Invalid intent signature")]
    InvalidSignature,
}
