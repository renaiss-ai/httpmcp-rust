# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4] - 2025-01-22

### Added
- **Multipart File Upload Support**: New `.multipart_endpoint()` builder method for handling file uploads
- `MultipartEndpointHandler` type for processing multipart/form-data requests
- Support for CSV and other file uploads through actix-multipart integration
- `multipart_upload.rs` example demonstrating file upload handling and CSV parsing
- Multipart endpoints process streams on the same task to avoid Send trait issues

### Changed
- Added `actix-multipart = "0.7"` dependency
- Updated handler types to support non-Send futures for multipart processing
- Enhanced README with multipart upload documentation and examples

### Technical Details
- Multipart handlers return non-Send futures since `Multipart` contains `Rc<RefCell<...>>`
- File processing happens immediately in the handler to avoid thread-safety issues
- OAuth validation works seamlessly with multipart endpoints

## [0.1.3] - 2025-01-22

### Added
- **Custom HTTP Endpoints**: New `.endpoint()` builder method to register REST API endpoints alongside MCP protocol
- `EndpointMeta` metadata builder for configuring custom routes, HTTP methods, and descriptions
- Dynamic route registration supporting GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- `endpoint_example.rs` demonstrating health checks and custom REST API endpoints
- Health endpoint added to `travel_planner.rs` example
- Custom endpoints share the same port as MCP protocol endpoints
- CORS support for custom endpoints with configurable methods

### Changed
- Updated `transport.rs` to dynamically register custom endpoint routes during app configuration
- Extended `HttpMcpServer` to store and manage custom endpoints
- Added `actix_web::HttpResponse` as return type for endpoint handlers

### Technical Details
- Endpoint handlers use `Arc` for thread-safe sharing across workers
- Endpoints have access to `RequestContext` for headers and metadata
- OAuth validation automatically applies to custom endpoints when configured
- All custom endpoints integrate seamlessly with existing CORS middleware

## [0.1.0] - 2025-01-XX

### Added
- Initial release of httpmcp-rust
- Function-based API for registering resources, tools, and prompts
- `HttpMcpServer` builder pattern for server configuration
- Metadata builders: `ToolMeta`, `ResourceMeta`, `PromptMeta`
- Full MCP protocol support (resources, tools, prompts, logging)
- HTTP POST endpoint for JSON-RPC requests
- SSE (Server-Sent Events) GET endpoint for streaming
- `RequestContext` with headers and metadata access
- OAuth 2.0 configuration support
- CORS middleware
- Request validation
- Type-safe error handling with `McpError`
- Examples: simple_server, full_server, travel_planner
- Comprehensive documentation

### Features
- Built on actix-web for high performance
- Async/await throughout
- Simple, minimal API design
- Production-ready error handling
- Header access (Authorization, Bearer tokens, custom headers)
- Request metadata (request ID, remote address, method, path)

[Unreleased]: https://github.com/renaiss-ai/httpmcp-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/renaiss-ai/httpmcp-rust/releases/tag/v0.1.0
