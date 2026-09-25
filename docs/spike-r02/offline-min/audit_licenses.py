"""Inventory locked crate manifest licenses from an existing local Cargo cache."""
import argparse
import json
import tomllib
from collections import Counter
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('--cargo-cache', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
lock = tomllib.loads((Path(__file__).parent / 'Cargo.lock').read_text())
rows = []
for package in lock['package']:
    if not package.get('source', '').startswith('registry+'):
        continue
    crate = args.cargo_cache / f"{package['name']}-{package['version']}" / 'Cargo.toml'
    if not crate.is_file():
        rows.append({'name': package['name'], 'version': package['version'], 'license': None, 'evidence': 'cached manifest missing'})
        continue
    meta = tomllib.loads(crate.read_text())['package']
    rows.append({'name': package['name'], 'version': package['version'], 'license': meta.get('license'), 'evidence': 'cached Cargo.toml'})
args.output.write_text(json.dumps({'lockfile': 'offline-min/Cargo.lock', 'packages': rows}, indent=2) + '\n')
counts = Counter(row['license'] for row in rows)
print(f"{len(rows)} registry packages; {sum(row['license'] is None for row in rows)} missing license declarations/manifests")
for license, count in sorted(counts.items(), key=lambda item: str(item[0])):
    print(f'{license}: {count}')
