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
/// use soil::{trace_to_root, grow_branch};
///
/// grow_branch("temp_trace_1").unwrap();
/// let canonical = trace_to_root("temp_trace_1").unwrap();
/// assert!(canonical.ends_with("temp_trace_1"));
/// assert!(canonical.starts_with("/"));
/// soil::clear_grove("temp_trace_1").unwrap();
/// ```
///
/// ```
/// use soil::trace_to_root;
///
/// assert!(trace_to_root("/non/existent/path").is_err());
/// ```
pub fn trace_to_root<P: AsRef<str>>(path: P) -> Result<String, std::io::Error> {
    match fs::canonicalize(path.as_ref()) {
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
/// use soil::{propagate_leaf, inscribe_leaf, read_chronicle};
///
/// inscribe_leaf("source_prop.txt", "content").unwrap();
/// propagate_leaf("source_prop.txt", "dest_prop.txt").unwrap();
/// let content = read_chronicle("dest_prop.txt").unwrap();
/// assert_eq!(content, "content");
/// soil::shed_leaf("source_prop.txt").unwrap();
/// soil::shed_leaf("dest_prop.txt").unwrap();
/// ```
pub fn propagate_leaf<P: AsRef<str>>(scion: P, rootstock: P) -> Result<(), std::io::Error> {
    match fs::copy(scion.as_ref(), rootstock.as_ref()) {
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
/// use soil::{grow_branch, exists, clear_grove};
///
/// grow_branch("test_grow/deep/nested/path").unwrap();
/// assert!(exists("test_grow/deep/nested/path"));
/// clear_grove("test_grow").unwrap();
/// ```
///
/// ```
/// use soil::{grow_branch, exists, shed_leaf};
///
/// grow_branch("existing_grow").unwrap();
/// grow_branch("existing_grow").unwrap();
/// assert!(exists("existing_grow"));
/// soil::prune_branch("existing_grow").unwrap();
/// ```
pub fn grow_branch<P: AsRef<str>>(path: P) -> Result<(), std::io::Error> {
    match fs::create_dir_all(path.as_ref()) {
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
/// use soil::{exists, grow_branch, inscribe_leaf};
///
/// grow_branch("temp_dir_exists").unwrap();
/// assert!(exists("temp_dir_exists"));
///
/// inscribe_leaf("temp_file_exists.txt", "content").unwrap();
/// assert!(exists("temp_file_exists.txt"));
///
/// assert!(!exists("non_existent_path"));
///
/// soil::prune_branch("temp_dir_exists").unwrap();
/// soil::shed_leaf("temp_file_exists.txt").unwrap();
/// ```
pub fn exists<P: AsRef<str>>(path: P) -> bool {
    fs::exists(path.as_ref()).unwrap_or(false)
}

/// Surveys the canopy of a directory, listing all its immediate contents.
///
/// In botanical terms, surveying the canopy means examining what grows directly
/// from a branch - the files and subdirectories contained within a directory.
/// This function returns an iterator over the directory entries.
///
/// # Arguments
///
/// * `path` - A string slice that holds the directory path to survey
///
/// # Returns
///
/// Returns `Ok(Vec<String>)` containing the names of all entries in the directory,
/// or an `Err` with the underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{grow_branch, survey_canopy, inscribe_leaf, clear_grove};
///
/// grow_branch("test_canopy_survey").unwrap();
/// inscribe_leaf("test_canopy_survey/leaf1.txt", "content1").unwrap();
/// inscribe_leaf("test_canopy_survey/leaf2.txt", "content2").unwrap();
/// grow_branch("test_canopy_survey/sub_branch").unwrap();
///
/// let contents = survey_canopy("test_canopy_survey").unwrap();
/// assert!(contents.len() >= 3);
/// assert!(contents.iter().any(|name| name.contains("leaf1.txt")));
/// assert!(contents.iter().any(|name| name.contains("leaf2.txt")));
/// assert!(contents.iter().any(|name| name.contains("sub_branch")));
///
/// clear_grove("test_canopy_survey").unwrap();
/// ```
pub fn survey_canopy<P: AsRef<str>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let entries = fs::read_dir(path.as_ref())?;
    let mut names = Vec::new();

    for entry in entries {
        match entry {
            Ok(dir_entry) => {
                if let Some(name) = dir_entry.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
            Err(error) => return Err(error),
        }
    }

    Ok(names)
}

/// Sprouts a single branch (directory) at the specified path.
///
/// Unlike `grow_branch` which creates all parent directories, this function
/// only creates the final directory in the path. The parent directory must
/// already exist for this operation to succeed.
///
/// # Arguments
///
/// * `path` - A string slice that holds the directory path to create
///
/// # Returns
///
/// Returns `Ok(())` if the directory creation succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{sprout_branch, grow_branch, exists, clear_grove};
///
/// grow_branch("parent_sprout").unwrap();
/// sprout_branch("parent_sprout/child").unwrap();
/// assert!(exists("parent_sprout/child"));
/// clear_grove("parent_sprout").unwrap();
/// ```
pub fn sprout_branch<P: AsRef<str>>(path: P) -> Result<(), std::io::Error> {
    fs::create_dir(path.as_ref())
}

/// Sheds a leaf (removes a file) from the file system.
///
/// In botanical terms, shedding a leaf is the natural process of a tree
/// dropping leaves. Here, we remove a file from the file system permanently.
///
/// # Arguments
///
/// * `path` - A string slice that holds the file path to remove
///
/// # Returns
///
/// Returns `Ok(())` if the file removal succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{shed_leaf, inscribe_leaf, exists};
///
/// inscribe_leaf("temp_leaf_shed.txt", "temporary content").unwrap();
/// shed_leaf("temp_leaf_shed.txt").unwrap();
/// assert!(!exists("temp_leaf_shed.txt"));
/// ```
pub fn shed_leaf<P: AsRef<str>>(path: P) -> Result<(), std::io::Error> {
    fs::remove_file(path.as_ref())
}

/// Prunes an empty branch (removes an empty directory).
///
/// In botanical terms, pruning involves cutting away dead or overgrown
/// branches. Here, we remove an empty directory from the file system.
/// The directory must be empty for this operation to succeed.
///
/// # Arguments
///
/// * `path` - A string slice that holds the directory path to remove
///
/// # Returns
///
/// Returns `Ok(())` if the directory removal succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{grow_branch, prune_branch, exists};
///
/// grow_branch("empty_branch_prune").unwrap();
/// assert!(exists("empty_branch_prune"));
/// prune_branch("empty_branch_prune").unwrap();
/// assert!(!exists("empty_branch_prune"));
/// ```
pub fn prune_branch<P: AsRef<str>>(path: P) -> Result<(), std::io::Error> {
    fs::remove_dir(path.as_ref())
}

/// Clears an entire grove (removes a directory and all its contents recursively).
///
/// In botanical terms, clearing a grove means removing all trees, branches,
/// and undergrowth in an area. Here, we remove a directory and everything
/// it contains, including subdirectories and files.
///
/// # Arguments
///
/// * `path` - A string slice that holds the directory path to remove recursively
///
/// # Returns
///
/// Returns `Ok(())` if the recursive removal succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{grow_branch, clear_grove, inscribe_leaf, exists};
///
/// grow_branch("grove_clear/deep/nested").unwrap();
/// inscribe_leaf("grove_clear/leaf.txt", "content").unwrap();
/// inscribe_leaf("grove_clear/deep/another_leaf.txt", "more content").unwrap();
/// clear_grove("grove_clear").unwrap();
/// assert!(!exists("grove_clear"));
/// ```
pub fn clear_grove<P: AsRef<str>>(path: P) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path.as_ref())
}

/// Transplants a file or directory from one location to another.
///
/// In botanical terms, transplanting involves moving a plant from one location
/// to another. Here, we move or rename a file or directory. This operation
/// can move items across different directories or simply rename them in place.
///
/// # Arguments
///
/// * `from` - A string slice that holds the current path
/// * `to` - A string slice that holds the destination path
///
/// # Returns
///
/// Returns `Ok(())` if the transplant succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{transplant, inscribe_leaf, exists, shed_leaf};
///
/// inscribe_leaf("original_trans.txt", "content").unwrap();
/// transplant("original_trans.txt", "transplanted_trans.txt").unwrap();
/// assert!(!exists("original_trans.txt"));
/// assert!(exists("transplanted_trans.txt"));
/// shed_leaf("transplanted_trans.txt").unwrap();
/// ```
pub fn transplant<P: AsRef<str>>(from: P, to: P) -> Result<(), std::io::Error> {
    fs::rename(from.as_ref(), to.as_ref())
}

/// Examines the vital characteristics of a specimen (gets file or directory metadata).
///
/// In botanical terms, examining a specimen involves studying its characteristics
/// like size, age, and type. Here, we gather metadata about a file or directory
/// including its size, modification time, and file type.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to examine
///
/// # Returns
///
/// Returns `Ok(fs::Metadata)` containing the file metadata if successful,
/// or an `Err` with the underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{examine_specimen, inscribe_leaf, shed_leaf};
///
/// inscribe_leaf("specimen_exam.txt", "sample content").unwrap();
/// let metadata = examine_specimen("specimen_exam.txt").unwrap();
/// assert!(metadata.len() > 0);
/// assert!(metadata.is_file());
/// shed_leaf("specimen_exam.txt").unwrap();
/// ```
pub fn examine_specimen<P: AsRef<str>>(path: P) -> Result<fs::Metadata, std::io::Error> {
    fs::metadata(path.as_ref())
}

/// Harvests the essence of a leaf as raw bytes (reads entire file content as bytes).
///
/// In botanical terms, harvesting essence means extracting the vital substances
/// from a plant. Here, we read the complete content of a file as raw bytes.
///
/// # Arguments
///
/// * `path` - A string slice that holds the file path to read
///
/// # Returns
///
/// Returns `Ok(Vec<u8>)` containing the file contents as bytes if successful,
/// or an `Err` with the underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{harvest_essence, inscribe_leaf, shed_leaf};
///
/// inscribe_leaf("essence_harvest.txt", "binary content").unwrap();
/// let content = harvest_essence("essence_harvest.txt").unwrap();
/// assert_eq!(content, b"binary content");
/// shed_leaf("essence_harvest.txt").unwrap();
/// ```
pub fn harvest_essence<P: AsRef<str>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path.as_ref())
}

/// Reads the chronicle inscribed in a leaf (reads file content as UTF-8 text).
///
/// In botanical terms, reading a chronicle means interpreting the story
/// written in the rings or markings of a plant. Here, we read the complete
/// text content of a file as a UTF-8 string.
///
/// # Arguments
///
/// * `path` - A string slice that holds the file path to read
///
/// # Returns
///
/// Returns `Ok(String)` containing the file contents as text if successful,
/// or an `Err` with the underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{read_chronicle, inscribe_leaf, shed_leaf};
///
/// inscribe_leaf("chronicle_read.txt", "Once upon a time...").unwrap();
/// let story = read_chronicle("chronicle_read.txt").unwrap();
/// assert_eq!(story, "Once upon a time...");
/// shed_leaf("chronicle_read.txt").unwrap();
/// ```
pub fn read_chronicle<P: AsRef<str>>(path: P) -> Result<String, std::io::Error> {
    fs::read_to_string(path.as_ref())
}

/// Inscribes content onto a leaf (writes data to a file).
///
/// In botanical terms, inscribing involves marking or writing information
/// onto a plant surface. Here, we write content to create or overwrite
/// a file. This function accepts both text strings and byte arrays.
///
/// # Arguments
///
/// * `path` - A string slice that holds the file path to write to
/// * `contents` - The content to write (can be string or bytes)
///
/// # Returns
///
/// Returns `Ok(())` if the write succeeds, or an `Err` with the underlying
/// `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{inscribe_leaf, read_chronicle, harvest_essence, shed_leaf};
///
/// inscribe_leaf("story_inscribe.txt", "A story of growth").unwrap();
/// let text = read_chronicle("story_inscribe.txt").unwrap();
/// assert_eq!(text, "A story of growth");
///
/// inscribe_leaf("data_inscribe.bin", b"binary data").unwrap();
/// let bytes = harvest_essence("data_inscribe.bin").unwrap();
/// assert_eq!(bytes, b"binary data");
///
/// shed_leaf("story_inscribe.txt").unwrap();
/// shed_leaf("data_inscribe.bin").unwrap();
/// ```
pub fn inscribe_leaf<P: AsRef<str>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> Result<(), std::io::Error> {
    fs::write(path.as_ref(), contents)
}

/// Creates a hard graft between two specimens (creates a hard link).
///
/// In botanical terms, grafting creates a permanent connection between two plants.
/// A hard graft represents a fundamental structural connection. Here, we create
/// a hard link where both paths refer to the same underlying file data.
///
/// # Arguments
///
/// * `original` - A string slice that holds the path to the existing file
/// * `link` - A string slice that holds the path where the hard link will be created
///
/// # Returns
///
/// Returns `Ok(())` if the hard link creation succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{inscribe_leaf, create_hard_graft, examine_specimen, shed_leaf};
///
/// inscribe_leaf("original_graft.txt", "shared content").unwrap();
/// create_hard_graft("original_graft.txt", "grafted_hard.txt").unwrap();
///
/// let orig_meta = examine_specimen("original_graft.txt").unwrap();
/// let graft_meta = examine_specimen("grafted_hard.txt").unwrap();
/// assert_eq!(orig_meta.len(), graft_meta.len());
///
/// shed_leaf("original_graft.txt").unwrap();
/// shed_leaf("grafted_hard.txt").unwrap();
/// ```
pub fn create_hard_graft<P: AsRef<str>>(original: P, link: P) -> Result<(), std::io::Error> {
    fs::hard_link(original.as_ref(), link.as_ref())
}

/// Reads the destination of a soft connection (follows a symbolic link).
///
/// In botanical terms, reading a soft connection means following the path
/// of a grafted connection to see where it leads. Here, we read the target
/// path that a symbolic link points to.
///
/// # Arguments
///
/// * `path` - A string slice that holds the symbolic link path to read
///
/// # Returns
///
/// Returns `Ok(String)` containing the target path if successful,
/// or an `Err` with the underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{inscribe_leaf, create_soft_connection, read_soft_connection, shed_leaf};
///
/// inscribe_leaf("target_soft.txt", "original content").unwrap();
/// create_soft_connection("target_soft.txt", "link_soft.txt").unwrap();
/// let target = read_soft_connection("link_soft.txt").unwrap();
/// assert_eq!(target, "target_soft.txt");
/// shed_leaf("target_soft.txt").unwrap();
/// shed_leaf("link_soft.txt").unwrap();
/// ```
pub fn read_soft_connection<P: AsRef<str>>(path: P) -> Result<String, std::io::Error> {
    match fs::read_link(path.as_ref()) {
        Ok(path_buf) => Ok(path_buf.to_string_lossy().into_owned()),
        Err(error) => Err(error),
    }
}

/// Adjusts the vitality of a specimen (sets file or directory permissions).
///
/// In botanical terms, adjusting vitality means changing the health and access
/// characteristics of a plant. Here, we modify the permissions of a file or
/// directory to control access rights.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to modify
/// * `permissions` - The new permissions to set
///
/// # Returns
///
/// Returns `Ok(())` if the permission change succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{inscribe_leaf, examine_specimen, adjust_vitality, shed_leaf};
///
/// inscribe_leaf("specimen_vital.txt", "content").unwrap();
/// let metadata = examine_specimen("specimen_vital.txt").unwrap();
/// let mut perms = metadata.permissions();
/// perms.set_readonly(true);
/// adjust_vitality("specimen_vital.txt", perms).unwrap();
/// shed_leaf("specimen_vital.txt").unwrap();
/// ```
pub fn adjust_vitality<P: AsRef<str>>(
    path: P,
    permissions: fs::Permissions,
) -> Result<(), std::io::Error> {
    fs::set_permissions(path.as_ref(), permissions)
}

/// Examines the outer characteristics of a specimen without following connections (gets symlink metadata).
///
/// In botanical terms, examining outer characteristics means studying what's visible
/// on the surface without following any grafted connections. Here, we get metadata
/// about a symbolic link itself rather than the file it points to.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to examine
///
/// # Returns
///
/// Returns `Ok(fs::Metadata)` containing the symlink metadata if successful,
/// or an `Err` with the underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{inscribe_leaf, create_soft_connection, examine_outer_characteristics, shed_leaf};
///
/// inscribe_leaf("target_outer.txt", "content").unwrap();
/// create_soft_connection("target_outer.txt", "link_outer.txt").unwrap();
/// let metadata = examine_outer_characteristics("link_outer.txt").unwrap();
/// assert!(metadata.file_type().is_symlink());
/// shed_leaf("target_outer.txt").unwrap();
/// shed_leaf("link_outer.txt").unwrap();
/// ```
pub fn examine_outer_characteristics<P: AsRef<str>>(
    path: P,
) -> Result<fs::Metadata, std::io::Error> {
    fs::symlink_metadata(path.as_ref())
}

/// Creates a soft connection between two specimens (creates a symbolic link).
///
/// In botanical terms, a soft connection represents a flexible link that can
/// point to different locations. Unlike hard grafts, soft connections can span
/// different file systems and can point to non-existent targets.
///
/// # Arguments
///
/// * `original` - A string slice that holds the target path
/// * `link` - A string slice that holds the symbolic link path to create
///
/// # Returns
///
/// Returns `Ok(())` if the symbolic link creation succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{inscribe_leaf, create_soft_connection, read_soft_connection, shed_leaf};
///
/// inscribe_leaf("original_connect.txt", "content").unwrap();
/// create_soft_connection("original_connect.txt", "soft_link_connect.txt").unwrap();
/// let target = read_soft_connection("soft_link_connect.txt").unwrap();
/// assert_eq!(target, "original_connect.txt");
/// shed_leaf("original_connect.txt").unwrap();
/// shed_leaf("soft_link_connect.txt").unwrap();
/// ```
#[cfg(unix)]
pub fn create_soft_connection<P: AsRef<str>>(original: P, link: P) -> Result<(), std::io::Error> {
    std::os::unix::fs::symlink(original.as_ref(), link.as_ref())
}

/// Creates a soft connection between two specimens (creates a symbolic link).
///
/// Windows version that handles both files and directories.
///
/// # Arguments
///
/// * `original` - A string slice that holds the target path
/// * `link` - A string slice that holds the symbolic link path to create
///
/// # Returns
///
/// Returns `Ok(())` if the symbolic link creation succeeds, or an `Err` with the
/// underlying `std::io::Error` if the operation fails.
///
/// # Examples
///
/// ```
/// use soil::{inscribe_leaf, create_soft_connection, read_soft_connection, shed_leaf};
///
/// inscribe_leaf("original.txt", "content").unwrap();
/// create_soft_connection("original.txt", "soft_link.txt").unwrap();
/// let target = read_soft_connection("soft_link.txt").unwrap();
/// assert_eq!(target, "original.txt");
/// shed_leaf("original.txt").unwrap();
/// shed_leaf("soft_link.txt").unwrap();
/// ```
#[cfg(windows)]
pub fn create_soft_connection<P: AsRef<str>>(original: P, link: P) -> Result<(), std::io::Error> {
    use std::path::Path;
    let original_path = Path::new(original.as_ref());
    if original_path.is_dir() {
        std::os::windows::fs::symlink_dir(original.as_ref(), link.as_ref())
    } else {
        std::os::windows::fs::symlink_file(original.as_ref(), link.as_ref())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn get_unique_test_root() -> String {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("{}_test_{}", TEST_ROOT, id)
    }

    struct TestGuard {
        test_root: String,
    }

    impl Drop for TestGuard {
        fn drop(&mut self) {
            let _ = clear_grove(&self.test_root);
        }
    }

    fn setup_test() -> TestGuard {
        let test_root = get_unique_test_root();
        let _ = grow_branch(&test_root);
        TestGuard { test_root }
    }

    #[test]
    fn test_trace_to_root() {
        let guard = setup_test();
        let _ = grow_branch(&format!("{}/test", guard.test_root));
        let canonical_path = trace_to_root(&format!("{}/test", guard.test_root));

        assert!(canonical_path.is_ok(), "Failed to canonicalize path");
    }

    #[test]
    fn test_propagate_leaf() {
        let guard = setup_test();
        let _ = grow_branch(&format!("{}/test", guard.test_root));

        let scion = &*format!("{}/test/scion.txt", guard.test_root);
        let rootstock = &*format!("{}/test/rootstock.txt", guard.test_root);

        inscribe_leaf(scion, "test content").expect("Failed to create test file");

        assert!(
            propagate_leaf(scion, rootstock).is_ok(),
            "Failed to propagate leaf"
        );

        assert!(exists(rootstock), "Rootstock file was not created");
    }

    #[test]
    fn test_survey_canopy() {
        let guard = setup_test();
        let test_dir = &*format!("{}/survey_test", guard.test_root);
        let _ = grow_branch(test_dir);

        inscribe_leaf(&format!("{}/file1.txt", test_dir), "content1").unwrap();
        inscribe_leaf(&format!("{}/file2.txt", test_dir), "content2").unwrap();
        let _ = grow_branch(&format!("{}/subdir", test_dir));

        let contents = survey_canopy(test_dir).unwrap();
        assert!(contents.len() >= 3);
        assert!(contents.contains(&"file1.txt".to_string()));
        assert!(contents.contains(&"file2.txt".to_string()));
        assert!(contents.contains(&"subdir".to_string()));
    }

    #[test]
    fn test_sprout_branch() {
        let guard = setup_test();
        let parent_dir = &*format!("{}/sprout_parent", guard.test_root);
        let child_dir = &*format!("{}/sprout_parent/sprout_child", guard.test_root);

        let _ = grow_branch(parent_dir);
        assert!(sprout_branch(child_dir).is_ok());
        assert!(exists(child_dir));
    }

    #[test]
    fn test_shed_leaf() {
        let guard = setup_test();
        let test_file = &*format!("{}/temp_leaf.txt", guard.test_root);
        inscribe_leaf(test_file, "temporary content").unwrap();
        assert!(exists(test_file));

        assert!(shed_leaf(test_file).is_ok());
        assert!(!exists(test_file));
    }

    #[test]
    fn test_prune_branch() {
        let guard = setup_test();
        let test_dir = &*format!("{}/empty_branch", guard.test_root);
        let _ = sprout_branch(test_dir);
        assert!(exists(test_dir));

        assert!(prune_branch(test_dir).is_ok());
        assert!(!exists(test_dir));
    }

    #[test]
    fn test_clear_grove() {
        let guard = setup_test();
        let grove_dir = &*format!("{}/test_grove", guard.test_root);
        let _ = grow_branch(&format!("{}/deep/nested", grove_dir));
        inscribe_leaf(&format!("{}/leaf.txt", grove_dir), "content").unwrap();
        inscribe_leaf(
            &format!("{}/deep/another_leaf.txt", grove_dir),
            "more content",
        )
        .unwrap();

        assert!(clear_grove(grove_dir).is_ok());
        assert!(!exists(grove_dir));
    }

    #[test]
    fn test_transplant() {
        let guard = setup_test();
        let original = &*format!("{}/original.txt", guard.test_root);
        let transplanted = &*format!("{}/transplanted.txt", guard.test_root);

        inscribe_leaf(original, "content").unwrap();
        assert!(exists(original));

        assert!(transplant(original, transplanted).is_ok());
        assert!(!exists(original));
        assert!(exists(transplanted));
    }

    #[test]
    fn test_examine_specimen() {
        let guard = setup_test();
        let test_file = &*format!("{}/specimen.txt", guard.test_root);
        inscribe_leaf(test_file, "sample content").unwrap();

        let metadata = examine_specimen(test_file).unwrap();
        assert!(metadata.len() > 0);
        assert!(metadata.is_file());
    }

    #[test]
    fn test_harvest_essence() {
        let guard = setup_test();
        let test_file = &*format!("{}/essence.txt", guard.test_root);
        let content = b"binary content";
        inscribe_leaf(test_file, content).unwrap();

        let harvested = harvest_essence(test_file).unwrap();
        assert_eq!(harvested, content);
    }

    #[test]
    fn test_read_chronicle() {
        let guard = setup_test();
        let test_file = &*format!("{}/chronicle.txt", guard.test_root);
        let content = "Once upon a time...";
        inscribe_leaf(test_file, content).unwrap();

        let chronicle = read_chronicle(test_file).unwrap();
        assert_eq!(chronicle, content);
    }

    #[test]
    fn test_inscribe_leaf_text() {
        let guard = setup_test();
        let test_file = &*format!("{}/inscribed_text.txt", guard.test_root);
        let content = "A story of growth and change";

        assert!(inscribe_leaf(test_file, content).is_ok());
        let retrieved = read_chronicle(test_file).unwrap();
        assert_eq!(retrieved, content);
    }

    #[test]
    fn test_inscribe_leaf_bytes() {
        let guard = setup_test();
        let test_file = &*format!("{}/inscribed_bytes.bin", guard.test_root);
        let content = b"binary content";

        assert!(inscribe_leaf(test_file, content).is_ok());
        let retrieved = harvest_essence(test_file).unwrap();
        assert_eq!(retrieved, content);
    }

    #[test]
    fn test_create_hard_graft() {
        let guard = setup_test();
        let original = &*format!("{}/graft_original.txt", guard.test_root);
        let grafted = &*format!("{}/graft_linked.txt", guard.test_root);

        inscribe_leaf(original, "shared content").unwrap();

        assert!(create_hard_graft(original, grafted).is_ok());
        assert!(exists(grafted));

        let orig_content = harvest_essence(original).unwrap();
        let graft_content = harvest_essence(grafted).unwrap();
        assert_eq!(orig_content, graft_content);
    }

    #[test]
    fn test_grow_branch() {
        let guard = setup_test();
        let new_branch = grow_branch(&format!("{}/test", guard.test_root));

        assert!(new_branch.is_ok(), "Failed to grow a branch");
    }

    #[test]
    fn test_exists() {
        let guard = setup_test();
        let _ = grow_branch(&format!("{}/test", guard.test_root));
        let path = &*format!("{}/test", guard.test_root);

        assert!(exists(path), "Path does not exist");
    }

    #[test]
    #[cfg(unix)]
    fn test_create_soft_connection() {
        let guard = setup_test();
        let original = &*format!("{}/original_soft.txt", guard.test_root);
        let link = &*format!("{}/soft_link.txt", guard.test_root);

        inscribe_leaf(original, "soft content").unwrap();
        assert!(create_soft_connection("original_soft.txt", link).is_ok());
        assert!(exists(link));

        let target = read_soft_connection(link).unwrap();
        assert_eq!(target, "original_soft.txt");
    }

    #[test]
    fn test_read_soft_connection() {
        let guard = setup_test();
        let original = &*format!("{}/read_target.txt", guard.test_root);
        let link = &*format!("{}/read_link.txt", guard.test_root);

        inscribe_leaf(original, "target content").unwrap();
        #[cfg(unix)]
        {
            create_soft_connection("read_target.txt", link).unwrap();
            let target = read_soft_connection(link).unwrap();
            assert_eq!(target, "read_target.txt");
        }
    }

    #[test]
    fn test_adjust_vitality() {
        let guard = setup_test();
        let test_file = &*format!("{}/vitality_test.txt", guard.test_root);
        inscribe_leaf(test_file, "vitality content").unwrap();

        let metadata = examine_specimen(test_file).unwrap();
        let mut perms = metadata.permissions();
        let original_readonly = perms.readonly();
        perms.set_readonly(!original_readonly);

        assert!(adjust_vitality(test_file, perms).is_ok());

        let new_metadata = examine_specimen(test_file).unwrap();
        assert_eq!(new_metadata.permissions().readonly(), !original_readonly);
    }

    #[test]
    #[cfg(unix)]
    fn test_examine_outer_characteristics() {
        let guard = setup_test();
        let original = &*format!("{}/outer_target.txt", guard.test_root);
        let link = &*format!("{}/outer_link.txt", guard.test_root);

        inscribe_leaf(original, "outer content").unwrap();
        create_soft_connection("outer_target.txt", link).unwrap();

        let metadata = examine_outer_characteristics(link).unwrap();
        assert!(metadata.file_type().is_symlink());

        let target_metadata = examine_specimen(original).unwrap();
        assert!(target_metadata.is_file());
    }
}
