"""Read-only integrity check for the preparation package and changed source files."""
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
package=Path(__file__).resolve().parent
manifest=json.loads((package/'manifest.json').read_text())
for name,expected in manifest['files'].items():
    path=ROOT/name
    if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest()!=expected:
        raise ValueError('Preparation binding changed: '+name)
print('PASS',len(manifest['files']),'preparation file hashes; independent review remains pending')
