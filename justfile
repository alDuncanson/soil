# Justfile for Soil project
# Run `just --list` to see all available commands

# Default recipe - shows help
default:
    @just --list

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run the application with arguments
run *args:
    cargo run -- {{args}}

# Run tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Check code formatting
fmt-check:
    cargo fmt -- --check

# Format code
fmt:
    cargo fmt

# Run clippy lints
clippy:
    cargo clippy -- -D warnings

# Run all checks (fmt, clippy, test)
check: fmt-check clippy test

# Watch for changes and rebuild
watch:
    cargo watch -x build

# Watch for changes and run tests
watch-test:
    cargo watch -x test

# Watch for changes and run the app
watch-run *args:
    cargo watch -x "run -- {{args}}"

# Clean build artifacts
clean:
    cargo clean

# Install the binary locally
install:
    cargo install --path .

# Generate documentation
doc:
    cargo doc --open

# Audit dependencies for security vulnerabilities
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Show dependency tree
deps:
    cargo tree

# Run with Nix
nix-run *args:
    nix run . -- {{args}}

# Enter Nix development shell
nix-dev:
    nix develop

# Build with Nix
nix-build:
    nix build

# Check Nix flake
nix-check:
    nix flake check

# Example commands - show common usage patterns
examples:
    @echo "Common soil commands:"
    @echo "  just run survey .                    # List current directory contents"
    @echo "  just run trace ./some/path          # Get canonical path"
    @echo "  just run grow ./new/directory       # Create directory tree"
    @echo "  just run propagate file1.txt file2.txt  # Copy file"
    @echo "  just run inscribe test.txt 'Hello'  # Write to file"
    @echo "  just run chronicle test.txt         # Read file content"
    @echo "  just run exists ./some/path         # Check if path exists"
