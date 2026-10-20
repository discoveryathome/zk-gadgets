# Contributor trial, preregistered 2026-10-20 21:12 UTC

Budget: 30–60 minutes of one agent's wall-clock work, including setup, verification,
benchmarking, and reporting. No parallel agents. Previous source audit is disclosed
as preparation, not charged as discovery during this trial. Local execution authorized
by the user's request for this contributor run. No uploads or publication.

Question: can a new contributor produce a reviewable, sound constraint reduction,
and does that offer a sustainable and publicly intelligible hill-climbing task?

Field: BN254 scalar Fr, modulus
21888242871839275222246405745257275088548364400416034343698204186575808495617.
Model: R1CS; primary objective total constraint rows after arkworks finalization.
Report multiplication rows, variables and matrix nonzeros as diagnostics. Secondary
measurements: constraint synthesis/witness assignment and Groth16 proving time;
these do not establish proportional speedups. Fixed local single-thread execution.
Dependencies: arkworks 0.5 series, exact primary versions plus Cargo.lock, deliberately
chosen as a reproducible release, not asserted to be the latest or globally best.

Freeze strategies before held-out evaluation:
1. Range: bounded decomposition versus full-field decomposition plus upper-zero
   checks; include arkworks' efficient bounded API as the strong control.
2. Arithmetic: a²−b² = (a+b)(a−b), and reuse a repeated decomposition in a composition.
3. Merkle ordering: L=select(bit,sibling,node); R=node+sibling−L instead of a second
   select. Keep hash function, parameters, public statement and Boolean checks fixed.

Development sizes: range 8 bits, arithmetic 1 pair, Merkle depth 2.
Held-out sizes/compositions: range 16/32/64 bits, arithmetic 4/16/64 pairs,
Merkle depths 4/8/16, and range+arithmetic+membership composition. Fixed public
validation suite, not an independently sealed blind benchmark. Sample inputs from
an executed seeded RNG; record seed. Do not retune strategies on held-out results.

Acceptance: actual synthesized matrices satisfy independent modular row checking;
wrong public outputs rejected; malicious auxiliary witnesses examined; exact
algebraic reasoning for transformations; exhaustive bounded relation checking
where feasible. Explicitly separate computed evidence from unformalized arguments.
No unsupported claim of general circuit soundness, production hash security,
novelty, independent review, or superiority over all optimized implementations.

Evaluation: successful candidates / attempted strategies, size of improvements
against each baseline, verification time and gaps, setup burden, home hardware,
public-demo clarity, and dependence on agent reasoning versus CPU search.
