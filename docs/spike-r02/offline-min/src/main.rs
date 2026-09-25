#[tokio::main]
async fn main() {
    let _pool = sqlx::postgres::PgPoolOptions::new();
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    tx.send("bounded").await.unwrap();
    assert_eq!(rx.recv().await, Some("bounded"));
    let options: sqlx::postgres::PgConnectOptions =
        "postgres://probe:synthetic@localhost:5432/probe"
            .parse()
            .unwrap();
    assert_eq!(options.get_host(), "localhost");
    println!("tokio/sqlx probe");
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn bounded_channel_backpressure() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        tx.send(1).await.unwrap();
        assert!(tx.try_send(2).is_err());
        assert_eq!(rx.recv().await, Some(1));
        tx.send(2).await.unwrap();
        assert_eq!(rx.recv().await, Some(2));
    }
}
