use clap::{Arg, Command};
use just_mcp_lib::mcp_server::JustMcpServer;
use rmcp::{ServiceExt, transport::stdio};
use std::error::Error;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("just-mcp")
        .version(env!("CARGO_PKG_VERSION"))
        .author("just-mcp contributors")
        .about("MCP Server for Justfile Integration")
        .arg(
            Arg::new("working-dir")
                .short('d')
                .long("directory")
                .value_name("DIR")
                .help("Working directory for the MCP server")
                .default_value("."),
        )
        .arg(
            Arg::new("stdio")
                .long("stdio")
                .help("Run as MCP server using stdio transport")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let working_dir = matches.get_one::<String>("working-dir").unwrap();
    let working_path = Path::new(working_dir);

    if matches.get_flag("stdio") {
        // Run as MCP server
        eprintln!(
            "Starting just-mcp MCP server in directory: {}",
            working_path.display()
        );

        let server = JustMcpServer::new(working_path);

        // Start the MCP server with stdio transport
        let running_service = server.serve(stdio()).await?;

        // Keep the server running
        running_service.waiting().await?;
    } else {
        // Show usage information
        println!("just-mcp v{}", env!("CARGO_PKG_VERSION"));
        println!("MCP Server for Justfile Integration");
        println!();
        println!("Usage:");
        println!(
            "  {} --stdio                    Run as MCP server with stdio transport",
            env!("CARGO_PKG_NAME")
        );
        println!(
            "  {} --directory <DIR> --stdio  Run MCP server in specific directory",
            env!("CARGO_PKG_NAME")
        );
        println!();
        println!("MCP Tools Available:");
        println!("  list_recipes      - List all available recipes in the justfile");
        println!("  run_recipe        - Execute a specific recipe with optional arguments");
        println!("  get_recipe_info   - Get detailed information about a specific recipe");
        println!("  validate_justfile - Validate the justfile for syntax and semantic errors");
        println!();
        println!("Example usage with MCP client:");
        println!("  {} --stdio | your-mcp-client", env!("CARGO_PKG_NAME"));
    }

    Ok(())
}
