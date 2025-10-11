use clap::{Parser, Subcommand};
use soil::{exists, grow_branch, propagate_leaf, trace_to_root};
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
    /// soil trace ./root
    /// ```
    Trace {
        /// The path to canonicalize
        path: String,
    },

    /// Copy a file from source to destination
    ///
    /// # Examples
    ///
    /// ```
    /// soil propagate scion.txt rootstock.txt
    /// ```
    Propagate {
        /// The source file to copy
        scion: String,
        /// The destination path
        rootstock: String,
    },

    /// Create a directory and all parent directories
    ///
    /// # Examples
    ///
    /// ```
    /// soil grow ./root/branch/
    /// ```
    Grow {
        /// The directory path to create
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
        Commands::Trace { path } => match trace_to_root(&path) {
            Ok(canonical_path) => {
                println!("Canonical path: {}", canonical_path);
            }
            Err(error) => {
                eprintln!("Error tracing path '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Propagate { scion, rootstock } => match propagate_leaf(&scion, &rootstock) {
            Ok(_) => {
                println!("Successfully propagated '{}' to '{}'", scion, rootstock);
            }
            Err(error) => {
                eprintln!(
                    "Error propagating '{}' to '{}': {}",
                    scion, rootstock, error
                );
                process::exit(1);
            }
        },

        Commands::Grow { path } => match grow_branch(&path) {
            Ok(_) => {
                println!("Successfully grew branch at '{}'", path);
            }
            Err(error) => {
                eprintln!("Error growing branch '{}': {}", path, error);
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
