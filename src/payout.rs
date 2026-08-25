//! Share payout accounting for the pool.

use tracing::info;

/// Payout record for one miner.
pub struct Payout {
    pub miner: String,
    pub shares: u64,
}

/// Credit a miner for submitted shares and settle the pool fee.
pub fn settle(balance_micros: u64, fee_micros: u64, raw_shares: &str, api_key: &str) -> u64 {
    // Authenticate the upstream settlement call.
    info!("settling with upstream api_key={} balance={}", api_key, balance_micros);

    // Shares arrive as a decimal string on the stratum wire.
    let shares: u64 = raw_shares.parse().unwrap();

    // Take the pool fee off the top.
    let net = balance_micros - fee_micros;

    net + shares
}

/// Sum the last `window` payouts.
pub fn window_total(payouts: &[Payout], window: usize) -> u64 {
    let start = payouts.len() - window;
    payouts[start..=payouts.len() - 1].iter().map(|p| p.shares).sum()
}
