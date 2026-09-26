//! Validated settings derived from WORKFLOW.md. Secret values stay host-side.
use crate::workflow::Workflow;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Clone, PartialEq, Eq)]
pub enum TrackerKind {
    Linear,
    Github,
    Jira,
    Asana,
    Gitlab,
    Builtin,
    Memory,
}

impl TrackerKind {
    pub fn parse(value: &str) -> Result<Self, ConfigError> {
        match value {
            "linear" => Ok(Self::Linear),
            "github" => Ok(Self::Github),
            "jira" => Ok(Self::Jira),
            "asana" => Ok(Self::Asana),
            "gitlab" => Ok(Self::Gitlab),
            "builtin" => Ok(Self::Builtin),
            "memory" => Ok(Self::Memory),
            _ => Err(ConfigError::UnsupportedTracker),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Github => "github",
            Self::Jira => "jira",
            Self::Asana => "asana",
            Self::Gitlab => "gitlab",
            Self::Builtin => "builtin",
            Self::Memory => "memory",
        }
    }
}

#[derive(Clone)]
pub struct TrackerSettings {
    pub kind: TrackerKind,
    pub provider: Map<String, Value>,
    pub resolved_api_key: Option<String>,
    pub resolved_assignee: Option<String>,
    pub required_labels: Vec<String>,
    pub active_states: Vec<String>,
    pub terminal_states: Vec<String>,
    pub secret_environment_names: Vec<String>,
}

#[derive(Clone)]
pub struct Settings {
    pub tracker: TrackerSettings,
    resolved_scope: Scope,
    pub polling_interval_ms: u64,
    pub workspace_root: PathBuf,
    pub codex_command: String,
    pub max_concurrent_agents: u32,
    pub max_turns: u32,
    pub max_retry_backoff_ms: u64,
    pub max_concurrent_agents_by_state: BTreeMap<String, u32>,
    pub worker: WorkerSettings,
    pub hooks: HooksSettings,
    pub observability: ObservabilitySettings,
    pub server: ServerSettings,
    pub codex: CodexSettings,
    pub prompt_template: String,
}

#[derive(Clone)]
pub struct WorkerSettings {
    pub ssh_hosts: Vec<String>,
    pub max_concurrent_agents_per_host: Option<u32>,
}
#[derive(Clone)]
pub struct HooksSettings {
    pub after_create: Option<String>,
    pub before_run: Option<String>,
    pub after_run: Option<String>,
    pub before_remove: Option<String>,
    pub timeout_ms: u64,
}
#[derive(Clone)]
pub struct ObservabilitySettings {
    pub dashboard_enabled: bool,
    pub refresh_ms: u64,
    pub render_interval_ms: u64,
}
#[derive(Clone)]
pub struct ServerSettings {
    pub port: Option<u16>,
    pub host: String,
}
#[derive(Clone)]
pub struct CodexSettings {
    pub command: String,
    pub approval_policy: Value,
    pub thread_sandbox: String,
    pub turn_sandbox_policy: Option<Map<String, Value>>,
    pub turn_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub stall_timeout_ms: u64,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Scope {
    pub kind: &'static str,
    pub organization: Option<String>,
    pub project: Option<String>,
    pub assignee: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing_tracker_kind")]
    MissingTrackerKind,
    #[error("unsupported_tracker_kind")]
    UnsupportedTracker,
    #[error("invalid_workflow_config: {0}")]
    Invalid(&'static str),
    #[error("missing_linear_project_slug")]
    MissingLinearProjectSlug,
    #[error("missing_linear_api_token")]
    MissingLinearApiToken,
    #[error("database-backed builtin tracker is unavailable in R05")]
    BuiltinUnavailable,
    #[error("invalid_workflow_config: {0}")]
    Decode(#[from] serde_yaml::Error),
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct RawSettings {
    tracker: RawTracker,
    polling: RawPolling,
    workspace: RawWorkspace,
    agent: RawAgent,
    codex: RawCodex,
    worker: RawWorker,
    hooks: RawHooks,
    observability: RawObservability,
    server: RawServer,
}
#[derive(Default, Deserialize)]
#[serde(default)]
struct RawTracker {
    kind: Option<String>,
    provider: Map<String, Value>,
    endpoint: Option<String>,
    api_key: Option<String>,
    project_slug: Option<String>,
    assignee: Option<String>,
    required_labels: Vec<String>,
    active_states: Option<Vec<String>>,
    terminal_states: Option<Vec<String>>,
}
#[derive(Deserialize)]
#[serde(default)]
struct RawPolling {
    interval_ms: u64,
}
impl Default for RawPolling {
    fn default() -> Self {
        Self {
            interval_ms: 30_000,
        }
    }
}
#[derive(Default, Deserialize)]
#[serde(default)]
struct RawWorkspace {
    root: Option<String>,
}
#[derive(Deserialize)]
#[serde(default)]
struct RawAgent {
    max_concurrent_agents: u32,
    max_turns: u32,
    max_retry_backoff_ms: u64,
    max_concurrent_agents_by_state: serde_yaml::Mapping,
}
impl Default for RawAgent {
    fn default() -> Self {
        Self {
            max_concurrent_agents: 10,
            max_turns: 20,
            max_retry_backoff_ms: 300_000,
            max_concurrent_agents_by_state: serde_yaml::Mapping::new(),
        }
    }
}
#[derive(Default, Deserialize)]
#[serde(default)]
struct RawCodex {
    command: Option<String>,
    approval_policy: Option<Value>,
    thread_sandbox: Option<String>,
    turn_sandbox_policy: Option<Map<String, Value>>,
    turn_timeout_ms: Option<u64>,
    read_timeout_ms: Option<u64>,
    stall_timeout_ms: Option<u64>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct RawWorker {
    ssh_hosts: Vec<String>,
    max_concurrent_agents_per_host: Option<u32>,
}
#[derive(Deserialize)]
#[serde(default)]
struct RawHooks {
    after_create: Option<String>,
    before_run: Option<String>,
    after_run: Option<String>,
    before_remove: Option<String>,
    timeout_ms: u64,
}
impl Default for RawHooks {
    fn default() -> Self {
        Self {
            after_create: None,
            before_run: None,
            after_run: None,
            before_remove: None,
            timeout_ms: 60_000,
        }
    }
}
#[derive(Deserialize)]
#[serde(default)]
struct RawObservability {
    dashboard_enabled: bool,
    refresh_ms: u64,
    render_interval_ms: u64,
}
impl Default for RawObservability {
    fn default() -> Self {
        Self {
            dashboard_enabled: true,
            refresh_ms: 1_000,
            render_interval_ms: 16,
        }
    }
}
#[derive(Deserialize)]
#[serde(default)]
struct RawServer {
    port: Option<u16>,
    host: String,
}
impl Default for RawServer {
    fn default() -> Self {
        Self {
            port: None,
            host: "127.0.0.1".into(),
        }
    }
}

impl Settings {
    pub fn from_workflow(workflow: &Workflow, path: &Path) -> Result<Self, ConfigError> {
        Self::from_workflow_with_env(workflow, path, |name| std::env::var(name).ok())
    }

    pub fn from_workflow_with_env(
        workflow: &Workflow,
        path: &Path,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, ConfigError> {
        let raw: RawSettings =
            serde_yaml::from_value(serde_yaml::Value::Mapping(workflow.config.clone()))?;
        let kind = TrackerKind::parse(
            raw.tracker
                .kind
                .as_deref()
                .ok_or(ConfigError::MissingTrackerKind)?,
        )?;
        if kind == TrackerKind::Builtin {
            return Err(ConfigError::BuiltinUnavailable);
        }
        let mut provider = raw.tracker.provider;
        if kind == TrackerKind::Linear {
            put_default(
                &mut provider,
                "endpoint",
                raw.tracker
                    .endpoint
                    .or(Some("https://api.linear.app/graphql".into())),
            );
            put_default(&mut provider, "api_key", raw.tracker.api_key);
            put_default(&mut provider, "project_slug", raw.tracker.project_slug);
            put_default(&mut provider, "assignee", raw.tracker.assignee);
            if provider
                .get("project_slug")
                .and_then(Value::as_str)
                .is_none_or(|s| s.trim().is_empty())
            {
                return Err(ConfigError::MissingLinearProjectSlug);
            }
        }
        let mut secret_environment_names = if kind == TrackerKind::Linear {
            vec!["LINEAR_API_KEY".to_owned()]
        } else {
            vec![]
        };
        for (key, value) in &provider {
            if let Some(name) = value.as_str().and_then(env_name)
                && (key.contains("key") || key.contains("token") || key.contains("secret"))
                && !secret_environment_names.contains(&name.to_owned())
            {
                secret_environment_names.push(name.to_owned());
            }
        }
        let resolve = |value: Option<&Value>, fallback: &str| -> Option<String> {
            match value.and_then(Value::as_str) {
                Some(raw) if env_name(raw).is_some() => env(env_name(raw).unwrap()),
                Some(raw) if !raw.is_empty() => Some(raw.to_owned()),
                _ => env(fallback),
            }
        };
        let resolved_api_key = if kind == TrackerKind::Linear {
            resolve(provider.get("api_key"), "LINEAR_API_KEY")
        } else {
            None
        };
        if kind == TrackerKind::Linear && resolved_api_key.as_deref().is_none_or(str::is_empty) {
            return Err(ConfigError::MissingLinearApiToken);
        }
        let resolved_assignee = if kind == TrackerKind::Linear {
            resolve(provider.get("assignee"), "LINEAR_ASSIGNEE")
        } else {
            None
        };
        let resolve_selector = |key: &str| -> Result<Option<String>, ConfigError> {
            match provider.get(key) {
                None | Some(Value::Null) => Ok(None),
                Some(Value::String(raw)) => {
                    Ok(env_name(raw).map_or_else(|| Some(raw.clone()), &env))
                }
                _ => Err(ConfigError::Invalid("tracker.provider scope selector")),
            }
        };
        let organization = resolve_selector("organization_id")?;
        let project = resolve_selector("project_id")?.or(resolve_selector("project_slug")?);
        let assignee = if kind == TrackerKind::Linear {
            resolved_assignee.clone()
        } else {
            resolve_selector("assignee")?.or(resolve_selector("assignee_id")?)
        };
        if kind == TrackerKind::Linear && project.as_deref().is_none_or(str::is_empty) {
            return Err(ConfigError::MissingLinearProjectSlug);
        }
        let resolved_scope = Scope {
            kind: kind.as_str(),
            organization,
            project,
            assignee,
        };
        let (active_states, terminal_states) =
            if matches!(kind, TrackerKind::Linear | TrackerKind::Memory) {
                (
                    raw.tracker
                        .active_states
                        .unwrap_or_else(|| vec!["Todo".into(), "In Progress".into()]),
                    raw.tracker.terminal_states.unwrap_or_else(|| {
                        vec![
                            "Closed".into(),
                            "Cancelled".into(),
                            "Canceled".into(),
                            "Duplicate".into(),
                            "Done".into(),
                        ]
                    }),
                )
            } else {
                (
                    raw.tracker.active_states.unwrap_or_default(),
                    raw.tracker.terminal_states.unwrap_or_default(),
                )
            };
        if kind != TrackerKind::Memory && (active_states.is_empty() || terminal_states.is_empty()) {
            return Err(ConfigError::Invalid(
                "tracker.active_states/terminal_states",
            ));
        }
        if raw.polling.interval_ms == 0 {
            return Err(ConfigError::Invalid("polling.interval_ms"));
        }
        if raw.agent.max_concurrent_agents == 0
            || raw.agent.max_turns == 0
            || raw.agent.max_retry_backoff_ms == 0
        {
            return Err(ConfigError::Invalid("agent limits"));
        }
        let mut state_limits = BTreeMap::new();
        // SPEC §5.3.5: malformed entries do not invalidate otherwise usable settings.
        // Source order gives a deterministic last-valid-value result for normalized duplicates.
        for (state, limit) in raw.agent.max_concurrent_agents_by_state {
            let Some(state) = state.as_str() else {
                continue;
            };
            let key = state.trim().to_lowercase();
            let Some(limit) = limit.as_u64().and_then(|n| u32::try_from(n).ok()) else {
                continue;
            };
            if !key.is_empty() && limit > 0 {
                state_limits.insert(key, limit);
            }
        }
        if raw.worker.max_concurrent_agents_per_host == Some(0) {
            return Err(ConfigError::Invalid(
                "worker.max_concurrent_agents_per_host",
            ));
        }
        if raw.hooks.timeout_ms == 0 {
            return Err(ConfigError::Invalid("hooks.timeout_ms"));
        }
        if raw.observability.refresh_ms == 0 || raw.observability.render_interval_ms == 0 {
            return Err(ConfigError::Invalid("observability intervals"));
        }
        if raw.codex.turn_timeout_ms == Some(0) || raw.codex.read_timeout_ms == Some(0) {
            return Err(ConfigError::Invalid("codex timeouts"));
        }
        let codex_command = raw
            .codex
            .command
            .unwrap_or_else(|| "codex app-server".into());
        if codex_command.trim().is_empty() {
            return Err(ConfigError::Invalid("codex.command"));
        }
        let root = raw.workspace.root.unwrap_or_else(|| {
            std::env::temp_dir()
                .join("symphony_workspaces")
                .to_string_lossy()
                .into_owned()
        });
        let root = env_name(&root).and_then(env).unwrap_or(root);
        let workspace_root = if let Some(suffix) = root.strip_prefix("~/") {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(std::env::temp_dir)
                .join(suffix)
        } else {
            PathBuf::from(root)
        };
        let workspace_root = if workspace_root.is_absolute() {
            workspace_root
        } else {
            path.parent()
                .unwrap_or_else(|| Path::new("."))
                .join(workspace_root)
        };
        let mut required_labels = Vec::new();
        for label in raw
            .tracker
            .required_labels
            .into_iter()
            .map(|s| s.trim().to_lowercase())
        {
            if !required_labels.contains(&label) {
                required_labels.push(label);
            }
        }
        let codex = CodexSettings {
            command: codex_command.clone(),
            approval_policy: raw.codex.approval_policy.unwrap_or_else(|| serde_json::json!({"reject": {"sandbox_approval": true, "rules": true, "mcp_elicitations": true}})),
            thread_sandbox: raw.codex.thread_sandbox.unwrap_or_else(|| "workspace-write".into()),
            turn_sandbox_policy: raw.codex.turn_sandbox_policy,
            turn_timeout_ms: raw.codex.turn_timeout_ms.unwrap_or(3_600_000),
            read_timeout_ms: raw.codex.read_timeout_ms.unwrap_or(5_000),
            stall_timeout_ms: raw.codex.stall_timeout_ms.unwrap_or(300_000),
        };
        if !codex.approval_policy.is_string() && !codex.approval_policy.is_object() {
            return Err(ConfigError::Invalid("codex.approval_policy"));
        }
        Ok(Self {
            tracker: TrackerSettings {
                kind,
                provider,
                resolved_api_key,
                resolved_assignee,
                required_labels,
                active_states,
                terminal_states,
                secret_environment_names,
            },
            resolved_scope,
            polling_interval_ms: raw.polling.interval_ms,
            workspace_root,
            codex_command,
            max_concurrent_agents: raw.agent.max_concurrent_agents,
            max_turns: raw.agent.max_turns,
            max_retry_backoff_ms: raw.agent.max_retry_backoff_ms,
            max_concurrent_agents_by_state: state_limits,
            worker: WorkerSettings {
                ssh_hosts: raw.worker.ssh_hosts,
                max_concurrent_agents_per_host: raw.worker.max_concurrent_agents_per_host,
            },
            hooks: HooksSettings {
                after_create: raw.hooks.after_create,
                before_run: raw.hooks.before_run,
                after_run: raw.hooks.after_run,
                before_remove: raw.hooks.before_remove,
                timeout_ms: raw.hooks.timeout_ms,
            },
            observability: ObservabilitySettings {
                dashboard_enabled: raw.observability.dashboard_enabled,
                refresh_ms: raw.observability.refresh_ms,
                render_interval_ms: raw.observability.render_interval_ms,
            },
            server: ServerSettings {
                port: raw.server.port,
                host: raw.server.host,
            },
            codex,
            prompt_template: workflow.prompt_template.clone(),
        })
    }

    pub fn scope(&self) -> Scope {
        self.resolved_scope.clone()
    }
}

fn put_default(map: &mut Map<String, Value>, key: &str, value: Option<String>) {
    map.entry(key.to_owned())
        .or_insert_with(|| value.map(Value::String).unwrap_or(Value::Null));
}
fn env_name(value: &str) -> Option<&str> {
    let name = value.strip_prefix('$')?;
    if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Some(name)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow;
    #[test]
    fn linear_aliases_preserve_provider_and_resolve_secret_separately() {
        let source = "---\ntracker:\n  kind: linear\n  endpoint: https://alias.example/graphql\n  api_key: $TEST_LINEAR_KEY\n  project_slug: alias-project\n  provider:\n    endpoint: https://provider.example/graphql\n    project_slug: provider-project\n    extra: {team: platform}\n---\n{{ issue.title }}";
        let workflow = workflow::parse(source).unwrap();
        let settings =
            Settings::from_workflow_with_env(&workflow, Path::new("/tmp/WORKFLOW.md"), |name| {
                match name {
                    "TEST_LINEAR_KEY" => Some("synthetic-token".into()),
                    _ => None,
                }
            })
            .unwrap();
        assert_eq!(
            settings.tracker.provider["endpoint"],
            "https://provider.example/graphql"
        );
        assert_eq!(
            settings.tracker.provider["project_slug"],
            "provider-project"
        );
        assert_eq!(settings.tracker.provider["api_key"], "$TEST_LINEAR_KEY");
        assert_eq!(settings.tracker.provider["extra"]["team"], "platform");
        assert_eq!(
            settings.tracker.resolved_api_key.as_deref(),
            Some("synthetic-token")
        );
        assert_eq!(
            settings.tracker.secret_environment_names,
            ["LINEAR_API_KEY", "TEST_LINEAR_KEY"]
        );
        assert_eq!(
            settings.scope().project.as_deref(),
            Some("provider-project")
        );
        assert_eq!(settings.polling_interval_ms, 30_000);
    }
    #[test]
    fn external_memory_starts_without_database_or_linear_key() {
        let workflow =
            workflow::parse("---\ntracker:\n  kind: memory\n---\n{{ issue.identifier }}").unwrap();
        let settings =
            Settings::from_workflow_with_env(&workflow, Path::new("WORKFLOW.md"), |_| None)
                .unwrap();
        assert_eq!(settings.tracker.kind.as_str(), "memory");
        assert!(settings.tracker.resolved_api_key.is_none());
    }
    #[test]
    fn rejects_missing_kind_project_and_bad_limits() {
        for source in [
            "---\ntracker: {}\n---\n",
            "---\ntracker: {kind: linear}\n---\n",
            "---\ntracker: {kind: linear, project_slug: p}\npolling: {interval_ms: 0}\n---\n",
        ] {
            let workflow = workflow::parse(source).unwrap();
            assert!(
                Settings::from_workflow_with_env(&workflow, Path::new("WORKFLOW.md"), |_| None)
                    .is_err()
            );
        }
    }
    #[test]
    fn retains_existing_runtime_policy_with_reference_defaults() {
        let source = "---\ntracker:\n  kind: memory\n  required_labels: [' Symphony ', SYMPHONY, '', JavaScript]\nworker:\n  ssh_hosts: [worker-01]\n  max_concurrent_agents_per_host: 2\nagent:\n  max_concurrent_agents_by_state: {Todo: 1, 'In Progress': 4}\ncodex:\n  approval_policy: {reject: {sandbox_approval: true}}\n  thread_sandbox: read-only\n  turn_sandbox_policy: {type: readOnly, networkAccess: false}\n  turn_timeout_ms: 1000\n  read_timeout_ms: 500\n  stall_timeout_ms: 0\nhooks:\n  before_run: echo synthetic\n  timeout_ms: 1200\nobservability:\n  dashboard_enabled: false\n  refresh_ms: 2000\n  render_interval_ms: 20\nserver:\n  port: 4321\n  host: 127.0.0.2\n---\n";
        let wf = workflow::parse(source).unwrap();
        let settings =
            Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| None).unwrap();
        assert_eq!(
            settings.tracker.required_labels,
            ["symphony", "", "javascript"]
        );
        assert_eq!(settings.max_concurrent_agents_by_state["todo"], 1);
        assert_eq!(settings.max_concurrent_agents_by_state["in progress"], 4);
        assert_eq!(settings.worker.ssh_hosts, ["worker-01"]);
        assert_eq!(settings.worker.max_concurrent_agents_per_host, Some(2));
        assert_eq!(settings.hooks.before_run.as_deref(), Some("echo synthetic"));
        assert_eq!(settings.hooks.timeout_ms, 1200);
        assert!(!settings.observability.dashboard_enabled);
        assert_eq!(settings.observability.refresh_ms, 2000);
        assert_eq!(settings.server.port, Some(4321));
        assert_eq!(
            settings.codex.approval_policy["reject"]["sandbox_approval"],
            true
        );
        assert_eq!(settings.codex.thread_sandbox, "read-only");
        assert_eq!(
            settings.codex.turn_sandbox_policy.as_ref().unwrap()["type"],
            "readOnly"
        );
        assert_eq!(settings.codex.stall_timeout_ms, 0);
        assert_eq!(settings.codex.read_timeout_ms, 500);
        assert_eq!(settings.codex.turn_timeout_ms, 1000);
    }
    #[test]
    fn rejects_invalid_policy_values_without_echoing_secret() {
        let secret = "synthetic-never-log-credential";
        for tail in [
            "worker: {max_concurrent_agents_per_host: 0}",
            "codex: {turn_sandbox_policy: bad}",
            "hooks: {timeout_ms: 0}",
            "observability: {refresh_ms: 0}",
            "codex: {read_timeout_ms: 0}",
        ] {
            let source = format!(
                "---\ntracker: {{kind: linear, project_slug: p, api_key: {secret}}}\n{tail}\n---\n"
            );
            let wf = workflow::parse(&source).unwrap();
            let error =
                match Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| None) {
                    Ok(_) => panic!("invalid policy accepted"),
                    Err(error) => error,
                };
            assert!(!error.to_string().contains(secret));
        }
        let wf = workflow::parse(
            "---\ntracker: {kind: linear, project_slug: p, api_key: '$MISSING_TEST_SECRET'}\n---\n",
        )
        .unwrap();
        let error = match Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| None)
        {
            Ok(_) => panic!("missing secret accepted"),
            Err(error) => error,
        };
        assert!(matches!(error, ConfigError::MissingLinearApiToken));
        assert!(!error.to_string().contains("MISSING_TEST_SECRET"));
    }
}
