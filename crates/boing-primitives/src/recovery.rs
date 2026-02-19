//! Social recovery — guardians, time locks.
//!
//! See BOING-BLOCKCHAIN-DESIGN-PLAN.md (Phase 5.3 Recoverability).

use serde::{Deserialize, Serialize};

use crate::AccountId;

/// Guardian for social recovery.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guardian {
    pub account: AccountId,
    /// Optional: weight for threshold schemes (1 = one vote).
    pub weight: u32,
}

/// Recovery request — initiate account recovery via guardians.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryRequest {
    /// Account to recover.
    pub target: AccountId,
    /// New owner/signer (after recovery).
    pub new_owner: AccountId,
    /// Guardian signatures (quorum required).
    pub guardian_approvals: Vec<(AccountId, crate::Signature)>,
    /// Time lock: executable after this block height.
    pub executable_after_block: u64,
}
