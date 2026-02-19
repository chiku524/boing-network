//! Boing Execution — VM and parallel transaction scheduler
//!
//! Declared dependencies (access lists) enable deterministic parallel execution.

mod bytecode;
mod executor;
mod gas;
mod interpreter;
mod parallel;
mod scheduler;
mod vm;

pub use bytecode::{gas as bytecode_gas, Opcode};
pub use executor::{BlockExecutor, ExecutionError};
pub use gas::GasConfig;
pub use interpreter::{Interpreter, StorageAccess};
pub use parallel::ExecutionView;
pub use scheduler::TransactionScheduler;
pub use vm::{TransferState, Vm, VmError};
pub use boing_primitives::{Transaction, AccessList};
