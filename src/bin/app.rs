use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use adapter::database::connect_database_with;
use adapter::redis::RedisClient;
use anyhow::Result;
use api::route::{auth, v1};
use axum::{http::Method, Router};

use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

use shared::env::{Environment, which};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use anyhow::Context;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);

    let kv = Arc::new(RedisClient::new(&app_config.redis)?);

    let registry = AppRegistry::new(pool, kv, app_config);

    let app = Router::new()
        .merge(v1::routes())
        .merge(auth::routes())
        .layer(cors())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,error.message = %e, "Unexpected error"
            )
        })
}

//CORS設定をしない場合、以下のCORSエラーがブラウザのコンソール上に出る。
//Access to fetch at 'http://localhost:8080/auth/login' from origin 'http://localhost:3000' has been blocked by CORS policy:
// Response to preflight request doesn't pass access control check: No 'Access-Control-Allow-Origin' header is present on the requested resource.
use tower_http::cors::{self, CorsLayer};
fn cors() -> CorsLayer {
    CorsLayer::new()
    .allow_headers(cors::Any)
    .allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
    ])
    .allow_origin(cors::Any)
}