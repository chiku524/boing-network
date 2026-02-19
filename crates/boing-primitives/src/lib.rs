//! Boing Blockchain — Core Primitives
//!
//! Types, hashing, signatures, and shared data structures.

pub mod hash;
pub mod hd;
pub mod intent;
pub mod paymaster;
pub mod randomness;
pub mod recovery;
pub mod signature;
pub mod types;

pub use hash::{Hash, hasher};
pub use signature::{
    sign_transaction, verify_signature, Signature, SignatureError, SignedTransaction,
};
pub use types::{
    tx_root, AccountId, Block, BlockHeader, Transaction, TransactionPayload, AccessList,
};
pub use types::{Account, AccountState};
pub use intent::{Intent, IntentKind, SignedIntent};
pub use randomness::{dummy_vrf_output, leader_from_vrf, VdfOutput, VrfOutput};
pub use paymaster::{PaymasterConfig, SponsoredTransaction};
pub use recovery::{Guardian, RecoveryRequest};
pub use hd::HdPath;

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_hash_deterministic() {
        let mut h1 = hasher();
        h1.update(b"hello");
        let out1 = h1.finalize();
        let mut h2 = hasher();
        h2.update(b"hello");
        let out2 = h2.finalize();
        assert_eq!(out1.as_bytes(), out2.as_bytes());
    }

    #[test]
    fn test_transaction_id_deterministic() {
        let id = AccountId::from_bytes([1u8; 32]);
        let tx = Transaction {
            nonce: 0,
            sender: id,
            payload: TransactionPayload::Transfer {
                to: id,
                amount: 100,
            },
            access_list: AccessList::default(),
        };
        assert_eq!(tx.id(), tx.id());
    }

    #[test]
    fn test_signed_transaction_verify() {
        let key = SigningKey::generate(&mut OsRng);
        let sender = AccountId::from_bytes(key.verifying_key().to_bytes());
        let tx = Transaction {
            nonce: 0,
            sender,
            payload: TransactionPayload::Transfer {
                to: AccountId::from_bytes([2u8; 32]),
                amount: 50,
            },
            access_list: AccessList::default(),
        };
        let signed = SignedTransaction::new(tx, &key);
        assert!(signed.verify().is_ok());
    }

    #[test]
    fn test_access_list_conflicts() {
        let a = AccountId::from_bytes([1u8; 32]);
        let b = AccountId::from_bytes([2u8; 32]);
        let c = AccountId::from_bytes([3u8; 32]);
        let al1 = AccessList::new(vec![a, b], vec![]);
        let al2 = AccessList::new(vec![c], vec![]);
        assert!(!al1.conflicts_with(&al2));
        let al3 = AccessList::new(vec![b, c], vec![]);
        assert!(al1.conflicts_with(&al3));
    }
}
