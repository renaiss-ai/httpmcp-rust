use crate::error::{McpError, Result};
use crate::jsonrpc::{JsonRpcRequest, JsonRpcResponse};
use crate::protocol::{Implementation, InitializeParams, InitializeResult, ServerCapabilities};
use serde_json::Value;

/// Handle initialize request
pub fn handle_initialize(
    req: &JsonRpcRequest,
    server_info: Implementation,
    capabilities: ServerCapabilities,
) -> Result<JsonRpcResponse> {
    let params: InitializeParams =
        serde_json::from_value(req.params.clone().unwrap_or(Value::Null))
            .map_err(|e| McpError::InvalidParams(format!("Invalid initialize params: {}", e)))?;

    // Validate protocol version
    if !params.protocol_version.starts_with("2024-")
        && !params.protocol_version.starts_with("2025-")
    {
        return Err(McpError::InvalidRequest(format!(
            "Unsupported protocol version: {}",
            params.protocol_version
        )));
    }

    let result = InitializeResult {
        protocol_version: params.protocol_version,
        capabilities,
        server_info,
    };

    Ok(JsonRpcResponse::success(
        serde_json::to_value(result)?,
        req.id.clone(),
    ))
}

/// Handle ping request
pub fn handle_ping(req: &JsonRpcRequest) -> Result<JsonRpcResponse> {
    Ok(JsonRpcResponse::success(
        serde_json::json!({}),
        req.id.clone(),
    ))
}
