use clap::{Parser, Subcommand};
use soil::{
    clear_grove, create_hard_graft, examine_specimen, exists, grow_branch, harvest_essence,
    inscribe_leaf, propagate_leaf, prune_branch, read_chronicle, shed_leaf, sprout_branch,
    survey_canopy, trace_to_root, transplant,
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
    /// soil trace ./test_root
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
    /// soil propagate source.txt destination.txt
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
    /// soil grow ./new/directory/path
    /// ```
    Grow {
        /// The directory path to create
        path: String,
    },

    /// Create a single directory (parent must exist)
    ///
    /// # Examples
    ///
    /// ```
    /// soil sprout ./parent/child
    /// ```
    Sprout {
        /// The directory path to create
        path: String,
    },

    /// List the contents of a directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil survey ./directory
    /// ```
    Survey {
        /// The directory path to list
        path: String,
    },

    /// Remove a file
    ///
    /// # Examples
    ///
    /// ```
    /// soil shed ./file.txt
    /// ```
    Shed {
        /// The file path to remove
        path: String,
    },

    /// Remove an empty directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil prune ./empty_directory
    /// ```
    Prune {
        /// The directory path to remove
        path: String,
    },

    /// Remove a directory and all its contents
    ///
    /// # Examples
    ///
    /// ```
    /// soil clear ./directory
    /// ```
    Clear {
        /// The directory path to remove recursively
        path: String,
    },

    /// Move or rename a file or directory
    ///
    /// # Examples
    ///
    /// ```
    /// soil transplant old_name.txt new_name.txt
    /// ```
    Transplant {
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
    /// soil examine ./file.txt
    /// ```
    Examine {
        /// The path to examine
        path: String,
    },

    /// Read file content as bytes
    ///
    /// # Examples
    ///
    /// ```
    /// soil harvest ./file.txt
    /// ```
    Harvest {
        /// The file path to read
        path: String,
    },

    /// Read file content as text
    ///
    /// # Examples
    ///
    /// ```
    /// soil chronicle ./file.txt
    /// ```
    Chronicle {
        /// The file path to read
        path: String,
    },

    /// Write content to a file
    ///
    /// # Examples
    ///
    /// ```
    /// soil inscribe ./file.txt "content"
    /// ```
    Inscribe {
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
    /// soil graft original.txt linked.txt
    /// ```
    Graft {
        /// The original file path
        original: String,
        /// The hard link path
        link: String,
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

        Commands::Sprout { path } => match sprout_branch(&path) {
            Ok(_) => {
                println!("Successfully sprouted branch at '{}'", path);
            }
            Err(error) => {
                eprintln!("Error sprouting branch '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Survey { path } => match survey_canopy(&path) {
            Ok(contents) => {
                println!("Contents of '{}':", path);
                for item in contents {
                    println!("  {}", item);
                }
            }
            Err(error) => {
                eprintln!("Error surveying canopy '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Shed { path } => match shed_leaf(&path) {
            Ok(_) => {
                println!("Successfully shed leaf '{}'", path);
            }
            Err(error) => {
                eprintln!("Error shedding leaf '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Prune { path } => match prune_branch(&path) {
            Ok(_) => {
                println!("Successfully pruned branch '{}'", path);
            }
            Err(error) => {
                eprintln!("Error pruning branch '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Clear { path } => match clear_grove(&path) {
            Ok(_) => {
                println!("Successfully cleared grove '{}'", path);
            }
            Err(error) => {
                eprintln!("Error clearing grove '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Transplant { from, to } => match transplant(&from, &to) {
            Ok(_) => {
                println!("Successfully transplanted '{}' to '{}'", from, to);
            }
            Err(error) => {
                eprintln!("Error transplanting '{}' to '{}': {}", from, to, error);
                process::exit(1);
            }
        },

        Commands::Examine { path } => match examine_specimen(&path) {
            Ok(metadata) => {
                println!("Specimen '{}' characteristics:", path);
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
                eprintln!("Error examining specimen '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Harvest { path } => match harvest_essence(&path) {
            Ok(content) => {
                println!("Harvested {} bytes from '{}'", content.len(), path);
                match String::from_utf8(content) {
                    Ok(text) => println!("Content:\n{}", text),
                    Err(_) => println!("Content contains non-UTF8 data"),
                }
            }
            Err(error) => {
                eprintln!("Error harvesting essence from '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Chronicle { path } => match read_chronicle(&path) {
            Ok(content) => {
                println!("Chronicle from '{}':", path);
                println!("{}", content);
            }
            Err(error) => {
                eprintln!("Error reading chronicle from '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Inscribe { path, content } => match inscribe_leaf(&path, &content) {
            Ok(_) => {
                println!(
                    "Successfully inscribed leaf at '{}' ({} characters)",
                    path,
                    content.len()
                );
            }
            Err(error) => {
                eprintln!("Error inscribing leaf at '{}': {}", path, error);
                process::exit(1);
            }
        },

        Commands::Graft { original, link } => match create_hard_graft(&original, &link) {
            Ok(_) => {
                println!(
                    "Successfully created hard graft from '{}' to '{}'",
                    original, link
                );
            }
            Err(error) => {
                eprintln!(
                    "Error creating hard graft from '{}' to '{}': {}",
                    original, link, error
                );
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
