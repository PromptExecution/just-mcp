# just-mcp

[![CI](https://github.com/PromptExecution/just-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/PromptExecution/just-mcp/actions/workflows/ci.yml)
[![Release](https://github.com/PromptExecution/just-mcp/actions/workflows/release.yml/badge.svg)](https://github.com/PromptExecution/just-mcp/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/just-mcp.svg)](https://crates.io/crates/just-mcp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**👋 A way to let LLMs speak Just**

A production-ready MCP server that provides seamless integration with [Just](https://github.com/casey/just) command runner, enabling AI assistants to discover, execute, and introspect Justfile recipes through the standardized MCP protocol.

## 🎯 **Why Just + MCP = Better Agent Execution**

### **Context-Saving Abstraction**
If it isn't immediately obvious, the benefit of having LLMs use Just vs. bash is that running Just commands (via MCP) provides a context-saving abstraction where they don't need to waste context opening/reading bash files, Python scripts, or other build artifacts. The LLM via MCP simply gets the command, parameters, and hints - it's in their memory as "these are commands available to you."

### **Eliminates the Justfile Learning Curve**
No more watching LLMs execute `just -l` to get command lists, inevitably start reading the justfile, then try to write justfile syntax (like it's a Makefile), corrupt the justfile, and create a bad experience. Just's evolving syntax simply doesn't have a large enough corpus in frontier models today - we need more popular repos with justfiles in the training dataset.

### **Safer Than Raw Bash Access**
Just-mcp is fundamentally safer than bash. If you read HackerNews, there's a story at least once daily about operators whose LLMs start forgetting, hallucinating, and eventually breaking down - deleting files and doing nasty unwanted things. Giving LLMs unsupervised, unrestricted bash access without carefully monitoring context consumption is a recipe for disaster.

**Using Justfile fixes that.** Even if the LLM modifies its own justfile, the next context is memoized by the justfile (hopefully in an idempotent git repo).  This abstraction shields the llm from the command line complexity where hallucinations or attention tracking the current working directory cause it to go over the rails and off the cliff.  

### **Powerful Agent Execution Tool**
Just-mcp is perfect for anybody doing agent execution:
- **Ultra-low overhead** - probably better than every other tool
- **Human-friendly** - justfiles are easy for humans and low overhead for LLMs  
- **Quick and dirty** - while some prefer full Python FastAPI servers, just-mcp is just easy-as
- **sm0l model friendly** - works great with self-hostable GPU/CPU open source models with 8k-32k context limits

### **Built-in Safety Patterns**
Just has useful patterns for introducing:
- **Transparent logging** without distracting the agent
- **Secondary model inspection** - use sm0l models to scan commands asking "is this harmful?" before execution
- **Python decorator-like patterns** for command validation
- **Idempotent execution** backed by git repos

## b00t
```
b00t mcp create just-mcp -- bash just-mcp --stdio "${REPO_ROOT}"
b00t mcp export just-mcp
```

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

### Installation

Choose your preferred installation method:

#### npm (JavaScript/TypeScript)
```bash
# Install globally
npm install -g just-mcp

# Or use with npx (no installation required)
npx just-mcp --stdio
```

#### pip (Python)
```bash
# Install with pip
pip install just-mcp

# Or use with uvx (recommended)
uvx just-mcp --stdio
```

#### Cargo (Rust)
```bash
# Install from crates.io
cargo install just-mcp

# Or build from source
git clone https://github.com/promptexecution/just-mcp
cd just-mcp
cargo build --release
```

#### Using Docker
```bash
# Pull the latest image from GitHub Container Registry
docker pull ghcr.io/promptexecution/just-mcp:latest

# Run with Docker
docker run --rm -v $(pwd):/workspace ghcr.io/promptexecution/just-mcp:latest --stdio

# Build locally
docker build -t just-mcp:local .
docker run --rm -v $(pwd):/workspace just-mcp:local --stdio
```

Available Docker image tags:
- `latest` - Latest stable release
- `X.Y.Z` - Specific version (e.g., `0.1.0`)
- `X.Y` - Latest patch version (e.g., `0.1`)
- `X` - Latest minor version (e.g., `0`)

### Claude Desktop Integration

#### Using npm/npx
Add to your Claude Desktop MCP configuration:

#### Using Binary
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

#### Using pip/uvx
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

#### Using cargo or manual install
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

#### Using Docker
```json
{
  "mcpServers": {
    "just-mcp": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "-v",
        "${workspaceFolder}:/workspace",
        "ghcr.io/promptexecution/just-mcp:latest",
        "--stdio"
      ]
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

# Using Docker
docker run --rm -v $(pwd):/workspace ghcr.io/promptexecution/just-mcp:latest --stdio
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

## 🚀 **Release Setup & CI/CD**

### ✅ **Completed Setup**

#### **Cocogitto & Conventional Commits**
- Installed cocogitto for conventional commit enforcement
- Configured `cog.toml` with proper commit types and changelog settings
- Set up git hooks for commit message linting (`commit-msg`) and pre-push testing

#### **GitHub Actions CI/CD**
- **CI Pipeline** (`ci.yml`): Multi-platform testing (Ubuntu, Windows, macOS), formatting, clippy, commit linting
- **Release Pipeline** (`release.yml`): Automated versioning, changelog generation, GitHub releases, and crates.io publishing
- **Binary Builds** (`build-binaries.yml`): Cross-platform binary compilation for npm and pip packages
- **Container Pipeline** (`container.yaml`): Multi-platform Docker image builds (linux/amd64, linux/arm64) pushed to GitHub Container Registry

#### **Docker Images**
- Multi-platform builds for `linux/amd64` and `linux/arm64`
- Minimal image size using static musl binaries and scratch base image
- Automatic tagging with semantic versioning (major, major.minor, major.minor.patch, latest)
- Published to GitHub Container Registry (ghcr.io)
- Integrated with release workflow for automatic deployment

#### **Crates.io Preparation**
- Updated both `Cargo.toml` files with complete metadata (description, keywords, categories, license, etc.)
- Added proper exclusions for development-only files
- Verified MIT license is in place

#### **Documentation & Structure**
- README.md is production-ready with installation and usage instructions
- Created initial `CHANGELOG.md` for release tracking
- Updated `.gitignore` with Rust-specific entries

### 🚀 **Production Deployment**

#### **Development Workflow:**
- All commits must follow conventional commit format (enforced by git hooks)
- Use `feat:`, `fix:`, `docs:`, etc. prefixes for automatic versioning
- Push to `main` branch triggers automated releases and crates.io publishing
- Library tests pass ✅ (25/25) with comprehensive test coverage

#### **Release Process:**
- **Automated Versioning**: Cocogitto analyzes commit messages for semantic versioning
- **GitHub Releases**: Automatic changelog generation and GitHub release creation
- **Binary Distribution**: Pre-built binaries for Linux (x86_64, aarch64), macOS (x86_64, aarch64), and Windows (x86_64)
- **Crates.io Publishing**: Library crate (`just-mcp-lib`) publishes first, then binary crate (`just-mcp`)
- **npm Publishing**: Wrapper package for easy Node.js/TypeScript integration
- **PyPI Publishing**: Python wrapper package for pip/uvx installation
- **CI/CD Pipeline**: Multi-platform testing (Ubuntu, Windows, macOS) with formatting and clippy checks

#### **Installation Methods:**
```bash
# npm (JavaScript/TypeScript ecosystems)
npm install -g just-mcp
# or
npx just-mcp --stdio

# pip (Python ecosystems)
pip install just-mcp
# or
uvx just-mcp --stdio

# cargo (Rust ecosystem)
cargo install just-mcp

# Download pre-built binaries
wget https://github.com/promptexecution/just-mcp/releases/latest/download/just-mcp-x86_64-unknown-linux-gnu.tar.gz
# Or use Docker
docker pull ghcr.io/promptexecution/just-mcp:latest

# Or download from GitHub releases
wget https://github.com/promptexecution/just-mcp/releases/latest/download/just-mcp
```

## 🔗 **Related Projects**

- [Just](https://github.com/casey/just) - The command runner this integrates with
- [Model Context Protocol](https://modelcontextprotocol.io/) - The protocol specification
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Official Rust MCP SDK

### **Friends of just-mcp**

- [just-vscode](https://github.com/promptexecution/just-vscode) - VSCode extension with LSP integration for enhanced Just authoring
- [just-awesome-agents](https://github.com/promptexecution/just-awesome-agents) - Collection of patterns and tools for agent execution with Just# Test change to trigger pre-push hook
