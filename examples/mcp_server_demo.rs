use std::path::PathBuf;
use just_mcp_lib::mcp_server::JustMcpServer;
use rmcp::ServerHandler;

/// Demonstration of the JustMcpServer with corrected rmcp 0.3.0 syntax
/// 
/// This example shows how to:
/// 1. Create a JustMcpServer instance
/// 2. Get server information
/// 3. Set up the server with stdio transport (commented out since it would block)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a server instance pointing to the current directory
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let server = JustMcpServer::new(&current_dir);
    
    // Get server information to verify it's working
    let info = server.get_info();
    println!("MCP Server Info:");
    println!("  Protocol Version: {:?}", info.protocol_version);
    println!("  Instructions: {}", info.instructions.unwrap_or_default());
    println!("  Tools Enabled: {}", info.capabilities.tools.is_some());
    
    // The server can be served with stdio transport like this:
    // let service = server.serve(stdio()).await?;
    // service.waiting().await?;
    
    println!("\nServer created successfully with rmcp 0.3.0 syntax!");
    println!("Tools available:");
    println!("  - list_recipes: List all available recipes in the justfile");
    println!("  - run_recipe: Execute a specific recipe with optional arguments");
    println!("  - get_recipe_info: Get detailed information about a specific recipe");
    println!("  - validate_justfile: Validate the justfile for syntax and semantic errors");
    
    Ok(())
}