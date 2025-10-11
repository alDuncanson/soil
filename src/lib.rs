use std::fs;

pub const TEST_ROOT: &str = "./test_root";

/// Returns the canonical, absolute path of a file or directory.
///
/// This function resolves symbolic links and relative path components (like `..` and `.`)
/// to produce an absolute path. It's equivalent to following the path to its root in the
/// file system tree.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to canonicalize
///
/// # Returns
///
/// Returns `Ok(String)` containing the canonical path if successful, or an `Err` with
/// the underlying `std::io::Error` if the path cannot be resolved.
///
/// # Examples
///
/// ```
/// use soil::trace_to_root;
///
/// // Trace a relative path to its absolute form
/// let canonical = trace_to_root("./src").unwrap();
/// assert!(canonical.ends_with("src"));
/// assert!(canonical.starts_with("/"));
/// ```
///
/// ```
/// use soil::trace_to_root;
///
/// // Non-existent paths return an error
/// assert!(trace_to_root("/non/existent/path").is_err());
/// ```
pub fn trace_to_root(path: &str) -> Result<String, std::io::Error> {
    match fs::canonicalize(path) {
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(error) => Err(error),
    }
}

/// Copies a file from the scion (source) to the rootstock (destination).
///
/// In botanical terms, propagation is the process of creating new plants from existing ones.
/// Here, we propagate a leaf (file) by copying it from the scion to the rootstock.
/// This operation preserves the original file while creating an exact copy.
///
/// # Arguments
///
/// * `scion` - A string slice that holds the path to the source file
/// * `rootstock` - A string slice that holds the path where the file should be copied
///
/// # Returns
///
/// Returns `Ok(())` if the copy operation succeeds, or an `Err` with the underlying
/// `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::propagate_leaf;
/// use std::fs;
///
/// // Create a source file
/// fs::write("source.txt", "content").unwrap();
///
/// // Propagate the leaf to a new location
/// propagate_leaf("source.txt", "destination.txt").unwrap();
///
/// // Verify the content was copied
/// let content = fs::read_to_string("destination.txt").unwrap();
/// assert_eq!(content, "content");
///
/// // Clean up
/// fs::remove_file("source.txt").unwrap();
/// fs::remove_file("destination.txt").unwrap();
/// ```
pub fn propagate_leaf(scion: &str, rootstock: &str) -> Result<(), std::io::Error> {
    match fs::copy(scion, rootstock) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

/// Creates a directory branch and all its parent directories if they don't exist.
///
/// In botanical terms, a branch is a structural part of a tree that grows from the trunk
/// or another branch. Here, we grow a branch in the file system by creating a directory
/// path, including any necessary parent directories.
///
/// # Arguments
///
/// * `path` - A string slice that holds the directory path to create
///
/// # Returns
///
/// Returns `Ok(())` if the directory creation succeeds, or an `Err` with the underlying
/// `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::grow_branch;
/// use std::fs;
/// use std::path::Path;
///
/// // Grow a new branch with nested directories
/// grow_branch("test/deep/nested/path").unwrap();
///
/// // Verify the branch exists
/// assert!(Path::new("test/deep/nested/path").exists());
///
/// // Clean up
/// fs::remove_dir_all("test").unwrap();
/// ```
///
/// ```
/// use soil::grow_branch;
/// use std::path::Path;
///
/// // Growing an existing branch is safe
/// grow_branch("existing").unwrap();
/// grow_branch("existing").unwrap(); // No error
///
/// assert!(Path::new("existing").exists());
/// std::fs::remove_dir("existing").unwrap();
/// ```
pub fn grow_branch(path: &str) -> Result<(), std::io::Error> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

/// Checks if a path exists in the file system.
///
/// This function determines whether a given path points to an existing file or directory.
/// It returns `false` for non-existent paths or when permission is denied to access the path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to check
///
/// # Returns
///
/// Returns `true` if the path exists and is accessible, `false` otherwise.
///
/// # Examples
///
/// ```
/// use soil::{exists, grow_branch};
/// use std::fs;
///
/// // Check if a directory exists
/// grow_branch("temp_dir").unwrap();
/// assert!(exists("temp_dir"));
///
/// // Check if a file exists
/// fs::write("temp_file.txt", "content").unwrap();
/// assert!(exists("temp_file.txt"));
///
/// // Non-existent paths return false
/// assert!(!exists("non_existent_path"));
///
/// // Clean up
/// fs::remove_dir("temp_dir").unwrap();
/// fs::remove_file("temp_file.txt").unwrap();
/// ```
pub fn exists(path: &str) -> bool {
    fs::exists(path).unwrap_or(false)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_trace_to_root() {
        let canonical_path = trace_to_root(&format!("{TEST_ROOT}/test"));

        assert!(canonical_path.is_ok(), "Failed to canonicalize path");
    }

    #[test]
    fn test_propagate_leaf() {
        let _ = grow_branch(&format!("{TEST_ROOT}/test"));

        let scion = &*format!("{TEST_ROOT}/test/scion.txt");
        let rootstock = &*format!("{TEST_ROOT}/test/rootstock.txt");

        std::fs::write(scion, "test content").expect("Failed to create test file");

        assert!(
            propagate_leaf(scion, rootstock).is_ok(),
            "Failed to propagate leaf"
        );

        assert!(
            std::fs::exists(rootstock).unwrap_or(false),
            "Rootstock file was not created"
        );
    }

    #[test]
    fn test_grow_branch() {
        let new_branch = grow_branch(&format!("{TEST_ROOT}/test"));

        assert!(new_branch.is_ok(), "Failed to grow a branch");
    }

    #[test]
    fn test_exists() {
        let path = &*format!("{TEST_ROOT}/test");

        assert!(exists(path), "Path does not exist");
    }
}
