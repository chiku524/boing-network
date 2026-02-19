//! Parallel execution support — isolated views for Transfer-only batches.

use std::collections::HashMap;

use boing_primitives::{Account, AccountId, AccountState};
use boing_state::StateStore;

/// Isolated state view for parallel execution. Only supports Transfer txs.
#[derive(Default)]
pub struct ExecutionView {
    accounts: HashMap<AccountId, AccountState>,
}

impl ExecutionView {
    pub fn from_snapshot(snapshot: HashMap<AccountId, AccountState>) -> Self {
        Self { accounts: snapshot }
    }

    pub fn get(&self, id: &AccountId) -> Option<AccountState> {
        self.accounts.get(id).cloned()
    }

    pub fn get_mut(&mut self, id: &AccountId) -> Option<&mut AccountState> {
        self.accounts.get_mut(id)
    }

    pub fn insert(&mut self, account: Account) {
        self.accounts.insert(account.id, account.state);
    }

    /// Account IDs touched by this view (for conflict detection).
    pub fn account_ids(&self) -> impl Iterator<Item = &AccountId> {
        self.accounts.keys()
    }

    /// Merge this view's updates back into state. Used after parallel execution.
    pub fn merge_into(&self, state: &mut StateStore) {
        for (id, account_state) in &self.accounts {
            state.merge_account(*id, account_state.clone());
        }
    }
}
