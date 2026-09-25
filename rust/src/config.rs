//! Validated settings derived from WORKFLOW.md. Secret values stay host-side.
use crate::workflow::Workflow;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};

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
    pub polling_interval_ms: u64,
    pub workspace_root: PathBuf,
    pub codex_command: String,
    pub max_concurrent_agents: u32,
    pub max_turns: u32,
    pub max_retry_backoff_ms: u64,
    pub prompt_template: String,
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
}
impl Default for RawAgent {
    fn default() -> Self {
        Self {
            max_concurrent_agents: 10,
            max_turns: 20,
            max_retry_backoff_ms: 300_000,
        }
    }
}
#[derive(Default, Deserialize)]
#[serde(default)]
struct RawCodex {
    command: Option<String>,
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
        let (active_states, terminal_states) = if kind == TrackerKind::Linear {
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
        let required_labels = raw
            .tracker
            .required_labels
            .into_iter()
            .map(|s| s.trim().to_lowercase())
            .collect();
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
            polling_interval_ms: raw.polling.interval_ms,
            workspace_root,
            codex_command,
            max_concurrent_agents: raw.agent.max_concurrent_agents,
            max_turns: raw.agent.max_turns,
            max_retry_backoff_ms: raw.agent.max_retry_backoff_ms,
            prompt_template: workflow.prompt_template.clone(),
        })
    }

    pub fn scope(&self) -> Scope {
        let value = |key: &str| {
            self.tracker
                .provider
                .get(key)
                .and_then(Value::as_str)
                .map(str::to_owned)
        };
        Scope {
            kind: self.tracker.kind.as_str(),
            organization: value("organization_id"),
            project: value("project_id").or_else(|| value("project_slug")),
            assignee: value("assignee").or_else(|| value("assignee_id")),
        }
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
}
