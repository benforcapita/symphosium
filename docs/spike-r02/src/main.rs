use askama::Template;
#[cfg(test)]
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
    response::sse::{Event, Sse},
    routing::{get, post},
};
use serde::Deserialize;
#[cfg(test)]
use std::convert::Infallible;
#[cfg(test)]
use tower::ServiceExt;

#[derive(Template)]
#[template(source = "<p>{{ name }}</p>", ext = "html")]
struct Page<'a> {
    name: &'a str,
}

#[derive(Deserialize)]
struct Config {
    enabled: bool,
}

#[cfg(test)]
fn app() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/login",
            get(|| async {
                (
                    [(
                        header::SET_COOKIE,
                        "sid=synthetic; Secure; HttpOnly; SameSite=Lax; Path=/",
                    )],
                    "ok",
                )
            }),
        )
        .route(
            "/events",
            get(|| async {
                Sse::new(tokio_stream::iter([Ok::<_, Infallible>(
                    Event::default().data("invalidate:42"),
                )]))
            }),
        )
        .route(
            "/mutate",
            post(|headers: axum::http::HeaderMap| async move {
                if headers.get(header::COOKIE).map(|v| v.as_bytes()) == Some(b"sid=synthetic")
                    && headers.get("x-csrf-token").map(|v| v.as_bytes()) == Some(b"synthetic-csrf")
                {
                    StatusCode::NO_CONTENT
                } else {
                    StatusCode::FORBIDDEN
                }
            }),
        )
}

fn safe_markdown(source: &str) -> String {
    let mut raw = String::new();
    let no_raw_html = pulldown_cmark::Parser::new(source).map(|event| match event {
        pulldown_cmark::Event::Html(_) | pulldown_cmark::Event::InlineHtml(_) => {
            pulldown_cmark::Event::Text(pulldown_cmark::CowStr::Borrowed(""))
        }
        other => other,
    });
    pulldown_cmark::html::push_html(&mut raw, no_raw_html);
    ammonia::Builder::new()
        .rm_tags(&["img"])
        .clean(&raw)
        .to_string()
}

#[tokio::main]
async fn main() {
    let config: Config = serde_yaml::from_str("enabled: true").unwrap();
    assert!(config.enabled);
    let _db = sqlx::postgres::PgPoolOptions::new();
    let html = Page { name: "<script>" }.render().unwrap();
    assert_eq!(html, "<p>&#60;script&#62;</p>");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("{{ issue.identifier }}")
        .unwrap();
    let globals = liquid::object!({"issue": {"identifier": "DEMO-1"}});
    assert_eq!(template.render(&globals).unwrap(), "DEMO-1");
    assert!(!safe_markdown("[x](javascript:alert(1))").contains("javascript:"));
    println!("capability spike ok");
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};

    #[tokio::test]
    async fn provider_client_scope_redirect_and_timeout() {
        use axum::http::HeaderMap;
        use tokio::net::TcpListener;
        let provider = Router::new()
            .route(
                "/scoped",
                get(|headers: HeaderMap| async move {
                    if headers.get("x-scope").map(|v| v.as_bytes()) == Some(b"demo-project") {
                        StatusCode::OK
                    } else {
                        StatusCode::FORBIDDEN
                    }
                }),
            )
            .route(
                "/redirect",
                get(|| async {
                    (
                        StatusCode::FOUND,
                        [(header::LOCATION, "https://example.invalid/")],
                    )
                }),
            )
            .route(
                "/slow",
                get(|| async {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    StatusCode::OK
                }),
            );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move { axum::serve(listener, provider).await.unwrap() });
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_millis(50))
            .build()
            .unwrap();
        assert_eq!(
            client
                .get(format!("{origin}/scoped"))
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            client
                .get(format!("{origin}/scoped"))
                .header("x-scope", "demo-project")
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::OK
        );
        assert_eq!(
            client
                .get(format!("{origin}/redirect"))
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::FOUND
        );
        assert!(
            client
                .get(format!("{origin}/slow"))
                .send()
                .await
                .unwrap_err()
                .is_timeout()
        );
        server.abort();
        assert!(server.await.unwrap_err().is_cancelled());
    }

    #[tokio::test]
    #[ignore = "requires a disposable PostgreSQL URL in R02_PG_URL"]
    async fn postgres_ddl_and_transaction_rollback() {
        let url = std::env::var("R02_PG_URL").expect("R02_PG_URL required");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(&url)
            .await
            .unwrap();
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("CREATE TABLE r02_probe (id INTEGER PRIMARY KEY, note TEXT NOT NULL)")
            .execute(&mut *tx)
            .await
            .unwrap();
        sqlx::query("INSERT INTO r02_probe (id, note) VALUES (1, 'synthetic')")
            .execute(&mut *tx)
            .await
            .unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM r02_probe")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(count.0, 1);
        tx.rollback().await.unwrap();
        let exists: (bool,) = sqlx::query_as(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'r02_probe')"
        ).fetch_one(&pool).await.unwrap();
        assert!(!exists.0);
        pool.close().await;
    }

    #[tokio::test]
    async fn route_sse_and_csrf_boundary() {
        let login = app()
            .oneshot(
                Request::builder()
                    .uri("/login")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            login.headers()[header::SET_COOKIE],
            "sid=synthetic; Secure; HttpOnly; SameSite=Lax; Path=/"
        );
        let health = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(health.status(), StatusCode::OK);
        let event = app()
            .oneshot(
                Request::builder()
                    .uri("/events")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(event.headers()[header::CONTENT_TYPE], "text/event-stream");
        let body = to_bytes(event.into_body(), 1024).await.unwrap();
        assert!(
            std::str::from_utf8(&body)
                .unwrap()
                .contains("invalidate:42")
        );
        let denied = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mutate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::FORBIDDEN);
        let csrf_without_session = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mutate")
                    .header("x-csrf-token", "synthetic-csrf")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(csrf_without_session.status(), StatusCode::FORBIDDEN);
        let session_without_csrf = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mutate")
                    .header(header::COOKIE, "sid=synthetic")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(session_without_csrf.status(), StatusCode::FORBIDDEN);
        let allowed = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mutate")
                    .header(header::COOKIE, "sid=synthetic")
                    .header("x-csrf-token", "synthetic-csrf")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(allowed.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn bounded_channel_shutdown() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        tx.send(1).await.unwrap();
        assert!(tx.try_send(2).is_err());
        assert_eq!(rx.recv().await, Some(1));
        drop(tx);
        assert_eq!(rx.recv().await, None);
    }

    #[test]
    fn template_config_and_markdown_boundaries() {
        let html = Page { name: "<script>" }.render().unwrap();
        assert_eq!(html, "<p>&#60;script&#62;</p>");
        assert!(!html.contains("<script>"));
        assert!(
            serde_yaml::from_str::<Config>("enabled: true")
                .unwrap()
                .enabled
        );
        assert!(serde_yaml::from_str::<Config>("enabled: maybe").is_err());
        let rendered = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse("{{ issue.identifier }}")
            .unwrap()
            .render(&liquid::object!({"issue": {"identifier": "DEMO-1"}}))
            .unwrap();
        assert_eq!(rendered, "DEMO-1");
        let cleaned = safe_markdown(
            "<script>bad()</script>\n\n<b>raw</b>\n\n[x](javascript:alert(1))\n\n[x](data:text/html,evil)\n\n![remote](https://example.test/x.png)",
        );
        assert!(!cleaned.contains("<script>"));
        assert!(!cleaned.contains("<b>"));
        assert!(!cleaned.contains("href=\"javascript:"));
        assert!(!cleaned.contains("href=\"data:"));
        assert!(!cleaned.contains("<img"));
    }

    #[test]
    fn argon2_hash_and_verify() {
        let salt = argon2::password_hash::SaltString::generate(
            &mut argon2::password_hash::rand_core::OsRng,
        );
        let argon = argon2::Argon2::default();
        let encoded = argon
            .hash_password(b"synthetic", &salt)
            .unwrap()
            .to_string();
        let parsed = PasswordHash::new(&encoded).unwrap();
        assert!(argon.verify_password(b"synthetic", &parsed).is_ok());
        assert!(argon.verify_password(b"wrong", &parsed).is_err());
    }
}
