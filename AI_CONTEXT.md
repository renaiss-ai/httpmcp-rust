# AI Context for httpmcp-rust

This document provides comprehensive information about the httpmcp-rust library for AI assistants to understand and help users implement MCP servers.

## Library Overview

**httpmcp-rust** is a Rust library for building MCP (Model Context Protocol) servers using Streamable HTTP transport. It provides a simple, function-based API built on actix-web.

## Core Concepts

### 1. Server Creation

Create an MCP server using the builder pattern:

```rust
use httpmcp_rust::HttpMcpServer;

let server = HttpMcpServer::builder()
    .name("my-server")
    .version("1.0.0")
    .build()
    .expect("Failed to build server");

server.run("127.0.0.1:8080").await
```

### 2. Architecture

The library uses a **function-based registration** approach (not trait-based). Users register async functions directly with the server builder.

**Key Types:**
- `HttpMcpServer` - Main server struct
- `HttpMcpServerBuilder` - Builder for configuring the server
- `RequestContext` - Context passed to all handlers (contains headers, request metadata)
- `ResourceMeta`, `ToolMeta`, `PromptMeta` - Metadata builders

## Handler Functions

### Resource Handlers

Resources require TWO functions: a list handler and a read handler.

```rust
use httpmcp_rust::{RequestContext, Result};
use httpmcp_rust::protocol::{Resource, ResourceContents};

// List handler - returns available resources
async fn list_resources(
    cursor: Option<String>,    // For pagination
    ctx: RequestContext,       // Request context
) -> Result<(Vec<Resource>, Option<String>)> {
    Ok((vec![
        Resource {
            uri: "file:///example.txt".to_string(),
            name: "Example".to_string(),
            description: Some("Example file".to_string()),
            mime_type: Some("text/plain".to_string()),
        }
    ], None))  // Return (resources, next_cursor)
}

// Read handler - returns resource contents
async fn read_resource(
    uri: String,              // Resource URI
    ctx: RequestContext,      // Request context
) -> Result<Vec<ResourceContents>> {
    Ok(vec![ResourceContents {
        uri,
        mime_type: Some("text/plain".to_string()),
        text: Some("Hello, World!".to_string()),
        blob: None,
    }])
}
```

**Registration:**

```rust
use httpmcp_rust::ResourceMeta;

let server = HttpMcpServer::builder()
    .resource(
        "file:///example.txt",  // URI pattern
        ResourceMeta::new()
            .name("Example")
            .description("Example file")
            .mime_type("text/plain"),
        list_resources,   // List handler
        read_resource,    // Read handler
    )
    .build()?;
```

### Tool Handlers

Tools are callable functions that perform actions.

```rust
use serde_json::{json, Value};
use std::collections::HashMap;

async fn my_tool(
    args: HashMap<String, Value>,  // Tool arguments
    ctx: RequestContext,           // Request context
) -> Result<Value> {
    let message = args
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("default");

    Ok(json!({
        "result": message
    }))
}
```

**Registration:**

```rust
use httpmcp_rust::ToolMeta;

let server = HttpMcpServer::builder()
    .tool(
        "my_tool",           // Tool name
        ToolMeta::new()
            .description("Does something")
            .param("message", "string", "The message")
            .param("count", "number", "How many times")
            .required(&["message"]),  // Required parameters
        my_tool,             // Handler function
    )
    .build()?;
```

### Prompt Handlers

Prompts return formatted messages for AI models.

```rust
use httpmcp_rust::protocol::{PromptMessage, PromptContent};

async fn my_prompt(
    name: String,                           // Prompt name
    args: Option<HashMap<String, String>>,  // Prompt arguments
    ctx: RequestContext,                    // Request context
) -> Result<(Option<String>, Vec<PromptMessage>)> {
    let input = args
        .and_then(|mut a| a.remove("input"))
        .unwrap_or_default();

    Ok((
        Some("Description".to_string()),  // Optional description
        vec![PromptMessage {
            role: "user".to_string(),
            content: PromptContent::Text {
                text: format!("Process this: {}", input),
            },
        }]
    ))
}
```

**Registration:**

```rust
use httpmcp_rust::PromptMeta;

let server = HttpMcpServer::builder()
    .prompt(
        "my_prompt",         // Prompt name
        PromptMeta::new()
            .description("Process input")
            .arg("input", "Input to process", true),  // (name, description, required)
        my_prompt,           // Handler function
    )
    .build()?;
```

## Request Context

The `RequestContext` provides access to request metadata and headers:

```rust
async fn my_handler(ctx: RequestContext) -> Result<Value> {
    // Get authorization header
    let auth = ctx.get_authorization();

    // Get bearer token
    let token = ctx.get_bearer_token();

    // Get custom headers
    let tenant_id = ctx.get_custom_header("x-tenant-id");

    // Access request metadata
    println!("Request ID: {}", ctx.request_id);
    println!("Method: {}", ctx.method);
    println!("Path: {}", ctx.path);
    println!("Remote: {:?}", ctx.remote_addr);

    Ok(json!({"status": "ok"}))
}
```

## Middleware & Configuration

### CORS

```rust
let server = HttpMcpServer::builder()
    .enable_cors(true)  // Enable CORS (default: true)
    .build()?;
```

### OAuth 2.0

```rust
let server = HttpMcpServer::builder()
    .with_oauth(
        "client_id",
        "client_secret",
        "https://auth.example.com/token",
        "https://auth.example.com/authorize",
    )
    .build()?;
```

## Complete Example

```rust
use httpmcp_rust::{HttpMcpServer, RequestContext, ResourceMeta, ToolMeta, Result};
use httpmcp_rust::protocol::{Resource, ResourceContents};
use serde_json::{json, Value};
use std::collections::HashMap;

// Resource handlers
async fn list_files(
    _cursor: Option<String>,
    _ctx: RequestContext,
) -> Result<(Vec<Resource>, Option<String>)> {
    Ok((vec![
        Resource {
            uri: "file:///data.json".to_string(),
            name: "Data File".to_string(),
            description: Some("Application data".to_string()),
            mime_type: Some("application/json".to_string()),
        }
    ], None))
}

async fn read_file(uri: String, _ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    let content = match uri.as_str() {
        "file:///data.json" => json!({"key": "value"}).to_string(),
        _ => return Err(httpmcp_rust::McpError::ResourceNotFound(uri)),
    };

    Ok(vec![ResourceContents {
        uri,
        mime_type: Some("application/json".to_string()),
        text: Some(content),
        blob: None,
    }])
}

// Tool handler
async fn calculate(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let a = args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let b = args.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let op = args.get("operation").and_then(|v| v.as_str()).unwrap_or("add");

    let result = match op {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" if b != 0.0 => a / b,
        _ => 0.0,
    };

    Ok(json!({"result": result}))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpMcpServer::builder()
        .name("example-server")
        .version("1.0.0")
        // Register resource
        .resource(
            "file:///data.json",
            ResourceMeta::new()
                .name("Data")
                .mime_type("application/json"),
            list_files,
            read_file,
        )
        // Register tool
        .tool(
            "calculate",
            ToolMeta::new()
                .description("Perform calculations")
                .param("a", "number", "First number")
                .param("b", "number", "Second number")
                .param("operation", "string", "Operation: add, subtract, multiply, divide")
                .required(&["a", "b", "operation"]),
            calculate,
        )
        .enable_cors(true)
        .build()
        .expect("Failed to build server");

    println!("Server running on http://127.0.0.1:8080");
    server.run("127.0.0.1:8080").await
}
```

## Common Patterns

### Error Handling

Use the `McpError` enum for errors:

```rust
use httpmcp_rust::McpError;

async fn my_handler(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let value = args
        .get("required_field")
        .ok_or_else(|| McpError::InvalidParams("Missing required_field".to_string()))?;

    Ok(json!({"status": "ok"}))
}
```

Available error types:
- `McpError::InvalidParams(String)`
- `McpError::ResourceNotFound(String)`
- `McpError::ToolNotFound(String)`
- `McpError::PromptNotFound(String)`
- `McpError::Unauthorized(String)`
- `McpError::InternalError(String)`

### Authentication

```rust
async fn secure_tool(args: HashMap<String, Value>, ctx: RequestContext) -> Result<Value> {
    // Check for bearer token
    let token = ctx
        .get_bearer_token()
        .ok_or_else(|| McpError::Unauthorized("Missing token".to_string()))?;

    // Validate token
    if !is_valid_token(&token) {
        return Err(McpError::Unauthorized("Invalid token".to_string()));
    }

    // Process request
    Ok(json!({"status": "authenticated"}))
}
```

### Multi-tenancy

```rust
async fn tenant_resource(uri: String, ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    let tenant_id = ctx
        .get_custom_header("x-tenant-id")
        .unwrap_or("default".to_string());

    // Load tenant-specific data
    let data = load_tenant_data(&tenant_id, &uri)?;

    Ok(vec![ResourceContents {
        uri,
        mime_type: Some("application/json".to_string()),
        text: Some(data),
        blob: None,
    }])
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::HeaderMap;

    #[tokio::test]
    async fn test_my_tool() {
        let ctx = RequestContext {
            request_id: "test-123".to_string(),
            method: "POST".to_string(),
            path: "/mcp".to_string(),
            remote_addr: None,
            headers: HeaderMap::new(),
        };

        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("Hello"));

        let result = my_tool(args, ctx).await;
        assert!(result.is_ok());
    }
}
```

## Dependencies

```toml
[dependencies]
httpmcp-rust = "0.1"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

## Important Notes for AI Assistants

1. **Handler signatures are strict** - Match the exact signatures shown above
2. **Resources need TWO functions** - Always provide both list and read handlers
3. **Use RequestContext correctly** - It's passed by value, clone if needed
4. **Metadata builders are required** - Always use `ToolMeta`, `ResourceMeta`, `PromptMeta`
5. **Result type** - All handlers return `Result<T>` which is `httpmcp_rust::Result<T>`
6. **Async handlers** - All handler functions must be `async fn`
7. **Builder pattern** - Server is built using `.builder()...build()`
8. **Function registration** - Not trait-based, functions are registered directly

## MCP Protocol

The library implements the Model Context Protocol:
- **Resources** - Read-only data (files, APIs, databases)
- **Tools** - Actions that can be called (calculations, API calls, mutations)
- **Prompts** - Template messages for AI models
- **Transport** - HTTP POST (JSON-RPC) and GET (SSE streaming)

## Common Mistakes to Avoid

❌ **Wrong**: Using traits
```rust
struct MyProvider;
impl ResourceProvider for MyProvider { ... }  // Don't do this
```

✅ **Correct**: Using functions
```rust
async fn my_handler(...) -> Result<...> { ... }  // Do this
```

❌ **Wrong**: Missing read handler
```rust
.resource("uri", meta, list_handler)  // Missing read handler
```

✅ **Correct**: Both handlers
```rust
.resource("uri", meta, list_handler, read_handler)  // Both handlers
```

❌ **Wrong**: Wrong Result type
```rust
async fn handler() -> std::io::Result<Value> { ... }
```

✅ **Correct**: Library Result type
```rust
async fn handler() -> Result<Value> { ... }  // httpmcp_rust::Result
```
