use httpmcp_rust::protocol::{Resource, ResourceContents};
use httpmcp_rust::{HttpMcpServer, RequestContext, ResourceMeta, Result, ToolMeta};
use serde_json::{json, Value};
use std::collections::HashMap;

// ============================================================================
// Resource Handlers
// ============================================================================

async fn list_example_resources(
    _cursor: Option<String>,
    _ctx: RequestContext,
) -> Result<(Vec<Resource>, Option<String>)> {
    let resources = vec![Resource {
        uri: "file:///example.txt".to_string(),
        name: "Example File".to_string(),
        description: Some("A simple example file".to_string()),
        mime_type: Some("text/plain".to_string()),
    }];

    Ok((resources, None))
}

async fn read_example_resource(uri: String, _ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    if uri == "file:///example.txt" {
        Ok(vec![ResourceContents {
            uri,
            mime_type: Some("text/plain".to_string()),
            text: Some("Hello from MCP server!".to_string()),
            blob: None,
        }])
    } else {
        Err(httpmcp_rust::McpError::ResourceNotFound(uri))
    }
}

// ============================================================================
// Tool Handlers
// ============================================================================

async fn echo_tool(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let message = args
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("No message provided");

    Ok(json!({
        "echo": message
    }))
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,httpmcp_rust=debug")
        .init();

    // Build server with function registration
    let server = HttpMcpServer::builder()
        .name("simple-mcp-server")
        .version("1.0.0")
        // Register resource
        .resource(
            "file:///example.txt",
            ResourceMeta::new()
                .name("Example File")
                .description("A simple example file")
                .mime_type("text/plain"),
            list_example_resources,
            read_example_resource,
        )
        // Register tool
        .tool(
            "echo",
            ToolMeta::new()
                .description("Echoes back the input message")
                .param("message", "string", "The message to echo")
                .required(&["message"]),
            echo_tool,
        )
        .build()
        .expect("Failed to build server");

    println!("🚀 MCP Server running on http://127.0.0.1:8080");
    println!("Try:");
    println!("  POST http://127.0.0.1:8080/mcp");
    println!("  GET  http://127.0.0.1:8080/mcp (SSE)");

    server.run("127.0.0.1:8080").await
}
