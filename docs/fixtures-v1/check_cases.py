"""Structural fixture/anchor check; does not execute either runtime."""
import argparse
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--reference-root', type=Path, required=True)
args = parser.parse_args()
reference = args.reference_root.resolve()
assert (reference / 'SPEC.md').is_file(), reference
data = json.loads((root / 'fixtures-v1/cases.json').read_text())
ids = set()
for case in data['cases']:
    assert case['id'] not in ids
    ids.add(case['id'])
    name, line = case['anchor'].rsplit(':', 1)
    source = (reference / name).read_text().splitlines()
    assert case['reference_test'] in source[int(line) - 1], case['id']
    assert case['input'] and case['expected'] and case['rust_module'] and case['rust_test']
    for assertion in case.get('assertion_anchors', []):
        assert assertion['contains'] in source[assertion['line'] - 1], (case['id'], assertion)
index_path = root / 'fixtures-v1/reference_assertions.json'
if index_path.is_file():
    index = json.loads(index_path.read_text())
    for entry in index['tests']:
        name, line = entry['anchor'].rsplit(':', 1)
        source = (reference / name).read_text().splitlines()
        assert entry['name'] in source[int(line) - 1], entry['anchor']
        for assertion in entry['assertions']:
            assert assertion['operator'] in source[assertion['line'] - 1], entry['anchor']
print(f"validated {len(ids)} fixture anchors and schemas; runtime parity untested")
