# AI Agent Guide for Soil 🌱

This guide helps AI agents understand the Soil project structure, conventions, and development practices when contributing to this open-source Rust CLI tool.

## Project Overview

Soil is a Rust CLI application that provides file system operations using nature-themed naming conventions. It offers an intuitive interface for common file and directory operations through botanical metaphors.

### Key Characteristics
- **Language**: Rust (2024 edition)
- **Architecture**: CLI with library structure (`src/lib.rs` + `src/main.rs`)
- **Dependencies**: Minimal (primarily `clap` for CLI parsing)
- **Development Environment**: Nix flake with comprehensive tooling
- **Testing**: Unit tests with isolated test environments

## Project Structure

```
soil/
├── src/
│   ├── lib.rs          # Core library functions with botanical naming
│   └── main.rs         # CLI interface using clap
├── Cargo.toml          # Rust package configuration
├── flake.nix           # Nix development environment
├── README.md           # Comprehensive documentation
└── target/             # Build artifacts (ignored in git)
```

## Botanical Naming Convention

The project uses botanical metaphors for file system operations. When adding new functionality, maintain this theme:

### Current Mapping
- **propagate_leaf**: Copy file (like plant propagation)
- **grow_branch**: Create directory tree (mkdir -p)
- **sprout_branch**: Create single directory
- **survey_canopy**: List directory contents
- **shed_leaf**: Remove file
- **prune_branch**: Remove empty directory
- **clear_grove**: Remove directory recursively
- **trace_to_root**: Get canonical path (follow roots)
- **transplant**: Move/rename (like transplanting plants)
- **examine_specimen**: Get metadata
- **harvest_essence**: Read file as bytes
- **read_chronicle**: Read file as text
- **inscribe_leaf**: Write content to file
- **create_hard_graft**: Create hard link (grafting branches)
- **create_soft_connection**: Create symbolic link
- **adjust_vitality**: Change permissions
- **examine_outer_characteristics**: Get symlink metadata

### Naming Guidelines
- Use botanical/gardening metaphors
- Function names should be descriptive and intuitive
- Maintain consistency with existing patterns
- CLI commands use shorter forms (e.g., `propagate` instead of `propagate_leaf`)

## Code Architecture Patterns

### Path Ergonomics
The project emphasizes excellent path handling using `AsRef<Path>`:

```rust
// All functions accept multiple path types seamlessly
pub fn some_operation<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    // implementation
}
```

### Error Handling
- Use `std::io::Result<T>` for file system operations
- Provide descriptive error messages in CLI layer
- Library functions return raw errors; CLI handles user-friendly display

### Function Structure Template
```rust
/// Brief description of the operation using botanical metaphor
/// 
/// # Arguments
/// * `path` - Description of path parameter
/// 
/// # Returns
/// * `Ok(result)` - Success case description
/// * `Err(error)` - Error conditions
/// 
/// # Examples
/// ```
/// use soil::function_name;
/// let result = function_name("path/to/file")?;
/// ```
pub fn function_name<P: AsRef<Path>>(path: P) -> io::Result<ReturnType> {
    let path = path.as_ref();
    // implementation
}
```

## Testing Patterns

### Test Environment Setup
The project uses isolated test environments with automatic cleanup:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Each test gets a unique directory that's cleaned up automatically
    fn setup_test() -> TestGuard {
        let test_root = get_unique_test_root();
        let _ = grow_branch(&test_root);
        TestGuard { test_root }
    }
    
    #[test]
    fn test_new_feature() {
        let guard = setup_test();
        // Use guard.test_root for test operations
        // Cleanup happens automatically when guard drops
    }
}
```

### Test Naming
- Use descriptive test names: `test_propagate_leaf_success`, `test_grow_branch_invalid_path`
- One assertion per test when possible
- Test both success and error conditions

## CLI Integration

### Adding New Commands
1. Add function to `src/lib.rs` following botanical naming
2. Add corresponding enum variant in `src/main.rs` `Commands`
3. Include documentation with examples in the enum variant
4. Implement the command handler in the main match statement
5. Update README.md with usage examples

### CLI Command Template
```rust
/// Brief description of the command
///
/// # Examples
///
/// ```
/// soil command-name path/to/target
/// ```
CommandName {
    /// Description of the parameter
    path: String,
},
```

## Development Environment

### Using Nix (Recommended)
```bash
# Enter development shell with all tools
nix develop

# Available commands in shell:
cargo build          # Build the project
cargo test           # Run tests
cargo clippy          # Linting
cargo fmt             # Formatting
bacon                 # Interactive development
```

### Without Nix
```bash
# Install Rust toolchain first
cargo build
cargo test
cargo run -- --help
```

## Code Quality Standards

### Formatting and Linting
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Use meaningful variable names that fit the botanical theme

### Documentation
- Document all public functions with examples
- Update README.md when adding new commands
- Include error conditions in function documentation

### Error Messages
- CLI error messages should be user-friendly
- Include the problematic path in error messages
- Use botanical language in error descriptions when appropriate

## Contributing Guidelines

### Before Making Changes
1. Understand the botanical metaphor for your change
2. Check if similar functionality already exists
3. Ensure the change fits the project's philosophy

### Development Process
1. Create/enter development environment: `nix develop`
2. Write tests first (TDD approach)
3. Implement functionality in `src/lib.rs`
4. Add CLI command in `src/main.rs` if needed
5. Update documentation
6. Run full test suite: `cargo test`
7. Check formatting and lints: `cargo fmt --check && cargo clippy`

### Commit Guidelines
- Use descriptive commit messages
- Mention the botanical function names in commits
- Include test coverage for new functionality

## Common Patterns and Anti-Patterns

### ✅ Good Practices
- Use `AsRef<Path>` for path parameters
- Provide comprehensive error handling
- Write isolated tests with cleanup
- Follow botanical naming consistently
- Document with examples

### ❌ Anti-Patterns
- Don't use `String` for path parameters (use `AsRef<Path>`)
- Don't ignore errors in library functions
- Don't break the botanical naming convention
- Don't write tests that depend on external file system state
- Don't forget to update CLI when adding library functions

## Debugging Tips

### Common Issues
- **Path not found**: Ensure test setup creates necessary directory structure
- **Permission denied**: Check if test cleanup is interfering
- **Type errors**: Remember to use `AsRef<Path>` for path parameters

### Testing Locally
```bash
# Build and test a specific command
cargo build
./target/debug/soil survey .

# Run specific test
cargo test test_propagate_leaf

# Run with verbose output
cargo test -- --nocapture
```

## Project Philosophy

Soil aims to make file system operations more intuitive through botanical metaphors. When contributing:

- **Intuitive**: Operations should feel natural using garden/plant terminology
- **Ergonomic**: Excellent path handling with minimal friction
- **Reliable**: Comprehensive error handling and testing
- **Minimal**: Keep dependencies light and focused
- **Educational**: Code should teach through clear examples and documentation

Remember: Every file system operation has a botanical equivalent. Think like a gardener! 🌱