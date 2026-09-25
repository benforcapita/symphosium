---
tracker:
  kind: linear
  provider:
    project_slug: example-project
    api_key: $LINEAR_API_KEY
  active_states: [Todo, In Progress]
  terminal_states: [Done, Cancelled]
polling:
  interval_ms: 30000
workspace:
  root: ./workspaces
---
Ticket {{ issue.identifier }}: {{ issue.title }}
