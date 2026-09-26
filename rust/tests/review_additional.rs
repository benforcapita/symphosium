use std::path::Path;
use symphosium::{config::Settings, prompt, workflow};

#[test]
fn invalid_per_state_limit_is_ignored_per_spec() {
    let wf = workflow::parse("---\ntracker: {kind: memory}\nagent:\n  max_concurrent_agents_by_state: {Todo: 0, Review: 2}\n---\n").unwrap();
    let settings =
        Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| None).unwrap();
    assert_eq!(
        settings.max_concurrent_agents_by_state.get("review"),
        Some(&2)
    );
    assert!(!settings.max_concurrent_agents_by_state.contains_key("todo"));
}

#[test]
fn malformed_prompt_error_retains_reference_context() {
    let err = symphosium::prompt::validate("{% if issue.identifier %}").unwrap_err();
    assert!(err.to_string().contains("template_parse_error:"));
    assert!(err.to_string().contains("template=\""));
}

#[test]
fn invalid_state_entries_are_skipped_and_normalized_duplicates_use_last_valid() {
    let wf = workflow::parse("---\ntracker: {kind: memory}\nagent:\n  max_concurrent_agents_by_state:\n    Todo: 1\n    ' todo ': 3\n    Review: bad\n    Done: -1\n    Blank: 0\n    '   ': 4\n    Valid: 2\n    42: 7\n---\n").unwrap();
    let settings =
        Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| None).unwrap();
    assert_eq!(settings.max_concurrent_agents_by_state.len(), 2);
    assert_eq!(
        settings.max_concurrent_agents_by_state.get("todo"),
        Some(&3)
    );
    assert_eq!(
        settings.max_concurrent_agents_by_state.get("valid"),
        Some(&2)
    );
}

#[test]
fn malformed_render_and_workflow_load_have_distinct_errors() {
    let issue = symphosium::tracker::Issue {
        id: "1".into(),
        native_ref: None,
        identifier: "S-1".into(),
        title: "Task".into(),
        description: None,
        priority: None,
        state: "Todo".into(),
        branch_name: None,
        url: None,
        assignee_id: None,
        blocked_by: vec![],
        labels: vec![],
        dispatchable: true,
        created_at: None,
        updated_at: None,
    };
    let error = prompt::render("{% if issue.identifier %}", &issue, None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("template_parse_error:"));
    assert!(error.contains("template=\""));
    let missing =
        match symphosium::workflow::load(Path::new("/nonexistent-r05-workflow-fixture.md")) {
            Ok(_) => panic!("missing fixture loaded"),
            Err(error) => error.to_string(),
        };
    assert!(!missing.contains("template_parse_error:"));
}
