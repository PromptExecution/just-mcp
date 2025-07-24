# Sample Justfile for testing parser

# Default recipe
default: lint test build

# Build the project
build:
    cargo build --release

# Run tests
test: build
    cargo test

# Run linter
lint:
    cargo clippy -- -D warnings
    cargo fmt --check

# Deploy to environment  
deploy env target='production': build test
    echo "Deploying to {{ env }} with target {{ target }}"
    ./scripts/deploy.sh {{ env }} {{ target }}

# Development server
serve port='8080':
    cargo run -- --port {{ port }}

# Clean up
clean:
    cargo clean
    rm -rf target/

# Show help
help:
    @echo "Available recipes:"
    @just --list

# Variables
version = "1.0.0" 
debug = false