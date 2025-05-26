use axum::{Router, extract::State, response::IntoResponse, routing::get};
use reqwest::Request;
use std::net::SocketAddr;
use std::sync::Arc;

async fn get_nodes(State(client): State<Arc<reqwest::Client>>) -> impl IntoResponse {
    let request_url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let req: Request = client.get(request_url).build().unwrap();
    let resp = client.execute(req).await.unwrap();

    let response_text: serde_json::Value = resp.json().await.unwrap();

    axum::response::Json(response_text)
}

#[tokio::main]
async fn main() {
    let client = Arc::new(reqwest::Client::new()); // Arc is required to set client on app state

    let app = Router::new().route("/", get(get_nodes)).with_state(client);

    println!("Running on http://localhost:3000");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
