#!/usr/bin/env python3
import copy
import unittest
from verify import P, relation_check, rows_satisfied, exhaustive_range

def swap():
    # z=(1,L,R,a,b,s,t). t=s(b-a), L=a+t, R=b-t.
    return {'field':str(P),'family':'swap','size':1,'method':'optimized','public_count':2,
        'bindings':{'a0':3,'b0':4,'direction0':5},'assignment':list(map(str,[1,11,7,7,11,1,4])),
        'a':[[['1',5]],[['1',5]],[],[]],
        'b':[[['1',5],[str(P-1),0]],[['1',4],[str(P-1),3]],[],[]],
        'c':[[],[['1',6]],[['1',1],[str(P-1),3],[str(P-1),6]],[['1',2],[str(P-1),4],['1',6]]]}

class VerifierTests(unittest.TestCase):
    def test_sound_swap(self):
        d=swap(); self.assertTrue(rows_satisfied(d)); relation_check(d,{})
    def test_missing_boolean(self):
        d=swap()
        for key in ('a','b','c'):d[key].pop(0)
        d['assignment']=list(map(str,[1,15,3,7,11,2,8]))
        self.assertTrue(rows_satisfied(d))
        with self.assertRaisesRegex(AssertionError,'missing boolean'):relation_check(d,{})
    def test_unique_wrong_output(self):
        d=swap();d['c'][2].append([str(P-1),0]);d['assignment'][1]='12'
        self.assertTrue(rows_satisfied(d))
        with self.assertRaisesRegex(AssertionError,'relation mismatch'):relation_check(d,{})
    def test_detached_output(self):
        d=swap()
        for key in ('a','b','c'):d[key].pop()
        with self.assertRaisesRegex(AssertionError,'unconstrained'):relation_check(d,{})
    def test_aliased_inputs(self):
        d=swap();d['bindings']['b0']=3
        with self.assertRaisesRegex(AssertionError,'aliased'):relation_check(d,{})
    def test_nonlinear_undefined_wire(self):
        d=swap();d['a'][1]=[['1',6]];d['b'][1]=[['1',6]];d['c'][1]=[['7',0]]
        with self.assertRaisesRegex(AssertionError,'nonlinear unknown'):relation_check(d,{})
    def test_wrong_modulus(self):
        d=swap();d['field']='17'
        with self.assertRaises(AssertionError):relation_check(d,{})
    def test_exhaustive_bound(self):self.assertEqual(exhaustive_range(8)['all_boolean_assignments'],256)

if __name__=='__main__':unittest.main()
