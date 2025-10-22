use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures::stream::StreamExt;
use httpmcp_rust::{EndpointMeta, HttpMcpServer, RequestContext};
use serde_json::json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let server = HttpMcpServer::builder()
        .name("multipart-upload-server")
        .version("1.0.0")
        // Multipart file upload endpoint
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

                        // Get field name and filename
                        if let Some(content_disposition) = field.content_disposition() {
                            if let Some(name) = content_disposition.get_name() {
                                println!("Field name: {}", name);
                            }
                            if let Some(fname) = content_disposition.get_filename() {
                                filename = fname.to_string();
                                println!("Filename: {}", filename);
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

                    // Convert bytes to string (assuming text file like CSV)
                    let content = String::from_utf8(file_contents).map_err(|e| {
                        httpmcp_rust::McpError::InvalidParams(format!("Invalid UTF-8: {}", e))
                    })?;

                    println!("Received file: {} ({} bytes)", filename, content.len());
                    println!("Content preview: {}", &content[..content.len().min(100)]);

                    // Parse CSV (simple example)
                    let lines: Vec<&str> = content.lines().collect();
                    let row_count = lines.len();
                    let column_count = lines
                        .first()
                        .map(|line| line.split(',').count())
                        .unwrap_or(0);

                    Ok(HttpResponse::Ok().json(json!({
                        "success": true,
                        "filename": filename,
                        "size_bytes": content.len(),
                        "rows": row_count,
                        "columns": column_count,
                        "preview": lines.iter().take(3).collect::<Vec<_>>()
                    })))
                }
            },
        )
        // Regular JSON endpoint
        .endpoint(
            EndpointMeta::new()
                .route("/health")
                .method("GET")
                .description("Health check"),
            |_ctx, _body| async move {
                Ok(HttpResponse::Ok().json(json!({
                    "status": "healthy",
                    "features": ["multipart_upload"]
                })))
            },
        )
        .build()
        .expect("Failed to build server");

    println!("🚀 Multipart Upload Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📤 POST http://127.0.0.1:3001/upload - Upload CSV file");
    println!("🏥 GET  http://127.0.0.1:3001/health - Health check");
    println!();
    println!("Test with curl:");
    println!("  curl -X POST http://127.0.0.1:3001/upload \\");
    println!("    -F \"file=@your_file.csv\"");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    server.run("127.0.0.1:3001").await
}
