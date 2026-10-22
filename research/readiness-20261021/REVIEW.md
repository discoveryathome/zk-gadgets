# Review request: ZK gadget optimization

Status: prepared locally; not sent. No reviewer has signed off.

Baseline commit: `88360c32ba664fb704ca8f78854633e37e9aac3e`. Review the preparation branch commit recorded in the cohort receipt when work is finalized.

BN254 scalar field, fixed Poseidon parameters, two public outputs and the existing 16-bit private-value predicate. Same interface and relation; minimize total rows while reporting nonzeros and witness variables. A row reduction is not a prover-speed result.

1. Audit the symbolic checker elimination rules and the private/public interface binding. Can an adversarial circuit pass with a different relation?
2. Approve a bounded set of rewrite/certificate families for intake; specify when specialist review is required.
3. Assess whether the new compositions and optimized baselines represent useful application work.

Return: the commit reviewed, commands run, exact outputs/artifact hashes, any
counterexamples or limitations, and a decision of revise / approve scoped beta /
defer. Specify the scope of approval and review capacity. Do not infer general
scientific validity from passing the automated suite. No signature is prefilled.
