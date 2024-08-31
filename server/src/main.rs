use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{extract::State, response::Html, routing::get, Router};
use tokio::time;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use common::{db::pool::get_pool, routing::app_state::AppState};
use config::Config;

mod common;
mod config;
mod errors;

#[tokio::main]
async fn main() {
    let config = Config::new().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rgiombini=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = get_pool(
        &config.db_uri,
        config.workers.into(),
        time::Duration::from_secs(30),
    )
    .await;

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.port);

    let app_state = AppState::new(db, config);
    let shared_state = Arc::new(app_state);

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_state);

    // run it
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<Arc<AppState>>) -> Html<&'static str> {
    // sqlx::query_as("SELECT * FROM accounts");

    Html("<h1>Hello, World!</h1>")
}
