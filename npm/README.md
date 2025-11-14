# just-mcp (npm package)

Production-ready MCP (Model Context Protocol) server for Justfile integration.

## Installation

```bash
# Install globally
npm install -g just-mcp

# Or use with npx (no installation required)
npx just-mcp --stdio
```

## Usage

### As MCP Server

```bash
just-mcp --stdio
```

### With Claude Desktop

Add to your Claude Desktop MCP configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "just-mcp": {
      "command": "npx",
      "args": ["-y", "just-mcp", "--stdio"]
    }
  }
}
```

Or if installed globally:

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

## Documentation

For full documentation, visit: https://github.com/promptexecution/just-mcp

## License

MIT License - see [LICENSE](https://github.com/promptexecution/just-mcp/blob/main/LICENSE)
