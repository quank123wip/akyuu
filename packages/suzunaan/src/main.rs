use axum::{
    routing::get,
    Router,
};

pub mod manifest;

use crate::manifest::manifest;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { manifest().greet }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
