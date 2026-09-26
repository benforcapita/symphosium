use symphosium::{
    providers::memory::MemoryTracker,
    tracker::{Issue, Tracker},
};

fn issue(id: &str, state: &str) -> Issue {
    Issue {
        id: id.into(),
        native_ref: None,
        identifier: format!("M-{id}"),
        title: "Task".into(),
        description: None,
        priority: None,
        state: state.into(),
        branch_name: None,
        url: None,
        assignee_id: None,
        blocked_by: vec![],
        labels: vec![],
        dispatchable: true,
        created_at: None,
        updated_at: None,
    }
}

#[tokio::test]
async fn memory_casefold_refresh_and_replacement() {
    let tracker = MemoryTracker::new(vec![issue("1", " In Progress "), issue("2", "Done")]);
    assert_eq!(
        tracker
            .fetch_issues_by_states(&["in progress".into()])
            .await
            .unwrap()[0]
            .id,
        "1"
    );
    assert_eq!(
        tracker
            .fetch_issues_by_ids(&["2".into(), "2".into()])
            .await
            .unwrap()
            .len(),
        1
    );
    tracker.replace(vec![issue("3", "Todo")]);
    assert!(
        tracker
            .fetch_issues_by_ids(&["2".into()])
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        tracker
            .fetch_issues_by_states(&[])
            .await
            .unwrap()
            .is_empty()
    );
}
