//! Node configuration.

#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Address this node advertises to peers.
    pub advertise_address: String,
    pub max_batch: usize,
}

impl NodeConfig {
    pub fn new(advertise_address: String) -> Self {
        Self { advertise_address, max_batch: 64 }
    }
}
