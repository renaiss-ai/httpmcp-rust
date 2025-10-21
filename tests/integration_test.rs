use httpmcp_rust::protocol::{Resource, ResourceContents};
use httpmcp_rust::{HttpMcpServer, RequestContext, ResourceMeta, Result, ToolMeta};
use serde_json::{json, Value};
use std::collections::HashMap;

// Test resource handlers
async fn test_list_resources(
    _cursor: Option<String>,
    _ctx: RequestContext,
) -> Result<(Vec<Resource>, Option<String>)> {
    Ok((
        vec![Resource {
            uri: "test://resource".to_string(),
            name: "Test Resource".to_string(),
            description: Some("A test resource".to_string()),
            mime_type: Some("text/plain".to_string()),
        }],
        None,
    ))
}

async fn test_read_resource(_uri: String, _ctx: RequestContext) -> Result<Vec<ResourceContents>> {
    Ok(vec![ResourceContents {
        uri: "test://resource".to_string(),
        mime_type: Some("text/plain".to_string()),
        text: Some("Test content".to_string()),
        blob: None,
    }])
}

// Test tool handler
async fn test_tool(args: HashMap<String, Value>, _ctx: RequestContext) -> Result<Value> {
    let message = args
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    Ok(json!({"result": message}))
}

#[test]
fn test_server_builder() {
    let result = HttpMcpServer::builder()
        .name("test-server")
        .version("0.1.0")
        .resource(
            "test://resource",
            ResourceMeta::new()
                .name("Test Resource")
                .mime_type("text/plain"),
            test_list_resources,
            test_read_resource,
        )
        .tool(
            "test_tool",
            ToolMeta::new()
                .description("Test tool")
                .param("message", "string", "Test message")
                .required(&["message"]),
            test_tool,
        )
        .enable_cors(true)
        .build();

    assert!(result.is_ok(), "Server should build successfully");
}

#[test]
fn test_metadata_builders() {
    // Test ResourceMeta
    let resource_meta = ResourceMeta::new()
        .name("Test")
        .description("Test description")
        .mime_type("application/json");
    let resource = resource_meta.to_resource("test://uri");
    assert_eq!(resource.uri, "test://uri");
    assert_eq!(resource.name, "Test");
    assert_eq!(resource.description, Some("Test description".to_string()));
    assert_eq!(resource.mime_type, Some("application/json".to_string()));

    // Test ToolMeta
    let tool_meta = ToolMeta::new()
        .description("Test tool")
        .param("arg1", "string", "Argument 1")
        .param("arg2", "number", "Argument 2")
        .required(&["arg1"]);
    let tool = tool_meta.to_tool("test_tool");
    assert_eq!(tool.name, "test_tool");
    assert_eq!(tool.description, Some("Test tool".to_string()));
    assert!(tool.input_schema["properties"].is_object());
    assert_eq!(tool.input_schema["required"][0], "arg1");
}

#[tokio::test]
async fn test_resource_handlers() {
    use actix_web::http::header::HeaderMap;

    let ctx = RequestContext {
        request_id: "test-123".to_string(),
        method: "POST".to_string(),
        path: "/mcp".to_string(),
        remote_addr: None,
        headers: HeaderMap::new(),
    };

    // Test list
    let result = test_list_resources(None, ctx.clone()).await;
    assert!(result.is_ok());
    let (resources, cursor) = result.unwrap();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].uri, "test://resource");
    assert!(cursor.is_none());

    // Test read
    let result = test_read_resource("test://resource".to_string(), ctx).await;
    assert!(result.is_ok());
    let contents = result.unwrap();
    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0].uri, "test://resource");
    assert_eq!(contents[0].text, Some("Test content".to_string()));
}

#[tokio::test]
async fn test_tool_handler() {
    use actix_web::http::header::HeaderMap;

    let ctx = RequestContext {
        request_id: "test-123".to_string(),
        method: "POST".to_string(),
        path: "/mcp".to_string(),
        remote_addr: None,
        headers: HeaderMap::new(),
    };

    let mut args = HashMap::new();
    args.insert("message".to_string(), json!("Hello"));

    let result = test_tool(args, ctx).await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["result"], "Hello");
}
