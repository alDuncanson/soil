use soil::*;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Soil Path Ergonomics Example");
    println!("Demonstrating how AsRef<Path> improves usability\n");

    // Clean up any existing test files
    let _ = clear_grove("ergonomics_demo");

    // Create a base directory for our demo
    grow_branch("ergonomics_demo")?;

    // === String literals work (as before) ===
    println!("1. Using string literals (works as before):");
    inscribe_leaf(
        "ergonomics_demo/string_literal.txt",
        "Hello from string literal!",
    )?;
    let content = read_chronicle("ergonomics_demo/string_literal.txt")?;
    println!("   Content: {}", content);

    // === String variables work (as before) ===
    println!("\n2. Using String variables (works as before):");
    let file_path = String::from("ergonomics_demo/string_var.txt");
    inscribe_leaf(&file_path, "Hello from String variable!")?;
    let content = read_chronicle(&file_path)?;
    println!("   Content: {}", content);

    // === PathBuf works (NEW ergonomic improvement!) ===
    println!("\n3. Using PathBuf (NEW - more ergonomic!):");
    let mut path_buf = PathBuf::from("ergonomics_demo");
    path_buf.push("pathbuf.txt");
    inscribe_leaf(&path_buf, "Hello from PathBuf!")?;
    let content = read_chronicle(&path_buf)?;
    println!("   Content: {}", content);

    // === Path references work (NEW ergonomic improvement!) ===
    println!("\n4. Using &Path (NEW - more ergonomic!):");
    let path_ref = Path::new("ergonomics_demo/path_ref.txt");
    inscribe_leaf(path_ref, "Hello from &Path!")?;
    let content = read_chronicle(path_ref)?;
    println!("   Content: {}", content);

    // === Mixed types in same function call (NEW flexibility!) ===
    println!("\n5. Mixed path types in operations (NEW flexibility!):");
    let source = PathBuf::from("ergonomics_demo/pathbuf.txt");
    let dest = "ergonomics_demo/copied_file.txt"; // string literal

    propagate_leaf(&source, dest)?; // PathBuf source, string literal dest
    let content = read_chronicle(dest)?;
    println!("   Copied content: {}", content);

    // === Path building with join (very ergonomic!) ===
    println!("\n6. Path building with join (very ergonomic!):");
    let base_dir = Path::new("ergonomics_demo");
    let nested_file = base_dir.join("nested").join("deep").join("file.txt");

    grow_branch(nested_file.parent().unwrap())?; // Create parent dirs
    inscribe_leaf(&nested_file, "Deep nested content!")?;

    let canonical = trace_to_root(&nested_file)?;
    println!("   Canonical path: {}", canonical);

    // === Survey with different path types ===
    println!("\n7. Directory operations with mixed path types:");
    let base_path = PathBuf::from("ergonomics_demo");
    let contents = survey_canopy(&base_path)?;
    println!("   Directory contents:");
    for item in contents {
        println!("     - {}", item);
    }

    // === Demonstrate path operations ===
    println!("\n8. Path existence checks:");
    let test_paths = [
        PathBuf::from("ergonomics_demo/pathbuf.txt"),
        PathBuf::from("ergonomics_demo/nonexistent.txt"),
    ];

    for path in &test_paths {
        let exists_result = exists(path);
        println!("   {} exists: {}", path.display(), exists_result);
    }

    // === Function that accepts generic path parameter ===
    fn process_file<P: AsRef<Path>>(path: P) -> Result<usize, std::io::Error> {
        let content = read_chronicle(path)?;
        Ok(content.len())
    }

    println!("\n9. Generic function accepting any path type:");

    // Works with all these different types!
    let string_len = process_file("ergonomics_demo/string_literal.txt")?;
    let pathbuf_len = process_file(&PathBuf::from("ergonomics_demo/pathbuf.txt"))?;
    let path_len = process_file(Path::new("ergonomics_demo/path_ref.txt"))?;

    println!("   String literal file size: {} bytes", string_len);
    println!("   PathBuf file size: {} bytes", pathbuf_len);
    println!("   Path ref file size: {} bytes", path_len);

    // === Cleanup ===
    println!("\n🧹 Cleaning up demo files...");
    clear_grove("ergonomics_demo")?;
    println!("✅ Demo completed successfully!");

    println!("\n💡 Key Benefits of AsRef<Path>:");
    println!("   - Accept String, &str, PathBuf, &Path seamlessly");
    println!("   - No more .to_string() or .as_str() conversions");
    println!("   - Better integration with std::path types");
    println!("   - More idiomatic Rust path handling");
    println!("   - Backward compatible with existing string-based code");

    Ok(())
}
