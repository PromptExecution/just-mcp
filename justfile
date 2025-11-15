# Say hello with no parameters
hello_simple:
	echo "Hello, World!" $(date)

# Say hello with optional name parameter
hello name="World":
	echo "Hello, {{name}}!"

# Write content to a specified file
write_file filename content="Hello from just-mcp!":
	@echo "{{content}}" > {{filename}}
	echo "Written '{{content}}' to {{filename}}"

b00t reason +args:
	# the `just b00t` syntax provides a curated toolkit of commands is ALLOWED via just-mcp (unjustified bash b00t-cli will be denied)
	# FUTURE: a sm0l agent will put here to inspect the justification and tool to decide if it is allowed
	b00t log {{reason}}
	b00t-cli {{args}}

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Run only library tests (known working)
test-lib:
	cargo test -p just-mcp-lib

# Run all tests including integration tests (some may fail)
test-all:
	cargo test

# Run the MCP server with stdio transport
server:
	cargo run -- --stdio

# Clean build artifacts
clean:
	cargo clean

# Keep server.json metadata aligned with the Cargo version
update-server-json:
	./scripts/update-server-json-version.sh

install-claude-mcp:
	claude mcp add context7 -- npx -y @upstash/context7-mcp
	claude mcp add sequential-thinking -- npx -y @modelcontextprotocol/server-sequential-thinking

	# WTF? NO!
	#claude mcp add kollektiv -- npx -y mcp-remote "https://mcp.thekollektiv.ai/mcp"

	# THIS IS BROKEN:
	# claude mcp add taskmaster-ai -- npx -y "git+https://github.com/promptexecution/task-master.git#feature/add-openai-api-base-env"
	claude mcp add taskmaster-ai -- npx -y --package=task-master-ai task-master-ai

	# cargo install --git https://github.com/PromptExecution/cratedocs-mcp --locked
	claude mcp add fetch-url-as-markdown -- npx -y @upstash/fetch-url-as-markdown
	claude mcp add rust-crate-doc -- cratedocs stdio --debug
