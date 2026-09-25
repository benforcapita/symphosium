//! One lock serializes generation publication and claim admission.
use crate::{
    config::{ConfigError, Settings},
    prompt::{self, PromptError},
    workflow::{self, WorkflowError},
};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

pub struct Snapshot {
    pub generation: u64,
    pub settings: Settings,
    pub scope: crate::config::Scope,
    source: String,
}

struct State {
    current: Arc<Snapshot>,
    active_claims: usize,
}

pub struct GenerationManager {
    path: PathBuf,
    state: Arc<Mutex<State>>,
}

pub struct ClaimLease {
    pub snapshot: Arc<Snapshot>,
    state: Arc<Mutex<State>>,
}

impl Drop for ClaimLease {
    fn drop(&mut self) {
        self.state
            .lock()
            .expect("generation mutex poisoned")
            .active_claims -= 1;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReloadError {
    #[error(transparent)]
    Workflow(#[from] WorkflowError),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Prompt(#[from] PromptError),
    #[error("tracker scope cannot change while a claim is active")]
    ScopeBusy,
}

impl GenerationManager {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ReloadError> {
        Self::new_with_env(path, |name| std::env::var(name).ok())
    }

    pub fn new_with_env(
        path: impl AsRef<Path>,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, ReloadError> {
        let path = path.as_ref().to_path_buf();
        let workflow = workflow::load(&path)?;
        let settings = Settings::from_workflow_with_env(&workflow, &path, env)?;
        prompt::validate(&settings.prompt_template)?;
        let current = Arc::new(Snapshot {
            generation: 1,
            scope: settings.scope(),
            settings,
            source: workflow.source,
        });
        Ok(Self {
            path,
            state: Arc::new(Mutex::new(State {
                current,
                active_claims: 0,
            })),
        })
    }

    pub fn current(&self) -> Arc<Snapshot> {
        self.state
            .lock()
            .expect("generation mutex poisoned")
            .current
            .clone()
    }

    /// The returned snapshot remains pinned for the whole claim, including retry or blocked states.
    pub fn admit_claim(&self) -> ClaimLease {
        let mut state = self.state.lock().expect("generation mutex poisoned");
        state.active_claims += 1;
        ClaimLease {
            snapshot: state.current.clone(),
            state: self.state.clone(),
        }
    }

    pub fn reload(&self) -> Result<bool, ReloadError> {
        self.reload_with_env(|name| std::env::var(name).ok())
    }

    pub fn reload_with_env(
        &self,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<bool, ReloadError> {
        let workflow = workflow::load(&self.path)?;
        let settings = Settings::from_workflow_with_env(&workflow, &self.path, env)?;
        prompt::validate(&settings.prompt_template)?;
        let mut state = self.state.lock().expect("generation mutex poisoned");
        let scope = settings.scope();
        if workflow.source == state.current.source
            && scope == state.current.scope
            && settings.tracker.resolved_api_key == state.current.settings.tracker.resolved_api_key
            && settings.workspace_root == state.current.settings.workspace_root
        {
            return Ok(false);
        }
        if state.active_claims > 0 && scope != state.current.scope {
            return Err(ReloadError::ScopeBusy);
        }
        state.current = Arc::new(Snapshot {
            generation: state.current.generation + 1,
            scope,
            settings,
            source: workflow.source,
        });
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    fn source(project: &str, key: &str) -> String {
        format!(
            "---\ntracker:\n  kind: linear\n  provider:\n    project_slug: {project}\n    api_key: {key}\n---\n{{{{ issue.title }}}}"
        )
    }
    #[test]
    fn atomic_reload_and_pinned_claim() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("WORKFLOW.md");
        fs::write(&path, source("alpha", "old")).unwrap();
        let manager = GenerationManager::new(&path).unwrap();
        let claim = manager.admit_claim();
        fs::write(&path, source("alpha", "new")).unwrap();
        assert!(manager.reload().unwrap());
        assert_eq!(claim.snapshot.generation, 1);
        assert_eq!(claim.snapshot.settings.tracker.provider["api_key"], "old");
        assert_eq!(
            manager.current().settings.tracker.provider["api_key"],
            "new"
        );
        fs::write(&path, source("beta", "new")).unwrap();
        assert!(matches!(manager.reload(), Err(ReloadError::ScopeBusy)));
        assert_eq!(manager.current().generation, 2);
        drop(claim);
        assert!(manager.reload().unwrap());
        assert_eq!(manager.current().generation, 3);
    }
    #[test]
    fn invalid_reload_keeps_last_good() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("WORKFLOW.md");
        fs::write(&path, source("alpha", "old")).unwrap();
        let manager = GenerationManager::new(&path).unwrap();
        fs::write(&path, "---\ntracker: [\n---\n").unwrap();
        assert!(manager.reload().is_err());
        assert_eq!(manager.current().generation, 1);
    }
    #[test]
    fn env_scope_change_is_fenced_even_when_file_is_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("WORKFLOW.md");
        fs::write(&path, "---\ntracker:\n  kind: linear\n  provider:\n    project_slug: $PROJECT\n    assignee: $ASSIGN\n    api_key: token\n---\n").unwrap();
        let environment = |project: &str, assignee: &str| {
            let project = project.to_owned();
            let assignee = assignee.to_owned();
            move |name: &str| match name {
                "PROJECT" => Some(project.clone()),
                "ASSIGN" => Some(assignee.clone()),
                _ => None,
            }
        };
        let manager = GenerationManager::new_with_env(&path, environment("p1", "a1")).unwrap();
        let claim = manager.admit_claim();
        assert!(matches!(
            manager.reload_with_env(environment("p1", "a2")),
            Err(ReloadError::ScopeBusy)
        ));
        assert!(matches!(
            manager.reload_with_env(environment("p2", "a1")),
            Err(ReloadError::ScopeBusy)
        ));
        assert_eq!(manager.current().scope.assignee.as_deref(), Some("a1"));
        drop(claim);
        assert!(manager.reload_with_env(environment("p1", "a2")).unwrap());
        assert_eq!(manager.current().scope.assignee.as_deref(), Some("a2"));
    }
}
