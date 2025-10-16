use clap::{Parser, Subcommand};
use soil::{
    copy_file, create_dir, create_hard_link, create_symlink, ensure_dir, exists, list_dir,
    metadata, move_path, read_bytes, read_symlink, read_text, remove_dir_all, remove_empty_dir,
    remove_file, resolve_path, set_permissions, symlink_metadata, write_file,
};
use std::process;

/// A CLI for soil
#[derive(Parser)]
#[command(name = "soil")]
#[command(about = "A CLI for soil")]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the canonical path of a file or directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil resolve ./test_root
    /// ```
    Resolve {
        /// The path to canonicalize
        path: String,
    },

    /// Copy a file from source to destination
    ///
    /// # Examples
    ///
    /// ```
    /// soil copy source.txt destination.txt
    /// ```
    Copy {
        /// The source file to copy
        src: String,
        /// The destination path
        dst: String,
    },

    /// Create a directory and all parent directories
    ///
    /// # Examples
    ///
    /// ```
    /// soil mkdirp ./new/directory/path
    /// ```
    Mkdirp {
        /// The directory path to create
        path: String,
    },

    /// Create a single directory (parent must exist)
    ///
    /// # Examples
    ///
    /// ```
    /// soil mkdir ./parent/child
    /// ```
    Mkdir {
        /// The directory path to create
        path: String,
    },

    /// List the contents of a directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil ls ./directory
    /// ```
    Ls {
        /// The directory path to list
        path: String,
    },

    /// Remove a file
    ///
    /// # Examples
    ///
    /// ```
    /// soil rm ./file.txt
    /// ```
    Rm {
        /// The file path to remove
        path: String,
    },

    /// Remove an empty directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil rmdir ./empty_directory
    /// ```
    Rmdir {
        /// The directory path to remove
        path: String,
    },

    /// Remove a directory and all its contents
    ///
    /// # Examples
    ///
    /// ```
    /// soil rmrf ./directory
    /// ```
    Rmrf {
        /// The directory path to remove recursively
        path: String,
    },

    /// Move or rename a file or directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil mv old_name.txt new_name.txt
    /// ```
    Mv {
        /// The current path
        from: String,
        /// The destination path
        to: String,
    },

    /// Get metadata information about a file or directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil stat ./file.txt
    /// ```
    Stat {
        /// The path to examine
        path: String,
    },

    /// Read file content as bytes
    ///
    /// # Examples
    ///
    /// ```
    /// soil read-bytes ./file.txt
    /// ```
    ReadBytes {
        /// The file path to read
        path: String,
    },

    /// Read file content as text
    ///
    /// # Examples
    ///
    /// ```
    /// soil read-text ./file.txt
    /// ```
    ReadText {
        /// The file path to read
        path: String,
    },

    /// Write content to a file
    ///
    /// # Examples
    ///
    /// ```
    /// soil write ./file.txt "content"
    /// ```
    Write {
        /// The file path to write to
        path: String,
        /// The content to write
        content: String,
    },

    /// Create a hard link
    ///
    /// # Examples
    ///
    /// ```
    /// soil hardlink original.txt linked.txt
    /// ```
    Hardlink {
        /// The original file path
        original: String,
        /// The hard link path
        link: String,
    },

    /// Create a symbolic link
    ///
    /// # Examples
    ///
    /// ```
    /// soil symlink original.txt symlink.txt
    /// ```
    Symlink {
        /// The target path
        original: String,
        /// The symbolic link path
        link: String,
    },

    /// Read the target of a symbolic link
    ///
    /// # Examples
    ///
    /// ```
    /// soil readlink symlink.txt
    /// ```
    Readlink {
        /// The symbolic link path
        path: String,
    },

    /// Modify file or directory permissions
    ///
    /// # Examples
    ///
    /// ```
    /// soil chmod file.txt readonly|writable
    /// ```
    Chmod {
        /// The path to modify
        path: String,
        /// Permission mode (readonly/writable)
        mode: String,
    },

    /// Get metadata of a symbolic link without following it
    ///
    /// # Examples
    ///
    /// ```
    /// soil lstat symlink.txt
    /// ```
    Lstat {
        /// The path to examine
        path: String,
    },

    /// Check if a path exists
    ///
    /// # Examples
    ///
    /// ```
    /// soil exists ./some/path
    /// ```
    Exists {
        /// The path to check
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Resolve { path } => match resolve_path(&path) {
            Ok(canonical_path) => {
                println!("Canonical path: {}", canonical_path);
            }
            Err(error) => {
                eprintln!("Error resolving path '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Copy { src, dst } => match copy_file(&src, &dst) {
            Ok(_) => {
                println!("Copied '{}' to '{}'", src, dst);
            }
            Err(error) => {
                eprintln!("Error copying '{}' to '{}': {}", src, dst, error);
                process::exit(1);
            }
        },

        Commands::Mkdirp { path } => match ensure_dir(&path) {
            Ok(_) => {
                println!("Created directories '{}'", path);
            }
            Err(error) => {
                eprintln!("Error creating directories '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Mkdir { path } => match create_dir(&path) {
            Ok(_) => {
                println!("Created directory '{}'", path);
            }
            Err(error) => {
                eprintln!("Error creating directory '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Ls { path } => match list_dir(&path) {
            Ok(contents) => {
                println!("Contents of '{}':", path);
                for item in contents {
                    println!("  {}", item);
                }
            }
            Err(error) => {
                eprintln!("Error listing directory '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Rm { path } => match remove_file(&path) {
            Ok(_) => {
                println!("Removed file '{}'", path);
            }
            Err(error) => {
                eprintln!("Error removing file '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Rmdir { path } => match remove_empty_dir(&path) {
            Ok(_) => {
                println!("Removed empty directory '{}'", path);
            }
            Err(error) => {
                eprintln!("Error removing empty directory '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Rmrf { path } => match remove_dir_all(&path) {
            Ok(_) => {
                println!("Removed directory recursively '{}'", path);
            }
            Err(error) => {
                eprintln!("Error removing directory recursively '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Mv { from, to } => match move_path(&from, &to) {
            Ok(_) => {
                println!("Moved '{}' to '{}'", from, to);
            }
            Err(error) => {
                eprintln!("Error moving '{}' to '{}': {}", from, to, error);
                process::exit(1);
            }
        },

        Commands::Stat { path } => match metadata(&path) {
            Ok(metadata) => {
                println!("Metadata for '{}':", path);
                println!("  Size: {} bytes", metadata.len());
                println!(
                    "  Type: {}",
                    if metadata.is_file() {
                        "File"
                    } else if metadata.is_dir() {
                        "Directory"
                    } else {
                        "Other"
                    }
                );
                println!("  Read-only: {}", metadata.permissions().readonly());
                if let Ok(modified) = metadata.modified() {
                    println!("  Modified: {:?}", modified);
                }
            }
            Err(error) => {
                eprintln!("Error reading metadata for '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::ReadBytes { path } => match read_bytes(&path) {
            Ok(content) => {
                println!("Read {} bytes from '{}'", content.len(), path);
                match String::from_utf8(content) {
                    Ok(text) => println!("Content:\n{}", text),
                    Err(_) => println!("Content contains non-UTF8 data"),
                }
            }
            Err(error) => {
                eprintln!("Error reading bytes from '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::ReadText { path } => match read_text(&path) {
            Ok(content) => {
                println!("Text from '{}':", path);
                println!("{}", content);
            }
            Err(error) => {
                eprintln!("Error reading text from '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Write { path, content } => match write_file(&path, &content) {
            Ok(_) => {
                println!("Wrote {} bytes to '{}'", content.len(), path);
            }
            Err(error) => {
                eprintln!("Error writing to '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Hardlink { original, link } => match create_hard_link(&original, &link) {
            Ok(_) => {
                println!("Created hard link '{}' -> '{}'", link, original);
            }
            Err(error) => {
                eprintln!(
                    "Error creating hard link '{}' -> '{}': {}",
                    link, original, error
                );
                process::exit(1);
            }
        },

        Commands::Symlink { original, link } => match create_symlink(&original, &link) {
            Ok(_) => {
                println!("Created symlink '{}' -> '{}'", link, original);
            }
            Err(error) => {
                eprintln!(
                    "Error creating symlink '{}' -> '{}': {}",
                    link, original, error
                );
                process::exit(1);
            }
        },

        Commands::Readlink { path } => match read_symlink(&path) {
            Ok(target) => {
                println!("Symbolic link '{}' points to: {}", path, target);
            }
            Err(error) => {
                eprintln!("Error reading symlink '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Chmod { path, mode } => match metadata(&path) {
            Ok(metadata) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::{MetadataExt, PermissionsExt};

                    let current_mode = metadata.mode();
                    let new_mode = match mode.as_str() {
                        // clear all write bits (owner/group/other)
                        "readonly" => current_mode & !0o222,
                        // ensure owner-writable; do not broaden group/other write bits
                        "writable" => current_mode | 0o200,
                        _ => {
                            eprintln!("Invalid mode '{}'. Use 'readonly' or 'writable'", mode);
                            process::exit(1);
                        }
                    };
                    let perms = PermissionsExt::from_mode(new_mode);
                    match set_permissions(&path, perms) {
                        Ok(_) => {
                            println!("Updated permissions of '{}' to {}", path, mode)
                        }
                        Err(error) => {
                            eprintln!("Error updating permissions of '{}': {}", path, error);
                            process::exit(1);
                        }
                    }
                }

                #[cfg(windows)]
                {
                    let mut perms = metadata.permissions();
                    match mode.as_str() {
                        "readonly" => perms.set_readonly(true),
                        "writable" => perms.set_readonly(false),
                        _ => {
                            eprintln!("Invalid mode '{}'. Use 'readonly' or 'writable'", mode);
                            process::exit(1);
                        }
                    }
                    match set_permissions(&path, perms) {
                        Ok(_) => {
                            println!("Updated permissions of '{}' to {}", path, mode)
                        }
                        Err(error) => {
                            eprintln!("Error updating permissions of '{}': {}", path, error);
                            process::exit(1);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading metadata for '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Lstat { path } => match symlink_metadata(&path) {
            Ok(metadata) => {
                println!("lstat for '{}':", path);
                println!("  Size: {} bytes", metadata.len());
                println!(
                    "  Type: {}",
                    if metadata.file_type().is_symlink() {
                        "Symbolic Link"
                    } else if metadata.is_file() {
                        "File"
                    } else if metadata.is_dir() {
                        "Directory"
                    } else {
                        "Other"
                    }
                );
                println!("  Read-only: {}", metadata.permissions().readonly());
                if let Ok(modified) = metadata.modified() {
                    println!("  Modified: {:?}", modified);
                }
            }
            Err(error) => {
                eprintln!("Error lstat '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Exists { path } => {
            let path_exists = exists(&path);
            println!("Path '{}' exists: {}", path, path_exists);

            if !path_exists {
                process::exit(1);
            }
        }
    }
}
