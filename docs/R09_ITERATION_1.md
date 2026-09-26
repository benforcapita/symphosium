# R09 external adapter port, iteration 1

Base `23b3f7f1a14eca8ed27b3812d9db10437149fb98`. This checkpoint implements an in-memory tracker and an initial GitHub read adapter. It is **not** R09 acceptance.

## Implemented

- Memory `Tracker` snapshots match the reference's case-insensitive state lookup and ID refresh; replacement supports offline reconciliation tests.
- GitHub `Tracker` makes host-side authenticated repository issue reads over Reqwest, pages candidate lists at 100, filters requested states after normalization, safely drops malformed candidates, errors on malformed requested ID refresh records, omits 404 results, deduplicates IDs and labels, preserves GitHub `GH-number` dispatch IDs/native refs, excludes pull requests from dispatch, and parses optional RFC 3339 timestamps. Empty inputs do not make requests.
- GitHub's adapter constructor requires repository and token; authorization stays in the host adapter. HTTPS is expected outside explicit loopback HTTP for offline tests. Transport errors carry no token or response body.
- Local TCP fake-server integration tests exercise actual auth headers, scoped paths, paging, normalization, 404 handling, malformed payload and empty-input behavior. The in-memory adapter has its own state/refresh test.

## Remaining R09 criteria

- Linear, Jira, Asana and GitLab reads, with per-provider scope, paging, ID refresh, normalization and error contracts.
- GitHub reference parity still needs config defaults/`$VAR` resolution, 403/429 and pagination failure tests, redirect handling, URL/path encoding review, and full provider fixtures. The initial GitHub adapter should not be selected in production yet.
- Provider-native tool specs, host execution and authorization for all five providers, including GitHub. The existing `Tracker` trait exposes only reads; a reviewed tool boundary is needed before integrating with R12 sessions.
- Adapter factory binds the immutable R05 generation/claim lease; stale async results must be discarded by R12. No scheduler or live provider/model calls in this iteration.
- Complete R09 compact profiles and all five fake-server suites; Senior2 review and QA2 independent verdict on an Operations-integrated revision.

Dependencies changed only in `rust/Cargo.toml` and lockfile (`tokio`, `reqwest`, `time`). No DB files touched; Mid1's R06 work remains separate. Maintain the 20% senior review reserve for R06.
