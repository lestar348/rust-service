use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use base64::{engine::general_purpose, Engine as _};
use service_core::{
    router::{RpcError, RpcRegistry, RpcRequest},
    Transport,
};

use crate::http::protocol::{HttpRpcError, HttpRpcRequest, HttpRpcResponse};

pub mod client;
pub mod protocol;

#[derive(Clone)]
pub struct HttpServerTransport {
    registry: Arc<dyn RpcRegistry>,
}

impl HttpServerTransport {
    pub fn new(registry: Arc<dyn RpcRegistry>) -> Self {
        Self { registry }
    }

    /// Build the Axum router handling HTTP RPC requests.
    pub fn router(&self) -> Router {
        let state = HttpServerState {
            registry: self.registry.clone(),
        };
        Router::new()
            .route("/rpc", post(handle_rpc))
            .with_state(state)
    }

    /// Start serving HTTP RPC requests on the provided socket address.
    pub async fn serve(self, addr: SocketAddr) -> anyhow::Result<()> {
        let router = self.router();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router).await?;
        Ok(())
    }
}

impl Transport for HttpServerTransport {
    fn id(&self) -> service_core::types::TransportId {
        "http".to_string()
    }

    fn start(&self) -> service_core::Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
struct HttpServerState {
    registry: Arc<dyn RpcRegistry>,
}

async fn handle_rpc(
    State(state): State<HttpServerState>,
    Json(request): Json<HttpRpcRequest>,
) -> Result<Json<HttpRpcResponse>, HttpHandlerError> {
    let payload = decode_payload(&request.payload_b64)?;
    let rpc_request = RpcRequest::new(request.service, request.method, payload, request.timeout_ms);
    let rpc_result = state.registry.dispatch(rpc_request).await;

    let response = match rpc_result {
        Ok(rpc_response) => HttpRpcResponse {
            payload_b64: encode_payload(&rpc_response.payload),
            error: None,
        },
        Err(err) => HttpRpcResponse {
            payload_b64: String::new(),
            error: Some(map_rpc_error(&err)),
        },
    };

    Ok(Json(response))
}

#[derive(Debug)]
enum HttpHandlerError {
    InvalidBase64(String),
}

impl IntoResponse for HttpHandlerError {
    fn into_response(self) -> Response {
        match self {
            HttpHandlerError::InvalidBase64(message) => (
                StatusCode::BAD_REQUEST,
                Json(HttpRpcResponse {
                    payload_b64: String::new(),
                    error: Some(HttpRpcError {
                        code: "decode".to_string(),
                        message,
                    }),
                }),
            )
                .into_response(),
        }
    }
}

fn decode_payload(encoded: &str) -> Result<Vec<u8>, HttpHandlerError> {
    general_purpose::STANDARD
        .decode(encoded)
        .map_err(|err| HttpHandlerError::InvalidBase64(err.to_string()))
}

fn encode_payload(payload: &[u8]) -> String {
    general_purpose::STANDARD.encode(payload)
}

fn map_rpc_error(err: &RpcError) -> HttpRpcError {
    let code = match err {
        RpcError::UnknownMethod => "unknown_method",
        RpcError::Decode(_) => "decode",
        RpcError::Internal(_) => "internal",
    }
    .to_string();

    HttpRpcError {
        code,
        message: err.to_string(),
    }
}
