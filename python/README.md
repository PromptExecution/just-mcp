# just-mcp (Python package)

Production-ready MCP (Model Context Protocol) server for Justfile integration.

## Installation

```bash
# Install with pip
pip install just-mcp

# Or use with uvx (no installation required)
uvx just-mcp --stdio
```

## Usage

### As MCP Server

```bash
just-mcp --stdio
```

### With Claude Desktop

Add to your Claude Desktop MCP configuration:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "just-mcp": {
      "command": "uvx",
      "args": ["just-mcp", "--stdio"]
    }
  }
}
```

Or if installed with pip:

```json
{
  "mcpServers": {
    "just-mcp": {
      "command": "just-mcp",
      "args": ["--stdio"]
    }
  }
}
```

## Features

- 🏗️ Complete MCP Server - Full rmcp 0.3.0 integration
- 📋 Recipe Discovery - Parse and list all available Justfile recipes
- ⚡ Recipe Execution - Execute recipes with parameters
- 🔍 Recipe Introspection - Get detailed recipe information
- ✅ Justfile Validation - Syntax and semantic validation
- 🌍 Environment Management - .env file support

## MCP Tools Available

1. **`list_recipes`** - List all available recipes in the justfile
2. **`run_recipe`** - Execute a specific recipe with optional arguments
3. **`get_recipe_info`** - Get detailed information about a specific recipe
4. **`validate_justfile`** - Validate the justfile for syntax errors

## Platform Support

- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64/Apple Silicon)
- Windows (x86_64)

The package automatically downloads the appropriate pre-built binary for your platform during installation.

## Documentation

For full documentation, visit: https://github.com/promptexecution/just-mcp

## License

MIT License - see [LICENSE](https://github.com/promptexecution/just-mcp/blob/main/LICENSE)
