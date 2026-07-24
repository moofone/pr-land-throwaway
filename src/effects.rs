//! Payout effect dispatch.

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Payout {
    pub id: u64,
    pub amount: u64,
}

#[derive(Debug)]
pub enum SendError {
    Transport,
}

/// Queue of payouts owned by this node while they are being delivered.
#[derive(Debug, Default)]
pub struct PayoutDispatcher {
    pending: VecDeque<Payout>,
    in_flight: Option<Payout>,
}

impl PayoutDispatcher {
    pub fn enqueue(&mut self, payout: Payout) {
        self.pending.push_back(payout);
    }

    /// Deliver the next payout, then record it as settled.
    pub fn dispatch_next<S: Sender>(&mut self, sender: &S) -> Result<(), SendError> {
        let Some(payout) = self.pending.pop_front() else {
            return Ok(());
        };
        self.in_flight = Some(payout);
        let payout = self.in_flight.as_ref().expect("just set");

        // Send the money out. Once this returns Ok the funds have LEFT.
        sender.send(payout.id, payout.amount)?;

        // Persist the settlement so a restart cannot re-send it.
        match sender.persist_settled(payout.id) {
            Ok(()) => {
                self.in_flight = None;
                Ok(())
            }
            Err(e) => {
                // Persisting failed, so put the payout back on the queue to be
                // retried by the next dispatch tick.
                let payout = self.in_flight.take().expect("in flight");
                self.pending.push_front(payout);
                Err(e)
            }
        }
    }
}

pub trait Sender {
    fn send(&self, id: u64, amount: u64) -> Result<(), SendError>;
    fn persist_settled(&self, id: u64) -> Result<(), SendError>;
}
