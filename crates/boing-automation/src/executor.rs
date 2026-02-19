//! Executor incentives and task definitions.

use boing_primitives::AccountId;

/// A task to be executed by a decentralized executor.
#[derive(Clone, Debug)]
pub struct AutomationTask {
    pub id: u64,
    pub schedule_id: u64,
    pub payload: Vec<u8>,
    pub gas_limit: u64,
}

/// Incentive model for executors (design).
/// - Executors stake BOING; slashed on failure.
/// - Rewards distributed per successful execution.
#[derive(Clone, Debug)]
pub struct ExecutorIncentive {
    /// Reward per successful execution.
    pub reward_per_task: u128,
    /// Slash amount on failure.
    pub slash_on_failure: u128,
    /// Min stake to register as executor.
    pub min_stake: u128,
}

impl Default for ExecutorIncentive {
    fn default() -> Self {
        Self {
            reward_per_task: 1000,
            slash_on_failure: 5000,
            min_stake: 10_000,
        }
    }
}

/// Executor registration (stub for future).
#[derive(Clone, Debug)]
pub struct ExecutorRegistration {
    pub executor: AccountId,
    pub stake: u128,
}

/// Compute reward for a successful task execution.
pub fn executor_reward(incentive: &ExecutorIncentive, _task: &AutomationTask) -> u128 {
    incentive.reward_per_task
}

/// Compute slash amount for failed or malicious execution.
pub fn executor_slash(incentive: &ExecutorIncentive, _task: &AutomationTask) -> u128 {
    incentive.slash_on_failure
}
