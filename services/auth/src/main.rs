extern crate core;

use dotenv::dotenv;
use std::env;

pub mod lib;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let routes = routes::create_routes().await;
    let host = env::var("SRV_HOST").expect("SRV_HOST is missing");
    let port = env::var("SRV_PORT").expect("SRV_PORT is missing");

    axum::Server::bind(&format!("{}:{}", host, port).parse().unwrap())
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
