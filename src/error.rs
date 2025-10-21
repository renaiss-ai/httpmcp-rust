use crate::jsonrpc::{error_codes, JsonRpcError};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Invalid params: {0}")]
    InvalidParams(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Prompt not found: {0}")]
    PromptNotFound(String),

    #[error("Authentication required")]
    AuthenticationRequired,

    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON-RPC error: {0}")]
    JsonRpcError(String),
}

impl From<JsonRpcError> for McpError {
    fn from(err: JsonRpcError) -> Self {
        McpError::JsonRpcError(err.message)
    }
}

impl McpError {
    pub fn to_jsonrpc_error(&self) -> JsonRpcError {
        match self {
            McpError::ParseError(msg) => JsonRpcError {
                code: error_codes::PARSE_ERROR,
                message: msg.clone(),
                data: None,
            },
            McpError::InvalidRequest(msg) => JsonRpcError {
                code: error_codes::INVALID_REQUEST,
                message: msg.clone(),
                data: None,
            },
            McpError::MethodNotFound(msg) => JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: msg.clone(),
                data: None,
            },
            McpError::InvalidParams(msg) => JsonRpcError {
                code: error_codes::INVALID_PARAMS,
                message: msg.clone(),
                data: None,
            },
            McpError::InternalError(msg) => JsonRpcError {
                code: error_codes::INTERNAL_ERROR,
                message: msg.clone(),
                data: None,
            },
            McpError::ResourceNotFound(uri) => JsonRpcError {
                code: error_codes::RESOURCE_NOT_FOUND,
                message: format!("Resource not found: {}", uri),
                data: Some(serde_json::json!({ "uri": uri })),
            },
            McpError::ToolNotFound(name) => JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: format!("Tool not found: {}", name),
                data: Some(serde_json::json!({ "tool": name })),
            },
            McpError::PromptNotFound(name) => JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: format!("Prompt not found: {}", name),
                data: Some(serde_json::json!({ "prompt": name })),
            },
            McpError::AuthenticationRequired => JsonRpcError {
                code: error_codes::INVALID_REQUEST,
                message: "Authentication required".to_string(),
                data: None,
            },
            McpError::AuthorizationFailed(msg) => JsonRpcError {
                code: error_codes::INVALID_REQUEST,
                message: format!("Authorization failed: {}", msg),
                data: None,
            },
            McpError::SerializationError(e) => JsonRpcError {
                code: error_codes::INTERNAL_ERROR,
                message: format!("Serialization error: {}", e),
                data: None,
            },
            McpError::IoError(e) => JsonRpcError {
                code: error_codes::INTERNAL_ERROR,
                message: format!("IO error: {}", e),
                data: None,
            },
            McpError::JsonRpcError(msg) => JsonRpcError {
                code: error_codes::INTERNAL_ERROR,
                message: msg.clone(),
                data: None,
            },
        }
    }
}

impl ResponseError for McpError {
    fn status_code(&self) -> StatusCode {
        match self {
            McpError::ParseError(_) => StatusCode::BAD_REQUEST,
            McpError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            McpError::MethodNotFound(_) => StatusCode::NOT_FOUND,
            McpError::InvalidParams(_) => StatusCode::BAD_REQUEST,
            McpError::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            McpError::ToolNotFound(_) => StatusCode::NOT_FOUND,
            McpError::PromptNotFound(_) => StatusCode::NOT_FOUND,
            McpError::AuthenticationRequired => StatusCode::UNAUTHORIZED,
            McpError::AuthorizationFailed(_) => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let jsonrpc_error = self.to_jsonrpc_error();
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "jsonrpc": "2.0",
            "error": jsonrpc_error,
            "id": null
        }))
    }
}

pub type Result<T> = std::result::Result<T, McpError>;
