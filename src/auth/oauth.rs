use crate::context::RequestContext;
use crate::error::{McpError, Result};

/// OAuth 2.0 configuration
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl OAuthConfig {
    /// Validate OAuth token from request context
    pub async fn validate_token(&self, ctx: &RequestContext) -> Result<()> {
        let token = ctx
            .get_bearer_token()
            .ok_or(McpError::AuthenticationRequired)?;

        // TODO: Implement actual OAuth token validation
        // For now, just check if token is present
        if token.is_empty() {
            return Err(McpError::AuthorizationFailed("Invalid token".to_string()));
        }

        Ok(())
    }
}
