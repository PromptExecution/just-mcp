# just-mcp

**Model Context Protocol (MCP) server for Justfile integration**

A production-ready MCP server that provides seamless integration with [Just](https://github.com/casey/just) command runner, enabling AI assistants to discover, execute, and introspect Justfile recipes through the standardized MCP protocol.

## 🚀 Current Status: **67% Complete** (8/12 core tasks)

### ✅ **Implemented Features**
- **🏗️ Complete MCP Server** - Full rmcp 0.3.0 integration with MCP 2024-11-05 protocol
- **📋 Recipe Discovery** - Parse and list all available Justfile recipes
- **⚡ Recipe Execution** - Execute recipes with parameters and capture structured output
- **🔍 Recipe Introspection** - Get detailed recipe information, parameters, and documentation
- **✅ Justfile Validation** - Syntax and semantic validation with error reporting
- **🌍 Environment Management** - Comprehensive .env file support and variable expansion
- **🧪 Full Test Coverage** - 33 passing tests across integration and unit test suites

### 🎯 **MCP Tools Available**
1. **`list_recipes`** - List all available recipes in the justfile
2. **`run_recipe`** - Execute a specific recipe with optional arguments  
3. **`get_recipe_info`** - Get detailed information about a specific recipe
4. **`validate_justfile`** - Validate the justfile for syntax and semantic errors

## 🏃 **Quick Start**

### Installation & Setup
```bash
# Clone and build
git clone <repository-url>
cd just-mcp
cargo build --release

# Test the server
cargo run -- --stdio
```

### Claude Desktop Integration
Add to your Claude Desktop MCP configuration:

```json
{
  "mcpServers": {
    "just-mcp": {
      "command": "/path/to/just-mcp",
      "args": ["--stdio"]
    }
  }
}
```

### Usage Examples
```bash
# Run as MCP server
just-mcp --stdio

# Run in specific directory  
just-mcp --directory /path/to/project --stdio
```

## 🧪 **Testing**

### Comprehensive Test Suite
```bash
# Run all tests (33 tests)
cargo test

# Run specific test suites
cargo test --test basic_mcp_test      # Protocol compliance testing
cargo test --test mcp_integration_working  # SDK integration testing
```

### Test Architecture
- **`basic_mcp_test.rs`** - Direct protocol compliance testing using raw JSON-RPC
- **`mcp_integration_working.rs`** - Type-safe SDK integration testing with rmcp client
- **Unit tests** - 25+ tests covering parser, executor, validator, and environment modules

## 📚 **Architecture**

### Project Structure
```
just-mcp/
├── src/main.rs              # CLI binary
├── just-mcp-lib/           # Core library
│   ├── parser.rs           # Justfile parsing
│   ├── executor.rs         # Recipe execution  
│   ├── validator.rs        # Validation logic
│   ├── environment.rs      # Environment management
│   └── mcp_server.rs       # MCP protocol implementation
├── tests/                  # Integration tests
└── justfile               # Demo recipes
```

### Tech Stack
- **Rust 1.82+** with async/await support
- **rmcp 0.3.0** - Official MCP SDK for Rust
- **serde/serde_json** - JSON serialization  
- **snafu** - Structured error handling
- **tokio** - Async runtime

## 🔄 **Development Roadmap**

### 🎯 **Next Priority Tasks** (Remaining 33%)
1. **LSP-Style Completion System** - Intelligent autocompletion for recipes and parameters
2. **Enhanced Diagnostics** - Advanced syntax error reporting and suggestions  
3. **Virtual File System** - Support for stdin, remote sources, and in-memory buffers
4. **Release Preparation** - Documentation, CI/CD, and crate publication

### 🚀 **Future Enhancements**
- Plugin system for custom recipe types
- Integration with other build tools
- Performance optimizations for large justfiles
- Advanced dependency visualization

## 📖 **Usage Patterns**

### Recipe Execution
```javascript
// List available recipes
await client.callTool("list_recipes", {});

// Execute recipe with parameters  
await client.callTool("run_recipe", {
  "recipe_name": "build",
  "args": "[\"--release\"]"
});

// Get recipe information
await client.callTool("get_recipe_info", {
  "recipe_name": "test"
});
```

### Validation
```javascript
// Validate justfile
await client.callTool("validate_justfile", {
  "justfile_path": "./custom.justfile"  
});
```

## 🤝 **Contributing**

This project follows the [_b00t_ development methodology](AGENTS.md):
- **TDD Approach** - Tests first, implementation second
- **Feature Branches** - Never work directly on main branch
- **Structured Errors** - Use snafu for error management
- **Git Workflow** - Clean commits with descriptive messages

### Development Commands
```bash
just build    # Build the project
just test     # Run tests  
just server   # Start MCP server
just clean    # Clean build artifacts
```

## 📄 **License**

This project is licensed under [LICENSE](LICENSE).

## 🔗 **Related Projects**

- [Just](https://github.com/casey/just) - The command runner this integrates with
- [Model Context Protocol](https://modelcontextprotocol.io/) - The protocol specification
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Official Rust MCP SDK