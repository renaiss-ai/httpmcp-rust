# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
