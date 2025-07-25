/*!
# MCP Integration Test with rmcp SDK - WORKING VERSION

## Purpose
**Type-safe SDK integration testing** - validates that our MCP server works correctly
with the official Rust MCP SDK (rmcp) using proper async service patterns.

This version has been fixed to work with rmcp 0.3.0 API requirements.
*/

use rmcp::{
    ServiceExt,
    model::CallToolRequestParam,
    transport::{ConfigureCommandExt, TokioChildProcess},
};
use serde_json::{Map, Value};
use std::borrow::Cow;
use tokio::time::{Duration, timeout};

#[tokio::test]
async fn test_mcp_server_integration_working() {
    // Create transport for our MCP server as child process
    let transport =
        TokioChildProcess::new(tokio::process::Command::new("cargo").configure(|cmd| {
            cmd.args(["run", "--", "--stdio"]);
        }))
        .expect("Failed to create transport");

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
        }),
    )
    .await
    .expect("List recipes timed out")
    .expect("Failed to call list_recipes");

    assert!(!list_result.content.is_empty());
    // Extract text content using proper rmcp API
    let content_str = match &list_result.content[0].raw {
        rmcp::model::RawContent::Text(text) => text,
        _ => panic!("Expected text content"),
    };
    assert!(content_str.text.contains("hello"));
    assert!(content_str.text.contains("write_file"));

    // Test calling hello_simple recipe (no parameters)
    let hello_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: Cow::Borrowed("run_recipe"),
            arguments: Some({
                let mut map = Map::new();
                map.insert(
                    "recipe_name".to_string(),
                    Value::String("hello_simple".to_string()),
                );
                map
            }),
        }),
    )
    .await
    .expect("Hello recipe timed out")
    .expect("Failed to call hello recipe");

    let content_str = match &hello_result.content[0].raw {
        rmcp::model::RawContent::Text(text) => text,
        _ => panic!("Expected text content"),
    };

    // Parse the JSON response and check stdout
    let result_json: serde_json::Value =
        serde_json::from_str(&content_str.text).expect("Failed to parse result JSON");

    let stdout = result_json["stdout"]
        .as_str()
        .expect("Expected stdout field");
    assert!(stdout.contains("Hello, World!"));

    // Test calling hello recipe with custom parameter
    let hello_custom_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: Cow::Borrowed("run_recipe"),
            arguments: Some({
                let mut map = Map::new();
                map.insert(
                    "recipe_name".to_string(),
                    Value::String("hello".to_string()),
                );
                map.insert(
                    "args".to_string(),
                    Value::String(r#"["Claude"]"#.to_string()),
                );
                map
            }),
        }),
    )
    .await
    .expect("Hello custom recipe timed out")
    .expect("Failed to call hello recipe with custom name");

    let content_str = match &hello_custom_result.content[0].raw {
        rmcp::model::RawContent::Text(text) => text,
        _ => panic!("Expected text content"),
    };
    assert!(content_str.text.contains("Hello, Claude!"));

    // Test write_file recipe
    let write_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: Cow::Borrowed("run_recipe"),
            arguments: Some({
                let mut map = Map::new();
                map.insert(
                    "recipe_name".to_string(),
                    Value::String("write_file".to_string()),
                );
                map.insert(
                    "args".to_string(),
                    Value::String(
                        r#"["test_output.txt", "Hello from MCP integration test!"]"#.to_string(),
                    ),
                );
                map
            }),
        }),
    )
    .await
    .expect("Write file recipe timed out")
    .expect("Failed to call write_file recipe");

    let content_str = match &write_result.content[0].raw {
        rmcp::model::RawContent::Text(text) => text,
        _ => panic!("Expected text content"),
    };
    assert!(content_str.text.contains("Written"));
    assert!(content_str.text.contains("test_output.txt"));

    // Cleanup
    client.cancel().await.expect("Failed to cancel client");
}

#[tokio::test]
async fn test_get_recipe_info_working() {
    // Create transport and client
    let transport =
        TokioChildProcess::new(tokio::process::Command::new("cargo").configure(|cmd| {
            cmd.args(["run", "--", "--stdio"]);
        }))
        .expect("Failed to create transport");

    let client = ().serve(transport).await.expect("Failed to initialize client");

    // Test get_recipe_info for hello recipe
    let info_result = timeout(
        Duration::from_secs(10),
        client.peer().call_tool(CallToolRequestParam {
            name: Cow::Borrowed("get_recipe_info"),
            arguments: Some({
                let mut map = Map::new();
                map.insert(
                    "recipe_name".to_string(),
                    Value::String("hello".to_string()),
                );
                map
            }),
        }),
    )
    .await
    .expect("Get recipe info timed out")
    .expect("Failed to get recipe info");

    let content_str = match &info_result.content[0].raw {
        rmcp::model::RawContent::Text(text) => text,
        _ => panic!("Expected text content"),
    };
    assert!(content_str.text.contains("hello"));
    assert!(content_str.text.contains("name"));
    // Should contain parameter information
    assert!(content_str.text.contains("parameter") || content_str.text.contains("param"));

    // Cleanup
    client.cancel().await.expect("Failed to cancel client");
}
