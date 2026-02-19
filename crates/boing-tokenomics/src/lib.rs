//! BOING Tokenomics — Sustainable Value Design
//!
//! Parameters for supply, emission, fees, and incentives.
//! See BOING-BLOCKCHAIN-DESIGN-PLAN.md for full design.

pub mod dapp_incentives;

/// Max supply (1 billion BOING). Hard cap; no infinite inflation.
pub const MAX_SUPPLY: u128 = 1_000_000_000;

/// Target block time in seconds.
pub const BLOCK_TIME_SECS: u64 = 2;

/// Fee split: share to validators (basis points, 10000 = 100%).
pub const FEE_VALIDATORS_BPS: u16 = 7_000; // 70%
/// Fee split: share to treasury (basis points).
pub const FEE_TREASURY_BPS: u16 = 2_000; // 20%
/// Fee split: share to burn (basis points).
pub const FEE_BURN_BPS: u16 = 1_000; // 10%

/// Emission decay factor per year. Year N emission = Year 1 * DECAY^(N-1).
pub const EMISSION_DECAY: f64 = 0.85;
/// Year 1 annual inflation (basis points, 800 = 8%).
pub const EMISSION_YEAR_1_BPS: u16 = 800;
/// Long-term inflation floor (basis points, 100 = 1%).
pub const EMISSION_FLOOR_BPS: u16 = 100;

/// Typical validator commission range (basis points).
pub const VALIDATOR_COMMISSION_MIN_BPS: u16 = 500;  // 5%
pub const VALIDATOR_COMMISSION_MAX_BPS: u16 = 1_000; // 10%

/// dApp incentive cap per epoch (governance parameter; placeholder).
pub const DAPP_CAP_PER_EPOCH: u128 = 50_000;

/// Blocks per year (approximate).
pub const BLOCKS_PER_YEAR: u64 = 365 * 24 * 3600 / BLOCK_TIME_SECS;

/// Block emission reward (validators' share). Year N = Year 1 * EMISSION_DECAY^(N-1).
pub fn block_emission_validators(block_height: u64) -> u128 {
    if block_height == 0 {
        return 0;
    }
    let year = (block_height - 1) / BLOCKS_PER_YEAR;
    let decay = EMISSION_DECAY.powi(year as i32);
    let year1_per_block = (MAX_SUPPLY * EMISSION_YEAR_1_BPS as u128 / 10_000) / BLOCKS_PER_YEAR as u128;
    let floor_per_block = (MAX_SUPPLY * EMISSION_FLOOR_BPS as u128 / 10_000) / BLOCKS_PER_YEAR as u128;
    let emission = (year1_per_block as f64 * decay) as u128;
    emission.max(floor_per_block) * FEE_VALIDATORS_BPS as u128 / 10_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_emission_year1() {
        assert_eq!(block_emission_validators(0), 0);
        let r1 = block_emission_validators(1);
        assert!(r1 > 0);
        assert!(r1 < 100); // sanity: per-block reward small
    }
}
