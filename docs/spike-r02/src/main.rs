use askama::Template;
use axum::{Router, routing::get};
use serde::Deserialize;

#[derive(Template)]
#[template(source = "<p>{{ name }}</p>", ext = "html")]
struct Page<'a> {
    name: &'a str,
}

#[derive(Deserialize)]
struct Config {
    enabled: bool,
}

#[tokio::main]
async fn main() {
    let config: Config = serde_yaml::from_str("enabled: true").unwrap();
    let _route = Router::<()>::new().route("/", get(|| async { "ok" }));
    let _db = sqlx::postgres::PgPoolOptions::new();
    let page = Page { name: "<script>" };
    let html = page.render().unwrap();
    assert!(html.contains("&lt;script&gt;"));
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("{{ issue.identifier }}")
        .unwrap();
    let globals = liquid::object!({"issue": {"identifier": "DEMO-1"}});
    assert_eq!(template.render(&globals).unwrap(), "DEMO-1");
    let markdown = pulldown_cmark::html::push_html;
    let mut raw = String::new();
    markdown(
        &mut raw,
        pulldown_cmark::Parser::new("[x](javascript:alert(1))"),
    );
    let clean = ammonia::clean(&raw);
    assert!(!clean.contains("javascript:"));
    let salt =
        argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    use argon2::PasswordHasher;
    let hash = argon2::Argon2::default()
        .hash_password(b"synthetic", &salt)
        .unwrap();
    assert!(hash.to_string().starts_with("$argon2"));
    assert!(config.enabled);
    println!("capability spike ok");
}
