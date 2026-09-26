use symphosium::{providers::github::GithubTracker, tracker::Tracker};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

async fn fake_server(
    responses: Vec<(u16, String)>,
) -> (String, tokio::task::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let mut requests = vec![];
        for (status, body) in responses {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut bytes = vec![0; 8192];
            let count = stream.read(&mut bytes).await.unwrap();
            requests.push(String::from_utf8_lossy(&bytes[..count]).to_string());
            let reply = format!(
                "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        }
        requests
    });
    (url, task)
}

fn issue(number: usize, state: &str) -> serde_json::Value {
    serde_json::json!({"number": number, "title": "Task", "state": state, "body": "Body", "id": number + 100, "labels": [{"name": " Bug "}, {"name": "bug"}], "assignee": {"login": "alice"}})
}

#[tokio::test]
async fn github_paging_scope_normalization_and_auth_are_real_http() {
    let first: Vec<_> = (1..=100).map(|n| issue(n, "open")).collect();
    let second = vec![
        issue(101, "open"),
        issue(102, "closed"),
        serde_json::json!({"bad": true}),
    ];
    let (base, server) = fake_server(vec![
        (200, serde_json::to_string(&first).unwrap()),
        (200, serde_json::to_string(&second).unwrap()),
    ])
    .await;
    let tracker = GithubTracker::new(&base, "owner/repo", "synthetic-token").unwrap();
    let issues = tracker
        .fetch_issues_by_states(&["OPEN".into()])
        .await
        .unwrap();
    assert_eq!(issues.len(), 101);
    assert_eq!(issues[0].identifier, "GH-1");
    assert_eq!(issues[0].labels, ["bug"]);
    assert_eq!(issues[0].assignee_id.as_deref(), Some("alice"));
    let requests = server.await.unwrap();
    assert!(requests[0].starts_with("GET /repos/owner/repo/issues?state=open&per_page=100&page=1"));
    assert!(requests[1].contains("page=2"));
    assert!(
        requests
            .iter()
            .all(|r| r.contains("Authorization: Bearer synthetic-token")
                || r.contains("authorization: Bearer synthetic-token"))
    );
}

#[tokio::test]
async fn github_refresh_404_omits_but_malformed_payload_fails() {
    let (base, server) = fake_server(vec![
        (404, "{}".into()),
        (
            200,
            serde_json::json!({"number": 2, "title": "Task", "state": "open"}).to_string(),
        ),
    ])
    .await;
    let tracker = GithubTracker::new(&base, "owner/repo", "synthetic-token").unwrap();
    let issues = tracker
        .fetch_issues_by_ids(&["1".into(), "2".into(), "2".into()])
        .await
        .unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].id, "2");
    assert_eq!(server.await.unwrap().len(), 2);

    let (base, server) = fake_server(vec![(200, "{\"bad\":true}".into())]).await;
    let tracker = GithubTracker::new(&base, "owner/repo", "synthetic-token").unwrap();
    assert!(tracker.fetch_issues_by_ids(&["1".into()]).await.is_err());
    server.await.unwrap();
}

#[tokio::test]
async fn github_empty_input_makes_no_request() {
    let tracker =
        GithubTracker::new("http://127.0.0.1:1", "owner/repo", "synthetic-token").unwrap();
    assert!(
        tracker
            .fetch_issues_by_states(&[])
            .await
            .unwrap()
            .is_empty()
    );
    assert!(tracker.fetch_issues_by_ids(&[]).await.unwrap().is_empty());
}
