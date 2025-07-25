# RMCP 0.3.0 Syntax Fixes

This document summarizes the key changes made to fix the rmcp syntax based on the official examples from the rust-sdk repository.

## Key Changes Made

### 1. Import Structure
**Before:**
```rust
use rmcp::{
    tool, tool_router,
    ErrorData as McpError,
    model::{CallToolResult, ServerCapabilities, ServerInfo, TextContent},
    handler::server::ServerHandler,
};
```

**After:**
```rust
use rmcp::{
    tool, tool_router, tool_handler,
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo, ProtocolVersion, Implementation, ErrorCode},
    schemars,
};
```

### 2. Tool Parameter Definitions
**Before:**
```rust
#[tool(description = "List all available recipes in the justfile")]
async fn list_recipes(
    &self,
    #[tool(param)]
    #[schemars(description = "Optional path to justfile")]
    justfile_path: Option<String>
) -> Result<CallToolResult, McpError>
```

**After:**
```rust
// Define parameter struct
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListRecipesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justfile_path: Option<String>,
}

#[tool(description = "List all available recipes in the justfile")]
async fn list_recipes(
    &self,
    Parameters(ListRecipesRequest { justfile_path }): Parameters<ListRecipesRequest>
) -> Result<CallToolResult, McpError>
```

### 3. Server Structure
**Before:**
```rust
#[derive(Clone)]
pub struct JustMcpServer {
    working_dir: std::path::PathBuf,
}
```

**After:**
```rust
#[derive(Clone)]
pub struct JustMcpServer {
    working_dir: std::path::PathBuf,
    tool_router: ToolRouter<JustMcpServer>,
}

impl JustMcpServer {
    pub fn new(working_dir: impl AsRef<Path>) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf(),
            tool_router: Self::tool_router(),
        }
    }
}
```

### 4. Response Creation
**Before:**
```rust
Ok(CallToolResult {
    content: vec![TextContent {
        type_: "text".to_string(),
        text: content,
    }],
    is_error: false,
})
```

**After:**
```rust
Ok(CallToolResult::success(vec![Content::text(content)]))
// Or for errors:
Ok(CallToolResult::error(vec![Content::text(content)]))
```

### 5. Error Handling
**Before:**
```rust
McpError {
    code: -1,
    message: err.to_string(),
    data: None,
}
```

**After:**
```rust
McpError {
    code: ErrorCode(-1),
    message: err.to_string().into(),
    data: None,
}
```

### 6. ServerHandler Implementation
**Before:**
```rust
impl ServerHandler for JustMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("...".to_string()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}
```

**After:**
```rust
#[tool_handler]
impl ServerHandler for JustMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("...".to_string()),
        }
    }
}
```

## Key Patterns from Official Examples

1. **Parameter Handling**: Use dedicated structs with `#[derive(JsonSchema, Deserialize)]` and wrap with `Parameters<T>`
2. **Tool Router**: Add `tool_router: ToolRouter<Self>` field and initialize with `Self::tool_router()`
3. **Attributes**: Use `#[tool_router]` and `#[tool_handler]` on impl blocks
4. **Content Creation**: Use `Content::text()` helper instead of manual `TextContent` construction
5. **Result Methods**: Use `CallToolResult::success()` and `CallToolResult::error()` helpers

## Working Example

See `/examples/mcp_server_demo.rs` for a complete working example that demonstrates the corrected syntax.

## Testing

All tests pass after these changes:
- `cargo check` - No compilation errors
- `cargo build` - Successful build
- `cargo test` - All existing tests continue to pass
- `cargo run --example mcp_server_demo` - Demo runs successfully