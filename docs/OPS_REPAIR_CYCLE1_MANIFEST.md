# Operations integration manifest — Symphosium S0 repair cycle 1

Repository: canonical Symphosium source repo, origin `https://github.com/benforcapita/symphosium.git` (upstream `openai/symphony` is reference-only). Branch: `ops/symphosium-s0`. Reference baseline: `be10a1b79df723d6d7612b5651c8522704dafb2e`.

## Checkpoint sequence

- Preservation commit: `5998f31f69d276d8b44ed3fbffdba3ca2f66f45f`, parent baseline. It adds only the previously untracked HLD and LLD revision 3 byte-for-byte.
- Current repair checkpoint: the full SHA is the commit that adds this manifest and the listed artifacts; its parent is the preservation commit above.
- Prior discovery commit imported selectively from the Operations discovery checkout: `2d5d6dce6627d3d6f373f5cd991cb2fd84c5de68`. Relevant discovery files are retained in `docs/provenance/prior-stage0/`; active fixture data is integrated under `docs/fixtures-v1/` and `docs/fixtures-provider-v1/`. Prior LLD2 plan/design files were not copied over LLD3.
- Publication attempt: normal push to verified origin was blocked because Git could not obtain a GitHub username with terminal prompts disabled. No remote update is claimed; the integration branch remains local.

## Repair inputs and operations

- Senior 1 repair patch: `docs/provenance/R01_FX_REPAIR_CYCLE1.patch` (original filename `R01_FX_REPAIR_CYCLE1.patch`), SHA-256 `56cc38b66512777e1d6eff04271e5a9440edc64f38660b40c2965231b542da2b`. `git apply --check` passed before application. Patch was applied only to `docs/fixtures-v1/cases.json`; FX-001 and FX-002 remain open until independent QA2 retest.
- Senior 1 companion artifacts: `R01_FX_REPAIR_CYCLE1.md`, `R01_RUNTIME_GAP_MAP_v4.md`, `R01_PARITY_MATRIX_v3.md`, `R02_EXECUTION_LOG_v3.md`, `fixtures-v1/lld_only_tests.json`, `fixtures-v1/reference_assertions.json`, `fixtures-v1/COMPLETENESS.md`, and `fixtures-v1/build_reference_index.py`.
- Mid 1 provider corpus imported separately, without overwriting runtime fixtures: `fixtures-provider-v1/{cases.json,check_cases.py,COVERAGE_MANIFEST.md}`.
- Latest PM `DELIVERY_PLAN.md`, `STAGE0_REVIEW.md`, routing document `DEFECT_ROUTING.md`, and QA2 `QA2_STAGE0_REVIEW.md` were copied from their named source paths byte-for-byte. Their checksums are recorded below.

## Checks actually run

- `python3 docs/fixtures-v1/check_cases.py --reference-root <canonical reference checkout>` → `validated 38 fixture anchors and schemas; runtime parity untested`.
- `python3 docs/fixtures-provider-v1/check_cases.py --reference-root <canonical reference checkout>` → `validated 40 provider fixture anchors and schemas; runtime parity untested`.
- `python3 -m json.tool docs/fixtures-v1/lld_only_tests.json` → passed.
- Gap-map line scan → 92 listed runtime scenario line anchors begin with `test`.
- `git apply --check` for the routed repair patch → passed before application.

These structural checks establish neither runtime parity nor QA acceptance. The copied QA2 review predates this 38/40 repaired corpus and its four Medium defects remain open pending QA2's independent review/retest of the exact integrated SHA.

## SHA-256 manifest

| Path | SHA-256 |
|---|---|
| `docs/PROJECT_HLD.md` | `b6bcf8aaf1774135185b343d4667c2a3ba73564a8f694de98029fb44e189cfa2` |
| `docs/PROJECT_LLD.md` | `4411658ffc280eedd198f13dc2ca2801d2492aceb52f07c05fc458fa59ff96f0` |
| `docs/DELIVERY_PLAN.md` | `e701067b1d4a3aa4583c5605570ac21482bdaa4d2a32b3a85d5e3d7d1327ce94` |
| `docs/DEFECT_ROUTING.md` | `900cb2d55f55c4e72adae4e33e0947ec41b5496e9fcfbe186416ac998dc81a67` |
| `docs/QA2_STAGE0_REVIEW.md` | `05d8b1ef2b3ca2e7b0a907c12565924eaf6d8bf83730c43cecc836b6687aec9e` |
| `docs/R01_FX_REPAIR_CYCLE1.md` | `eb8d8cf5e9012da04b36715597a29b0fa8c8338b7c08d79fbddc7f3b2e7d6bc4` |
| `docs/R01_RUNTIME_GAP_MAP_v4.md` | `d0f79fbaa5d2b45410e9dcdaabde13cdad5bc1d581fb735727dd8f522e90fef6` |
| `docs/R01_PARITY_MATRIX_v3.md` | `63925b9cc78f56e85c03d22ceaf1e4b044227fcf3960f9f8ab09306c978c009f` |
| `docs/R02_EXECUTION_LOG_v3.md` | `1abc0f890c5414ad5d9f48a9e3424b4c3d944270820cb9fa20af44a813da1aa0` |
| `docs/fixtures-v1/cases.json` | `eab09daf61cd6acf8567f7c248ba4557439e7b2bd3ed12838b2465de8adda771` |
| `docs/fixtures-v1/check_cases.py` | `40fdeeb7f4066799364b183ebeb22cda7d7a5ac15086434633acb46a8ed38171` |
| `docs/fixtures-v1/lld_only_tests.json` | `f62d0bacce5464919a0e02be735d835c7fe5a796c4e75d9732446d7ac937647b` |
| `docs/fixtures-provider-v1/cases.json` | `881cf67025a418e306cc724c67d77b08d67ad09ccd72a5d4ecf1a18bcb29a6c9` |
| `docs/fixtures-provider-v1/check_cases.py` | `6e057d362639b66f677e5d9c6be5dafc08d748b741efc12a6c33668ffb41acbc` |
| `docs/fixtures-provider-v1/COVERAGE_MANIFEST.md` | `406669cf43ac679eea580c5550880f031e6b2375897430c3e72e243033e037bd` |
| `docs/spike-r02/Cargo.toml` | `8760356687402f1de2e963784452703fd8d000c57641ddb7bd197cd920946a09` |
| `docs/spike-r02/rust-toolchain.toml` | `e7155b5437585c3c3acc3053b08c6936020e9e93cc343e47e7e25f0033418e84` |
| `docs/spike-r02/src/main.rs` | `f47fdb79aa21e88bd4fcd722fefcdfb2df8d5efea26e7db5a87ecd5dbb9c402a` |

LLD3 supersedes earlier design revisions. HLD and LLD source hashes above match the original untracked source documents exactly. No source-code implementation changed. No release tag or production deployment was made.
