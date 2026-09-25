# QA2 independent S0 R01/R02 gate review

**Project/repository:** Symphosium (`ops/symphosium-s0`)  
**Exact integrated SHA:** `cd2a7e98eaddfa988e093371ec1e227779a9a50f`  
**Reference baseline:** `be10a1b79df723d6d7612b5651c8522704dafb2e`  
**Authority:** `docs/PROJECT_LLD.md` revision 3, §§11–12.  
**Gate scope:** readiness to begin R05 only. This is not product parity, sprint/release acceptance, or authorization to relax later acceptance gates.

## Repository and execution evidence

- `git rev-parse HEAD` returned the exact SHA above; `git status --short` returned no entries before testing.
- Runtime fixture validator passed: `validated 38 fixture anchors and schemas; runtime parity untested`.
- Provider fixture validator passed: `validated 40 provider fixture anchors and schemas; runtime parity untested`.
- The disposable PostgreSQL 15 test container was named `qa2-r02-pg-stage0`, bound to loopback only, and was stopped/absent after the probe.
- No network dependency resolution, live provider, model call, paid service, Elixir suite, or product implementation test was run.

### Independently executed offline spike commands

From `docs/spike-r02`, after adding the documented direct Rust toolchain bin to `PATH`:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed. |
| `cargo clippy --offline --locked --all-targets -- -D warnings` | Passed. |
| `cargo test --offline --locked` | Passed: 5 passed, 0 failed, 1 ignored (the DB probe). |
| `cargo run --offline --locked` | Passed; output: `capability spike ok`. |
| `cargo build --release --offline --locked` | Passed. |
| `R02_PG_URL=<QA2 disposable loopback PostgreSQL 15 URL> cargo test --offline --locked postgres_ddl_and_transaction_rollback -- --ignored` | Passed: 1 passed, 0 failed. |

The first sandboxed attempt could not open Cargo's existing `target/debug/.cargo-build-lock` (`Operation not permitted`). Re-running the documented offline suite with approved normal host permissions passed; that permission was also necessary for the loopback-only provider test and disposable DB container. No external endpoint was used.

## R01 verdict — **Accepted for the Stage-0 mapping gate**

`R01_RUNTIME_STAGE0_MAP.md` and `R01_PROVIDER_STAGE0_MAP.md` provide finite, attributed planned-test coverage for required reference behavior, existing extensions, and LLD3-only behavior. The maps distinguish source anchors, existing fixtures, bounded later fixture work, and LLD-only requirements; they do not turn the 815 assertion-start index into an invented test requirement. The outstanding later-fixture rows are explicitly classified and planned, satisfying the LLD3 Stage-0 boundary rather than claiming executable parity.

The previously closed fixture defects remain closed at their limited fixture-review scope because their corpus/closure hashes are unchanged:

| Evidence | SHA-256 |
|---|---|
| `docs/fixtures-v1/cases.json` | `eab09daf61cd6acf8567f7c248ba4557439e7b2bd3ed12838b2465de8adda771` |
| `docs/fixtures-provider-v1/cases.json` | `881cf67025a418e306cc724c67d77b08d67ad09ccd72a5d4ecf1a18bcb29a6c9` |
| `docs/QA2_R01_REPAIR1_RETEST.md` | `b8fa978e62bd638c5e1c94bfe2ae306535eb23e3fd3fa28385594454adc176cd` |
| `docs/QA2_R01_FIXTURE_DEFECT_LEDGER.md` | `e125336ef752690ba20617f178e3c5674155d7dbd4950d1d0c336db9515991f9` |
| `docs/R01_RUNTIME_STAGE0_MAP.md` | `2ea0bb12f958eba229b7a00f88bdea9231506bb86663acdf60dc794e16285d3f` |
| `docs/R01_PROVIDER_STAGE0_MAP.md` | `39a05a6ac10e5c27039bfc0550d5a0f02220252563a5e85b0a3bda65cfd8e62e` |

**R01 limits:** named planned tests, fixture mappings, and LLD-only decisions must be implemented and executed on later integrated Rust SHAs. Provider pagination/normalization/config/native-tool matrices and runtime authorization/lifecycle/worker-isolation tests are not passed merely because they are mapped.

## R02 verdict — **Accepted for the bounded feasibility gate**

The exact SHA supplies a locked 269-package spike and direct, reproducible Darwin arm64 evidence for Tokio/Axum, SQLx/PostgreSQL transaction rollback, template/Markdown safety probes, YAML/Liquid, Argon2 hash/verify, synthetic cookie/CSRF boundary, local scoped Reqwest redirect/timeout behavior, bounded channel/SSE behavior, release build and lint/test gates. The independently repeated commands above confirm bounded feasibility rather than relying on developer claims.

Darwin arm64 license inventory/review covers 215 active target packages with cached manifest license declarations. Two MPL-2.0 crates (`cssparser`, `dtoa-short`) and one CDLA-Permissive-2.0 crate (`webpki-roots`) are recorded with notices for later redistribution review; this is not legal or vulnerability sign-off. The resolved artifacts are:

| Artifact | SHA-256 |
|---|---|
| `docs/spike-r02/Cargo.lock` | `31143f3d49238ab5aafe38d46b6d4755a5557e1147951f4f462d42b0d497ff7b` |
| `docs/spike-r02/Cargo.toml` | `342125bce274bedd7b60d83091b6c8d3aeb32d21107d95b94243614f0ff03d90` |
| `docs/spike-r02/license_inventory.json` | `49a71ef1699dd71da2d4b1cc36c3e5415f2a4963020b01b9d83af044350d48bf` |
| `docs/spike-r02/license_review_darwin_arm64.json` | `d1c4c052d958ea4a12b025405179473dc9f20d2b128d9dc4c32f05d906bbdf0a` |
| `docs/spike-r02/tree_darwin_arm64.txt` | `f6ba1588662085893fe33c10553e3b437d3ea3ed61b6c2fd4b6bc4b29876e42d` |

**R02 later obligations:** license and artifact evidence remains required for macOS x86_64, Linux arm64, and Linux x86_64; 40 locked packages outside the active Darwin graph lack cached manifests. Packaging, vulnerability/legal review, actual project dependency selection, product SQL schema/concurrency, auth/session/revocation, provider behavior, worker isolation, and browser/end-to-end parity are later gates.

## Gate conclusion

No open Critical/High/Medium defect blocks this **S0 R01/R02 readiness gate** at the reviewed SHA. R05 may proceed subject to ordinary Operations/PM workflow. This conclusion neither passes R05 nor certifies a product/release: later relevant changes invalidate it, and all R19/full-acceptance conditions remain mandatory.
