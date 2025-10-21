use crate::context::RequestContext;
use crate::error::Result;
use crate::protocol::{Resource, ResourceContents, ResourceTemplate};
use async_trait::async_trait;

/// Trait for implementing resource providers
///
/// Implement this trait to provide resources (files, data, etc.) to MCP clients.
/// All methods receive a RequestContext with access to headers and request metadata.
#[async_trait]
pub trait ResourceProvider: Send + Sync {
    /// List available resources
    ///
    /// # Arguments
    /// * `cursor` - Optional pagination cursor
    /// * `ctx` - Request context with headers and metadata
    ///
    /// # Returns
    /// A tuple of (resources, next_cursor)
    async fn list_resources(
        &self,
        cursor: Option<&str>,
        ctx: &RequestContext,
    ) -> Result<(Vec<Resource>, Option<String>)>;

    /// Read a specific resource by URI
    ///
    /// # Arguments
    /// * `uri` - The URI of the resource to read
    /// * `ctx` - Request context with headers and metadata
    ///
    /// # Returns
    /// The resource contents
    async fn read_resource(&self, uri: &str, ctx: &RequestContext)
        -> Result<Vec<ResourceContents>>;

    /// List resource templates (optional)
    ///
    /// Resource templates define URI patterns for dynamic resources.
    /// Return empty vec if not supported.
    async fn list_templates(&self, _ctx: &RequestContext) -> Result<Vec<ResourceTemplate>> {
        Ok(vec![])
    }

    /// Subscribe to resource changes (optional)
    ///
    /// Called when a client subscribes to changes for a specific resource.
    /// Return Ok if subscription is accepted.
    async fn subscribe(&self, _uri: &str, _ctx: &RequestContext) -> Result<()> {
        Ok(())
    }

    /// Unsubscribe from resource changes (optional)
    async fn unsubscribe(&self, _uri: &str, _ctx: &RequestContext) -> Result<()> {
        Ok(())
    }
}
