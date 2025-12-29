use serde::{Deserialize, Serialize};

/// RPC request envelope accepted by the HTTP transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRpcRequest {
    pub service: String,
    pub method: String,
    /// Base64-encoded payload (empty string represents an empty payload).
    pub payload_b64: String,
    pub timeout_ms: u64,
}

/// RPC response envelope returned by the HTTP transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRpcResponse {
    pub payload_b64: String,
    pub error: Option<HttpRpcError>,
}

/// Error payload returned for RPC-level failures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRpcError {
    pub code: String,
    pub message: String,
}
