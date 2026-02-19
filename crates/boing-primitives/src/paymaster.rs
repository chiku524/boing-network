//! Paymaster / gasless transaction support.
//!
//! See BOING-BLOCKCHAIN-DESIGN-PLAN.md (Phase 5.1 Frictionless UX).

use serde::{Deserialize, Serialize};

use crate::SignedTransaction;

/// Paymaster that sponsors gas for a user transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymasterConfig {
    /// Paymaster account that pays gas.
    pub paymaster: crate::AccountId,
    /// Max gas the paymaster will sponsor per tx.
    pub max_gas_per_tx: u64,
}

/// Sponsored (gasless) transaction — user tx + paymaster signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SponsoredTransaction {
    /// The user's signed transaction (user signs, paymaster pays gas).
    pub inner: SignedTransaction,
    /// Paymaster account that will pay gas.
    pub paymaster: crate::AccountId,
    /// Paymaster's signature authorizing sponsorship.
    pub paymaster_signature: crate::Signature,
}
