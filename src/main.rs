use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;



// async関数の場合には'static をつけておかないと、文字列がFuture解決前にdropしてしまい、ダングリングポイントとなってしまう。
async fn hello_world () -> &'static str{
    "Hello World!"
}


#[tokio::main]
async fn main() {

    let app = Router::new().route("/hello", get(hello_world));

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listner = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);


    axum::serve(listner, app).await.unwrap();


}
