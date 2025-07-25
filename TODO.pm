# ✅ `just-mcp` Development Roadmap

A step-by-step plan to build a fully-featured, test-covered, LLM/MCP-compatible Justfile server.

---

## Phase 0: Project Initialization
- [ ] Create project: `cargo new --bin just-mcp`
- [ ] Set up workspace with `just-mcp` and `just-mcp-lib` crates
- [ ] Add `clap` for CLI argument parsing
- [ ] Write a hello-world CLI that prints `MCP server started`
- [ ] Add a smoke test that runs the binary and asserts it exits cleanly

---

## Phase 1: Justfile Parsing + Basic Introspection
- [ ] Parse Justfile and list available recipe names
- [ ] Extract parameter names and defaults
- [ ] Parse recipe comments as doc strings
- [ ] Implement `mcp-just introspect` returning structured JSON
- [ ] Write integration test for basic introspection

---

## Phase 2: Command Execution Support
- [ ] Implement `mcp-just run <recipe> [--arg val]`
- [ ] Capture stdout, stderr, and exit code
- [ ] Return structured JSON result for execution
- [ ] Handle unknown recipe and invalid args with error output
- [ ] Add tests: success, failure, missing args

---

## Phase 3: Argument Validation & Signature Help
- [ ] Implement `signature <recipe>` returning param list
- [ ] Include required/optional markers and default values
- [ ] Validate arguments before execution
- [ ] Return validation errors in structured form
- [ ] Add unit tests for arg validation and signature help

---

## Phase 4: Completion & Suggestions
- [ ] Implement `completion <prefix>` for recipe name completions
- [ ] Add argument name completion per recipe
- [ ] Add static value completions for known enums (if any)
- [ ] Include type/kind metadata: recipe | param | value
- [ ] Add tests for completions (valid, partial, empty input)

---

## Phase 5: LSP Integration Scaffolding
- [ ] Add `--lsp` or `--stdio` mode for JSON-RPC
- [ ] Define MCP-compatible JSON-RPC methods: `introspect`, `signature`, `run`, `completion`
- [ ] Map CLI output to LSP-compatible JSON responses
- [ ] Add `--json` to all CLI modes
- [ ] Write simulated LSP test harness for RPC calls

---

## Phase 6: Diagnostics & Validation
- [ ] Detect and report Justfile syntax errors
- [ ] Return diagnostics: line, column, message, severity
- [ ] Detect duplicate recipes or undefined variables
- [ ] Add LSP `textDocument/publishDiagnostics` support
- [ ] Add tests: syntax errors, unused/missing params

---

## Phase 7: Environment & Context Awareness
- [ ] Support `.env` loading and inject into runtime
- [ ] Allow per-recipe env overrides
- [ ] Add `env` command to print active vars
- [ ] Track env usage in introspection output
- [ ] Add tests: `.env` files, overrides, missing vars

---

## Phase 8: Dependency Graph Support
- [ ] Parse dependencies between recipes
- [ ] Implement `graph` command: output DOT or JSON
- [ ] Implement `run --plan` dry-run with execution order
- [ ] Expose recipe dependencies in `introspect`
- [ ] Add tests for tree, chain, cyclic dependencies

---

## Phase 9: Cross-File & VFS Support
- [ ] Support Justfile input via stdin
- [ ] Add support for remote Justfile sources (e.g. GitHub URL)
- [ ] Accept in-memory buffers via file URI (LSP)
- [ ] Normalize paths for Docker/WSL/devcontainers
- [ ] Add tests for all input types: local, stdin, URL, virtual

---

## Phase 10: Plugin Support & Custom Metadata
- [ ] Support inline annotations via `# mcp: {...}`
- [ ] Add optional recipe tags, descriptions, and categories
- [ ] Implement before/after plugin hook execution
- [ ] Add customizable output templates (e.g. JSON, table)
- [ ] Add tests: plugin metadata parsing, hook exec

---

## Phase 11: Completion of Full MCP Protocol Spec
- [ ] Define and publish MCP protocol JSON schema
- [ ] Implement version negotiation and `mcp-version` endpoint
- [ ] Add exhaustive protocol conformance test suite
- [ ] Add fuzz tests for JSON input and edge cases
- [ ] Finalize and document all CLI + RPC modes

---

## Phase 12: Final Polish and Packaging
- [ ] Add `cargo-release` and tag as `v1.0.0`
- [ ] Publish crates: `just-mcp-lib`, `just-mcp`
- [ ] Write full usage docs (CLI + LSP + JSON RPC)
- [ ] Set up CI: format, lint, tests, coverage
- [ ] Add `just release` and GitHub Actions to automate full release

---

