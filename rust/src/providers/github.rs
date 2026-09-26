//! GitHub repository issue reads. Token is held only by the host adapter.
use crate::{
    providers::{normalized_labels, required},
    tracker::{Issue, Tracker, TrackerError, TrackerFuture},
};
use serde_json::{Value, json};
use std::{collections::HashSet, time::Duration};

const PAGE_SIZE: usize = 100;
const MAX_PAGES: usize = 1000;

pub struct GithubTracker {
    client: reqwest::Client,
    base: reqwest::Url,
    repo: String,
    token: String,
}

impl GithubTracker {
    pub fn new(base: &str, repo: &str, token: &str) -> Result<Self, TrackerError> {
        let base = reqwest::Url::parse(base)
            .map_err(|_| TrackerError::Config("invalid_github_api_url".into()))?;
        let valid_scheme = base.scheme() == "https"
            || (base.scheme() == "http" && matches!(base.host_str(), Some("127.0.0.1" | "[::1]")));
        if !valid_scheme
            || !base.username().is_empty()
            || base.password().is_some()
            || base.fragment().is_some()
            || base.query().is_some()
            || base.host_str().is_none()
        {
            return Err(TrackerError::Config("invalid_github_api_url".into()));
        }
        if repo.split('/').count() != 2
            || repo.split('/').any(|part| {
                part.is_empty()
                    || part == "."
                    || part == ".."
                    || part.chars().any(char::is_whitespace)
            })
        {
            return Err(TrackerError::Config("invalid_github_repo".into()));
        }
        if token.trim().is_empty() {
            return Err(TrackerError::Config("missing_github_token".into()));
        }
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| TrackerError::Config("github_http_client".into()))?;
        Ok(Self {
            client,
            base,
            repo: repo.into(),
            token: token.into(),
        })
    }

    fn issue_path(&self) -> reqwest::Url {
        let mut url = self.base.clone();
        let mut segments = url.path_segments_mut().expect("validated hierarchical URL");
        segments.pop_if_empty();
        let mut parts = self.repo.split('/');
        segments
            .push("repos")
            .push(parts.next().unwrap())
            .push(parts.next().unwrap())
            .push("issues");
        drop(segments);
        url
    }

    async fn get(
        &self,
        url: reqwest::Url,
        allow_missing: bool,
    ) -> Result<Option<Value>, TrackerError> {
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "symphony")
            .send()
            .await
            .map_err(|_| TrackerError::Transport("github_api_request".into()))?;
        let status = response.status();
        if allow_missing && status == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            return Err(TrackerError::Transport(format!(
                "github_api_status: {}",
                status.as_u16()
            )));
        }
        response
            .json::<Value>()
            .await
            .map(Some)
            .map_err(|_| TrackerError::Payload("github_unknown_payload".into()))
    }

    async fn by_states(&self, states: &[String]) -> Result<Vec<Issue>, TrackerError> {
        let wanted: HashSet<String> = states.iter().map(|s| s.trim().to_lowercase()).collect();
        let state = match (wanted.contains("open"), wanted.contains("closed")) {
            (false, false) => return Ok(vec![]),
            (true, true) => "all",
            (true, false) => "open",
            (false, true) => "closed",
        };
        let mut output = Vec::new();
        let mut seen = HashSet::new();
        for page in 1..=MAX_PAGES {
            let mut url = self.issue_path();
            url.query_pairs_mut()
                .append_pair("state", state)
                .append_pair("per_page", &PAGE_SIZE.to_string())
                .append_pair("page", &page.to_string())
                .append_pair("sort", "created")
                .append_pair("direction", "asc");
            let body = self
                .get(url, false)
                .await?
                .ok_or_else(|| TrackerError::Payload("github_unknown_payload".into()))?;
            let items = body
                .as_array()
                .ok_or_else(|| TrackerError::Payload("github_unknown_payload".into()))?;
            for raw in items {
                // The reference safely drops individually malformed candidates.
                if let Ok(issue) = normalize(raw, &self.repo)
                    && wanted.contains(&issue.state.trim().to_lowercase())
                    && seen.insert(issue.id.clone())
                {
                    output.push(issue);
                }
            }
            if items.len() < PAGE_SIZE {
                return Ok(output);
            }
        }
        Err(TrackerError::Payload("github_pagination_limit".into()))
    }

    async fn by_ids(&self, ids: &[String]) -> Result<Vec<Issue>, TrackerError> {
        let mut output = Vec::new();
        let mut seen = HashSet::new();
        for id in ids {
            let number: u64 = id
                .parse()
                .map_err(|_| TrackerError::Config("invalid_github_issue_id".into()))?;
            if number == 0 {
                return Err(TrackerError::Config("invalid_github_issue_id".into()));
            }
            if !seen.insert(number) {
                continue;
            }
            let mut url = self.issue_path();
            url.path_segments_mut()
                .expect("validated hierarchical URL")
                .push(&number.to_string());
            if let Some(raw) = self.get(url, true).await? {
                let issue = normalize(&raw, &self.repo)?;
                if issue.id == number.to_string() {
                    output.push(issue);
                }
            }
        }
        Ok(output)
    }
}

impl Tracker for GithubTracker {
    fn fetch_issues_by_states<'a>(&'a self, states: &'a [String]) -> TrackerFuture<'a> {
        Box::pin(async move {
            if states.is_empty() {
                Ok(vec![])
            } else {
                self.by_states(states).await
            }
        })
    }
    fn fetch_issues_by_ids<'a>(&'a self, ids: &'a [String]) -> TrackerFuture<'a> {
        Box::pin(async move {
            if ids.is_empty() {
                Ok(vec![])
            } else {
                self.by_ids(ids).await
            }
        })
    }
}

fn normalize(raw: &Value, repo: &str) -> Result<Issue, TrackerError> {
    let number = raw
        .get("number")
        .and_then(Value::as_u64)
        .filter(|n| *n > 0)
        .ok_or_else(|| TrackerError::Payload("github issue number".into()))?;
    let title = required(raw, "title")?.to_owned();
    let state = required(raw, "state")?.to_owned();
    let mut native = serde_json::Map::new();
    for key in ["id", "node_id", "number"] {
        if let Some(value) = raw.get(key).filter(|v| !v.is_null()) {
            native.insert(key.into(), value.clone());
        }
    }
    native.insert("repo".into(), json!(repo));
    let optional = |key: &str| raw.get(key).and_then(Value::as_str).map(str::to_owned);
    Ok(Issue {
        id: number.to_string(),
        native_ref: Some(Value::Object(native)),
        identifier: format!("GH-{number}"),
        title,
        description: optional("body"),
        priority: None,
        state,
        branch_name: None,
        url: optional("html_url"),
        assignee_id: raw
            .pointer("/assignee/login")
            .and_then(Value::as_str)
            .map(str::to_owned),
        blocked_by: vec![],
        labels: normalized_labels(raw.get("labels")),
        dispatchable: raw.get("pull_request").is_none(),
        created_at: timestamp(raw.get("created_at")),
        updated_at: timestamp(raw.get("updated_at")),
    })
}

fn timestamp(raw: Option<&Value>) -> Option<String> {
    use time::{OffsetDateTime, format_description::well_known::Rfc3339};
    let parsed = OffsetDateTime::parse(raw?.as_str()?, &Rfc3339).ok()?;
    parsed.to_offset(time::UtcOffset::UTC).format(&Rfc3339).ok()
}
