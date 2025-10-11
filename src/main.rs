use clap::Parser;
use soil::TEST_ROOT;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let _ = soil::grow_branch(&format!("{TEST_ROOT}/test"));
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
