use actix_web::http::header::HeaderMap;
use std::net::SocketAddr;
use uuid::Uuid;

/// Request context passed to all handler methods
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// HTTP headers from the request
    pub headers: HeaderMap,

    /// Request ID for tracing
    pub request_id: String,

    /// HTTP method (GET, POST)
    pub method: String,

    /// Request path
    pub path: String,

    /// Remote client address
    pub remote_addr: Option<SocketAddr>,
}

impl RequestContext {
    pub fn new(
        headers: HeaderMap,
        method: String,
        path: String,
        remote_addr: Option<SocketAddr>,
    ) -> Self {
        Self {
            headers,
            request_id: Uuid::new_v4().to_string(),
            method,
            path,
            remote_addr,
        }
    }

    /// Get a header value as string
    pub fn get_header(&self, name: &str) -> Option<String> {
        self.headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }

    /// Get authorization header
    pub fn get_authorization(&self) -> Option<String> {
        self.get_header("authorization")
    }

    /// Get bearer token from authorization header
    pub fn get_bearer_token(&self) -> Option<String> {
        self.get_authorization()
            .and_then(|auth| auth.strip_prefix("Bearer ").map(|s| s.to_string()))
    }

    /// Get custom header by name
    pub fn get_custom_header(&self, name: &str) -> Option<String> {
        self.get_header(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};

    #[test]
    fn test_get_bearer_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_static("Bearer test_token_123"),
        );

        let ctx = RequestContext::new(headers, "POST".to_string(), "/mcp".to_string(), None);

        assert_eq!(ctx.get_bearer_token(), Some("test_token_123".to_string()));
    }

    #[test]
    fn test_get_custom_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-tenant-id"),
            HeaderValue::from_static("tenant-123"),
        );

        let ctx = RequestContext::new(headers, "POST".to_string(), "/mcp".to_string(), None);

        assert_eq!(
            ctx.get_custom_header("x-tenant-id"),
            Some("tenant-123".to_string())
        );
    }
}
