//! Boing Automation — native scheduler and trigger primitives.
//!
//! Enables decentralized executors to run scheduled and event-driven tasks.

pub mod scheduler;
pub mod trigger;
pub mod executor;
pub mod verification;
pub mod hooks;

pub use scheduler::{CronSchedule, Scheduler};
pub use trigger::{Trigger, TriggerCondition};
pub use hooks::{ChainEvent, EvalContext, TriggerRegistry};
pub use executor::{AutomationTask, ExecutorIncentive};
pub use verification::{
    ExecutionProof, ExecutorAttestation, FraudProof, OracleAttestation, VerificationError, ZkpProof,
};
