//! Tests for Bond and Unbond staking transactions.

use boing_execution::Vm;
use boing_primitives::{AccessList, Account, AccountId, AccountState, Transaction, TransactionPayload};
use boing_state::StateStore;

#[test]
fn test_bond_and_unbond() {
    let vm = Vm::new();
    let a = AccountId::from_bytes([1u8; 32]);
    let mut state = StateStore::new();
    state.insert(Account {
        id: a,
        state: AccountState { balance: 1000, nonce: 0, stake: 0 },
    });

    let bond_tx = Transaction {
        nonce: 0,
        sender: a,
        payload: TransactionPayload::Bond { amount: 300 },
        access_list: AccessList::new(vec![a], vec![a]),
    };
    vm.execute(&bond_tx, &mut state).unwrap();
    assert_eq!(state.get(&a).unwrap().balance, 700);
    assert_eq!(state.get(&a).unwrap().stake, 300);
    assert_eq!(state.get(&a).unwrap().nonce, 1);

    let unbond_tx = Transaction {
        nonce: 1,
        sender: a,
        payload: TransactionPayload::Unbond { amount: 100 },
        access_list: AccessList::new(vec![a], vec![a]),
    };
    vm.execute(&unbond_tx, &mut state).unwrap();
    assert_eq!(state.get(&a).unwrap().balance, 800);
    assert_eq!(state.get(&a).unwrap().stake, 200);
}
