use crate::context::RequestContext;
use crate::error::Result;
use crate::protocol::{Prompt, PromptMessage};
use async_trait::async_trait;
use std::collections::HashMap;

/// Trait for implementing prompt providers
///
/// Implement this trait to provide prompts (templates, instructions) to MCP clients.
/// All methods receive a RequestContext with access to headers and request metadata.
#[async_trait]
pub trait PromptProvider: Send + Sync {
    /// List available prompts
    ///
    /// # Arguments
    /// * `cursor` - Optional pagination cursor
    /// * `ctx` - Request context with headers and metadata
    ///
    /// # Returns
    /// A tuple of (prompts, next_cursor)
    async fn list_prompts(
        &self,
        cursor: Option<&str>,
        ctx: &RequestContext,
    ) -> Result<(Vec<Prompt>, Option<String>)>;

    /// Get a specific prompt with arguments
    ///
    /// # Arguments
    /// * `name` - The name of the prompt to get
    /// * `arguments` - Prompt arguments to populate the template
    /// * `ctx` - Request context with headers and metadata
    ///
    /// # Returns
    /// A tuple of (description, messages)
    async fn get_prompt(
        &self,
        name: &str,
        arguments: Option<HashMap<String, String>>,
        ctx: &RequestContext,
    ) -> Result<(Option<String>, Vec<PromptMessage>)>;
}
