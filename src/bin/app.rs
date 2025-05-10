use std::net::{Ipv4Addr, SocketAddr};

use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

use anyhow::Result;

use sqlx::{PgPool, postgres::PgConnectOptions};



// impl From<DatabaseConfig> for PgConnectOptions {
//     fn from(cfg: DatabaseConfig) -> Self {
//         Self::new()
//             .host(&cfg.host)
//             .port(cfg.port)
//             .username(&cfg.username)
//             .password(&cfg.password)
//             .database(&cfg.database)
//     }
// }

// fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
//     PgPool::connect_lazy_with(cfg.into())
// }

// // async関数の場合には'static をつけておかないと、文字列がFuture解決前にdropしてしまい、ダングリングポイントとなってしまう。
async fn hello_world() -> &'static str {
    "Hello World!"
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_db (State(db): State<PgPool>) -> StatusCode {
let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
match connection_result {
    Ok(_) => StatusCode::OK,
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
}
}

#[tokio::main]
async fn main() -> Result<()> {

    let database_cfg = DatabaseConfig {
        host: "localhost".into(),
        port: 5432,
        username: "app".into(),
        password: "passwd".into(),
        database: "app".into(),
    };

    let conn_pool = connect_database_with(database_cfg);


    let app = Router::new().route("/health", get(health_check))
    .route("/health/db", get(health_check_db))
    .with_state(conn_pool);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listner = TcpListener::bind(addr).await?; //?演算子はその時点でエラーが出ると呼び出し元の関数へとエラーを伝播させる。

    println!("Listening on {}", addr);

    Ok(axum::serve(listner, app).await?)
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;

    assert_eq!(status_code, StatusCode::OK);
}

#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let status_code  = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}

#[tokio::test]
async fn check_hello_world_works() {
    let str = hello_world().await;
    assert_eq!(str,"Hello World!")
}