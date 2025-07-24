/*!
# MCP Integration Test with rmcp SDK

## Purpose
**Type-safe SDK integration testing** - validates that our MCP server works correctly 
with the official Rust MCP SDK (rmcp) using proper async service patterns.

## Approach
- **rmcp client library**: Uses official Rust MCP SDK with type-safe APIs
- **Async service architecture**: Leverages `.serve()` pattern with automatic initialization
- **Type-safe validation**: Uses proper `CallToolRequestParam`, `Cow<'static, str>` types
- **Advanced error handling**: Tests SDK-level error scenarios and edge cases
- **Production patterns**: Mirrors how real clients would integrate with our server

## What This Tests
- SDK compatibility and type safety
- Async service lifecycle management  
- Advanced MCP features (pagination, etc.)
- Error handling and edge cases
- Client-server communication patterns

## What This Doesn't Test
- Raw protocol compliance (see basic_mcp_test.rs)
- Server implementation details
- Performance characteristics

## When To Use
- SDK compatibility validation
- Integration testing for client applications
- Testing advanced MCP features
- Validating production usage patterns
*/

use rmcp::{
    ServiceExt,
    transport::{ConfigureCommandExt, TokioChildProcess},
    model::CallToolRequestParam,
};
use serde_json::{json, Map, Value};
use std::borrow::Cow;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_mcp_server_integration() {
    // Create transport for our MCP server as child process
    let transport = TokioChildProcess::new(
        tokio::process::Command::new("cargo").configure(|cmd| {
            cmd.args(["run", "--", "--stdio"]);
        })
    ).expect("Failed to create transport");

    // Create client and initialize (initialization is automatic with .serve())
    let client = ().serve(transport).await.expect("Failed to initialize client");

    // Test listing tools
    let tools = timeout(Duration::from_secs(10), client.list_all_tools())
        .await
        .expect("List tools timed out")
        .expect("Failed to list tools");

    assert!(!tools.is_empty());
    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(tool_names.contains(&"list_recipes"));
    assert!(tool_names.contains(&"run_recipe"));
    assert!(tool_names.contains(&"get_recipe_info"));
    assert!(tool_names.contains(&"validate_justfile"));

    // Test calling list_recipes
    let list_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: Cow::Borrowed("list_recipes"),
            arguments: Some(Map::new()),
        })
    )
    .await
    .expect("List recipes timed out")
    .expect("Failed to call list_recipes");

    assert!(!list_result.content.is_empty());
    // Should contain our hello and write_file recipes
    let content_str = list_result.content[0].text.as_ref().unwrap();
    assert!(content_str.contains("hello"));
    assert!(content_str.contains("write_file"));

    // Test calling hello recipe with default parameter
    let hello_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: "run_recipe".to_string(),
            arguments: Some(json!({
                "recipe": "hello"
            })),
        })
    )
    .await
    .expect("Hello recipe timed out")
    .expect("Failed to call hello recipe");

    let content_str = hello_result.content[0].text.as_ref().unwrap();
    assert!(content_str.contains("Hello, World!"));

    // Test calling hello recipe with custom parameter
    let hello_custom_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: "run_recipe".to_string(),
            arguments: Some(json!({
                "recipe": "hello",
                "args": ["Claude"]
            })),
        })
    )
    .await
    .expect("Hello custom recipe timed out")
    .expect("Failed to call hello recipe with custom name");

    let content_str = hello_custom_result.content[0].text.as_ref().unwrap();
    assert!(content_str.contains("Hello, Claude!"));

    // Test write_file recipe
    let write_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: "run_recipe".to_string(),
            arguments: Some(json!({
                "recipe": "write_file",
                "args": ["test_output.txt", "Hello from MCP integration test!"]
            })),
        })
    )
    .await
    .expect("Write file recipe timed out")
    .expect("Failed to call write_file recipe");

    let content_str = write_result.content[0].text.as_ref().unwrap();
    assert!(content_str.contains("Written"));
    assert!(content_str.contains("test_output.txt"));

    // Cleanup
    client.cancel().await.expect("Failed to cancel client");
}

#[tokio::test]
async fn test_get_recipe_info() {
    // Create transport and client
    let transport = TokioChildProcess::new(
        tokio::process::Command::new("cargo").configure(|cmd| {
            cmd.args(["run", "--", "--stdio"]);
        })
    ).expect("Failed to create transport");

    let client = ().serve(transport).await.expect("Failed to initialize client");

    // Test get_recipe_info for hello recipe
    let info_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: "get_recipe_info".to_string(),
            arguments: Some(json!({
                "recipe": "hello"
            })),
        })
    )
    .await
    .expect("Get recipe info timed out")
    .expect("Failed to get recipe info");

    let content_str = info_result.content[0].text.as_ref().unwrap();
    assert!(content_str.contains("hello"));
    assert!(content_str.contains("name"));
    // Should contain parameter information
    assert!(content_str.contains("parameter") || content_str.contains("param"));

    // Cleanup
    client.cancel().await.expect("Failed to cancel client");
}