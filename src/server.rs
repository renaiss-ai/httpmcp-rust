use crate::auth::OAuthConfig;
use crate::handler_types::{RegisteredPrompt, RegisteredResource, RegisteredTool};
use crate::jsonrpc::JsonRpcResponse;
use crate::metadata::{PromptMeta, ResourceMeta, ToolMeta};
use crate::protocol::{Implementation, ServerCapabilities};
use crate::transport::create_app;
use actix_web::{middleware::Logger, App, HttpServer};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Main HTTP MCP Server
pub struct HttpMcpServer {
    pub(crate) server_info: Implementation,
    pub(crate) capabilities: ServerCapabilities,
    pub(crate) tools: HashMap<String, RegisteredTool>,
    pub(crate) resources: HashMap<String, RegisteredResource>,
    pub(crate) prompts: HashMap<String, RegisteredPrompt>,
    pub(crate) oauth_config: Option<OAuthConfig>,
    pub(crate) enable_cors: bool,
    pub(crate) response_tx: broadcast::Sender<JsonRpcResponse>,
}

impl HttpMcpServer {
    /// Create a new server builder
    pub fn builder() -> HttpMcpServerBuilder {
        HttpMcpServerBuilder::new()
    }

    /// Run the server on the specified address
    pub async fn run(self, addr: impl Into<String>) -> std::io::Result<()> {
        let addr = addr.into();
        let server = Arc::new(self);

        tracing::info!("Starting MCP server on {}", addr);

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .configure(|cfg| create_app(cfg, server.clone()))
        })
        .bind(&addr)?
        .run()
        .await
    }
}

/// Builder for HttpMcpServer
pub struct HttpMcpServerBuilder {
    name: String,
    version: String,
    tools: HashMap<String, RegisteredTool>,
    resources: HashMap<String, RegisteredResource>,
    prompts: HashMap<String, RegisteredPrompt>,
    oauth_config: Option<OAuthConfig>,
    enable_cors: bool,
}

impl HttpMcpServerBuilder {
    pub fn new() -> Self {
        Self {
            name: "httpmcp-server".to_string(),
            version: "1.0.0".to_string(),
            tools: HashMap::new(),
            resources: HashMap::new(),
            prompts: HashMap::new(),
            oauth_config: None,
            enable_cors: true,
        }
    }

    /// Set server name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set server version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Register a tool with handler
    pub fn tool<F, Fut>(mut self, name: impl Into<String>, meta: ToolMeta, handler: F) -> Self
    where
        F: Fn(HashMap<String, serde_json::Value>, crate::context::RequestContext) -> Fut
            + Send
            + Sync
            + 'static,
        Fut: std::future::Future<Output = crate::error::Result<serde_json::Value>> + Send + 'static,
    {
        let name_str = name.into();
        let tool = RegisteredTool {
            meta: meta.to_tool(name_str.clone()),
            handler: Box::new(move |args, ctx| Box::pin(handler(args, ctx))),
        };
        self.tools.insert(name_str, tool);
        self
    }

    /// Register a resource with list and read handlers
    pub fn resource<FL, FR, FutL, FutR>(
        mut self,
        uri: impl Into<String>,
        meta: ResourceMeta,
        list_handler: FL,
        read_handler: FR,
    ) -> Self
    where
        FL: Fn(Option<String>, crate::context::RequestContext) -> FutL + Send + Sync + 'static,
        FutL: std::future::Future<
                Output = crate::error::Result<(Vec<crate::protocol::Resource>, Option<String>)>,
            > + Send
            + 'static,
        FR: Fn(String, crate::context::RequestContext) -> FutR + Send + Sync + 'static,
        FutR: std::future::Future<
                Output = crate::error::Result<Vec<crate::protocol::ResourceContents>>,
            > + Send
            + 'static,
    {
        let uri_str = uri.into();
        let resource = RegisteredResource {
            meta: meta.to_resource(uri_str.clone()),
            list_handler: Box::new(move |cursor, ctx| Box::pin(list_handler(cursor, ctx))),
            read_handler: Box::new(move |uri, ctx| Box::pin(read_handler(uri, ctx))),
        };
        self.resources.insert(uri_str, resource);
        self
    }

    /// Register a prompt with handler
    pub fn prompt<F, Fut>(mut self, name: impl Into<String>, meta: PromptMeta, handler: F) -> Self
    where
        F: Fn(String, Option<HashMap<String, String>>, crate::context::RequestContext) -> Fut
            + Send
            + Sync
            + 'static,
        Fut: std::future::Future<
                Output = crate::error::Result<(
                    Option<String>,
                    Vec<crate::protocol::PromptMessage>,
                )>,
            > + Send
            + 'static,
    {
        let name_str = name.into();
        let prompt = RegisteredPrompt {
            meta: meta.to_prompt(name_str.clone()),
            handler: Box::new(move |name, args, ctx| Box::pin(handler(name, args, ctx))),
        };
        self.prompts.insert(name_str, prompt);
        self
    }

    /// Configure OAuth 2.0
    pub fn with_oauth(
        mut self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        _token_url: impl Into<String>,
        _auth_url: impl Into<String>,
    ) -> Self {
        self.oauth_config = Some(OAuthConfig {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        });
        self
    }

    /// Enable or disable CORS
    pub fn enable_cors(mut self, enable: bool) -> Self {
        self.enable_cors = enable;
        self
    }

    /// Build the server
    pub fn build(self) -> crate::error::Result<HttpMcpServer> {
        let capabilities = ServerCapabilities {
            logging: Some(Default::default()),
            prompts: if self.prompts.is_empty() {
                None
            } else {
                Some(Default::default())
            },
            resources: if self.resources.is_empty() {
                None
            } else {
                Some(Default::default())
            },
            tools: if self.tools.is_empty() {
                None
            } else {
                Some(Default::default())
            },
        };

        // Create broadcast channel for SSE responses
        let (response_tx, _) = broadcast::channel(100);

        Ok(HttpMcpServer {
            server_info: Implementation {
                name: self.name,
                version: self.version,
            },
            capabilities,
            tools: self.tools,
            resources: self.resources,
            prompts: self.prompts,
            oauth_config: self.oauth_config,
            enable_cors: self.enable_cors,
            response_tx,
        })
    }
}

impl Default for HttpMcpServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
