# Soil 🌱

A CLI for file system operations with nature-themed naming conventions. Soil provides an intuitive interface for common file and directory operations using botanical metaphors.

## Features

- **File Operations**: Copy (propagate), read (chronicle/harvest), write (inscribe)
- **Directory Operations**: Create (grow/sprout), list (survey), remove (clear/prune)
- **Path Operations**: Canonicalize (trace), move/rename (transplant), check existence
- **Link Operations**: Create hard links (graft), symbolic links (connect), follow links
- **Permission Management**: Modify permissions (vitalize), examine metadata (examine/surface)

## Installation & Usage

### Using Nix Flakes (Recommended)

This project includes a comprehensive Nix flake for easy development and deployment.

#### Quick Run
```bash
# Run directly without installing
nix run github:yourusername/soil -- survey .

# Or locally
nix run . -- --help
```

#### Development Environment
```bash
# Enter development shell with all tools
nix develop

# Or with direnv (after installing direnv)
echo "use flake" > .envrc && direnv allow
```

#### Building
```bash
# Build the package
nix build

# The binary will be in ./result/bin/soil
./result/bin/soil --help
```

### Using Cargo (Traditional Rust)

```bash
# Install from source
cargo install --path .

# Or run directly
cargo run -- --help
```

## Command Reference

### File Operations
```bash
# Copy a file (propagate from source to destination)
soil propagate source.txt destination.txt

# Read file content as text
soil chronicle file.txt

# Read file content as bytes
soil harvest binary_file.dat

# Write content to file
soil inscribe new_file.txt "Hello, World!"
```

### Directory Operations
```bash
# List directory contents
soil survey ./some/directory

# Create directory tree (like mkdir -p)
soil grow ./path/to/new/directory

# Create single directory (parent must exist)
soil sprout ./existing/parent/new_child

# Remove empty directory
soil prune ./empty_directory

# Remove directory and all contents
soil clear ./directory_to_delete
```

### Path Operations
```bash
# Get canonical/absolute path
soil trace ./relative/path

# Move or rename file/directory
soil transplant old_name new_name

# Check if path exists
soil exists ./some/path
```

### Link Operations
```bash
# Create hard link
soil graft original.txt hard_link.txt

# Create symbolic link
soil connect target.txt symlink.txt

# Read symbolic link target
soil follow symlink.txt
```

### Metadata Operations
```bash
# Examine file/directory metadata
soil examine file.txt

# Examine symlink metadata (without following)
soil surface symlink.txt

# Change permissions
soil vitalize file.txt readonly
soil vitalize file.txt writable
```

## Development

### Using Nix (Recommended)

The Nix flake provides a complete development environment:

```bash
# Enter development shell
nix develop

# Available in the shell:
cargo build          # Build the project
cargo run -- <args>  # Run with arguments
cargo test           # Run tests
cargo watch -x run   # Watch for changes
bacon                # Interactive build watcher
just --list          # Show all available tasks
```

### Using Just (Task Runner)

If you have `just` installed, you can use the included justfile:

```bash
# Show all available commands
just

# Common development tasks
just build           # Build the project
just test            # Run tests
just run survey .    # Run with arguments
just watch           # Watch for changes and rebuild
just check           # Run all checks (fmt, clippy, test)

# Nix-specific tasks
just nix-build       # Build with Nix
just nix-run survey . # Run with Nix
just nix-dev         # Enter Nix development shell
```

### Available Tools in Development Environment

The Nix development shell includes:
- **Rust toolchain** with rustfmt, clippy, rust-src
- **cargo-watch** for file watching and auto-rebuild
- **cargo-edit** for dependency management
- **cargo-audit** for security auditing
- **bacon** for interactive development
- **just** for task running
- **fd** and **ripgrep** for file searching
- **mdbook** for documentation

## Nix Flake Outputs

The flake provides several outputs:

- `packages.default` / `packages.soil` - The soil binary
- `devShells.default` - Development environment with all tools
- `apps.default` / `apps.soil` - Run the soil CLI
- `apps.dev` - Start development mode with file watching
- `apps.test` - Run the test suite
- `checks.build` - Build verification for CI

### Flake Commands

```bash
# Check flake validity
nix flake check

# Show flake info
nix flake show

# Update flake inputs
nix flake update

# Build and run
nix build && ./result/bin/soil --help
nix run . -- --help

# Development
nix develop
nix run .#dev        # Start development mode
nix run .#test       # Run tests
```

## Examples

### Basic File Management
```bash
# Create a directory structure
soil grow ./projects/my-app/src

# Write some content
soil inscribe ./projects/my-app/src/main.txt "Hello, Soil!"

# Copy the file
soil propagate ./projects/my-app/src/main.txt ./backup/main.txt

# Read it back
soil chronicle ./backup/main.txt

# List the directory
soil survey ./projects/my-app
```

### Working with Links
```bash
# Create a file
soil inscribe original.txt "Original content"

# Create a hard link
soil graft original.txt hardlink.txt

# Create a symbolic link
soil connect original.txt symlink.txt

# Follow the symbolic link
soil follow symlink.txt

# Examine the different types
soil examine original.txt
soil examine hardlink.txt
soil surface symlink.txt  # Don't follow the link
```

## Philosophy

Soil uses botanical metaphors to make file system operations more intuitive:
- **Propagate**: Like plant propagation, copying spreads files
- **Grow**: Directories grow like plants, creating full structures
- **Sprout**: Individual directories sprout from existing ones
- **Survey**: Observe the canopy (directory contents)
- **Trace**: Follow roots back to their origin (canonical paths)
- **Transplant**: Moving files like transplanting plants
- **Graft**: Hard links like grafting branches
- **Chronicle**: Files tell their story through their content

## Contributing

1. Enter the development environment: `nix develop`
2. Make your changes
3. Run tests: `just test` or `cargo test`
4. Check formatting and lints: `just check`
5. Build and test: `just build && just run survey .`

## License

[Add your license here]