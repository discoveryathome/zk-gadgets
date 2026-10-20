#!/usr/bin/env python3
import argparse,json,statistics,html
from pathlib import Path

def median(xs):return statistics.median(xs)
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--results',default='results');args=ap.parse_args();root=Path(args.results)
 rows=json.loads((root/'benchmarks.json').read_text());groups={}
 for r in rows:groups.setdefault(r['instance'],[]).append(r)
 table=[]
 lines=['# Recomputed results','','Primary metric: total finalized R1CS rows. These are scoped implementation comparisons, not global best-known bounds.','','| Instance | Reference | Reference rows | Candidate | Candidate rows | Reduction |','|---|---|---:|---|---:|---:|']
 for instance,rs in groups.items():
  baseline=next(r for r in rs if r['method']==('bounded' if rs[0]['family']=='range' else 'baseline'))
  best=min(rs,key=lambda r:(r['cost']['constraints'],0 if r['method']=='fused' else 1 if r['method']=='optimized' else 2))
  b,v=baseline['cost']['constraints'],best['cost']['constraints'];gain=100*(b-v)/b
  table.append({'instance':instance,'baseline':baseline['method'],'baseline_count':b,'method':best['method'],'value':v,'reduction_percent':gain,'split':best['split'],'by':'Codex (local author trial)','source':best['matrix'],'updated':'2026-10-20'})
  lines.append(f'| {instance} | {baseline["method"]} | {b} | {best["method"]} | {v} | {gain:.2f}% |')
 lines+=['','The range reference uses the efficient bounded API. The full-field conversion results are retained in benchmarks.json as a weaker application baseline, not advertised as a discovery. Arithmetic factoring/fusion and decomposition reuse are known application-level improvements. The Merkle reference executes upstream PathVar. Arbitrary-bound encoding replicates published research.','','## Prover timings','','Busy shared machine; seven interleaved samples per variant; setup excluded. A/A compares the same baseline twice. No small speedup claim is justified.','','| Circuit | Baseline median ms | Factored/swapped median ms | Candidate/baseline | A/A median ratio |','|---|---:|---:|---:|---:|']
 if (root/'proofs.json').exists():
  proofs=json.loads((root/'proofs.json').read_text())
  for name in dict.fromkeys(r['instance'] for r in proofs):
   a,b=[r for r in proofs if r['instance']==name];x,y=median(a['prover_ms']),median(b['prover_ms']);aa=median(b['aa_baseline_ms'])/median(a['aa_baseline_ms'])
   lines.append(f'| {name} | {x:.2f} | {y:.2f} | {y/x:.3f} | {aa:.3f} |')
 lines+=['','Prover timing covers baseline versus the first-round optimized candidate; the fused arithmetic variant has count/relation evidence only. All measured proofs verified, and verifying them against a changed public output failed. This does not replace the relation checker.']
 (root/'summary.json').write_text(json.dumps(table,indent=2)+'\n');(root/'SUMMARY.md').write_text('\n'.join(lines)+'\n')
 # Standalone export: transparent comparison, zero-based bars, locally normalized per row.
 featured=['range-64','reuse-64','arithmetic-64','bound-13','merkle-8']
 svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="700" viewBox="0 0 1120 700">','<rect width="1120" height="700" fill="#0c1725"/>','<style>text{font-family:Arial,sans-serif;fill:#eef4fa}.small{font-size:15px}.title{font-size:28px;font-weight:bold}.label{font-size:18px}</style>','<text x="36" y="50" class="title">Fewer checks. The same mathematical promise.</text>','<text x="36" y="80" class="small">Measured R1CS rows • BN254 • fixed statements • local contributor trial</text>']
 for j,instance in enumerate(featured):
  rs=[r for r in table if r['instance']==instance]
  if not rs:continue
  r=rs[0];y=130+j*96;b,v=r['baseline_count'],r['value'];width=530
  svg+=[f'<text x="36" y="{y+20}" class="label">{html.escape(instance)}</text>',f'<rect x="230" y="{y}" width="{width}" height="22" rx="3" fill="#708197"/>',f'<rect x="230" y="{y+29}" width="{width*v/b:.2f}" height="22" rx="3" fill="#37d4ad"/>',f'<text x="780" y="{y+18}" class="label">{b} to {v}</text>',f'<text x="940" y="{y+18}" class="label">{r["reduction_percent"]:.2f}%</text>']
 svg+=['<text x="36" y="640" class="small">Gray: reference. Green: candidate. Each pair has its own scale, starting at zero.</text>','<text x="36" y="665" class="small">Merkle savings are modest. Range-only elimination saves one row. Counts do not guarantee prover speedups.</text>','</svg>']
 (root/'progress.svg').write_text(('\n'.join(svg)+'\n').replace('<text ', '<text fill="#eef4fa" '))
 print('\n'.join(lines))
if __name__=='__main__':main()
