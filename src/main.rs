use std::net::{Ipv4Addr, SocketAddr};

use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

use anyhow::Result;



// async関数の場合には'static をつけておかないと、文字列がFuture解決前にdropしてしまい、ダングリングポイントとなってしまう。
async fn hello_world () -> &'static str{
    "Hello World!"
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {

    let app = Router::new().route("/health", get(health_check));

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listner = TcpListener::bind(addr).await?; //?演算子はその時点でエラーが出ると呼び出し元の関数へとエラーを伝播させる。

    println!("Listening on {}", addr);


    Ok(axum::serve(listner, app).await?)


}

#[tokio::test]
async fn health_check_works(){
    let status_code = health_check().await;

    assert_eq!(status_code, StatusCode::OK);
}