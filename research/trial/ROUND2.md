# Exploratory second round (2026-10-20 21:23 UTC)

The first measured round found the expected arithmetic/selection improvements and
zero gain over the bounded-range control. Before implementing further candidates:

* Test output fusion with arkworks' existing `mul_equals` API after factoring the
  arithmetic expression. This is an application/compiler choice, not a new primitive.
  Keep original sizes and add fresh confirmation sizes 7 and 31. Original held-out
  sizes are now reused validation; do not call this a blinded test.
* Replicate the literature's noncanonical encoding for an arbitrary upper bound X:
  x=(X−2^(n−1))*b_(n−1)+sum_(i<n−1)2^i*b_i, n=ceil(log2 X).
  Compare to bounded binary decomposition plus arkworks' constant comparator.
  Develop on X=13; fixed confirmation bounds X=29,53,181. This proves the RANGE
  relation only and must not replace canonical bits when downstream consumers use
  them. Its overlapping witnesses are a useful test that uniqueness is not required.

These are reasoning-led variations suggested by source inspection and the range
literature, not a solver search. Preserve the round-one counts and report both
successes and zero-gain controls. No claim of literature novelty.
