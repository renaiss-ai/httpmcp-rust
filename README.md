# httpmcp-rust

[![CI](https://github.com/renaiss-ai/httpmcp-rust/workflows/CI/badge.svg)](https://github.com/renaiss-ai/httpmcp-rust/actions)
[![Crates.io](https://img.shields.io/crates/v/httpmcp-rust.svg)](https://crates.io/crates/httpmcp-rust)
[![Documentation](https://docs.rs/httpmcp-rust/badge.svg)](https://docs.rs/httpmcp-rust)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

> ⚠️ **Beta Status**: This library is currently in beta. The API is still evolving and may have breaking changes. Not recommended for production use yet.

A **fast** and **simple** Rust library for building MCP (Model Context Protocol) servers using Streamable HTTP.

## Features

- ✅ **Simple API** - Function-based registration with builder pattern
- ✅ **Fast** - Built on actix-web with async/await
- ✅ **Type-safe** - Strong typing throughout
- ✅ **Extensible** - Easy to add custom resources, tools, and prompts
- ✅ **Full MCP Support** - All protocol features (resources, tools, prompts, logging)
- ✅ **Custom HTTP Endpoints** - Add REST API endpoints on the same port as MCP
- ✅ **Multipart File Uploads** - Handle file uploads with `.multipart_endpoint()`
- ✅ **Headers & Context** - Access request headers, remote IP, request ID
- ✅ **Middleware** - Built-in CORS and OAuth 2.0 configuration
- 🚧 **Beta Features** - SSE resumption, OAuth validation (in development)

## Quick Start

```toml
[dependencies]
httpmcp-rust = "0.1"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

```rust
use httpmcp_rust::{HttpMcpServer, RequestContext, ResourceMeta, ToolMeta, Result};
use httpmcp_rust::protocol::{Resource, ResourceContents};
use serde_json::{json, Value};
use std::collections::HashMap;

// Define handler functions
async fn list_resources(
    _cursor: Option<String>,
    _ctx: RequestContext,
) -> Result<(Vec<Resource>, Option<String>)> {
    Ok((vec![Resource {
        uri: "file:///example.txt".to_string(),
        name: "Example".to_string(),
        description: Some("Example file".to_string()),
        mime_type: Some("text/plain".to_string()),
    }], None))
}

async fn read_resource(uri: String, _ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    Ok(vec![ResourceContents {
        uri,
        mime_type: Some("text/plain".to_string()),
        text: Some("Hello, MCP!".to_string()),
        blob: None,
    }])
}

async fn echo_tool(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    Ok(json!({"echo": args.get("message")}))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpMcpServer::builder()
        .name("my-server")
        .version("1.0.0")
        // Register resource with metadata
        .resource(
            "file:///example.txt",
            ResourceMeta::new().name("Example").mime_type("text/plain"),
            list_resources,
            read_resource,
        )
        // Register tool with metadata
        .tool(
            "echo",
            ToolMeta::new()
                .description("Echo a message")
                .param("message", "string", "Message to echo")
                .required(&["message"]),
            echo_tool,
        )
        .build()?
        .run("127.0.0.1:8080")
        .await
}
```

## Usage Guide

### Server Builder

Configure your MCP server using the builder pattern:

```rust
let server = HttpMcpServer::builder()
    .name("my-server")           // Server name
    .version("1.0.0")             // Server version
    // Register resources with metadata
    .resource(
        "file:///example.txt",
        ResourceMeta::new().name("Example").mime_type("text/plain"),
        list_resources,
        read_resource,
    )
    // Register tools with metadata
    .tool(
        "add",
        ToolMeta::new()
            .description("Add two numbers")
            .param("a", "number", "First number")
            .param("b", "number", "Second number")
            .required(&["a", "b"]),
        add_tool,
    )
    // Register prompts with metadata
    .prompt(
        "code_review",
        PromptMeta::new()
            .description("Review code")
            .arg("code", "Code to review", true),
        code_review_prompt,
    )
    .enable_cors(true)            // Enable CORS
    .build()?;

server.run("127.0.0.1:8080").await?;
```

### Implementing Handlers

#### Resource Handlers

```rust
use httpmcp_rust::{RequestContext, ResourceMeta, Result};
use httpmcp_rust::protocol::{Resource, ResourceContents};

// List handler - returns available resources
async fn list_resources(
    _cursor: Option<String>,
    ctx: RequestContext,
) -> Result<(Vec<Resource>, Option<String>)> {
    // Access headers if needed
    let auth = ctx.get_authorization();

    Ok((vec![
        Resource {
            uri: "file:///example.txt".to_string(),
            name: "Example".to_string(),
            description: Some("Example file".to_string()),
            mime_type: Some("text/plain".to_string()),
        }
    ], None))
}

// Read handler - returns resource contents
async fn read_resource(
    uri: String,
    ctx: RequestContext,
) -> Result<Vec<ResourceContents>> {
    Ok(vec![ResourceContents {
        uri,
        mime_type: Some("text/plain".to_string()),
        text: Some("Hello, MCP!".to_string()),
        blob: None,
    }])
}
```

#### Tool Handlers

```rust
use httpmcp_rust::{RequestContext, ToolMeta, Result};
use std::collections::HashMap;
use serde_json::{json, Value};

async fn add_tool(
    args: HashMap<String, Value>,
    ctx: RequestContext,
) -> Result<Value> {
    let a = args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let b = args.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);

    Ok(json!({
        "result": a + b
    }))
}
```

#### Prompt Handlers

```rust
use httpmcp_rust::{RequestContext, PromptMeta, Result};
use httpmcp_rust::protocol::{PromptMessage, PromptContent};
use std::collections::HashMap;

async fn code_review_prompt(
    _name: String,
    args: Option<HashMap<String, String>>,
    ctx: RequestContext,
) -> Result<(Option<String>, Vec<PromptMessage>)> {
    let code = args.and_then(|mut a| a.remove("code")).unwrap_or_default();

    let messages = vec![PromptMessage {
        role: "user".to_string(),
        content: PromptContent::Text {
            text: format!("Review this code:\n\n{}", code),
        },
    }];

    Ok((Some("Code review".to_string()), messages))
}
```

#### Custom HTTP Endpoint Handlers (JSON)

Add REST API endpoints alongside MCP protocol on the same port:

```rust
use httpmcp_rust::{EndpointMeta, HttpMcpServer, RequestContext, Result};
use actix_web::HttpResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpMcpServer::builder()
        .name("my-server")
        .version("1.0.0")
        // Add custom GET endpoint
        .endpoint(
            EndpointMeta::new()
                .route("/health")
                .method("GET")
                .description("Health check endpoint"),
            |_ctx: RequestContext, _body| async move {
                Ok(HttpResponse::Ok().json(json!({
                    "status": "healthy",
                    "version": "1.0.0"
                })))
            },
        )
        // Add custom POST endpoint
        .endpoint(
            EndpointMeta::new()
                .route("/api/data")
                .method("POST")
                .description("Create data"),
            |_ctx: RequestContext, body| async move {
                Ok(HttpResponse::Created().json(json!({
                    "message": "Created successfully",
                    "data": body
                })))
            },
        )
        .build()?
        .run("127.0.0.1:8080")
        .await
}
```

#### Multipart File Upload Endpoints

Handle file uploads using multipart/form-data:

```rust
use httpmcp_rust::{EndpointMeta, HttpMcpServer, RequestContext};
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures::stream::StreamExt;
use serde_json::json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpMcpServer::builder()
        .name("upload-server")
        .version("1.0.0")
        .multipart_endpoint(
            EndpointMeta::new()
                .route("/upload")
                .method("POST")
                .description("Upload CSV file"),
            |_ctx: RequestContext, multipart: Multipart| {
                async move {
                    let mut multipart = multipart;
                    let mut file_contents = Vec::new();
                    let mut filename = String::from("unknown");

                    // Process multipart form fields
                    while let Some(field) = multipart.next().await {
                        let mut field = field.map_err(|e| {
                            httpmcp_rust::McpError::InvalidParams(format!("Multipart error: {}", e))
                        })?;

                        // Get filename
                        if let Some(content_disposition) = field.content_disposition() {
                            if let Some(fname) = content_disposition.get_filename() {
                                filename = fname.to_string();
                            }
                        }

                        // Read field data
                        while let Some(chunk) = field.next().await {
                            let data = chunk.map_err(|e| {
                                httpmcp_rust::McpError::InvalidParams(format!("Chunk error: {}", e))
                            })?;
                            file_contents.extend_from_slice(&data);
                        }
                    }

                    // Process file contents
                    let content = String::from_utf8(file_contents).map_err(|e| {
                        httpmcp_rust::McpError::InvalidParams(format!("Invalid UTF-8: {}", e))
                    })?;

                    Ok(HttpResponse::Ok().json(json!({
                        "success": true,
                        "filename": filename,
                        "size_bytes": content.len()
                    })))
                }
            },
        )
        .build()?
        .run("127.0.0.1:8080")
        .await
}
```

Test custom endpoints:

```bash
# Health check
curl http://localhost:8080/health

# POST JSON data
curl -X POST http://localhost:8080/api/data \
  -H "Content-Type: application/json" \
  -d '{"name": "test"}'

# Upload file
curl -X POST http://localhost:8080/upload \
  -F "file=@data.csv"

# MCP protocol still works on /mcp
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"ping"}'
```

### Request Context

Access headers and request metadata in all handlers:

```rust
async fn read_resource(uri: String, ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    // Get authorization header
    let auth = ctx.get_authorization();

    // Get bearer token
    let token = ctx.get_bearer_token();

    // Get custom headers
    let tenant = ctx.get_custom_header("x-tenant-id");

    // Access request metadata
    println!("Request ID: {}", ctx.request_id);
    println!("Method: {}", ctx.method);
    println!("Path: {}", ctx.path);
    println!("Remote: {:?}", ctx.remote_addr);

    // Your logic here
    Ok(vec![])
}
```

### Headers Example

Use headers for authentication, tenant isolation, or custom metadata:

```rust
use httpmcp_rust::{HttpMcpServer, RequestContext, ToolMeta, Result};
use serde_json::{json, Value};
use std::collections::HashMap;

async fn secure_tool(args: HashMap<String, Value>, ctx: RequestContext) -> Result<Value> {
    // Check authorization
    let token = ctx.get_bearer_token()
        .ok_or_else(|| httpmcp_rust::McpError::Unauthorized("Missing token".to_string()))?;

    // Validate token (pseudo-code)
    if !validate_token(token) {
        return Err(httpmcp_rust::McpError::Unauthorized("Invalid token".to_string()));
    }

    // Get tenant ID for multi-tenancy
    let tenant_id = ctx.get_custom_header("x-tenant-id")
        .unwrap_or("default");

    // Log request details
    tracing::info!(
        "Request {} from {:?} for tenant {}",
        ctx.request_id,
        ctx.remote_addr,
        tenant_id
    );

    // Your logic here
    Ok(json!({"status": "success"}))
}

fn validate_token(token: &str) -> bool {
    // Token validation logic
    !token.is_empty()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpMcpServer::builder()
        .name("secure-server")
        .version("1.0.0")
        .tool(
            "secure_tool",
            ToolMeta::new().description("Tool with auth"),
            secure_tool,
        )
        .build()?
        .run("127.0.0.1:8080")
        .await
}
```

### Middleware Configuration

Enable CORS and OAuth 2.0:

```rust
use httpmcp_rust::HttpMcpServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpMcpServer::builder()
        .name("middleware-example")
        .version("1.0.0")
        // Enable CORS for browser-based clients
        .enable_cors(true)
        // Configure OAuth 2.0 (basic setup)
        .with_oauth(
            "your-client-id",
            "your-client-secret",
            "https://auth.example.com/token",
            "https://auth.example.com/authorize",
        )
        .build()?;

    server.run("127.0.0.1:8080").await
}
```

Test with headers:

```bash
# Call with authorization
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-token" \
  -H "x-tenant-id: acme-corp" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "secure_tool",
      "arguments": {}
    }
  }'
```

## Examples

### Available Examples

**1. Simple Server** (`simple_server.rs`)
- Basic resources and tools
- Minimal setup

**2. Full Server** (`full_server.rs`)
- All MCP capabilities
- Custom headers
- File system resources
- Calculator tools
- Code review prompts

**3. Travel Planner** (`travel_planner.rs`) 🌟
- Real-world domain example
- Complete travel planning system
- Resources: destinations, itineraries, bookings, guides
- Tools: flight search, hotel search, weather, budget calculator, currency converter
- Prompts: trip planning, budget advice, packing lists
- Includes custom health endpoint

**4. Endpoint Example** (`endpoint_example.rs`)
- Custom HTTP REST API endpoints
- Health check, user list, data creation endpoints
- Shows how to mix MCP protocol with custom REST APIs on the same port

**5. Multipart Upload Example** (`multipart_upload.rs`)
- File upload handling with multipart/form-data
- CSV file processing and parsing
- Demonstrates `.multipart_endpoint()` usage

### Run Examples

```bash
# Simple server
cargo run --example simple_server

# Full-featured server
cargo run --example full_server

# Travel planner (recommended to see full capabilities)
cargo run --example travel_planner

# Endpoint example (custom REST API + MCP)
cargo run --example endpoint_example

# Multipart upload example
cargo run --example multipart_upload

# Test travel planner with automated suite
./examples/travel_planner_test.sh
```

### Testing with curl

```bash
# Initialize connection
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
      "protocolVersion": "2024-11-05",
      "capabilities": {},
      "clientInfo": {"name": "test", "version": "1.0"}
    }
  }'

# List resources
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "resources/list"
  }'

# Call a tool
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 3,
    "method": "tools/call",
    "params": {
      "name": "echo",
      "arguments": {"message": "Hello"}
    }
  }'

# SSE stream
curl -N http://localhost:8080/mcp \
  -H "Accept: text/event-stream"
```

## Architecture

```
httpmcp-rust/
├── src/
│   ├── lib.rs              # Public API
│   ├── server.rs           # HttpMcpServer builder
│   ├── transport.rs        # HTTP + SSE handlers
│   ├── jsonrpc.rs          # JSON-RPC types
│   ├── protocol.rs         # MCP protocol types
│   ├── context.rs          # RequestContext
│   ├── error.rs            # Error handling
│   ├── handlers/           # Trait definitions
│   │   ├── resources.rs
│   │   ├── tools.rs
│   │   ├── prompts.rs
│   │   └── lifecycle.rs
│   ├── auth/               # OAuth 2.0
│   ├── sse/                # Server-Sent Events
│   └── middleware/         # CORS, validation
└── examples/
    ├── simple_server.rs
    └── full_server.rs
```

## Features

### ✅ Completed

- JSON-RPC 2.0 support
- All MCP protocol methods (resources, tools, prompts)
- HTTP POST endpoint
- SSE GET endpoint with event IDs
- RequestContext with headers access
- OAuth 2.0 configuration
- CORS middleware
- Request validation
- Type-safe error handling
- Comprehensive examples

### 🚧 TODO

- Full OAuth token validation
- SSE resumption logic
- Rate limiting
- Metrics/observability
- More examples
- Integration tests

## MCP Protocol Support

This library implements the [Model Context Protocol](https://modelcontextprotocol.io) specification:

- ✅ Initialization & lifecycle
- ✅ Resources (list, read, templates, subscribe)
- ✅ Tools (list, call)
- ✅ Prompts (list, get)
- ✅ Logging (setLevel)
- ✅ Ping/pong
- ✅ JSON-RPC 2.0
- ✅ HTTP with SSE transport

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Development

```bash
# Clone the repository
git clone https://github.com/renaiss-ai/httpmcp-rust.git
cd httpmcp-rust

# Run tests
cargo test

# Run examples
cargo run --example simple_server
cargo run --example full_server
cargo run --example travel_planner

# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Resources

- [Model Context Protocol Specification](https://modelcontextprotocol.io)
- [Documentation](https://docs.rs/httpmcp-rust)
- [Crates.io](https://crates.io/crates/httpmcp-rust)
- [Examples](examples/)
- [Changelog](CHANGELOG.md)
