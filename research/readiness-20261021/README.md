# ZK gadget optimization: second-cohort preparation

Local preparation authorized by the project coordinator on 2026-10-21. Evidence is
**author-checked**; outside scientific review and public intake approval are pending.
No external contact, hub submission, or new reviewer consent is implied.

## Prepared work

Four additional composed range/arithmetic/Merkle circuits at depths 2, 4, 12 and 16. Both reference and optimized matrices are exported and relation-checked; the optimized implementation is the starting baseline. Exact row, nonzero and witness counts are recorded. The historical 73-case rebuild remains unchanged.

The machine-readable portfolio is [`checks.json`](checks.json).
Historical experiments retain their original exposure and evidence labels. These
preparation cases are now exposed development material, not a sealed test set.

## First contributor step

Activate the repository's documented Python environment, then run from its root:

```sh
python3 scripts/check_composition.py --candidate research/readiness-20261021/export/matrices/composition-2-optimized.json --depth 2
```

BN254 scalar field, fixed Poseidon parameters, two public outputs and the existing 16-bit private-value predicate. Same interface and relation; minimize total rows while reporting nonzeros and witness variables. A row reduction is not a prover-speed result.

Before searching, read the portfolio and select one bounded task. Preserve the
starting witness, source hashes, seed and budget. Reserve time to verify the final
artifact. Record unsuccessful runs. A useful handoff can be a replication, a
reproducible defect, a bounded negative finding, or an improvement under the same
formulation; identify which one it is. Passing a local check does not imply novelty
or independent acceptance.

## Outside review

[REVIEW.md](REVIEW.md) contains concrete questions; [review.json](review.json) is an
unfilled review receipt. Review must identify a commit and exact artifact hashes.
The person reproducing a result should disclose any involvement in its creation.

## Independent contributor trial protocol

Choose a task using the refreshed catalogue and give a new contributor only the
published instructions. Record setup time, time to first valid artifact, total
compute, failed attempts and reviewer minutes. Start from the inherited frontier.
Freeze algorithm submissions before evaluating on a separately prepared unseen
case pool. Author-run smoke checks here do not count as independent trials.

Remaining gates:

- Independent cryptographic soundness review.
- Quiet-machine prover replication for performance claims.
- Fresh contributor trials and an explicit reviewer capacity.
