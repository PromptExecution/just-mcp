use just_mcp_lib::mcp_server::JustMcpServer;
use rmcp::ServerHandler;
use std::path::PathBuf;

#[test]
fn test_mcp_server_instantiation() {
    let temp_dir = std::env::temp_dir();
    let server = JustMcpServer::new(&temp_dir);

    // Test that the server can be created without errors
    let info = server.get_info();

    // Verify basic server info
    assert!(info.instructions.is_some());
    assert!(info.instructions.unwrap().contains("MCP server"));
    assert!(info.capabilities.tools.is_some());
}

#[test]
fn test_mcp_server_with_working_directory() {
    let working_dir = PathBuf::from("test-fixtures");
    let server = JustMcpServer::new(&working_dir);

    // Just verify it can be created - actual functionality tests are in integration tests
    let info = server.get_info();
    assert!(info.instructions.is_some());
}
