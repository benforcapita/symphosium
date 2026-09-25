"""Build a sanitized assertion index from offline non-provider ExUnit tests.

This is extraction evidence, not a runnable or parity fixture. It keeps only
test names, line numbers, assertion operators and safe assertion text.
"""
import argparse
import json
import re
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('--reference-root', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
root = args.reference_root.resolve()
assert (root / 'SPEC.md').is_file()
test_dir = root / 'elixir/test/symphony_elixir'
files = [
    'core_test.exs', 'workspace_and_config_test.exs', 'extensions_test.exs',
    'app_server_test.exs', 'cli_test.exs', 'orchestrator_status_test.exs',
    'ssh_test.exs', 'dynamic_tool_test.exs', 'status_dashboard_snapshot_test.exs',
    'observability_pubsub_test.exs',
]
test_re = re.compile(r'^\s*test "([^"]+)" do\s*$')
assert_re = re.compile(r'^\s*(assert|refute|assert_receive|refute_receive)\b(.*)$')
secret_re = re.compile(r'(?i)(secret|token|password|api_key|credential|authorization|cookie|env\[)')
provider_re = re.compile(r'(?i)\b(linear|github|jira|asana|gitlab|provider adapter)\b')
entries = []
for file in files:
    lines = (test_dir / file).read_text().splitlines()
    current = None
    for n, line in enumerate(lines, 1):
        match = test_re.match(line)
        if match:
            current = None
            if not provider_re.search(match.group(1)):
                current = {'name': match.group(1), 'anchor': f'elixir/test/symphony_elixir/{file}:{n}', 'assertions': []}
                entries.append(current)
            continue
        if current and (match := assert_re.match(line)):
            expression = match.group(2).strip()
            if secret_re.search(expression):
                expression = '[redacted: secret-related assertion; inspect reference locally]'
            current['assertions'].append({'line': n, 'operator': match.group(1), 'expression': expression})
args.output.parent.mkdir(parents=True, exist_ok=True)
args.output.write_text(json.dumps({'schema_version': 1, 'reference_sha': 'be10a1b79df723d6d7612b5651c8522704dafb2e', 'tests': entries}, indent=2) + '\n')
print(f'indexed {len(entries)} offline tests and {sum(len(e["assertions"]) for e in entries)} assertion starts')
