use crate::context::RequestContext;
use crate::error::Result;
use crate::protocol::*;
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures::future::BoxFuture;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

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

/// Endpoint handler function signature
pub type EndpointHandler = Arc<
    dyn Fn(RequestContext, Option<Value>) -> BoxFuture<'static, Result<HttpResponse>> + Send + Sync,
>;

/// Multipart endpoint handler function signature
/// Note: Multipart streams are not Send, so they must be processed immediately
/// The returned future does not need to be Send as it's processed on the same task
pub type MultipartEndpointHandler = Arc<
    dyn Fn(
            RequestContext,
            Multipart,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<HttpResponse>>>>
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

/// Registered endpoint
pub struct RegisteredEndpoint {
    pub route: String,
    pub method: String,
    pub description: Option<String>,
    pub handler: EndpointHandler,
}

/// Registered multipart endpoint
pub struct RegisteredMultipartEndpoint {
    pub route: String,
    pub method: String,
    pub description: Option<String>,
    pub handler: MultipartEndpointHandler,
}
