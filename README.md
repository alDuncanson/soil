# soil

Ergonomic filesystem helpers and a tiny CLI.

## Library overview

Thin, consistent wrappers over std::fs with simple return types and thorough docs.

Functions:
- resolve_path(path) -> String: Canonical absolute path (follows symlinks).
- copy_file(src, dst): Copy a file.
- ensure_dir(path): Create a directory and all parents (mkdir -p).
- create_dir(path): Create a single directory (parent must exist).
- list_dir(path) -> Vec<String>: List immediate entries (names only).
- remove_file(path): Delete a file.
- remove_empty_dir(path): Delete an empty directory.
- remove_dir_all(path): Recursively delete a directory.
- move_path(from, to): Move or rename files/directories.
- metadata(path) -> fs::Metadata: File or directory metadata.
- symlink_metadata(path) -> fs::Metadata: lstat-equivalent (does not follow symlinks).
- read_bytes(path) -> Vec<u8>: Read entire file as bytes.
- read_text(path) -> String: Read entire file as UTF-8.
- write_file(path, bytes|&str): Write data (creates or overwrites).
- create_hard_link(original, link): Create a hard link.
- create_symlink(original, link): Create a symbolic link (Unix and Windows supported).
- read_symlink(path) -> String: Read a symlink’s target.
- set_permissions(path, perms): Set file or directory permissions.
- exists(path) -> bool: Check if a path exists.

Quick example:

```rust
use soil::{ensure_dir, write_file, read_text, remove_dir_all};

ensure_dir("tmp/nested").unwrap();
write_file("tmp/nested/hello.txt", "hi").unwrap();
assert_eq!(read_text("tmp/nested/hello.txt").unwrap(), "hi");
remove_dir_all("tmp").unwrap();
```

## CLI

Install from source:

```sh
cargo install --path .
```

Commands:
- resolve <path>
- copy <src> <dst>
- mkdirp <path>
- mkdir <path>
- ls <path>
- rm <path>
- rmdir <path>
- rmrf <path>
- mv <from> <to>
- stat <path>
- lstat <path>
- read-bytes <path>
- read-text <path>
- write <path> <content>
- hardlink <original> <link>
- symlink <original> <link>
- readlink <path>
- chmod <path> readonly|writable
- exists <path>

Examples:

```sh
soil mkdirp ./tmp/demo/nested
soil write ./tmp/demo/nested/file.txt "hello"
soil read-text ./tmp/demo/nested/file.txt
soil ls ./tmp/demo
soil rmrf ./tmp/demo
```

Notes:
- Symlink creation differs by OS. On Unix, a single create_symlink is used. On Windows, the implementation chooses file vs directory symlink as needed (developer mode or elevated privileges may be required).
- chmod on Unix adjusts POSIX mode bits. On Windows it toggles the readonly attribute.
