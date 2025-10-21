use crate::context::RequestContext;
use crate::error::Result;
use crate::protocol::{Tool, ToolContent};
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

/// Trait for implementing tool providers
///
/// Implement this trait to provide tools (functions, operations) to MCP clients.
/// All methods receive a RequestContext with access to headers and request metadata.
#[async_trait]
pub trait ToolProvider: Send + Sync {
    /// List available tools
    ///
    /// # Arguments
    /// * `ctx` - Request context with headers and metadata
    ///
    /// # Returns
    /// A vector of available tools
    async fn list_tools(&self, ctx: &RequestContext) -> Result<Vec<Tool>>;

    /// Call a specific tool
    ///
    /// # Arguments
    /// * `name` - The name of the tool to call
    /// * `arguments` - Tool arguments as a key-value map
    /// * `ctx` - Request context with headers and metadata
    ///
    /// # Returns
    /// A tuple of (content, is_error)
    async fn call_tool(
        &self,
        name: &str,
        arguments: Option<HashMap<String, Value>>,
        ctx: &RequestContext,
    ) -> Result<(Vec<ToolContent>, bool)>;
}
