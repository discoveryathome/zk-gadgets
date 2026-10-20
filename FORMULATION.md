# Fixed formulation: bn254-r1cs-v1

All arithmetic is in the BN254 **scalar field** (not its curve-coordinate field):

`p = 21888242871839275222246405745257275088548364400416034343698204186575808495617`.

An R1CS assignment is `z = (1, public inputs, private inputs and auxiliaries)`.
Every row enforces `(A_i z)(B_i z) = C_i z (mod p)`. The accepted relation is the
projection of **all** satisfying assignments onto the declared interface. The
witness-generation program does not define or restrict a malicious witness.

Primary objective: total R1CS rows after `OptimizationGoal::Constraints` and
`finalize()`. Linear equalities count as rows too. Secondary diagnostics: rows with
nonconstant A and B (called multiplication rows), private variables, and matrix
nonzeros. A count reduction is not automatically a prover speedup.

## Exact interfaces

| Family | Public statement | Private interface | Accepted relationship |
|---|---|---|---|
| `range-n` | field x | n little-endian bits, implementation auxiliaries | canonical integer x is in [0,2^n); bits reconstruct x |
| `reuse-n` | x, t | decomposition bits; baseline has two copies | x in [0,2^n), t = sum (i+1)b_i; both copies, if present, decompose the same x |
| `arithmetic-n` | y_0…y_(n−1) | a_i,b_i | every y_i = a_i²−b_i² mod p |
| `swap-1` | L,R | a,b,s | s is 0 or 1; if s=0 then (L,R)=(a,b), otherwise (b,a) |
| `merkle-d` | root | leaf x, d sibling digests, d direction bits | hash the leaf, then d ordered hash steps; the result equals root |
| `composition-8` | root,y | x,b, membership path for x | x in [0,2^16), y=x²−b², and x's leaf hash belongs to root via the supplied path |

The arithmetic examples are microgadget relations, not useful standalone privacy
applications: existential claims about unconstrained a,b may be trivial. The
composition shows why interface correctness matters when embedded in a larger
statement. The leaf position is private; this does not claim membership at a
particular public index or bind to an external database. `d` is fixed per circuit.

Each saved matrix declares name-to-wire `bindings`. These are part of its public
specification, not hints from the witness generator. The independent verifier
checks the expected interface name set and wire coverage. All output and auxiliary
wires are constrained; no witness value is assumed honest in the symbolic pass.

## Hash and proving assumptions

Both Merkle variants use exactly the same arkworks Poseidon permutation over Fr:
width 3, rate 2, capacity 1, alpha 5, 8 full rounds, 57 partial rounds, Grain-generated
constants with skip_matrices=0. Exact constants are in `results/poseidon.json`.
Leaf hash starts capacity at 1, internal compression starts capacity at 0. In Rust
the leaf domain is implemented by adding 1 to the first capacity round constant;
the independent Python implementation sets the initial capacity to 1 instead.
The leaf absorbs one element; an internal node absorbs left and right, then
squeezes one field element. This is a precisely specified benchmark instantiation,
not a newly certified production hash parameter set. Membership-as-a-commitment
additionally assumes hash collision/preimage resistance; algebraic circuit
correctness does not prove those cryptographic assumptions.

The reference Merkle circuit calls the actual upstream `PathVar::calculate_root`
and enforces equality to the public root. The optimized circuit changes only child
ordering and preserves hashing. Using `calculate_root` plus equality avoids
charging the reference for an unnecessary generic equality-test Boolean.

Groth16 over BN254 is benchmarked with a circuit-specific setup per variant,
fixed pseudorandom seeds, release build and `RAYON_NUM_THREADS=1`. Setup and proof
verification time are excluded from prover samples. Synthesis timing includes
allocation, witness assignment, constraints and finalization, **not** a separately
compiled witness calculator. Setup seeds are public for reproducibility and are
not safe setup ceremonies. No keys are deployed or proof security asserted.

## Soundness and completeness arguments

* Booleanity: in a prime field b(b−1)=0 implies b∈{0,1}. Removing this row is invalid.
* Bounded decomposition: sum 2^i b_i lies below 2^n < p, so its field value has that
  canonical integer representation. Thus binding it to x proves the range. Every
  x in range has the usual binary witness. Full 254-bit decomposition needs the
  additional canonicality condition; this pilot does not remove that condition.
* Reuse: two canonical n-bit decompositions of the same x are identical. One copy
  can serve both consumers. This is a composition optimization, not a better
  primitive than arkworks' existing bounded decomposition.
* Arithmetic: (a+b)(a−b)=a²−b² identically over the whole field. Internal outputs
  are uniquely determined. No sampled-input inference is used.
* Swap: L=a+s(b−a), R=a+b−L=b+s(a−b). Boolean s establishes the ordered-pair
  semantics. This identity holds in every field, but booleanity is essential to
  the intended selection relation.
* Membership: apply the swap lemma at each level and preserve the same deterministic
  hash circuit. Induction over depth preserves the exact root relation. The
  Python verifier also checks the exported matrices symbolically against an
  independently written hash specification; it does not merely trust this prose.

The restricted checker solves each fresh auxiliary/output wire only when its
coefficient is a nonzero field constant, verifies all required Boolean and range
rows, and compares output expressions exactly. This establishes soundness for
supported exported circuits, conditional on checker correctness. For completeness,
the same triangular equations define every auxiliary for valid interface inputs;
the only residual conditions are the specified range and Boolean conditions.
The checker is custom Python, not a proof-assistant kernel or independent human
review. It intentionally reports full-field canonical decomposition as
assignment/native-output checked only, pending an additional proof rule.

## Exploratory second-round encodings

`arithmetic/fused` enforces `(a+b)*(a−b)=y` directly with arkworks `mul_equals`.
It removes the intermediate multiplication result and its output equality. This
is a second application-level improvement, using an existing arkworks API.

`bound-X` has public field x, private Boolean digits, and accepted public relation
`canonical(x) < X`. Let n=ceil(log2 X), h=2^(n−1), k=X−h. The alternative encoding
is `x = k*b_(n−1) + sum_(i<n−1) 2^i*b_i`. With Boolean bits the possible integers
are [0,h−1] union [k,k+h−1] = [0,X−1], with no reduction modulo p. This proves both
soundness and completeness. Where these intervals overlap, different digit
witnesses represent the same x; that is allowed. These digits are **not** canonical
binary decomposition and must not be passed to a consumer requiring canonical bits.
For X=13, digits (0,0,0,1) represent x=5, not x=8. The baseline performs canonical
bounded decomposition and `Boolean::enforce_smaller_or_equal_than_le(X−1)`.
The optimized actual matrices are exhaustively checked over all Boolean digit
assignments after the Boolean-root lemma; the baseline comparator is only checked
numerically and retained as trusted upstream code in this trial.
