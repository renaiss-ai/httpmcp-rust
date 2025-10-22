use actix_web::HttpResponse;
use httpmcp_rust::{EndpointMeta, HttpMcpServer, RequestContext};
use serde_json::json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let server = HttpMcpServer::builder()
        .name("endpoint-example")
        .version("1.0.0")
        // GET endpoint
        .endpoint(
            EndpointMeta::new()
                .route("/api/health")
                .method("GET")
                .description("Health check endpoint"),
            |_ctx: RequestContext, _body| async move {
                Ok(HttpResponse::Ok().json(json!({
                    "status": "healthy"
                })))
            },
        )
        // GET endpoint with path
        .endpoint(
            EndpointMeta::new()
                .route("/api/users")
                .method("GET")
                .description("List all users"),
            |_ctx: RequestContext, _body| async move {
                Ok(HttpResponse::Ok().json(json!({
                    "users": [
                        {"id": 1, "name": "Alice"},
                        {"id": 2, "name": "Bob"}
                    ]
                })))
            },
        )
        // POST endpoint
        .endpoint(
            EndpointMeta::new()
                .route("/api/data")
                .method("POST")
                .description("Create data"),
            |_ctx: RequestContext, body| async move {
                println!("Received data: {:?}", body);
                Ok(HttpResponse::Created().json(json!({
                    "message": "Data created successfully",
                    "data": body
                })))
            },
        )
        .build()
        .expect("Failed to build server");

    println!("Server starting on http://127.0.0.1:3000");
    println!("Try:");
    println!("  curl http://127.0.0.1:3000/api/health");
    println!("  curl http://127.0.0.1:3000/api/users");
    println!("  curl -X POST http://127.0.0.1:3000/api/data -H 'Content-Type: application/json' -d '{{\"foo\":\"bar\"}}'");

    server.run("127.0.0.1:3000").await
}
