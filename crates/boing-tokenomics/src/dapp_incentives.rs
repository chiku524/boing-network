//! Success-based dApp incentives — formula, value cap, metrics.
//!
//! Governance-adjustable parameters. See DEVELOPMENT-RECOMMENDATIONS.md.

use boing_primitives::AccountId;

/// Success metrics for a dApp (epoch or monthly).
#[derive(Clone, Debug, Default)]
pub struct DappMetrics {
    pub tx_count: u64,
    pub fees_collected: u128,
    pub volume: u128,
    pub unique_users: u64,
}

/// Value cap per dApp owner (governance parameter).
/// E.g. max 10M BOING/month per dApp.
pub const VALUE_CAP_PER_DAPP: u128 = 10_000_000;

/// Weight for tx_count in f(metrics).
pub const WEIGHT_TX_COUNT: u64 = 1;
/// Weight for fees in f(metrics).
pub const WEIGHT_FEES: u64 = 10;
/// Weight for volume in f(metrics).
pub const WEIGHT_VOLUME: u64 = 5;

/// Compute incentive for a dApp: `f(metrics)` with cap.
/// Formula: min(cap, tx_count * w1 + fees * w2 + volume_scale * w3).
pub fn dapp_incentive(metrics: &DappMetrics) -> u128 {
    let score = (metrics.tx_count as u128)
        .saturating_mul(WEIGHT_TX_COUNT as u128)
        .saturating_add(metrics.fees_collected.saturating_mul(WEIGHT_FEES as u128) / 1000)
        .saturating_add(metrics.volume.saturating_mul(WEIGHT_VOLUME as u128) / 1_000_000);
    score.min(VALUE_CAP_PER_DAPP)
}

/// Registered dApp for incentive tracking.
#[derive(Clone, Debug)]
pub struct DappRegistration {
    pub contract: AccountId,
    pub owner: AccountId,
}

/// Royalty split for dApp fees — developer, library, treasury (basis points, 10000 = 100%).
#[derive(Clone, Debug, Default)]
pub struct DappRoyaltySplit {
    pub developer_bps: u16,
    pub library_bps: u16,
    pub treasury_bps: u16,
}

impl DappRoyaltySplit {
    pub fn new(developer_bps: u16, library_bps: u16, treasury_bps: u16) -> Self {
        Self {
            developer_bps,
            library_bps,
            treasury_bps,
        }
    }

    /// Validate: total <= 10000.
    pub fn is_valid(&self) -> bool {
        (self.developer_bps as u32 + self.library_bps as u32 + self.treasury_bps as u32) <= 10_000
    }

    /// Distribute `amount` according to splits. Returns (developer, library, treasury).
    pub fn distribute(&self, amount: u128) -> (u128, u128, u128) {
        let d = amount * self.developer_bps as u128 / 10_000;
        let l = amount * self.library_bps as u128 / 10_000;
        let t = amount * self.treasury_bps as u128 / 10_000;
        (d, l, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dapp_incentive_capped() {
        let m = DappMetrics {
            tx_count: 1_000_000,
            fees_collected: 1_000_000,
            volume: 100_000_000,
            unique_users: 10_000,
        };
        let inc = dapp_incentive(&m);
        assert!(inc <= VALUE_CAP_PER_DAPP);
    }

    #[test]
    fn test_dapp_incentive_zero() {
        let m = DappMetrics::default();
        assert_eq!(dapp_incentive(&m), 0);
    }

    #[test]
    fn test_dapp_royalty_split() {
        let split = DappRoyaltySplit::new(100, 50, 850); // 1%, 0.5%, 8.5%
        assert!(split.is_valid());
        let (d, l, t) = split.distribute(10_000);
        assert_eq!(d, 100);
        assert_eq!(l, 50);
        assert_eq!(t, 850);
    }
}
