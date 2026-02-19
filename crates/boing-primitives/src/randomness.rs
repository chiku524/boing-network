//! Verifiable randomness for leader selection (VDF/VRF placeholders).
//!
//! See DECENTRALIZATION-STRATEGY.md for design.

use serde::{Deserialize, Serialize};

use crate::{AccountId, Hash};

/// VDF output — verifiable delay function result for fair ordering.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VdfOutput {
    pub input: Hash,
    pub output: [u8; 32],
    pub proof: Vec<u8>,
}

/// VRF output — verifiable random function for leader election.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VrfOutput {
    pub proof: Vec<u8>,
    pub output: [u8; 32],
}

/// Select leader from validator set using VRF output (stub).
/// Given validators and a VRF output, deterministically returns the elected leader.
/// Production implementations would verify the VRF proof.
pub fn leader_from_vrf(validators: &[AccountId], vrf_output: &VrfOutput) -> Option<AccountId> {
    if validators.is_empty() {
        return None;
    }
    let mut idx = 0u64;
    for (i, b) in vrf_output.output.iter().take(8).enumerate() {
        idx |= (*b as u64) << (i * 8);
    }
    Some(validators[(idx as usize) % validators.len()])
}

/// Produce a dummy VRF output for a given round (stub for testing).
/// Production would use real VRF (e.g. ECVRF) with round seed.
pub fn dummy_vrf_output(round: u64) -> VrfOutput {
    let mut output = [0u8; 32];
    output[..8].copy_from_slice(&round.to_le_bytes());
    VrfOutput { proof: vec![], output }
}
