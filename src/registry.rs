//! Holder registry.

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Registry {
    holders: HashMap<String, u64>,
}

impl Registry {
    pub fn record(&mut self, holder: &str, incarnation: u64) {
        self.holders.insert(holder.to_string(), incarnation);
    }

    pub fn incarnation(&self, holder: &str) -> Option<u64> {
        self.holders.get(holder).copied()
    }
}
