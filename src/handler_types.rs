use crate::context::RequestContext;
use crate::error::Result;
use crate::protocol::*;
use futures::future::BoxFuture;
use serde_json::Value;
use std::collections::HashMap;

/// Tool handler function signature
pub type ToolHandler = Box<
    dyn Fn(HashMap<String, Value>, RequestContext) -> BoxFuture<'static, Result<Value>>
        + Send
        + Sync,
>;

/// Resource list handler function signature
pub type ResourceListHandler = Box<
    dyn Fn(
            Option<String>,
            RequestContext,
        ) -> BoxFuture<'static, Result<(Vec<Resource>, Option<String>)>>
        + Send
        + Sync,
>;

/// Resource read handler function signature
pub type ResourceReadHandler = Box<
    dyn Fn(String, RequestContext) -> BoxFuture<'static, Result<Vec<ResourceContents>>>
        + Send
        + Sync,
>;

/// Prompt handler function signature
pub type PromptHandler = Box<
    dyn Fn(
            String,
            Option<HashMap<String, String>>,
            RequestContext,
        ) -> BoxFuture<'static, Result<(Option<String>, Vec<PromptMessage>)>>
        + Send
        + Sync,
>;

/// Registered tool
pub struct RegisteredTool {
    pub meta: Tool,
    pub handler: ToolHandler,
}

/// Registered resource
pub struct RegisteredResource {
    pub meta: Resource,
    pub list_handler: ResourceListHandler,
    pub read_handler: ResourceReadHandler,
}

/// Registered prompt
pub struct RegisteredPrompt {
    pub meta: Prompt,
    pub handler: PromptHandler,
}
