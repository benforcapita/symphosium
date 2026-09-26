# R09 GitHub security repair, cycle 1

Base `67a9a45c2f4c76107fd467474236a15aff615a20` (initial adapter code `c2166c6d`). This is a scoped repair of Senior2 findings S2-R09-001/002/003; full R09 remains open.

- Parse the effective API URL and permit plain HTTP only when the parsed host is exact IPv4/IPv6 loopback. Reject userinfo, query and fragment on the base URL. The credential never reaches a destination selected through a misleading raw prefix.
- Disable every redirect at the HTTP client. A 3xx becomes a transport/status failure; no redirected payload is accepted or bearer forwarded. This conservative policy also rejects same-origin redirects and any HTTPS-to-HTTP downgrade.
- Use URL path-segment construction for repository owner/name and ID refresh. Reserved `?`, `#`, `%`, and encoded-slash text remains in the segment. Literal `.` and `..` names and extra separators are rejected.

`rust/tests/github_security.rs` uses synthetic tokens and loopback listeners for origin/userinfo, encoded segment, and redirect cases. The earlier GitHub and memory tests remain in the suite. This patch does not select the adapter in production, implement the other four providers/native tools, or claim QA acceptance.
