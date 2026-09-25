# Symphony Linear-Style Project Manager — High-Level Design

Status: Draft for CTO decomposition  
Upstream: `openai/symphony` at `be10a1b`  
Working repository: `symphony-linear`

## Problem

Symphony orchestrates coding agents from external issue trackers. The fork should also provide a
self-hosted, Linear-style project-management experience so a team can create, prioritize, assign,
and track work in the same product that dispatches it to Symphony.

## Users and outcomes

- Product/engineering leads organize projects and see delivery state without a separate tracker.
- Developers and agents share one ticket record, including dependencies, comments, and run status.
- Operators can run the product in a trusted single-organization environment without a Linear API
  subscription or token.
- Existing Symphony users may continue using supported external tracker adapters.

## MVP scope

1. A built-in tracker adapter using the same normalized `Issue` contract as existing adapters.
2. Persistent storage for organizations (one configured organization in MVP), teams, projects,
   workflow states, users, tickets, labels, assignments, dependencies, and comments/activity.
3. Ticket identifiers, titles, Markdown descriptions, priority, assignee, labels, state, project,
   blockers, timestamps, and stable URLs.
4. Web UI with project list, backlog/list view, Kanban board, ticket detail/editing, filters, and
   basic keyboard-friendly navigation.
5. Symphony run state visible on tickets, including queued/running/blocked/retrying/completed and
   links or metadata for proof of work.
6. CRUD/API boundaries that let agents update ticket state and add comments without receiving raw
   database credentials.
7. Migration/setup documentation, seed workflow states, local development instructions, and a
   production-oriented deployment path.
8. Automated unit, integration, adapter-contract, and end-to-end acceptance coverage.

## Non-goals for MVP

- SaaS multi-tenancy, billing, public signup, or enterprise SSO.
- Native mobile/desktop applications.
- Full Linear feature parity, roadmaps, cycles, estimates, custom views, or advanced analytics.
- Replacing GitHub pull requests or CI.
- Removing Linear, GitHub Issues, Jira, Asana, or GitLab adapters.
- Distributed orchestration or hardening Symphony beyond the selected deployment threat model.

## System boundaries

- Extend the existing Elixir/Phoenix reference implementation and LiveView surface.
- Keep orchestration tracker-agnostic; the built-in tracker is another adapter, not special-case
  scheduler logic.
- Use the application database as the authoritative ticket store and Phoenix boundaries for all
  user/agent mutations.
- Keep repository-owned workflow policy in `WORKFLOW.md`.
- Preserve upstream license and notices; use original product naming/visual identity for this fork.

## Constraints

- Symphony is an engineering preview intended for trusted environments.
- Secrets must remain host-side and must not be injected into agent workspaces or prompts.
- Existing tracker configurations must continue to work.
- Schema changes must be migratable and reversible where practical.
- Accessibility, responsive layout, and safe HTML/Markdown rendering are release requirements.
- No paid service, purchase, or new temporary hire is authorized for this work.

## Acceptance measures

1. A user can create a project and ticket, move it through configured workflow states from list,
   board, and detail views, and see the change persist after restart.
2. Symphony dispatches an eligible built-in ticket, respects blockers and concurrency, reconciles
   state changes, and stops/cleans terminal work exactly as with external adapters.
3. An agent can add a comment and transition its ticket through a scoped application tool/API
   without access to database credentials.
4. External tracker adapter contract and regression suites remain green.
5. Unauthorized mutations, unsafe Markdown/HTML, secret leakage, and cross-project identifier
   collisions have explicit tests and zero critical/high/medium defects at release.
6. Setup documentation enables a clean local install and a documented production deployment.
7. The integrated release revision passes formatting, compilation, static analysis, unit,
   integration, and end-to-end checks.

## Delivery recommendation

Build incrementally: (1) domain/schema and adapter contract, (2) API and authorization, (3) list and
ticket detail, (4) board and run-state integration, (5) hardening, migration docs, and release.
Reuse the permanent delivery team. Do not invoke Hiring Bot unless CTO identifies a concrete skill,
cost, or schedule gap that the existing team cannot reasonably cover.

## Alternatives and tradeoffs

- **Use Linear unchanged:** fastest and lowest engineering risk, but retains an external dependency
  and does not deliver a first-party project-management product.
- **Build a separate tracker service:** strongest separation and future reuse, but adds deployment,
  authentication, API-versioning, and operational cost. Not recommended for MVP.
- **Embed the tracker in Phoenix (recommended):** maximizes reuse and provides one deployable unit;
  it couples the initial tracker lifecycle to Symphony, which can be revisited after product fit.

## Budget and governance

Budget is existing permanent-team capacity and open-source/local infrastructure only; monetary
spend is zero unless the user separately authorizes it. CEO owns this HLD and scope, CTO owns the
LLD and technical decisions, Project Manager owns delivery scheduling and routing, QA owns the
independent verdict, and Operations owns checkpoint/release writes.
