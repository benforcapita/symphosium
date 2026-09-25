#!/usr/bin/env python3
"""Validate fixture structure and test-header anchors; does not execute a runtime."""
import argparse
import json
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("--reference-root", required=True, type=Path)
args = parser.parse_args()
data = json.loads((Path(__file__).parent / "cases.json").read_text())
ids = set()
for case in data["cases"]:
    assert case["id"] not in ids, case["id"]
    assert case["id"].split("_", 1)[0] in {"github", "jira", "asana", "gitlab", "linear"}
    ids.add(case["id"])
    file_name, line = case["anchor"].rsplit(":", 1)
    actual = (args.reference_root / file_name).read_text().splitlines()[int(line) - 1]
    assert case["reference_test"] in actual, (case["id"], actual)
    for assertion_anchor in case.get("assertion_anchors", []):
        assertion_file, assertion_line = assertion_anchor.rsplit(":", 1)
        assertion = (args.reference_root / assertion_file).read_text().splitlines()[int(assertion_line) - 1]
        assert assertion.strip(), (case["id"], assertion_anchor)
    assert case["input"] and case["expected"] and case["rust_module"] and case["rust_test"]
print(f"validated {len(ids)} provider fixture anchors and schemas; runtime parity untested")
