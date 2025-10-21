use actix_web::http::header;
use actix_web::{HttpResponse, Result};

/// CORS middleware configuration
pub fn cors_middleware() -> actix_web::middleware::DefaultHeaders {
    actix_web::middleware::DefaultHeaders::new()
        .add((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .add((header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS"))
        .add((
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            "Content-Type, Authorization, Accept, Last-Event-ID",
        ))
}

/// Request validation middleware
pub async fn validate_request(
    req: actix_web::dev::ServiceRequest,
    next: actix_web_lab::middleware::Next<actix_web::body::BoxBody>,
) -> Result<actix_web::dev::ServiceResponse> {
    // Validate Content-Type for POST requests
    if req.method() == actix_web::http::Method::POST {
        if let Some(content_type) = req.headers().get(header::CONTENT_TYPE) {
            if let Ok(ct) = content_type.to_str() {
                if !ct.contains("application/json") {
                    let (req, _) = req.into_parts();
                    let response = HttpResponse::BadRequest().json(serde_json::json!({
                        "jsonrpc": "2.0",
                        "error": {
                            "code": -32600,
                            "message": "Invalid Content-Type. Expected application/json"
                        },
                        "id": null
                    }));
                    return Ok(actix_web::dev::ServiceResponse::new(req, response));
                }
            }
        }
    }

    next.call(req).await
}
