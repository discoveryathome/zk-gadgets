#!/usr/bin/env python3
"""Check a candidate against an explicit composition contract, not its own label."""
import argparse, hashlib, json, sys, time
from pathlib import Path
from verify import P, native_check, relation_check, rows_satisfied
ROOT=Path(__file__).resolve().parents[1]
PACKAGE=ROOT/'research/readiness-20261021'

def check(path, depth):
    start=time.perf_counter(); d=json.loads(Path(path).read_text())
    if (d.get('family'),d.get('size'),d.get('field'),d.get('public_count')) != ('composition',depth,str(P),2):
        raise ValueError('candidate does not match requested composition interface')
    params=json.loads((ROOT/'spec/poseidon.json').read_text())
    if not rows_satisfied(d): raise ValueError('assignment fails')
    native_check(d,params)
    proof=relation_check(d,params)
    for j in (1,2):
        bad=list(map(int,d['assignment']));bad[j]=(bad[j]+1)%P
        if rows_satisfied(d,bad):raise ValueError('altered public input accepted')
    return dict(depth=depth,rows=len(d['a']),nonzeros=sum(len(r) for k in ('a','b','c') for r in d[k]),
        witness_variables=len(d['assignment'])-d['public_count']-1,
        relation=proof,seconds=time.perf_counter()-start,
        sha256=hashlib.sha256(Path(path).read_bytes()).hexdigest())

def main():
    p=argparse.ArgumentParser();p.add_argument('--candidate');p.add_argument('--depth',type=int,choices=[2,4,12,16]);a=p.parse_args()
    if a.candidate:
        if a.depth is None:p.error('--depth is required with --candidate')
        print(json.dumps(check(a.candidate,a.depth),indent=2));return
    rows=[]
    for n in (2,4,12,16):
        for method in ('baseline','optimized'):
            file=PACKAGE/f'export/matrices/composition-{n}-{method}.json'
            rows.append(dict(instance=f'composition-{n}',method=method,path=str(file.relative_to(ROOT)),**check(file,n)))
    out={'formulation':'bn254-r1cs-v1; readiness composition portfolio v1','status':'author-checked; independent soundness review pending',
         'exposure':'All cases are now public development benchmarks; no sealed holdout claim.',
         'starting_method':'optimized','rows':rows,'performance_claim':'Exact circuit costs only; no prover speedup measured.'}
    (PACKAGE/'checks.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps([{k:r[k] for k in ('instance','method','rows','nonzeros')} for r in rows],indent=2))
if __name__=='__main__':main()
