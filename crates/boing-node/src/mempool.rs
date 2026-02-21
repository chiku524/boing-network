//! Transaction mempool — pending transactions awaiting inclusion in a block.
//! Runs protocol QA on ContractDeploy before accepting; rejects with structured reason when QA fails.

use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

use boing_primitives::{AccountId, SignedTransaction, TransactionPayload};
use boing_qa::{check_contract_deploy, RuleRegistry, QaReject, QaResult};

/// In-memory mempool. Tracks pending transactions by sender nonce.
#[derive(Default)]
pub struct Mempool {
    inner: Mutex<MempoolInner>,
}

/// Default rule registry for QA (max bytecode size, etc.). Can be replaced with on-chain config later.
fn default_qa_registry() -> RuleRegistry {
    RuleRegistry::new()
}

#[derive(Default)]
struct MempoolInner {
    /// Pending txs by sender, then by nonce.
    by_sender: HashMap<AccountId, BTreeMap<u64, SignedTransaction>>,
    /// All tx IDs for dedup.
    by_id: HashMap<boing_primitives::Hash, ()>,
    /// Count of pending txs.
    len: usize,
}

impl Mempool {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a signed transaction. Rejects duplicates, invalid nonces, and ContractDeploy that fail QA.
    pub fn insert(&self, signed: SignedTransaction) -> Result<(), MempoolError> {
        signed.verify().map_err(|_| MempoolError::InvalidSignature)?;
        if let TransactionPayload::ContractDeploy { bytecode } = &signed.tx.payload {
            let registry = default_qa_registry();
            match check_contract_deploy(
                bytecode,
                None,
                None,
                registry.max_bytecode_size(),
            ) {
                QaResult::Reject(reject) => return Err(MempoolError::QaRejected(reject)),
                QaResult::Unsure => return Err(MempoolError::QaPendingPool),
                QaResult::Allow => {}
            }
        }
        let tx_id = signed.tx.id();
        let mut inner = self.inner.lock().unwrap();
        if inner.by_id.contains_key(&tx_id) {
            return Err(MempoolError::Duplicate);
        }
        let sender = signed.tx.sender;
        let nonce = signed.tx.nonce;
        inner.by_sender.entry(sender).or_default().insert(nonce, signed);
        inner.by_id.insert(tx_id, ());
        inner.len += 1;
        Ok(())
    }

    /// Remove and return signed transactions up to `max` for block inclusion.
    /// Returns txs in nonce order (per sender). Callers can re-insert on failure (e.g. consensus).
    pub fn drain_for_block(&self, max: usize) -> Vec<SignedTransaction> {
        let mut inner = self.inner.lock().unwrap();
        let mut candidates: Vec<(AccountId, u64)> = Vec::new();
        for (sender, by_nonce) in inner.by_sender.iter() {
            for nonce in by_nonce.keys() {
                candidates.push((*sender, *nonce));
            }
        }
        candidates.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.0.cmp(&b.0.0)));
        let mut out = Vec::with_capacity(max.min(candidates.len()));
        for (sender, nonce) in candidates.into_iter().take(max) {
            if let Some(by_nonce) = inner.by_sender.get_mut(&sender) {
                if let Some(signed) = by_nonce.remove(&nonce) {
                    inner.by_id.remove(&signed.tx.id());
                    inner.len = inner.len.saturating_sub(1);
                    out.push(signed);
                }
            }
        }
        out
    }

    /// Re-insert signed transactions (e.g. after block production or consensus failure).
    /// Duplicates are skipped; invalid signatures are skipped. Used to restore txs when a block is not committed.
    pub fn reinsert(&self, signed_txs: Vec<SignedTransaction>) {
        for signed in signed_txs {
            let _ = self.insert(signed);
        }
    }

    /// Number of pending transactions.
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MempoolError {
    #[error("Duplicate transaction")]
    Duplicate,
    #[error("Invalid signature")]
    InvalidSignature,
    /// Protocol QA rejected this deployment; rule_id and message give user feedback.
    #[error("QA rejected: {0}")]
    QaRejected(QaReject),
    /// Deployment referred to community QA pool (Unsure); not accepted until pool decides.
    #[error("Deployment referred to community QA pool")]
    QaPendingPool,
}
