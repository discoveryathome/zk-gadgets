# Discovery@Home: sound R1CS gadget optimization

Read `README.md`, `FORMULATION.md`, and `discover/frontier.json`. The frontier is a
local author-checked benchmark, not an externally accepted record. Read
`research/trial/REPORT.md` for the featured-pilot assessment and all limitations.
Sources and settled mistakes are in `SOURCES.md`. The exact same public statement
and private-interface relation must be preserved by an optimization.

The bottleneck is **soundness verification and strong baseline selection**, not
brute-force compute. Improvements over a deliberately generic implementation do
not establish improvements over an efficient gadget. Constraint savings are exact;
wall-clock speedups need separate controlled evidence. Avoid changing a hash to win
a benchmark, or dropping a Boolean/range constraint because examples still work.

This research proposal is CLOSED to donor intake: no maintainer has consented and no
review capacity is assigned. The repository is published on GitHub but has not been
submitted to the Discovery@Home hub. A maintainer must review the
checker and authorize acceptance before this can become a featured live project.
The schema requires a maintainer entry; `UNASSIGNED` is an explicit sentinel, not a
person or a claim of consent. `review.paused=true` enforces this status in metadata.

The task catalogue has four weighted tracks. For a future open-ended contribution,
use `python3 protocol/discover.pyz pick . --seed <32 hex digits>` (or omit --seed for
executed randomness), record the seed, read prior packages/ledger, and run the card's
first step. An explicit requested task overrides random selection. Local claims and
handoffs go into research packages, not a fictitious online coordination board.

A candidate must include source, exact field/interface, pinned baseline, exported
matrices, raw cost/timing samples, independently runnable checks and rejected
negative controls. Extend the verifier conservatively if the candidate uses an
unsupported encoding: unsupported must never mean verified. Record whether evidence
is exact computation, a custom algebraic checker, a prose argument, or kernel-checked
proof. Do not label this pilot formally verified or independently reviewed.

No agent may declare novel mathematics from a lower score. Demonstrate held-out
compositions and cite known identities/encodings. Distinguish fixed public validation
from sealed testing. Reuse of a validation set must be disclosed. Keep primary total
R1CS rows fixed; different proof systems or fields define a different formulation.

Credit contributors and reviewers in `CONTRIBUTORS.md`; preserve upstream citations.
New project code/docs use MIT; third-party snapshots retain their licenses.
Changes to the verifier or frontier deserve the most scrutiny. Review capacity,
not donated compute, will set the pace.
