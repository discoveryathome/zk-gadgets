# Featured-pilot assessment: sound ZK gadget optimization

**Recommendation: a conditional yes as a technical featured pilot, but not yet the
sole launch headline.** The trial produced a real, explainable sequence of
improvements on home-class hardware. It also showed why raw score improvement is
not enough: large gains often repair inefficient applications or replicate known
encodings, while the gain against an existing Merkle library was below 0.5%.
Sustained discovery yield and independent security review remain unproven.

## What this trial actually tested

One agent, no sub-agents or solver search. The run began at
2026-10-20 21:10:33 UTC. It included building the pilot infrastructure, making
candidate changes, benchmarking, adversarial checks, and a clean-checkout replay.
The previous turn's source audit and downloaded guidance are preparation, not
credited as discoveries during this run. This is one author-run trial, not evidence
about the success rate of independent contributors. The completed run took **33.7 minutes**. Timing and replay receipts
are recorded in `results/trial-receipt.json` and `results/fresh-replay.log`.

The machine was an Apple M2 with 24 GiB RAM. Proving used one Rayon thread. The
workspace stayed below 1 GiB, with no GPU or distributed search. Unrelated CPU work
was substantial; timing comparisons are exploratory. The first cold release build
took 6m37s, then roughly a minute for each edited binary. A future contributor can
start from the saved Python-checkable examples without compiling Rust immediately.

We fixed BN254 Fr and counted **total finalized R1CS rows**. Other counts, raw
synthesis timings, actual Groth16 prover samples, and A/A controls are preserved.
There are 26 scoped instances and 73 implementation cases. We used development
sizes, larger held-out compositions, then explicitly disclosed reuse of validation
instances during later rounds. Fresh confirmation cases were added for later rules;
there was no independently sealed benchmark.

## Tangible progress

| Experiment | Rows before → intermediate → final | Interpretation |
|---|---:|---|
| 64 difference-of-squares expressions | 192 → 128 → 64 | Factoring, then direct output enforcement. Known algebra and an existing arkworks API; meaningful application/compiler optimization. |
| Range predicate x < 13 | 10 → 5 → 4 | Published alternative digit encoding, then witness elimination. The encoding is not canonical binary decomposition. |
| Range predicate x < 181 | 20 → 9 → 8 | The same rules transfer to another bound. |
| 64-bit range predicate | 65 → 65 → 64 | Initial attempt tied the strong bounded API. Eliminating the top witness later saved one total row, but **zero multiplication rows**. |
| Two consumers of a 64-bit decomposition | 131 → 66 | Reuse avoids duplicate work; an already well-written caller would do this. |
| Depth-8 Poseidon membership | 2182 → 2174 | Actual upstream PathVar baseline; one row saved per level, **0.37% overall**. |
| Range + arithmetic + depth-8 membership | 2202 → 2193 | First-round rules compose; **0.41% overall**. |

The strongest hill-climbing demonstration is the progression across rounds: a null
result on the efficient range API prompted a different algebraic representation,
which then improved the declared total-row objective. No brute-force search was
needed. However, the last change also increases matrix density on some instances;
a multi-metric scoreboard is necessary to prevent a misleading sense of progress.
Concretely, 64-bit range elimination increases matrix nonzeros from 258 to 318
(+23.3%) while saving one row; this is not an established prover-work improvement.
The x<13 encoding is better behaved: 31 → 18 nonzeros across the whole change.

These results do **not** establish new mathematics, best-known global counts, or
an automatically mergeable upstream patch. The Merkle optimization specializes to
field-valued digests; the generic library also supports other digest types. The
range and arithmetic improvements must be compared against equally tuned callers,
not advertised as beating every implementation in arkworks.

## Soundness checking was feasible, with a narrow scope

The checker reads the actual exported R1CS matrices. Its symbolic pass never uses
honest witness values to justify the relation. It derives auxiliary/output variables
only through equations with invertible constant coefficients, checks Boolean and
range conditions, and compares the resulting expressions with a separately written
specification, including Poseidon and complete membership paths.

All optimized candidates are covered by that restricted relation checker. Ten
reference cases (canonical full-field decomposition and upstream constant comparison)
receive assignment/native-output checks only; their complete relation proof remains
an upstream trust assumption. The custom Python checker itself has no independent
review or proof-assistant certification. Its acceptance rules are substantially more
informative than sample-output equality, but are not a solution to arbitrary R1CS
soundness verification.

The adversarial controls include a circuit that admits a fully satisfying witness
with direction=2, a uniquely determined but wrong output, a detached output, aliased
inputs, wrong field, nonlinear undefined auxiliaries, and an incorrect range-root
weight. The verifier rejects these. Small exported range circuits are exhaustively
checked after the exact Boolean/two-root reduction, over the **same BN254 field**;
we did not substitute a tiny field. A separate 12-bit check covers all 4096 bit
assignments. Multiple valid digit witnesses for one x are retained where allowed.

The initial 58-case full check took 290 seconds on the shared machine. The final
73-case fresh-checkout pass took **121.8 seconds wall time**
and **34.4 CPU seconds**, with 63 full symbolic relation checks
and ten numerical-only reference checks. All 73 matrices and assignments rebuilt
byte-for-byte. The verifier unit tests passed (nine tests). This is practical
for home contributors and review batches, although the trustworthiness and extension
of the checker are likely to consume more expert attention than proposing rewrites.

Actual Groth16 proofs were generated and verified for arithmetic, membership, and
composition comparisons, including warmups and A/A controls. Proof verification also
rejected altered public outputs. This exercises integration, not circuit soundness.

## Less prover work is not yet demonstrated generally

Seven interleaved samples per variant, setup excluded:

| Circuit | Baseline median | First-round candidate median | A/A median ratio |
|---|---:|---:|---:|
| Arithmetic-64 | 484.5 ms | 300.8 ms | 0.807 |
| Merkle-8 | 3187.0 ms | 3248.5 ms | 0.907 |
| Composition-8 | 2500.2 ms | 2101.6 ms | 1.166 |

The same-code controls varied by about 9–19% in their medians. Arithmetic suggests
an improvement, but this machine cannot support a clean quantitative speedup claim.
Merkle did not get faster in this run. The fused/eliminated later variants have
count and relation evidence, not separately measured Groth16 speedups. More sparse
constraints can still cost more per row, and FFT domain boundaries can make the
relationship between row count and prover time discontinuous.

## Suitability against the launch criteria

| Criterion | Assessment from this trial |
|---|---|
| Early tangible progress | Good with curated tasks. Multiple exact score improvements appeared; small examples verify quickly. Infrastructure setup cost is now paid. |
| Sustained hill-climbing | Uncertain. The basic identities will saturate quickly. Real compositions and stronger baselines are essential for subsequent rounds. |
| Public explanation | Moderate to good with a carefully scoped demo: “make privacy proofs cheaper without opening loopholes.” The broken-switch example shows why verification matters. Big toy-gadget percentages must be paired with honest whole-circuit numbers. |
| Meaningful results | Plausible engineering value and a useful certificate/checker workflow. One small field-digest Merkle improvement is grounded in upstream code; research novelty is not established. |
| Home PC feasibility | Good. Commodity CPU, no GPU, no solver marathon, below 1 GiB for this session including artifacts and caches. Rechecking saved cases needs only Python. |
| Agent intelligence matters | Strong in this trial: interface interpretation, finding algebraic substitutions, distinguishing range from canonical bits, constructing adversarial witnesses, and implementing checks. Compute mainly validates the reasoning. |
| Verification/review scaling | Main launch risk. Safe automated acceptance should be restricted to reviewed rewrite/certificate families. Arbitrary circuit changes still need specialist review. |

## Recommended public demo and launch gate

Run `python3 scripts/demo.py`: a deliberately broken circuit accepts a forged
witness, the checker catches it, and a legitimate cheaper circuit passes. Then show
`results/progress.svg` and the staged arithmetic/range counts. Explain that a ZK
proof can hide the witness, but it cannot repair a circuit that forgot a rule.
The saved demo witnesses are public synthetic debugging examples, not a privacy app.

Before making this the lead feature, run a small cohort of genuinely independent
contributor sessions on a curated set of realistic compositions, excluding the
already-demonstrated elementary wins. Record time to first accepted improvement,
null-result rate, reviewer time, and gains over tuned baselines. Obtain independent
review of the checker, a consenting cryptography maintainer, and a quiet-machine
prover replication. Look for repeatable useful gains, not one large toy percentage.

A launch could responsibly feature this as a **verified optimization workshop** with
bounded certificate families and explicit replication credit. It should not promise
that every volunteer will discover a new cryptographic result. No maintainer is
assigned, intake is paused, and nothing has been published.
