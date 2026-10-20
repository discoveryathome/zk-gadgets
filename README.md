# Make privacy proofs cheaper without opening loopholes

Can the same mathematical statements be proved with fewer constraints and less
prover computation? This **Discovery@Home pilot, developed in LOCAL mode,** tests that question using
R1CS over the BN254 scalar field, pinned arkworks libraries, and a restricted
independent relation verifier.

Start with [the contributor-trial assessment](research/trial/REPORT.md),
[measured results](results/SUMMARY.md), or [the progress graphic](results/progress.svg).
No maintainer has consented yet; donor intake is paused. This repository has not
been submitted to the Discovery@Home hub.

## Check it on a home computer

Clone the pilot:

```sh
git clone https://github.com/discoveryathome/zk-gadgets.git
cd zk-gadgets
```

Python 3.9+ is sufficient to check the saved examples, with no packages or network:

```sh
python3 scripts/test_verifier.py
python3 scripts/demo.py
python3 scripts/verify.py --output results/verification-recheck.json
```

For a quick single case: `python3 scripts/verify.py --row arithmetic-64`.
The final full matrix/relation check took 122 seconds on the busy M2 trial machine;
small examples and the demo take seconds. Exact runtimes are in the report.
Do not use Python `-O`; the checker rejects assertion-disabled execution.

To regenerate the exact matrices from Rust source (Rust/Cargo required):

```sh
bash scripts/rebuild_check.sh
```

To repeat all synthesis and Groth16 measurements, including seven paired samples,
A/A controls, and wrong-public-input proof rejection:

```sh
bash scripts/reproduce.sh replay
```

The initial clean build took 6m37s on a heavily shared Apple M2; expect additional
minutes for proofs and checks. No GPU, large dataset, or solver farm is required.
Cargo downloads/builds remain in this folder. Main dependencies are exact-version
pinned; Cargo.lock pins transitive versions/checksums. The compiler version used
is recorded in `results/environment.json`, not installed globally by these scripts.

## What the result means

Arithmetic factoring and output fusion, decomposition reuse, a one-selection Merkle
ordering gadget, and a published alternative range encoding yield exact reductions.
The first bounded-range candidate tied the efficient arkworks API. A later range-only witness-elimination rule saves one total row, with no reduction in multiplication rows. See the table
for strong versus weaker reference choices; these are not universal lower bounds
or claims of previously unknown identities. The current authoritative record is
`discover/frontier.json`, at **author-checked** status.

Unlike output-only tests, the Python checker eliminates auxiliary variables from
actual exported R1CS matrices, checks Boolean/range obligations, and compares the
resulting expressions with a separately written relation specification. It also
constructs a satisfying malicious witness for a deliberately broken circuit.
Small bounded range encodings are exhaustively checked after exact Boolean-root
reduction. Full-field canonical decomposition and the upstream arbitrary-bound
comparator have only assignment/native-output checks in this trial; all optimized
candidates use supported relation rules. This is custom verification code, not
proof-assistant certification or independent security review.

[FORMULATION.md](FORMULATION.md) defines every input, witness, public statement,
constraint relation, field/hash parameter, and proving assumption.
[SOURCES.md](SOURCES.md) records prior work and limits on novelty.
[AGENTS.md](AGENTS.md) leads contributors to the protocol, tasks, and review status.
All samples are synthetic, and saved witnesses are deliberately public debug
artifacts. No production proving keys or security certification are supplied.
