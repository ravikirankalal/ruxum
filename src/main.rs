use std::net::SocketAddr;
use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", 
        get(hello_handler));

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000)); 
    println!("Listening on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
#[derive(Debug, serde::Deserialize)]
struct HelloParams {
    name: Option<String>,
}
async fn hello_handler(Query(params):Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler hello_handler - ", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, {name}!</h1>"))
}

