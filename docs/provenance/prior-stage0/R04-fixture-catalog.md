# R04 — Reference fixture catalog and offline baseline

Status: inventory and command discovery complete; execution checks blocked by missing Elixir/Mix. No source checkout files were changed.

## Scope and provenance

- Reference checkout: `/Users/benblum/.openmausbot/task-workspaces/d6f30c19-02d5-4618-a1fe-3ce7aec47fd1/5f84a53f-c70a-4bd5-8fb4-338abf6c691d/symphony-linear`
- Verified source HEAD: `be10a1b79df723d6d7612b5651c8522704dafb2e`; only source status item was untracked `docs/PROJECT_HLD.md`.
- Design authority: CTO `PROJECT_LLD.md` revision 2 and `RUST_CHANGE_REQUEST.md`; HLD retained as provenance.
- Read-only working snapshot: `reference-copy/` in this workspace. It was copied from the reference, then locally snapshotted as `ce5d991b7f54d6de192da1a9c745487bed5a80ca`; that local snapshot SHA is not the upstream/source revision.
- Current implementation is under `elixir/`; there is no `rust/` directory or Cargo manifest in the source baseline.

## On-disk test fixtures (11 files)

| Fixture | Count | Purpose / likely parity use | Sanitization notes |
|---|---:|---|---|
| `elixir/test/fixtures/startup_workflow.md` | 1 | Minimal `tracker.kind: memory` workflow and prompt. `elixir/config/config.exs` selects it for the test workflow path. Useful for front matter, config defaults, and prompt behavior. | Contains no credential; command is `codex app-server`. |
| `elixir/test/fixtures/status_dashboard_snapshots/idle.{snapshot.txt,evidence.md}` | 2 | Empty scheduler/dashboard projection; asserted by `status_dashboard_snapshot_test.exs`. | Sample identifiers and placeholder Linear URL only. |
| `.../idle_with_dashboard_url.{snapshot.txt,evidence.md}` | 2 | Idle rendering including loopback dashboard URL; same test module. | Uses `127.0.0.1`; no credential. |
| `.../super_busy.{snapshot.txt,evidence.md}` | 2 | Busy status, token totals, model label, and long-running row rendering; same test module. | Synthetic-looking IDs/PIDs/counts and truncated session display (`thre...567890`); no raw token found. |
| `.../backoff_queue.{snapshot.txt,evidence.md}` | 2 | Retry/backoff ordering and queue overflow rendering; same test module. | Synthetic-looking IDs/PIDs/counts and truncated session display; no raw token found. |
| `.../credits_unlimited.{snapshot.txt,evidence.md}` | 2 | Unlimited-credit dashboard formatting; same test module. | Synthetic-looking IDs/PIDs/counts and truncated session display; no raw token found. |

All five dashboard cases have ANSI snapshot output plus human-readable evidence output. The evidence is not independent runtime proof; Rust parity should compare behavior with both test fixture forms. `test/support/snapshot_support.exs` warns that `UPDATE_SNAPSHOTS=1` rewrites fixtures, so never set that variable during a read-only parity run.

No separate JSON/YAML provider-response fixture files exist under `elixir/test/fixtures`; adapter/app-server test scenarios are built inline in ExUnit test modules and helper code. Test inventory below identifies those modules for Senior 1's mapping.

A targeted scan of the fixture directory found no password, bearer authorization, API key, or credential value. It did find the generic `https://linear.app/project/project/issues` sample and loopback URL. This is a code-review scan, not a secret-scanner certification.

## Test-module inventory (25 files)

**Offline/local candidates** (do not imply passing):

- Mix tasks: `test/mix/tasks/{pr_body_check,specs_check_task,workspace_before_remove}_test.exs`
- Runtime/config/protocol: `app_server_test.exs`, `cli_test.exs`, `core_test.exs`, `dynamic_tool_test.exs`, `extensions_test.exs`, `log_file_test.exs`, `orchestrator_status_test.exs`, `specs_check_test.exs`, `ssh_test.exs`, `status_dashboard_snapshot_test.exs`, `workspace_and_config_test.exs`
- Adapter/local behavior: `asana_adapter_test.exs`, `github_adapter_test.exs`, `gitlab_adapter_test.exs`, `gitlab_redirect_test.exs`, `jira_adapter_test.exs`
- Local HTTP/UI/observability behavior is also represented in `core_test.exs`, `observability_pubsub_test.exs`, `status_dashboard_snapshot_test.exs`, and `workspace_and_config_test.exs`.

**Live tests — do not run in R04 or ordinary offline parity checks:**

| File | Opt-in variable | Side effects / dependencies |
|---|---|---|
| `asana_live_e2e_test.exs` | `SYMPHONY_RUN_ASANA_LIVE_E2E=1` | Creates a real Asana task, invokes Codex, then cleans up; requires Asana credentials/workspace and model access. |
| `github_live_e2e_test.exs` | `SYMPHONY_RUN_GITHUB_LIVE_E2E=1` | Creates/closes a real GitHub issue and comments; invokes Codex; requires GitHub token/repository and model access. |
| `gitlab_live_e2e_test.exs` | `SYMPHONY_RUN_GITLAB_LIVE_E2E=1` | Creates/comments/closes/deletes a real GitLab issue; invokes Codex; requires GitLab token/project and model access. |
| `jira_live_e2e_test.exs` | `SYMPHONY_RUN_JIRA_LIVE_E2E=1` | Creates/transitions/comments on a real Jira issue; invokes Codex; requires Jira credentials/project and model access. |
| `live_e2e_test.exs` (Linear) | `SYMPHONY_RUN_LIVE_E2E=1` | Creates tracker records and worker containers, runs Codex agents and mutates remote issues; requires Linear and model access plus Docker. |

Each live module is tagged `:live_e2e`. Provider suites also put a skip reason on their tests unless the corresponding opt-in variable equals `1`. `elixir/Makefile` target `e2e` forcibly opts in to Linear live E2E. Do not run `make e2e`, set any opt-in variable, or supply live credentials for this task. Docker is present locally, but that is not reason to run the live suite.

## Safe command/setup findings

- `elixir/mise.toml` declares Erlang/OTP 28 and Elixir 1.19.5-otp-28; `mix.exs` specifies Elixir `~> 1.19`; `mix.lock` is present.
- This environment has no `mix`, `elixir`, `erl`, or `mise` executable on PATH. The copied project has neither `elixir/deps/` nor `elixir/_build/`; therefore no cached dependencies or compiled test artifacts were available.
- Rust is installed via rustup (Cargo 1.97.1, rustc 1.97.1, rustfmt and clippy components), but the binaries are not on PATH. No Rust project exists in the baseline, so there is no Rust check target yet.
- Candidate strictly offline Elixir checks after the Elixir toolchain and locked dependencies have been provisioned by the authorized setup owner: from `elixir/`, `mix format --check-formatted`; `mix lint`; `mix test --cover --exclude live_e2e`. Keep all `SYMPHONY_RUN_*_LIVE_E2E` variables unset. These commands were not run successfully here.
- `make all` is not an offline check: `elixir/Makefile` expands `ci` to `make setup`, and `setup` runs `mix setup` (dependency retrieval); `dialyzer` also runs `mix deps.get`. Do not use `make all` as a no-network baseline command. The explicit `make e2e` target opts into live Linear E2E.

## Actual command results (run only in `reference-copy/`)

- `git rev-parse HEAD` in the source checkout: `be10a1b79df723d6d7612b5651c8522704dafb2e`; source `git status --short`: `?? docs/PROJECT_HLD.md`.
- `find elixir/test -type f -name '*_test.exs' | wc -l` → 25.
- `find elixir/test/fixtures -type f | wc -l` → 11.
- `make fmt-check` → exit 2, `make: mix: No such file or directory`.
- `mix test --cover --exclude live_e2e` → exit 127, `mix: command not found`.
- `make -n all` → exit 0; confirms `make all` invokes `mix setup`, `mix build`, `mix format --check-formatted`, `mix lint`, `mix test --cover`, `mix deps.get`, and `mix dialyzer --format short`; this was dry-run only, no install attempted.
- `rustup run stable cargo --version` → Cargo 1.97.1; `rustup run stable rustc --version` → rustc 1.97.1.
- `cargo fmt --all -- --check` via PATH → exit 127, `cargo: command not found`; `rustup` can locate tool binaries, but the reference has no `Cargo.toml` so Rust checks are not applicable to this baseline.

No tests passed or failed: test execution is blocked before startup by the missing Mix executable. No dependencies were installed, no live suite was run, and no source/reference files were modified.

## Owner status / estimate

- R04 planned size: 1–2 person-days. Inventory, safe command discovery and this report are complete; execution evidence remains blocked pending a toolchain/dependency environment. No Rust implementation work was part of this task.
- Rust competency: no implementation evidence from this task; only an installed rustup toolchain was confirmed. Do not infer Rust proficiency from the role label or binary presence.
- Availability and competing assignments: not supplied to this task context; PM should record as unverified before calendar commitments.

## Handoff

Send this artifact to PM for Senior 1 parity mapping and Operations for the required working-branch checkpoint. Source revision is `be10a1b79df723d6d7612b5651c8522704dafb2e`; local snapshot revision above is only this isolated copy. No QA verdict is claimed.
