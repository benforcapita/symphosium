# Offline subset probe

This isolated Cargo package pins cached Tokio 1.53.1, SQLx 0.8.6 and Serde 1.0.229 to test the local Rust 1.97.1 toolchain. It validates a bounded Tokio channel and SQLx PostgreSQL connection-option parsing. It does not connect to a database or satisfy the full R02 dependency gate. See `../../R02_EXECUTION_LOG_v3.md` for executed commands and limitations. `Cargo.lock` is included; `target/` is excluded.
