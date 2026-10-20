# Exploratory third round (2026-10-20 21:32 UTC)

The initial bounded-range candidate tied the efficient arkworks API. A further
algebraic substitution is now proposed BEFORE its measurements:

Let s be the weighted sum of the lower n−1 Boolean bits, and h the highest weight
(2^(n−1) for power-of-two ranges, X−2^(n−1) for the alternative bound encoding).
Eliminate the separately allocated top bit and the reconstruction equality by
constraining `(x−s)*(x−s−h)=0` directly. In the fixed prime field, x−s is 0 or h.
Thus the exact accepted public x-set is preserved with n total rows rather than
n+1, but still n nonlinear rows. This is a range-predicate rewrite; it does not
return arkworks' Vec<Boolean>. A top digit can be recovered algebraically as
(x−s)/h if the consuming interface needs it, but that integration is not benchmarked.

Test the same public validation bounds, disclose reuse of these instances, and add
fresh power-of-two width 5 and arbitrary bound 93 as confirmation cases. The new
rule must be checked in the exported matrices and exhaustively exercised for small
widths/bounds. Record nonzeros to expose a possible tradeoff against row count.
This is elementary witness elimination, not claimed novel mathematics.
