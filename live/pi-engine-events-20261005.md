# pi-engine PR lifecycle event verification

This intentionally harmless change exercises the owner-authorized throwaway
repository's real GitHub lifecycle and author waiter. It is not production code.

Correlation: `pi-engine-events-20261005`

Required evidence (to be collected externally, not claimed by this marker):
- the new pull request is observed by the webhook/discovery path;
- `git pr-await` hands that PR to the configured controller once;
- actual review-update events wake the subscribed waiter;
- a merged event reaches the owning pi session;
- the merged commit is verified against the fetched Git history.

A waiter launch, latch-file presence, or this file alone is not passing evidence.
