use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn get_pool(dsn: &str, max_connection: u32, timeout: Duration) -> PgPool {
    let db = PgPoolOptions::new()
        .max_connections(max_connection)
        .acquire_timeout(timeout)
        .connect(dsn)
        .await
        .expect("can't establish database connection");

    tracing::debug!("established connection with database");

    db
}
