use httpmcp_rust::protocol::*;
use httpmcp_rust::{HttpMcpServer, PromptMeta, RequestContext, ResourceMeta, Result, ToolMeta};
use serde_json::{json, Value};
use std::collections::HashMap;

// ============================================================================
// Resource Handlers
// ============================================================================

async fn list_file_resources(
    _cursor: Option<String>,
    ctx: RequestContext,
) -> Result<(Vec<Resource>, Option<String>)> {
    let tenant_id = ctx.get_custom_header("x-tenant-id");
    tracing::info!("Listing resources for tenant: {:?}", tenant_id);

    let resources = vec![
        Resource {
            uri: "file:///docs/readme.md".to_string(),
            name: "README".to_string(),
            description: Some("Project documentation".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "file:///data/users.json".to_string(),
            name: "Users Data".to_string(),
            description: Some("User database".to_string()),
            mime_type: Some("application/json".to_string()),
        },
    ];

    Ok((resources, None))
}

async fn read_file_resource(uri: String, ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    tracing::debug!(
        "Reading resource: {} from {}",
        uri,
        ctx.remote_addr.map(|a| a.to_string()).unwrap_or_default()
    );

    let content = match uri.as_str() {
        "file:///docs/readme.md" => ResourceContents {
            uri,
            mime_type: Some("text/markdown".to_string()),
            text: Some("# MCP Server\n\nFull-featured example".to_string()),
            blob: None,
        },
        "file:///data/users.json" => ResourceContents {
            uri,
            mime_type: Some("application/json".to_string()),
            text: Some(r#"{"users": [{"id": 1, "name": "Alice"}]}"#.to_string()),
            blob: None,
        },
        _ => return Err(httpmcp_rust::McpError::ResourceNotFound(uri)),
    };

    Ok(vec![content])
}

// ============================================================================
// Tool Handlers
// ============================================================================

async fn add_tool(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let a = args
        .get("a")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| httpmcp_rust::McpError::InvalidParams("Invalid 'a'".to_string()))?;

    let b = args
        .get("b")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| httpmcp_rust::McpError::InvalidParams("Invalid 'b'".to_string()))?;

    Ok(json!({
        "result": a + b
    }))
}

async fn multiply_tool(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let a = args
        .get("a")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| httpmcp_rust::McpError::InvalidParams("Invalid 'a'".to_string()))?;

    let b = args
        .get("b")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| httpmcp_rust::McpError::InvalidParams("Invalid 'b'".to_string()))?;

    Ok(json!({
        "result": a * b
    }))
}

// ============================================================================
// Prompt Handlers
// ============================================================================

async fn code_review_prompt(
    _name: String,
    arguments: Option<HashMap<String, String>>,
    _ctx: RequestContext,
) -> Result<(Option<String>, Vec<PromptMessage>)> {
    let code = arguments
        .and_then(|mut args| args.remove("code"))
        .unwrap_or_default();

    let messages = vec![PromptMessage {
        role: "user".to_string(),
        content: PromptContent::Text {
            text: format!(
                "Please review this code:\n\n```\n{}\n```\n\nProvide feedback on:\n- Code quality\n- Best practices\n- Potential bugs",
                code
            ),
        },
    }];

    Ok((Some("Code review prompt".to_string()), messages))
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

    // Build server with all features
    let server = HttpMcpServer::builder()
        .name("full-mcp-server")
        .version("2.0.0")
        // Resources
        .resource(
            "file:///docs/readme.md",
            ResourceMeta::new()
                .name("README")
                .description("Project documentation")
                .mime_type("text/markdown"),
            list_file_resources,
            read_file_resource,
        )
        // Tools
        .tool(
            "add",
            ToolMeta::new()
                .description("Add two numbers")
                .param("a", "number", "First number")
                .param("b", "number", "Second number")
                .required(&["a", "b"]),
            add_tool,
        )
        .tool(
            "multiply",
            ToolMeta::new()
                .description("Multiply two numbers")
                .param("a", "number", "First number")
                .param("b", "number", "Second number")
                .required(&["a", "b"]),
            multiply_tool,
        )
        // Prompts
        .prompt(
            "code_review",
            PromptMeta::new()
                .description("Review code for quality and best practices")
                .arg("code", "The code to review", true),
            code_review_prompt,
        )
        .enable_cors(true)
        .build()
        .expect("Failed to build server");

    println!("🚀 Full-featured MCP Server running on http://127.0.0.1:3000");
    println!("\nFeatures:");
    println!("  ✓ Resources (file system)");
    println!("  ✓ Tools (calculator)");
    println!("  ✓ Prompts (code review)");
    println!("  ✓ CORS enabled");
    println!("  ✓ Custom headers support");
    println!("\nEndpoints:");
    println!("  POST http://127.0.0.1:3000/mcp - JSON-RPC requests");
    println!("  GET  http://127.0.0.1:3000/mcp - SSE stream");

    server.run("127.0.0.1:3000").await
}
