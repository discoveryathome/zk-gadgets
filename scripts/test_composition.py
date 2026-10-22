import copy,json,tempfile,unittest
from pathlib import Path
from check_composition import check,PACKAGE
class CompositionTests(unittest.TestCase):
 def test_rejects_wrong_interface_and_forged_output(self):
  c=json.loads((PACKAGE/'export/matrices/composition-2-optimized.json').read_text())
  with tempfile.TemporaryDirectory() as tmp:
   p=Path(tmp)/'c.json';p.write_text(json.dumps(c));check(p,2)
   with self.assertRaises(ValueError):check(p,4)
   bad=copy.deepcopy(c);bad['assignment'][1]=str(int(bad['assignment'][1])+1);p.write_text(json.dumps(bad))
   with self.assertRaises((ValueError,AssertionError)):check(p,2)
if __name__=='__main__':unittest.main()
