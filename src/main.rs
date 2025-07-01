
use axum::{routing::get, routing::Router, Json};
use axum::extract::Path;
use serde_json::{json, Value};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/echo/{msg}", get(echo));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service()
    )
    .await
    .unwrap();
}


async fn root() -> &'static str {
    "Hello, world!"
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn echo(Path(msg): Path<String>) -> Json<Value> {
    Json(json!({ "echo": msg }))
}
