//! # httpmcp-rust
//!
//! A fast, simple, production-ready library for building MCP (Model Context Protocol) servers
//! using Streamable HTTP.
//!
//! ## Features
//!
//! - **Simple API**: Function-based registration with builder pattern
//! - **Fast**: Built on actix-web with async/await
//! - **Production-ready**: OAuth 2.0, SSE with resumption, proper error handling
//! - **Type-safe**: Strong typing throughout
//! - **Extensible**: Easy to add custom resources, tools, and prompts
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use httpmcp_rust::{HttpMcpServer, RequestContext, ResourceMeta, ToolMeta, Result};
//! use httpmcp_rust::protocol::{Resource, ResourceContents};
//! use serde_json::{json, Value};
//! use std::collections::HashMap;
//!
//! async fn list_resources(
//!     _cursor: Option<String>,
//!     _ctx: RequestContext,
//! ) -> Result<(Vec<Resource>, Option<String>)> {
//!     Ok((vec![Resource {
//!         uri: "file:///example.txt".to_string(),
//!         name: "Example".to_string(),
//!         description: Some("Example file".to_string()),
//!         mime_type: Some("text/plain".to_string()),
//!     }], None))
//! }
//!
//! async fn read_resource(uri: String, _ctx: RequestContext) -> Result<Vec<ResourceContents>> {
//!     Ok(vec![ResourceContents {
//!         uri,
//!         mime_type: Some("text/plain".to_string()),
//!         text: Some("Hello, MCP!".to_string()),
//!         blob: None,
//!     }])
//! }
//!
//! async fn echo_tool(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
//!     Ok(json!({"echo": args.get("message")}))
//! }
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let server = HttpMcpServer::builder()
//!         .name("my-server")
//!         .version("1.0.0")
//!         .resource(
//!             "file:///example.txt",
//!             ResourceMeta::new().name("Example").mime_type("text/plain"),
//!             list_resources,
//!             read_resource,
//!         )
//!         .tool(
//!             "echo",
//!             ToolMeta::new()
//!                 .description("Echo a message")
//!                 .param("message", "string", "Message to echo")
//!                 .required(&["message"]),
//!             echo_tool,
//!         )
//!         .build()
//!         .expect("Failed to build server");
//!
//!     server.run("127.0.0.1:8080").await
//! }
//! ```

pub mod auth;
pub mod context;
pub mod error;
pub mod handler_types;
pub mod handlers;
pub mod jsonrpc;
pub mod metadata;
pub mod middleware;
pub mod protocol;
pub mod server;
pub mod sse;
pub mod transport;

// Re-export commonly used types
pub use context::RequestContext;
pub use error::{McpError, Result};
pub use metadata::{EndpointMeta, PromptMeta, ResourceMeta, ToolMeta};
pub use server::{HttpMcpServer, HttpMcpServerBuilder};

// Re-export protocol types
pub use protocol::{
    Implementation, Prompt, PromptArgument, PromptContent, PromptMessage, PromptsGetParams,
    PromptsGetResult, PromptsListParams, PromptsListResult, Resource, ResourceContents,
    ResourceTemplate, ResourcesListParams, ResourcesListResult, ResourcesReadParams,
    ResourcesReadResult, ServerCapabilities, Tool, ToolContent, ToolsCallParams, ToolsCallResult,
    ToolsListResult,
};
