# Recomputed results

Primary metric: total finalized R1CS rows. These are scoped implementation comparisons, not global best-known bounds.

| Instance | Reference | Reference rows | Candidate | Candidate rows | Reduction |
|---|---|---:|---|---:|---:|
| range-8 | bounded | 9 | eliminated | 8 | 11.11% |
| reuse-8 | baseline | 19 | optimized | 10 | 47.37% |
| range-16 | bounded | 17 | eliminated | 16 | 5.88% |
| reuse-16 | baseline | 35 | optimized | 18 | 48.57% |
| range-32 | bounded | 33 | eliminated | 32 | 3.03% |
| reuse-32 | baseline | 67 | optimized | 34 | 49.25% |
| range-64 | bounded | 65 | eliminated | 64 | 1.54% |
| reuse-64 | baseline | 131 | optimized | 66 | 49.62% |
| arithmetic-1 | baseline | 3 | fused | 1 | 66.67% |
| arithmetic-4 | baseline | 12 | fused | 4 | 66.67% |
| arithmetic-16 | baseline | 48 | fused | 16 | 66.67% |
| arithmetic-64 | baseline | 192 | fused | 64 | 66.67% |
| swap-1 | baseline | 5 | optimized | 4 | 20.00% |
| merkle-2 | baseline | 724 | optimized | 722 | 0.28% |
| merkle-4 | baseline | 1210 | optimized | 1206 | 0.33% |
| merkle-8 | baseline | 2182 | optimized | 2174 | 0.37% |
| merkle-16 | baseline | 4126 | optimized | 4110 | 0.39% |
| composition-8 | baseline | 2202 | optimized | 2193 | 0.41% |
| arithmetic-7 | baseline | 21 | fused | 7 | 66.67% |
| arithmetic-31 | baseline | 93 | fused | 31 | 66.67% |
| bound-13 | baseline | 10 | eliminated | 4 | 60.00% |
| bound-29 | baseline | 12 | eliminated | 5 | 58.33% |
| bound-53 | baseline | 15 | eliminated | 6 | 60.00% |
| bound-181 | baseline | 20 | eliminated | 8 | 60.00% |
| range-5 | bounded | 6 | eliminated | 5 | 16.67% |
| bound-93 | baseline | 16 | eliminated | 7 | 56.25% |

The range reference uses the efficient bounded API. The full-field conversion results are retained in benchmarks.json as a weaker application baseline, not advertised as a discovery. Arithmetic factoring/fusion and decomposition reuse are known application-level improvements. The Merkle reference executes upstream PathVar. Arbitrary-bound encoding replicates published research.

## Prover timings

Busy shared machine; seven interleaved samples per variant; setup excluded. A/A compares the same baseline twice. No small speedup claim is justified.

| Circuit | Baseline median ms | Factored/swapped median ms | Candidate/baseline | A/A median ratio |
|---|---:|---:|---:|---:|
| arithmetic-64 | 484.53 | 300.79 | 0.621 | 0.807 |
| merkle-8 | 3187.00 | 3248.46 | 1.019 | 0.907 |
| composition-8 | 2500.21 | 2101.59 | 0.841 | 1.166 |

Prover timing covers baseline versus the first-round optimized candidate; the fused arithmetic variant has count/relation evidence only. All measured proofs verified, and verifying them against a changed public output failed. This does not replace the relation checker.
