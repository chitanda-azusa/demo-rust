use axum::{
    routing::get,
    Router,
};

use crate::handlers::hello_world::hello_world;

pub(crate) mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind("localhost:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
