//! Block import and validation — validate blocks from peers.

use boing_primitives::{tx_root, Account, AccountId, AccountState, Block, Hash};
use boing_consensus::ConsensusEngine;
use boing_execution::BlockExecutor;
use boing_state::StateStore;
use boing_tokenomics::block_emission_validators;

/// Validate and execute a block. Returns updated state on success.
/// Caller must ensure block chains to parent (parent_hash, height).
pub fn validate_and_execute_block(
    block: &Block,
    parent_state: &StateStore,
    validator_set: &[AccountId],
    executor: &BlockExecutor,
) -> Result<StateStore, BlockValidationError> {
    // 1. Tx root
    let expected_tx_root = tx_root(&block.transactions);
    if block.header.tx_root != expected_tx_root {
        return Err(BlockValidationError::InvalidTxRoot);
    }

    // 2. Proposer in validator set
    if !validator_set.contains(&block.header.proposer) {
        return Err(BlockValidationError::InvalidProposer);
    }

    // 3. Execute on snapshot
    let mut state = parent_state.snapshot();
    if let Err(e) = executor.execute_block(&block.transactions, &mut state) {
        return Err(BlockValidationError::ExecutionFailed(e.to_string()));
    }

    // 4. Block reward
    let reward = block_emission_validators(block.header.height);
    if reward > 0 {
        match state.get_mut(&block.header.proposer) {
            Some(s) => s.balance = s.balance.saturating_add(reward),
            None => {
                state.insert(Account {
                    id: block.header.proposer,
                    state: AccountState { balance: reward, nonce: 0, stake: 0 },
                });
            }
        }
    }

    // 5. State root
    let computed_root = state.state_root();
    if block.header.state_root != computed_root {
        return Err(BlockValidationError::InvalidStateRoot {
            expected: block.header.state_root,
            computed: computed_root,
        });
    }

    Ok(state)
}

/// Check that a block chains to our tip. Does not execute.
pub fn chains_to(block: &Block, our_latest_hash: Hash, our_height: u64) -> bool {
    block.header.parent_hash == our_latest_hash && block.header.height == our_height + 1
}

/// Full import: validate block and return new state if it chains and is valid.
pub fn import_block(
    block: &Block,
    our_latest_hash: Hash,
    our_height: u64,
    parent_state: &StateStore,
    consensus: &ConsensusEngine,
    executor: &BlockExecutor,
) -> Result<StateStore, BlockValidationError> {
    if !chains_to(block, our_latest_hash, our_height) {
        return Err(BlockValidationError::DoesNotChain);
    }
    let validator_set = consensus.validators();
    if validator_set.is_empty() {
        return Err(BlockValidationError::NoValidators);
    }
    validate_and_execute_block(block, parent_state, validator_set, executor)
}

#[derive(Debug, thiserror::Error)]
pub enum BlockValidationError {
    #[error("Block does not chain to our tip")]
    DoesNotChain,
    #[error("Invalid tx root")]
    InvalidTxRoot,
    #[error("Invalid proposer")]
    InvalidProposer,
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid state root: expected {expected:?}, computed {computed:?}")]
    InvalidStateRoot { expected: Hash, computed: Hash },
    #[error("No validators configured")]
    NoValidators,
}
