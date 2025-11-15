# Package Testing Guide

This guide explains how to test the npm and pip packages locally before publishing.

## Prerequisites

- Node.js 14+ (for npm testing)
- Python 3.8+ (for pip testing)
- A local build of just-mcp binary

## Testing npm Package

### 1. Build the Rust Binary

```bash
# From project root
cargo build --release
```

### 2. Test Installation Script Locally

Instead of downloading from GitHub, you can test with a local binary:

```bash
cd npm

# Create bin directory and copy binary
mkdir -p bin
cp ../target/release/just-mcp bin/

# Test the wrapper
node index.js --help
```

### 3. Test as npm Package

```bash
cd npm

# Pack the package
npm pack

# Install globally from the tarball
npm install -g just-mcp-0.1.0.tgz

# Test the command
just-mcp --help

# Clean up
npm uninstall -g just-mcp
```

### 4. Test with npx (Simulated)

```bash
# In a test directory
mkdir /tmp/test-npm
cd /tmp/test-npm

# Install locally
npm install /path/to/just-mcp/npm/just-mcp-0.1.0.tgz

# Test with npx
npx just-mcp --help
```

## Testing Python Package

### 1. Build the Package

```bash
cd python

# Install build tools
pip install build

# Build the package
python -m build
```

This creates:
- `dist/just_mcp-0.1.0-py3-none-any.whl` (wheel)
- `dist/just_mcp-0.1.0.tar.gz` (source distribution)

### 2. Test Installation

```bash
# Install from wheel
pip install dist/just_mcp-0.1.0-py3-none-any.whl

# Test the command
just-mcp --help

# Clean up
pip uninstall just-mcp
```

### 3. Test with a Local Binary

To avoid downloading during testing:

```bash
cd python

# Create bin directory and copy binary
mkdir -p just_mcp/bin
cp ../target/release/just-mcp just_mcp/bin/

# Rebuild the package
python -m build

# Install and test
pip install dist/just_mcp-0.1.0-py3-none-any.whl
just-mcp --help
```

### 4. Test with uvx (Simulated)

```bash
# Build the package first
cd python
python -m build

# In a test directory
mkdir /tmp/test-uvx
cd /tmp/test-uvx

# Test with uvx pointing to local package
uvx --from /path/to/just-mcp/python/dist/just_mcp-0.1.0-py3-none-any.whl just-mcp --help
```

## Testing Binary Download Logic

### Mock GitHub Release for Testing

For comprehensive testing, you can:

1. Create a test release on a fork
2. Build binaries for your platform
3. Upload to the test release
4. Modify package URLs to point to test release
5. Test installation

### Testing Without Mock Release

You can modify the install scripts to use a local HTTP server:

```bash
# Start a simple HTTP server in your build directory
cd target/release
python -m http.server 8000

# In install.js or __init__.py, temporarily change:
# const downloadUrl = `http://localhost:8000/just-mcp`
```

## Integration Testing

### Test with MCP Client

After installation, test with an actual MCP client:

```bash
# Using stdio mode
echo '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}' | just-mcp --stdio

# Should get an MCP protocol response
```

### Test with Claude Desktop

1. Install package (npm or pip)
2. Configure Claude Desktop:
   ```json
   {
     "mcpServers": {
       "just-mcp-test": {
         "command": "just-mcp",
         "args": ["--stdio"]
       }
     }
   }
   ```
3. Restart Claude Desktop
4. Check MCP server connection in settings

## Troubleshooting

### npm Package Issues

**Binary not found after install:**
- Check if `npm/bin/` directory was created
- Verify install.js ran successfully
- Check npm install logs for errors

**Permission errors:**
- Ensure binary has execute permissions
- On Unix: `chmod +x npm/bin/just-mcp`

### Python Package Issues

**Import errors:**
- Verify package structure: `just_mcp/__init__.py` exists
- Check Python path includes package location

**Binary download fails:**
- Test with local binary first
- Verify internet connection
- Check GitHub release exists for version

## Platform-Specific Testing

### Linux

```bash
# Test on different architectures
docker run --rm -it --platform linux/amd64 ubuntu:latest
docker run --rm -it --platform linux/arm64 ubuntu:latest
```

### macOS

```bash
# Test on Intel and Apple Silicon
# Rosetta 2 will handle x86_64 on arm64 if needed
file $(which just-mcp)
```

### Windows

```powershell
# PowerShell
Get-Command just-mcp
(Get-Command just-mcp).Source
```

## Checklist Before Publishing

- [ ] Tested npm package installation locally
- [ ] Tested pip package installation locally
- [ ] Verified binary downloads work (or mock setup)
- [ ] Tested on target platform (Linux/macOS/Windows)
- [ ] Verified MCP protocol communication
- [ ] Checked package version matches release
- [ ] Reviewed package metadata (description, keywords, etc.)
- [ ] Tested with Claude Desktop or similar MCP client
- [ ] Verified fallback to cargo build works (optional)
- [ ] Checked package size is reasonable
