//! Normalized issue and asynchronous tracker boundary.
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blocker {
    pub id: String,
    pub identifier: String,
    pub state: String,
}

/// Stable dispatch identity may differ from a provider's native issue ID.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub native_ref: Option<serde_json::Value>,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub state: String,
    pub branch_name: Option<String>,
    pub url: Option<String>,
    pub assignee_id: Option<String>,
    pub blocked_by: Vec<Blocker>,
    pub labels: Vec<String>,
    pub dispatchable: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Issue {
    /// Required label matching trims and case-folds; a blank configured label fails closed.
    pub fn routable(&self, required_labels: &[String]) -> bool {
        self.dispatchable
            && required_labels.iter().all(|required| {
                let required = required.trim();
                !required.is_empty()
                    && self
                        .labels
                        .iter()
                        .any(|label| label.trim().eq_ignore_ascii_case(required))
            })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TrackerError {
    #[error("tracker_config: {0}")]
    Config(String),
    #[error("tracker_transport: {0}")]
    Transport(String),
    #[error("tracker_payload: {0}")]
    Payload(String),
}

pub type TrackerFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Vec<Issue>, TrackerError>> + Send + 'a>>;

/// Read results distinguish an empty successful result from a transport failure.
/// Implementations bind one immutable config generation per read/session.
pub trait Tracker: Send + Sync {
    fn fetch_issues_by_states<'a>(&'a self, states: &'a [String]) -> TrackerFuture<'a>;
    fn fetch_issues_by_ids<'a>(&'a self, ids: &'a [String]) -> TrackerFuture<'a>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn labels_are_all_required_and_normalized() {
        let issue = Issue {
            id: "1".into(),
            native_ref: None,
            identifier: "MT-1".into(),
            title: "Task".into(),
            description: None,
            priority: None,
            state: "Todo".into(),
            branch_name: None,
            url: None,
            assignee_id: None,
            blocked_by: vec![],
            labels: vec![" Symphony ".into(), "JavaScript".into()],
            dispatchable: true,
            created_at: None,
            updated_at: None,
        };
        assert!(issue.routable(&["symphony".into(), "javascript".into()]));
        assert!(!issue.routable(&["symphony".into(), "rust".into()]));
        assert!(!issue.routable(&[" ".into()]));
    }
}
