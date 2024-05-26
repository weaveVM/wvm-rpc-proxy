use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use shuttle_axum::ShuttleAxum;

#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    jsonrpc: String,
    id: u64,
    result: Option<Value>,
    error: Option<Value>,
}

struct AppState {
    client: Client,
    eth_rpc_url: String,
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    dotenv::dotenv().ok();

    let client = Client::new();
    let eth_rpc_url = "http://34.141.88.80:8545".to_string();

    let shared_state = Arc::new(Mutex::new(AppState { client, eth_rpc_url }));

    let app = Router::new()
        .route("/", post(handle_rpc_request))
        .with_state(shared_state);

    Ok(app.into())
}

async fn handle_rpc_request(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(req): Json<RpcRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let state = state.lock().await;

    let response = state
        .client
        .post(&state.eth_rpc_url)
        .json(&req)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<RpcResponse>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(response)))
}
