use std::path::Path;
use symphosium::{config::Settings, prompt, tracker::Issue, workflow};

#[test]
fn default_prompt_keeps_reference_description() {
    let issue = Issue {
        id: "1".into(),
        native_ref: None,
        identifier: "S-1".into(),
        title: "Task".into(),
        description: Some("Important body".into()),
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
    assert!(
        prompt::render("", &issue, None)
            .unwrap()
            .contains("Important body")
    );
}

#[test]
fn required_labels_deduplicate_like_reference() {
    let wf = workflow::parse(
        "---\ntracker:\n  kind: memory\n  required_labels: [' Symphony ', SYMPHONY]\n---\n",
    )
    .unwrap();
    let settings =
        Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| None).unwrap();
    assert_eq!(settings.tracker.required_labels, ["symphony"]);
}

#[test]
fn effective_assignee_scope_changes_when_env_changes() {
    let wf = workflow::parse("---\ntracker:\n  kind: linear\n  provider:\n    project_slug: p\n    api_key: token\n    assignee: $ASSIGN\n---\n").unwrap();
    let a = Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| Some("a".into()))
        .unwrap();
    let b = Settings::from_workflow_with_env(&wf, Path::new("WORKFLOW.md"), |_| Some("b".into()))
        .unwrap();
    assert!(a.scope() != b.scope());
}
