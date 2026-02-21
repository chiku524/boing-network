//! Protocol Quality Assurance — deterministic checks for asset deployment.
//!
//! See [QUALITY-ASSURANCE-NETWORK.md](https://github.com/boing-network/boing-network/blob/main/docs/QUALITY-ASSURANCE-NETWORK.md) for the full design.
//!
//! This crate provides:
//! - [QaResult]: Allow | Reject | Unsure
//! - [RuleId] and [QaReject] for structured rejection
//! - [check_contract_deploy]: stub implementation (to be replaced with full rules)


/// Outcome of a QA check: allow deployment, reject, or send to community pool (unsure).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QaResult {
    /// Deployment passes all checks; allow inclusion.
    Allow,
    /// Deployment fails a rule; reject with reason.
    Reject(QaReject),
    /// Automation cannot firmly decide; refer to community QA pool.
    Unsure,
}

/// Structured rejection reason for diagnostics and RPC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QaReject {
    pub rule_id: RuleId,
    pub message: String,
}

impl std::fmt::Display for QaReject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} — {}", self.rule_id.0, self.message)
    }
}

/// Identifies a QA rule (e.g. max size, opcode whitelist, blocklist).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuleId(pub String);

impl RuleId {
    pub const MAX_BYTECODE_SIZE: &'static str = "MAX_BYTECODE_SIZE";
    pub const INVALID_OPCODE: &'static str = "INVALID_OPCODE";
    pub const MALFORMED_BYTECODE: &'static str = "MALFORMED_BYTECODE";
    pub const BLOCKLIST_MATCH: &'static str = "BLOCKLIST_MATCH";
    pub const PURPOSE_DECLARATION_INVALID: &'static str = "PURPOSE_DECLARATION_INVALID";
}

/// Default maximum bytecode size (bytes). Governance can change via rule registry.
pub const DEFAULT_MAX_BYTECODE_SIZE: usize = 32 * 1024; // 32 KiB

/// Stub: check ContractDeploy bytecode. Returns Allow for non-empty bytecode within size limit;
/// Reject for empty or over size; full rule set (opcodes, well-formedness, purpose, blocklist)
/// to be implemented per QUALITY-ASSURANCE-NETWORK.md.
pub fn check_contract_deploy(
    bytecode: &[u8],
    _purpose_category: Option<&str>,
    _description_hash: Option<&[u8]>,
    max_bytecode_size: usize,
) -> QaResult {
    if bytecode.is_empty() {
        return QaResult::Reject(QaReject {
            rule_id: RuleId(RuleId::MALFORMED_BYTECODE.to_string()),
            message: "Bytecode must not be empty".to_string(),
        });
    }
    if bytecode.len() > max_bytecode_size {
        return QaResult::Reject(QaReject {
            rule_id: RuleId(RuleId::MAX_BYTECODE_SIZE.to_string()),
            message: format!(
                "Bytecode size {} exceeds maximum {}",
                bytecode.len(),
                max_bytecode_size
            ),
        });
    }
    // TODO: opcode whitelist, well-formedness, purpose declaration, blocklist, known edge-case resolutions (§11)
    QaResult::Allow
}

/// In-memory rule registry stub. Production: on-chain or governance-driven registry.
#[derive(Default)]
pub struct RuleRegistry {
    max_bytecode_size: usize,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self {
            max_bytecode_size: DEFAULT_MAX_BYTECODE_SIZE,
        }
    }

    pub fn with_max_bytecode_size(mut self, size: usize) -> Self {
        self.max_bytecode_size = size;
        self
    }

    pub fn max_bytecode_size(&self) -> usize {
        self.max_bytecode_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_empty_bytecode() {
        let r = check_contract_deploy(&[], None, None, DEFAULT_MAX_BYTECODE_SIZE);
        assert!(matches!(r, QaResult::Reject(_)));
    }

    #[test]
    fn reject_over_size() {
        let big = vec![0u8; DEFAULT_MAX_BYTECODE_SIZE + 1];
        let r = check_contract_deploy(&big, None, None, DEFAULT_MAX_BYTECODE_SIZE);
        assert!(matches!(r, QaResult::Reject(_)));
    }

    #[test]
    fn allow_small_bytecode() {
        let r = check_contract_deploy(&[0x00], None, None, DEFAULT_MAX_BYTECODE_SIZE); // STOP
        assert!(matches!(r, QaResult::Allow));
    }
}
