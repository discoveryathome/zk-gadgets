# Source audit (2026-10-20)

The question is an optimization program, not an unverified assertion that a particular
new lower bound exists. There is no identified universal best gadget for all public
interfaces and cost models. Our frontier is a pinned, scoped benchmark table.

| Source | Evidence used | Status and limitation |
|---|---|---|
| [ark-r1cs-std 0.5.0](https://github.com/arkworks-rs/r1cs-std/tree/v0.5.0) | Native field arithmetic, Booleanity, `to_bits_le`, `to_bits_le_with_top_bits_zero`, conditional selection | Executable baseline, exact crate pinned in Cargo.lock. Bounded API is already efficient; full-field conversion is a weaker application choice. |
| [ark-crypto-primitives PathVar 0.5.0](https://github.com/arkworks-rs/crypto-primitives/blob/v0.5.0/crypto-primitives/src/merkle_tree/constraints.rs) | Two field-valued selections per level | Actual `PathVar::calculate_root` used in the trial; identical Poseidon constants and leaf/root interface. |
| [PathVar 0.6.0](https://github.com/arkworks-rs/crypto-primitives/blob/v0.6.0/crypto-primitives/src/merkle_tree/constraints.rs) | Same two-selection source pattern | Read-only audit, not a 0.6 benchmark. Does not establish novelty or upstream acceptance. Snapshot in protocol/. |
| [Khovratovich, Simple R1CS range check and truncation (2022)](https://hackmd.io/@7dpNYqjKQGeYC7wMlPxHtQ/B1w_9nq2Y) | Alternative encodings for arbitrary bounds | Literature direction, not implemented in this trial. Such range encodings can have nonunique witnesses; uniqueness is not the definition of correctness. |
| [Poseidon, ePrint 2019/458](https://eprint.iacr.org/2019/458) | Field-native hashing as an important constraint cost | Motivation and parameter family. Trial does not re-prove collision resistance or certify new parameter choices. |
| [Picus / QED² (PLDI 2023)](https://github.com/Veridise/Picus) | Automated detection of under-constrained signals | Uniqueness tooling is relevant, but alone is not equivalence to an intended relation. Picus was not installed or benchmarked. |
| [Formal Verification of Zero-Knowledge Circuits (2023)](https://arxiv.org/abs/2311.08858) | ACL2/PFCS compositional verification | Supports a longer-term verification track; no claim that this pilot's custom Python is equivalent to proof-assistant checking. |

Source claims confirmed: arkworks implements the named gadgets; bounded bit checks
already exist; the pinned Merkle source uses two selections; malicious auxiliary
assignments must be constrained independently of the witness generator.

Unverified/open: a globally best constraint count; originality of any identity;
security certification of this exact hash instantiation; scalability of our checker
to arbitrary optimized R1CS; durable yield across many contributors.

Settled within this scope: do not drop Booleanity; do not truncate a full-field
representation without proving upper bits zero; do not silently change the hash or
public statement; do not equate a deterministic output with the correct output;
do not call a gain over full-field decomposition a gain over arkworks' bounded API.

Cargo.lock contains registry checksums for all compiled dependencies. Source snapshots
are evidence excerpts and retain upstream MIT terms in `protocol/ARKWORKS-LICENSE-MIT`.
The Discovery@Home playbook and schemas are retained in protocol/ as fetched guidance,
not relicensed project code.
