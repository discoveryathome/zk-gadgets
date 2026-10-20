use ark_bn254::{Bn254, Fr};
use ark_crypto_primitives::{
    crh::{
        poseidon::{
            constraints::{CRHGadget, CRHParametersVar, TwoToOneCRHGadget},
            TwoToOneCRH, CRH,
        },
        CRHScheme, CRHSchemeGadget, TwoToOneCRHScheme, TwoToOneCRHSchemeGadget,
    },
    merkle_tree::{
        constraints::{ConfigGadget, PathVar},
        Config, IdentityDigestConverter, Path,
    },
    sponge::poseidon::{find_poseidon_ark_and_mds, PoseidonConfig},
};
use ark_ff::{Field, PrimeField, UniformRand};
use ark_groth16::Groth16;
use ark_r1cs_std::{
    alloc::AllocVar,
    boolean::Boolean,
    eq::EqGadget,
    fields::{fp::FpVar, FieldVar},
    prelude::ToBitsGadget,
};
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, OptimizationGoal, SynthesisError,
    Variable,
};
use ark_snark::SNARK;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, sync::Arc, time::Instant};

type V = FpVar<Fr>;
type Bindings = BTreeMap<String, usize>;
fn index(v: Variable, cs: &ConstraintSystemRef<Fr>) -> usize {
    match v {
        Variable::Instance(i) => i,
        Variable::Witness(i) => cs.num_instance_variables() + i,
        _ => panic!("not a wire"),
    }
}
fn witness(
    cs: ConstraintSystemRef<Fr>,
    val: Fr,
    name: String,
    bind: &mut Bindings,
) -> Result<V, SynthesisError> {
    let x = V::new_witness(cs.clone(), || Ok(val))?;
    if let V::Var(v) = &x {
        bind.insert(name, index(v.variable, &cs));
    }
    Ok(x)
}
fn bit_witness(
    cs: ConstraintSystemRef<Fr>,
    val: bool,
    name: String,
    bind: &mut Bindings,
) -> Result<Boolean<Fr>, SynthesisError> {
    let x = Boolean::new_witness(cs.clone(), || Ok(val))?;
    if let Boolean::Var(v) = &x {
        bind.insert(name, index(v.variable(), &cs));
    }
    Ok(x)
}
fn bind_bits(
    bits: &[Boolean<Fr>],
    prefix: &str,
    cs: &ConstraintSystemRef<Fr>,
    bind: &mut Bindings,
) {
    for (i, b) in bits.iter().enumerate() {
        if let Boolean::Var(v) = b {
            bind.insert(format!("{prefix}{i}"), index(v.variable(), cs));
        }
    }
}
struct Tree;
impl Config for Tree {
    type Leaf = [Fr];
    type LeafDigest = Fr;
    type LeafInnerDigestConverter = IdentityDigestConverter<Fr>;
    type InnerDigest = Fr;
    type LeafHash = CRH<Fr>;
    type TwoToOneHash = TwoToOneCRH<Fr>;
}
struct TreeVar;
impl ConfigGadget<Tree, Fr> for TreeVar {
    type Leaf = [V];
    type LeafDigest = V;
    type LeafInnerConverter = IdentityDigestConverter<V>;
    type InnerDigest = V;
    type LeafHash = CRHGadget<Fr>;
    type TwoToOneHash = TwoToOneCRHGadget<Fr>;
}
fn params() -> PoseidonConfig<Fr> {
    let (ark, mds) = find_poseidon_ark_and_mds::<Fr>(254, 2, 8, 57, 0);
    PoseidonConfig::new(8, 57, 5, mds, ark, 2, 1)
}
fn leaf_params(p: &PoseidonConfig<Fr>) -> PoseidonConfig<Fr> {
    // Separate leaf and internal-node functions by a fixed capacity-domain offset.
    let mut q = p.clone();
    q.ark[0][0] += Fr::from(1u64);
    q
}
#[derive(Clone)]
struct Circuit {
    family: String,
    size: usize,
    method: String,
    a: Vec<Fr>,
    b: Vec<Fr>,
    bits: Vec<bool>,
    outputs: Vec<Fr>,
    p: Arc<PoseidonConfig<Fr>>,
}
fn root(c: &Circuit, x: Fr) -> Fr {
    let mut h = CRH::<Fr>::evaluate(&leaf_params(&c.p), [x]).unwrap();
    for i in 0..c.size {
        let (l, r) = if c.bits[i] { (c.b[i], h) } else { (h, c.b[i]) };
        h = TwoToOneCRH::<Fr>::evaluate(&c.p, l, r).unwrap();
    }
    h
}
fn make(family: &str, size: usize, method: &str, p: Arc<PoseidonConfig<Fr>>, seed: u64) -> Circuit {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut c = Circuit {
        family: family.into(),
        size,
        method: method.into(),
        a: vec![],
        b: vec![],
        bits: vec![],
        outputs: vec![],
        p,
    };
    match family {
        "range" | "reuse" => {
            let mask = if size == 64 {
                u64::MAX
            } else {
                (1u64 << size) - 1
            };
            let x = rng.next_u64() & mask;
            c.a.push(Fr::from(x));
            c.outputs.push(Fr::from(x));
            if family == "reuse" {
                c.outputs.push(Fr::from(
                    (0..size)
                        .filter(|i| (x >> i) & 1 == 1)
                        .map(|i| (i + 1) as u64)
                        .sum::<u64>(),
                ));
            }
        }
        "arithmetic" => {
            for _ in 0..size {
                let a = Fr::rand(&mut rng);
                let b = Fr::rand(&mut rng);
                c.a.push(a);
                c.b.push(b);
                c.outputs.push(a.square() - b.square());
            }
        }
        "swap" => {
            let a = Fr::rand(&mut rng);
            let b = Fr::rand(&mut rng);
            let s = rng.next_u32() % 2 == 1;
            c.a.push(a);
            c.b.push(b);
            c.bits.push(s);
            c.outputs = if s { vec![b, a] } else { vec![a, b] };
        }
        "merkle" | "composition" => {
            c.a.push(if family == "composition" {
                Fr::from(rng.next_u32() as u16)
            } else {
                Fr::rand(&mut rng)
            });
            for _ in 0..size {
                c.b.push(Fr::rand(&mut rng));
                c.bits.push(rng.next_u32() % 2 == 1);
            }
            c.outputs.push(root(&c, c.a[0]));
            if family == "composition" {
                c.a.push(Fr::rand(&mut rng));
                c.outputs.push(c.a[0].square() - c.a[1].square());
            }
        }
        _ => panic!("unknown family"),
    }
    c
}
fn bounded(x: &V, n: usize) -> Result<Vec<Boolean<Fr>>, SynthesisError> {
    Ok(x.to_bits_le_with_top_bits_zero(n)?.0)
}
fn merkle(
    c: &Circuit,
    cs: ConstraintSystemRef<Fr>,
    x: &V,
    bind: &mut Bindings,
) -> Result<V, SynthesisError> {
    let p = CRHParametersVar {
        parameters: (*c.p).clone(),
    };
    let lp = CRHParametersVar {
        parameters: leaf_params(&c.p),
    };
    if c.method == "baseline" {
        let path = Path::<Tree> {
            leaf_sibling_hash: c.b[0],
            auth_path: c.b[1..].iter().rev().copied().collect(),
            leaf_index: c
                .bits
                .iter()
                .enumerate()
                .map(|(i, b)| (*b as usize) << i)
                .sum(),
        };
        let start = cs.num_instance_variables() + cs.num_witness_variables();
        let pv = PathVar::<Tree, Fr, TreeVar>::new_witness(cs.clone(), || Ok(path))?;
        bind.insert("b0".into(), start);
        bind_bits(&pv.get_leaf_position(), "direction", &cs, bind);
        for i in 1..c.size {
            bind.insert(format!("b{i}"), start + c.size + 1 + (c.size - 1 - i));
        }
        pv.calculate_root(&lp, &p, &[x.clone()])
    } else {
        let mut h = CRHGadget::<Fr>::evaluate(&lp, &[x.clone()])?;
        for i in 0..c.size {
            let sibling = witness(cs.clone(), c.b[i], format!("b{i}"), bind)?;
            let bit = bit_witness(cs.clone(), c.bits[i], format!("direction{i}"), bind)?;
            let l = bit.select(&sibling, &h)?;
            let r = &h + &sibling - &l;
            h = TwoToOneCRHGadget::<Fr>::compress(&p, &l, &r)?;
        }
        Ok(h)
    }
}
impl Circuit {
    fn generate(
        self,
        cs: ConstraintSystemRef<Fr>,
        bind: &mut Bindings,
    ) -> Result<(), SynthesisError> {
        let out: Vec<V> = self
            .outputs
            .iter()
            .map(|v| V::new_input(cs.clone(), || Ok(v)))
            .collect::<Result<_, _>>()?;
        match self.family.as_str() {
            "range" | "reuse" => {
                let x = &out[0];
                let bits = if self.method == "full" {
                    let bits = x.to_bits_le()?;
                    for bit in &bits[self.size..] {
                        bit.enforce_equal(&Boolean::FALSE)?;
                    }
                    bits[..self.size].to_vec()
                } else {
                    bounded(x, self.size)?
                };
                bind_bits(&bits, "bit", &cs, bind);
                if self.family == "reuse" {
                    let again = if self.method == "baseline" {
                        bounded(x, self.size)?
                    } else {
                        bits
                    };
                    bind_bits(&again, "again", &cs, bind);
                    let sum = again.iter().enumerate().fold(V::zero(), |v, (i, b)| {
                        v + V::from(b.clone()) * Fr::from((i + 1) as u64)
                    });
                    sum.enforce_equal(&out[1])?;
                }
            }
            "arithmetic" => {
                for i in 0..self.size {
                    let a = witness(cs.clone(), self.a[i], format!("a{i}"), bind)?;
                    let b = witness(cs.clone(), self.b[i], format!("b{i}"), bind)?;
                    let y = if self.method == "baseline" {
                        a.square()? - b.square()?
                    } else {
                        (&a + &b) * (&a - &b)
                    };
                    y.enforce_equal(&out[i])?;
                }
            }
            "swap" => {
                let a = witness(cs.clone(), self.a[0], "a0".into(), bind)?;
                let b = witness(cs.clone(), self.b[0], "b0".into(), bind)?;
                let bit = bit_witness(cs.clone(), self.bits[0], "direction0".into(), bind)?;
                let l = bit.select(&b, &a)?;
                let r = if self.method == "baseline" {
                    bit.select(&a, &b)?
                } else {
                    &a + &b - &l
                };
                l.enforce_equal(&out[0])?;
                r.enforce_equal(&out[1])?;
            }
            "merkle" | "composition" => {
                let x = witness(cs.clone(), self.a[0], "a0".into(), bind)?;
                if self.family == "composition" {
                    let bits = bounded(&x, 16)?;
                    bind_bits(&bits, "bit", &cs, bind);
                    let y = witness(cs.clone(), self.a[1], "a1".into(), bind)?;
                    let result = if self.method == "baseline" {
                        x.square()? - y.square()?
                    } else {
                        (&x + &y) * (&x - &y)
                    };
                    result.enforce_equal(&out[1])?;
                }
                merkle(&self, cs.clone(), &x, bind)?.enforce_equal(&out[0])?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
impl ConstraintSynthesizer<Fr> for Circuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        self.generate(cs, &mut Bindings::new())
    }
}
fn synth(c: &Circuit) -> ConstraintSystemRef<Fr> {
    let cs = ConstraintSystem::<Fr>::new_ref();
    cs.set_optimization_goal(OptimizationGoal::Constraints);
    c.clone().generate_constraints(cs.clone()).unwrap();
    cs.finalize();
    cs
}
fn fsval(x: &Fr) -> String {
    x.into_bigint().to_string()
}
fn matrix(m: &[Vec<(Fr, usize)>]) -> Value {
    json!(m
        .iter()
        .map(|r| r
            .iter()
            .map(|(v, i)| json!([fsval(v), i]))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn export(c: &Circuit, path: &str) -> Value {
    let cs = ConstraintSystem::<Fr>::new_ref();
    cs.set_optimization_goal(OptimizationGoal::Constraints);
    let mut bindings = Bindings::new();
    c.clone().generate(cs.clone(), &mut bindings).unwrap();
    cs.finalize();
    assert!(cs.is_satisfied().unwrap());
    let m = cs.to_matrices().unwrap();
    let borrow = cs.borrow().unwrap();
    let mut assignment = borrow.instance_assignment.clone();
    assignment.extend(&borrow.witness_assignment);
    let nonlinear =
        m.a.iter()
            .zip(&m.b)
            .filter(|(a, b)| a.iter().any(|(_, i)| *i != 0) && b.iter().any(|(_, i)| *i != 0))
            .count();
    let value = json!({"field":Fr::MODULUS.to_string(),"family":c.family,"size":c.size,"method":c.method,"public_count":m.num_instance_variables-1,"assignment":assignment.iter().map(fsval).collect::<Vec<_>>(),"a":matrix(&m.a),"b":matrix(&m.b),"c":matrix(&m.c),"bindings":bindings,"inputs":{"a":c.a.iter().map(fsval).collect::<Vec<_>>(),"b":c.b.iter().map(fsval).collect::<Vec<_>>(),"bits":c.bits},"outputs":c.outputs.iter().map(fsval).collect::<Vec<_>>()});
    fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
    json!({"constraints":m.num_constraints,"multiplication_rows":nonlinear,"witness_variables":m.num_witness_variables,"nonzeros":m.a_num_non_zero+m.b_num_non_zero+m.c_num_non_zero})
}
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dest = args.get(1).map(String::as_str).unwrap_or("results");
    let repeats: usize = std::env::var("PILOT_REPEATS")
        .unwrap_or("7".into())
        .parse()
        .unwrap();
    fs::create_dir_all(format!("{dest}/matrices")).unwrap();
    let p = Arc::new(params());
    fs::write(format!("{dest}/poseidon.json"),serde_json::to_vec_pretty(&json!({"field":Fr::MODULUS.to_string(),"full_rounds":8,"partial_rounds":57,"alpha":5,"rate":2,"capacity":1,"leaf_capacity_offset":"1","ark":p.ark.iter().map(|r|r.iter().map(fsval).collect::<Vec<_>>()).collect::<Vec<_>>(),"mds":p.mds.iter().map(|r|r.iter().map(fsval).collect::<Vec<_>>()).collect::<Vec<_>>()})).unwrap()).unwrap();
    let mut cases = vec![];
    for n in [8, 16, 32, 64] {
        cases.push(("range", n, vec!["full", "bounded", "optimized"]));
        cases.push(("reuse", n, vec!["baseline", "optimized"]));
    }
    for n in [1, 4, 16, 64] {
        cases.push(("arithmetic", n, vec!["baseline", "optimized"]));
    }
    cases.push(("swap", 1, vec!["baseline", "optimized"]));
    for n in [2, 4, 8, 16] {
        cases.push(("merkle", n, vec!["baseline", "optimized"]));
    }
    cases.push(("composition", 8, vec!["baseline", "optimized"]));
    let mut rows = vec![];
    for (family, n, methods) in cases {
        let seed = 20261020 + n as u64;
        let circuits: Vec<_> = methods
            .iter()
            .map(|m| make(family, n, m, p.clone(), seed))
            .collect();
        let mut measurements = vec![vec![]; methods.len()];
        // Warmups, then alternating order. Every sample includes synthesis and finalize.
        for c in &circuits {
            synth(c);
        }
        for rep in 0..repeats {
            for k in 0..methods.len() {
                let i = if rep % 2 == 0 {
                    k
                } else {
                    methods.len() - 1 - k
                };
                let t = Instant::now();
                let cs = synth(&circuits[i]);
                let elapsed = ms(t);
                assert!(cs.is_satisfied().unwrap());
                measurements[i].push(elapsed);
            }
        }
        for (i, c) in circuits.iter().enumerate() {
            let name = format!("{family}-{n}-{}", c.method);
            let file = format!("matrices/{name}.json");
            let costs = export(c, &format!("{dest}/{file}"));
            let mut bad = c.clone();
            if family == "range" || family == "reuse" { bad.outputs[0] = Fr::from(2u64).pow([n as u64]); } else { bad.outputs[0] += Fr::from(1u64); }
            assert!(!synth(&bad).is_satisfied().unwrap());
            let row = json!({"instance":format!("{family}-{n}"),"family":family,"size":n,"method":c.method,"seed":seed,"split":if (family=="range"||family=="reuse")&&n==8 || family=="arithmetic"&&n==1 || family=="merkle"&&n==2 || family=="swap" {"development"} else {"held-out"},"cost":costs,"synthesis_ms":measurements[i],"matrix":file,"wrong_public_rejected":true});
            println!("{}: {} rows", name, costs["constraints"]);
            rows.push(row);
        }
    }
    fs::write(
        format!("{dest}/benchmarks.json"),
        serde_json::to_vec_pretty(&rows).unwrap(),
    )
    .unwrap();
    // Groth16 fixed single-thread benchmark. Setup excluded, proof verification included as a check only.
    let mut proofrows = vec![];
    for (family, n) in [("arithmetic", 64), ("merkle", 8), ("composition", 8)] {
        let circuits: Vec<_> = ["baseline", "optimized"]
            .iter()
            .map(|m| make(family, n, m, p.clone(), 20261020 + n as u64))
            .collect();
        let mut rng = ChaCha8Rng::seed_from_u64(601_2026);
        let mut keys = vec![];
        for c in &circuits {
            let t = Instant::now();
            let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(c.clone(), &mut rng).unwrap();
            keys.push((pk, Groth16::<Bn254>::process_vk(&vk).unwrap(), ms(t)));
        }
        let mut samples = vec![vec![]; 2];
        let mut aa = vec![vec![]; 2];
        for rep in 0..repeats + 1 {
            for k in 0..2 {
                let i = if rep % 2 == 0 { k } else { 1 - k };
                let t = Instant::now();
                let proof =
                    Groth16::<Bn254>::prove(&keys[i].0, circuits[i].clone(), &mut rng).unwrap();
                let elapsed = ms(t);
                assert!(Groth16::<Bn254>::verify_with_processed_vk(
                    &keys[i].1,
                    &circuits[i].outputs,
                    &proof
                )
                .unwrap());
                let mut bad = circuits[i].outputs.clone();
                bad[0] += Fr::from(1u64);
                assert!(
                    !Groth16::<Bn254>::verify_with_processed_vk(&keys[i].1, &bad, &proof).unwrap()
                );
                if rep > 0 {
                    samples[i].push(elapsed);
                }
            }
        }
        for rep in 0..repeats {
            for k in 0..2 {
                let i = if rep % 2 == 0 { k } else { 1 - k };
                let t = Instant::now();
                let proof =
                    Groth16::<Bn254>::prove(&keys[0].0, circuits[0].clone(), &mut rng).unwrap();
                let elapsed = ms(t);
                assert!(Groth16::<Bn254>::verify_with_processed_vk(
                    &keys[0].1,
                    &circuits[0].outputs,
                    &proof
                )
                .unwrap());
                aa[i].push(elapsed);
            }
        }
        for i in 0..2 {
            proofrows.push(json!({"instance":format!("{family}-{n}"),"method":circuits[i].method,"setup_ms":keys[i].2,"prover_ms":samples[i],"aa_baseline_ms":aa[i],"proofs_verified":repeats+1,"wrong_public_rejected":true}));
        }
        println!("Groth16 {family}-{n} verified");
    }
    fs::write(
        format!("{dest}/proofs.json"),
        serde_json::to_vec_pretty(&proofrows).unwrap(),
    )
    .unwrap();
}
