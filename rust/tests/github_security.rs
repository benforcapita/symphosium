use symphosium::{providers::github::GithubTracker, tracker::Tracker};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    time::{Duration, timeout},
};

#[test]
fn origin_policy_rejects_userinfo_fragments_queries_and_nonloopback_http() {
    for base in [
        "http://127.0.0.1:80@evil.example",
        "http://127.0.0.1:80@127.0.0.1:81",
        "https://user:pass@api.github.com",
        "https://api.github.com/#fragment",
        "https://api.github.com/?query=1",
        "http://example.com",
    ] {
        assert!(
            GithubTracker::new(base, "owner/repo", "synthetic-token").is_err(),
            "{base}"
        );
    }
}

async fn one_request(
    base_reply: impl FnOnce(&str) -> String + Send + 'static,
) -> (String, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).await.unwrap();
        let request = String::from_utf8_lossy(&buf[..n]).into_owned();
        stream
            .write_all(base_reply(&request).as_bytes())
            .await
            .unwrap();
        request
    });
    (base, task)
}

#[tokio::test]
async fn repository_segments_are_encoded_and_dot_segments_rejected() {
    for (repo, expected) in [
        ("owner/repo?x=1", "repo%3Fx=1"),
        ("owner/repo#x", "repo%23x"),
        ("owner/repo%2Fother", "repo%252Fother"),
        ("owner/repo%", "repo%25"),
    ] {
        let (base, task) = one_request(|_| "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n[]".into()).await;
        let tracker = GithubTracker::new(&base, repo, "synthetic-token").unwrap();
        assert!(
            tracker
                .fetch_issues_by_states(&["open".into()])
                .await
                .unwrap()
                .is_empty()
        );
        let request = task.await.unwrap();
        assert!(
            request.lines().next().unwrap().contains(expected),
            "{request}"
        );
        assert!(
            request
                .lines()
                .next()
                .unwrap()
                .contains("/issues?state=open")
        );
    }
    for repo in [
        "owner/.",
        "owner/..",
        "./repo",
        "../repo",
        "owner/name/extra",
    ] {
        assert!(GithubTracker::new("https://api.github.com", repo, "synthetic-token").is_err());
    }
}

#[tokio::test]
async fn redirects_never_load_foreign_or_same_origin_payload() {
    let target = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let target_url = format!("http://{}/foreign", target.local_addr().unwrap());
    for destination in [
        target_url,
        "http://127.0.0.1:1/downgrade".into(),
        "/same-origin".into(),
    ] {
        let location = destination.clone();
        let (base, task) = one_request(move |_| format!("HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")).await;
        let tracker = GithubTracker::new(&base, "owner/repo", "synthetic-token").unwrap();
        let error = tracker
            .fetch_issues_by_states(&["open".into()])
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("302"), "{error}");
        assert!(
            task.await
                .unwrap()
                .to_lowercase()
                .contains("authorization: bearer synthetic-token")
        );
    }
    assert!(
        timeout(Duration::from_millis(100), target.accept())
            .await
            .is_err()
    );
}
