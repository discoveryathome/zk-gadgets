#!/usr/bin/env python3
"""A short, honest public demonstration: run the checker, then show measured counts."""
import json
from pathlib import Path
from verify import mutant_checks, relation_check, rows_satisfied
root=Path(__file__).resolve().parents[1];base=root/'results'
params=json.loads((base/'poseidon.json').read_text())
print('Make privacy proofs cheaper without opening loopholes.\n')
print('A small example: a private switch must choose (7,11) or (11,7).')
print('With no rule that the switch is 0 or 1, switch=2 can produce (15,3).')
print('Honest examples alone would not expose that bug.\n')
result=mutant_checks(params,base)
assert result['missing_boolean_rejected']
print('LIVE CHECK: a forged witness satisfies the broken exported circuit.')
print('LIVE CHECK: the relation verifier rejects that broken circuit.\n')
for method in ['baseline','optimized']:
 d=json.loads((base/f'matrices/swap-1-{method}.json').read_text())
 assert rows_satisfied(d);relation_check(d,params)
 print(f'Valid swap {method}: {len(d["a"])} total checks; full-field algebra check passes.')
print('\nMeasured larger examples (saved results; use verify.py to recheck):')
records=json.loads((base/'summary.json').read_text())
for name in ['range-64','bound-13','arithmetic-64','merkle-8']:
 r=next(x for x in records if x['instance']==name)
 print(f'  {name}: {r["baseline_count"]} -> {r["value"]} checks ({r["reduction_percent"]:.2f}% fewer)')
print('\nKnown techniques, not claimed new mathematics. Merkle savings are small.')
print('Fewer constraints do not automatically mean proportionally faster proving.')
