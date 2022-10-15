use std::env;
use dotenv::dotenv;
use std::net::{SocketAddr, ToSocketAddrs};

pub mod routes;
pub mod handlers;
pub mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let routes = routes::create_routes().await;
    let listen_host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let listen_port = env::var("PORT").unwrap_or("3000".to_string());

    println!("Listening on {}:{}", listen_host, listen_port);
    axum::Server::bind(&format!("{listen_host}:{listen_port}").parse::<SocketAddr>().unwrap())
        .serve(routes.into_make_service())
        .await
        .unwrap()
}
