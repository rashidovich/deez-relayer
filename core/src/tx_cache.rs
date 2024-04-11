use dashmap::DashSet;

/// Returns true if transaction is ofac-related, false if not
pub fn is_tx_unique(tx_cache: &DashSet<String>, tx_signature: &str) -> bool {
    tx_cache.contains(tx_signature)
}
