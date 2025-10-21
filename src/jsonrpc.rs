use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Request ID can be string or number
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(i64),
}

/// JSON-RPC Error Codes
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;

    // MCP specific errors
    pub const RESOURCE_NOT_FOUND: i32 = -32002;
}

impl JsonRpcRequest {
    pub fn new(method: impl Into<String>, params: Option<Value>, id: Option<RequestId>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.into(),
            params,
            id,
        }
    }

    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }

    pub fn validate(&self) -> Result<(), JsonRpcError> {
        if self.jsonrpc != "2.0" {
            return Err(JsonRpcError {
                code: error_codes::INVALID_REQUEST,
                message: "Invalid JSON-RPC version".to_string(),
                data: None,
            });
        }

        if self.method.is_empty() {
            return Err(JsonRpcError {
                code: error_codes::INVALID_REQUEST,
                message: "Method cannot be empty".to_string(),
                data: None,
            });
        }

        Ok(())
    }
}

impl JsonRpcResponse {
    pub fn success(result: Value, id: Option<RequestId>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    pub fn error(error: JsonRpcError, id: Option<RequestId>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id,
        }
    }
}

impl JsonRpcError {
    pub fn parse_error() -> Self {
        Self {
            code: error_codes::PARSE_ERROR,
            message: "Parse error".to_string(),
            data: None,
        }
    }

    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: error_codes::INVALID_REQUEST,
            message: message.into(),
            data: None,
        }
    }

    pub fn method_not_found(method: impl Into<String>) -> Self {
        Self {
            code: error_codes::METHOD_NOT_FOUND,
            message: format!("Method not found: {}", method.into()),
            data: None,
        }
    }

    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self {
            code: error_codes::INVALID_PARAMS,
            message: message.into(),
            data: None,
        }
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self {
            code: error_codes::INTERNAL_ERROR,
            message: message.into(),
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_validation() {
        let req = JsonRpcRequest::new("test", None, Some(RequestId::Number(1)));
        assert!(req.validate().is_ok());

        let invalid = JsonRpcRequest {
            jsonrpc: "1.0".to_string(),
            method: "test".to_string(),
            params: None,
            id: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_is_notification() {
        let req = JsonRpcRequest::new("test", None, None);
        assert!(req.is_notification());

        let req = JsonRpcRequest::new("test", None, Some(RequestId::Number(1)));
        assert!(!req.is_notification());
    }
}
