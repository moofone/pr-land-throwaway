//! Transfer fencing.

/// Highest transfer token this node has observed.
#[derive(Debug, Default)]
pub struct Fence {
    highest: u64,
}

impl Fence {
    /// Accept a token only if it advances the fence.
    pub fn observe(&mut self, token: u64) -> bool {
        if token < self.highest {
            return false;
        }
        self.highest = token;
        true
    }
}
