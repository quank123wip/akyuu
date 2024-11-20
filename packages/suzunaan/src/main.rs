use axum::{
    routing::get,
    Router,
};

pub mod config;


async fn init_app() {
    let manifest = crate::manifest::manifest();
    let app = Router::new().route("/", get(|| async { manifest.greet }));

    let listener = tokio::net::TcpListener::bind(manifest.listen_address).await.unwrap();
    eprintln!("Akyuu web api endpoint 「鈴奈庵」 is running.");
    axum::serve(listener, app).await.unwrap();
}

fn main() {

}
