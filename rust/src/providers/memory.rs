use crate::tracker::{Issue, Tracker, TrackerFuture};
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct MemoryTracker {
    issues: Arc<RwLock<Vec<Issue>>>,
}

impl MemoryTracker {
    pub fn new(issues: Vec<Issue>) -> Self {
        Self {
            issues: Arc::new(RwLock::new(issues)),
        }
    }
    pub fn replace(&self, issues: Vec<Issue>) {
        *self.issues.write().expect("memory lock poisoned") = issues;
    }
}

impl Tracker for MemoryTracker {
    fn fetch_issues_by_states<'a>(&'a self, states: &'a [String]) -> TrackerFuture<'a> {
        Box::pin(async move {
            if states.is_empty() {
                return Ok(vec![]);
            }
            let wanted: Vec<String> = states.iter().map(|s| s.trim().to_lowercase()).collect();
            Ok(self
                .issues
                .read()
                .expect("memory lock poisoned")
                .iter()
                .filter(|issue| wanted.contains(&issue.state.trim().to_lowercase()))
                .cloned()
                .collect())
        })
    }
    fn fetch_issues_by_ids<'a>(&'a self, ids: &'a [String]) -> TrackerFuture<'a> {
        Box::pin(async move {
            if ids.is_empty() {
                return Ok(vec![]);
            }
            Ok(self
                .issues
                .read()
                .expect("memory lock poisoned")
                .iter()
                .filter(|issue| ids.contains(&issue.id))
                .cloned()
                .collect())
        })
    }
}
