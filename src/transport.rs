use crate::context::RequestContext;
use crate::error::{McpError, Result};
use crate::handlers::lifecycle::{handle_initialize, handle_ping};
use crate::jsonrpc::{JsonRpcRequest, JsonRpcResponse};
use crate::protocol::*;
use crate::server::HttpMcpServer;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};
use actix_web_lab::sse;
use futures::stream;
use serde_json::Value;
use std::sync::Arc;

/// Configure actix-web application
pub fn create_app(cfg: &mut web::ServiceConfig, server: Arc<HttpMcpServer>) {
    if server.enable_cors {
        cfg.default_service(web::to(|| async {
            HttpResponse::Ok()
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .insert_header(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
                .insert_header((
                    "Access-Control-Allow-Headers",
                    "Content-Type, Authorization, Accept, Last-Event-ID",
                ))
                .finish()
        }));
    }

    cfg.app_data(Data::new(server.clone()))
        .service(handle_post)
        .service(handle_get);
}

/// POST /mcp - Handle JSON-RPC requests
#[post("/mcp")]
async fn handle_post(
    req: HttpRequest,
    body: web::Json<JsonRpcRequest>,
    server: Data<Arc<HttpMcpServer>>,
) -> Result<impl Responder> {
    let ctx = create_request_context(&req);

    // Validate OAuth if configured
    if let Some(oauth) = &server.oauth_config {
        oauth.validate_token(&ctx).await?;
    }

    // Validate JSON-RPC request
    body.validate()?;

    let response = route_request(&body, &ctx, &server).await?;

    Ok(HttpResponse::Ok().json(response))
}

/// GET /mcp - SSE stream for server-to-client messages
#[get("/mcp")]
async fn handle_get(req: HttpRequest, server: Data<Arc<HttpMcpServer>>) -> Result<impl Responder> {
    let ctx = create_request_context(&req);

    // Validate OAuth if configured
    if let Some(oauth) = &server.oauth_config {
        oauth.validate_token(&ctx).await?;
    }

    // Check for Last-Event-ID header for resumption
    let _last_event_id = req
        .headers()
        .get("Last-Event-ID")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Create SSE stream
    let event_stream = stream::iter(vec![Ok::<_, actix_web::Error>(sse::Event::Data(
        sse::Data::new("SSE stream connected").event("message"),
    ))]);

    Ok(sse::Sse::from_stream(event_stream))
}

/// Route JSON-RPC request to appropriate handler
async fn route_request(
    req: &JsonRpcRequest,
    ctx: &RequestContext,
    server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    tracing::debug!("Routing request: method={}", req.method);

    match req.method.as_str() {
        // Lifecycle
        "initialize" => {
            handle_initialize(req, server.server_info.clone(), server.capabilities.clone())
        }
        "ping" => handle_ping(req),

        // Resources
        "resources/list" => handle_resources_list(req, ctx, server).await,
        "resources/read" => handle_resources_read(req, ctx, server).await,
        "resources/templates/list" => handle_resources_templates(req, ctx, server).await,
        "resources/subscribe" => handle_resources_subscribe(req, ctx, server).await,

        // Tools
        "tools/list" => handle_tools_list(req, ctx, server).await,
        "tools/call" => handle_tools_call(req, ctx, server).await,

        // Prompts
        "prompts/list" => handle_prompts_list(req, ctx, server).await,
        "prompts/get" => handle_prompts_get(req, ctx, server).await,

        // Logging
        "logging/setLevel" => handle_logging_set_level(req),

        _ => Err(McpError::MethodNotFound(req.method.clone())),
    }
}

// ============================================================================
// Resource Handlers
// ============================================================================

async fn handle_resources_list(
    req: &JsonRpcRequest,
    ctx: &RequestContext,
    server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    let params: ResourcesListParams =
        serde_json::from_value(req.params.clone().unwrap_or(Value::Null))
            .unwrap_or(ResourcesListParams { cursor: None });

    // Collect all resources from registered handlers
    let mut all_resources = Vec::new();
    for registered in server.resources.values() {
        let (resources, _) = (registered.list_handler)(params.cursor.clone(), ctx.clone()).await?;
        all_resources.extend(resources);
    }

    let result = ResourcesListResult {
        resources: all_resources,
        next_cursor: None,
    };

    Ok(JsonRpcResponse::success(
        serde_json::to_value(result)?,
        req.id.clone(),
    ))
}

async fn handle_resources_read(
    req: &JsonRpcRequest,
    ctx: &RequestContext,
    server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    let params: ResourcesReadParams =
        serde_json::from_value(req.params.clone().unwrap_or(Value::Null))
            .map_err(|e| McpError::InvalidParams(format!("Invalid params: {}", e)))?;

    // Try to find matching resource handler
    let mut contents = Vec::new();
    for registered in server.resources.values() {
        let result = (registered.read_handler)(params.uri.clone(), ctx.clone()).await?;
        contents.extend(result);
    }

    if contents.is_empty() {
        return Err(McpError::ResourceNotFound(params.uri));
    }

    let result = ResourcesReadResult { contents };

    Ok(JsonRpcResponse::success(
        serde_json::to_value(result)?,
        req.id.clone(),
    ))
}

async fn handle_resources_templates(
    req: &JsonRpcRequest,
    _ctx: &RequestContext,
    _server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    // Resource templates are not supported in the new function-based API
    Ok(JsonRpcResponse::success(
        serde_json::json!({ "resourceTemplates": [] }),
        req.id.clone(),
    ))
}

async fn handle_resources_subscribe(
    req: &JsonRpcRequest,
    _ctx: &RequestContext,
    _server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    // Resource subscription is not supported in the new function-based API
    Ok(JsonRpcResponse::success(Value::Null, req.id.clone()))
}

// ============================================================================
// Tool Handlers
// ============================================================================

async fn handle_tools_list(
    req: &JsonRpcRequest,
    _ctx: &RequestContext,
    server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    // Collect all registered tools
    let tools: Vec<Tool> = server
        .tools
        .values()
        .map(|registered| registered.meta.clone())
        .collect();

    let result = ToolsListResult {
        tools,
        next_cursor: None,
    };

    Ok(JsonRpcResponse::success(
        serde_json::to_value(result)?,
        req.id.clone(),
    ))
}

async fn handle_tools_call(
    req: &JsonRpcRequest,
    ctx: &RequestContext,
    server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    let params: ToolsCallParams = serde_json::from_value(req.params.clone().unwrap_or(Value::Null))
        .map_err(|e| McpError::InvalidParams(format!("Invalid params: {}", e)))?;

    // Find the registered tool
    let registered = server
        .tools
        .get(&params.name)
        .ok_or_else(|| McpError::ToolNotFound(params.name.clone()))?;

    // Call the tool handler
    let result_value =
        (registered.handler)(params.arguments.unwrap_or_default(), ctx.clone()).await?;

    // Convert result to ToolContent
    let content = vec![ToolContent::Text {
        text: result_value.to_string(),
    }];

    let result = ToolsCallResult {
        content,
        is_error: None,
    };

    Ok(JsonRpcResponse::success(
        serde_json::to_value(result)?,
        req.id.clone(),
    ))
}

// ============================================================================
// Prompt Handlers
// ============================================================================

async fn handle_prompts_list(
    req: &JsonRpcRequest,
    _ctx: &RequestContext,
    server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    // Collect all registered prompts
    let prompts: Vec<Prompt> = server
        .prompts
        .values()
        .map(|registered| registered.meta.clone())
        .collect();

    let result = PromptsListResult {
        prompts,
        next_cursor: None,
    };

    Ok(JsonRpcResponse::success(
        serde_json::to_value(result)?,
        req.id.clone(),
    ))
}

async fn handle_prompts_get(
    req: &JsonRpcRequest,
    ctx: &RequestContext,
    server: &HttpMcpServer,
) -> Result<JsonRpcResponse> {
    let params: PromptsGetParams =
        serde_json::from_value(req.params.clone().unwrap_or(Value::Null))
            .map_err(|e| McpError::InvalidParams(format!("Invalid params: {}", e)))?;

    // Find the registered prompt
    let registered = server
        .prompts
        .get(&params.name)
        .ok_or_else(|| McpError::PromptNotFound(params.name.clone()))?;

    // Call the prompt handler
    let (description, messages) =
        (registered.handler)(params.name.clone(), params.arguments, ctx.clone()).await?;

    let result = PromptsGetResult {
        description,
        messages,
    };

    Ok(JsonRpcResponse::success(
        serde_json::to_value(result)?,
        req.id.clone(),
    ))
}

// ============================================================================
// Logging Handlers
// ============================================================================

fn handle_logging_set_level(req: &JsonRpcRequest) -> Result<JsonRpcResponse> {
    let _params: LoggingSetLevelParams =
        serde_json::from_value(req.params.clone().unwrap_or(Value::Null))
            .map_err(|e| McpError::InvalidParams(format!("Invalid params: {}", e)))?;

    // TODO: Implement actual log level setting
    Ok(JsonRpcResponse::success(Value::Null, req.id.clone()))
}

// ============================================================================
// Utilities
// ============================================================================

fn create_request_context(req: &HttpRequest) -> RequestContext {
    RequestContext::new(
        req.headers().clone(),
        req.method().to_string(),
        req.path().to_string(),
        req.peer_addr(),
    )
}
