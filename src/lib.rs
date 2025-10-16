use std::fs;
use std::path::Path;

pub const TEST_ROOT: &str = "./test_root";

/// Return the canonical absolute path of a file or directory.
///
/// Resolves symbolic links and relative components (`.` and `..`).
///
/// Arguments
/// - `path`: Path to resolve
///
/// Returns the canonical absolute path on success.
///
/// Examples
///
/// ```
/// use soil::{ensure_dir, resolve_path, remove_dir_all};
///
/// ensure_dir("temp_trace_1").unwrap();
/// let canonical = resolve_path("temp_trace_1").unwrap();
/// assert!(canonical.ends_with("temp_trace_1"));
/// assert!(canonical.starts_with("/"));
/// remove_dir_all("temp_trace_1").unwrap();
/// ```
///
/// ```
/// use soil::resolve_path;
/// assert!(resolve_path("/non/existent/path").is_err());
/// ```
pub fn resolve_path<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    match fs::canonicalize(path.as_ref()) {
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(error) => Err(error),
    }
}

/// Copy a file from `src` to `dst`.
///
/// Arguments
/// - `src`: Source file path
/// - `dst`: Destination file path
///
/// Examples
/// ```
/// use soil::{copy_file, write_file, read_text, remove_file};
/// write_file("copy_src.txt", "content").unwrap();
/// copy_file("copy_src.txt", "copy_dst.txt").unwrap();
/// let content = read_text("copy_dst.txt").unwrap();
/// assert_eq!(content, "content");
/// remove_file("copy_src.txt").unwrap();
/// remove_file("copy_dst.txt").unwrap();
/// ```
pub fn copy_file<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) -> Result<(), std::io::Error> {
    match fs::copy(src.as_ref(), dst.as_ref()) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

/// Create a directory and all missing parents (mkdir -p).
///
/// Examples
/// ```
/// use soil::{ensure_dir, exists, remove_dir_all};
/// ensure_dir("tmp_mkdirp/deep/nested/path").unwrap();
/// assert!(exists("tmp_mkdirp/deep/nested/path"));
/// remove_dir_all("tmp_mkdirp").unwrap();
/// ```
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    match fs::create_dir_all(path.as_ref()) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

/// Check whether a path exists.
///
/// Returns `true` if the file or directory exists, `false` otherwise.
///
/// Examples
/// ```
/// use soil::{exists, ensure_dir, write_file};
/// ensure_dir("temp_dir_exists").unwrap();
/// assert!(exists("temp_dir_exists"));
/// write_file("temp_file_exists.txt", "content").unwrap();
/// assert!(exists("temp_file_exists.txt"));
/// assert!(!exists("non_existent_path"));
/// soil::remove_empty_dir("temp_dir_exists").unwrap();
/// soil::remove_file("temp_file_exists.txt").unwrap();
/// ```
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    fs::exists(path.as_ref()).unwrap_or(false)
}

/// List the immediate contents of a directory.
///
/// Returns the entry names (files and subdirectories) as strings.
///
/// Examples
/// ```
/// use soil::{ensure_dir, list_dir, write_file, remove_dir_all};
/// ensure_dir("test_list").unwrap();
/// write_file("test_list/file1.txt", "content1").unwrap();
/// write_file("test_list/file2.txt", "content2").unwrap();
/// ensure_dir("test_list/subdir").unwrap();
/// let contents = list_dir("test_list").unwrap();
/// assert!(contents.len() >= 3);
/// assert!(contents.iter().any(|name| name.contains("file1.txt")));
/// assert!(contents.iter().any(|name| name.contains("file2.txt")));
/// assert!(contents.iter().any(|name| name.contains("subdir")));
/// remove_dir_all("test_list").unwrap();
/// ```
pub fn list_dir<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
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

/// Create a single directory. Parent must already exist.
///
/// Examples
/// ```
/// use soil::{create_dir, ensure_dir, exists, remove_dir_all};
/// ensure_dir("parent_dir").unwrap();
/// create_dir("parent_dir/child").unwrap();
/// assert!(exists("parent_dir/child"));
/// remove_dir_all("parent_dir").unwrap();
/// ```
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    fs::create_dir(path.as_ref())
}

/// Remove a file.
///
/// Examples
/// ```
/// use soil::{remove_file, write_file, exists};
/// write_file("temp_remove.txt", "temporary content").unwrap();
/// remove_file("temp_remove.txt").unwrap();
/// assert!(!exists("temp_remove.txt"));
/// ```
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    fs::remove_file(path.as_ref())
}

/// Remove an empty directory.
///
/// Examples
/// ```
/// use soil::{ensure_dir, remove_empty_dir, exists};
/// ensure_dir("empty_dir").unwrap();
/// assert!(exists("empty_dir"));
/// remove_empty_dir("empty_dir").unwrap();
/// assert!(!exists("empty_dir"));
/// ```
pub fn remove_empty_dir<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    fs::remove_dir(path.as_ref())
}

/// Remove a directory and all of its contents (recursive delete).
///
/// Examples
/// ```
/// use soil::{ensure_dir, remove_dir_all, write_file, exists};
/// ensure_dir("tmp_dir/deep/nested").unwrap();
/// write_file("tmp_dir/file.txt", "content").unwrap();
/// write_file("tmp_dir/deep/another.txt", "more content").unwrap();
/// remove_dir_all("tmp_dir").unwrap();
/// assert!(!exists("tmp_dir"));
/// ```
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path.as_ref())
}

/// Move or rename a file or directory.
///
/// Examples
/// ```
/// use soil::{move_path, write_file, exists, remove_file};
/// write_file("original.txt", "content").unwrap();
/// move_path("original.txt", "moved.txt").unwrap();
/// assert!(!exists("original.txt"));
/// assert!(exists("moved.txt"));
/// remove_file("moved.txt").unwrap();
/// ```
pub fn move_path<P1: AsRef<Path>, P2: AsRef<Path>>(from: P1, to: P2) -> Result<(), std::io::Error> {
    fs::rename(from.as_ref(), to.as_ref())
}

/// Get metadata for a file or directory.
///
/// Examples
/// ```
/// use soil::{metadata, write_file, remove_file};
/// write_file("file.txt", "sample content").unwrap();
/// let md = metadata("file.txt").unwrap();
/// assert!(md.len() > 0);
/// assert!(md.is_file());
/// remove_file("file.txt").unwrap();
/// ```
pub fn metadata<P: AsRef<Path>>(path: P) -> Result<fs::Metadata, std::io::Error> {
    fs::metadata(path.as_ref())
}

/// Read entire file contents as bytes.
///
/// Examples
/// ```
/// use soil::{read_bytes, write_file, remove_file};
/// write_file("bytes.txt", "binary content").unwrap();
/// let content = read_bytes("bytes.txt").unwrap();
/// assert_eq!(content, b"binary content");
/// remove_file("bytes.txt").unwrap();
/// ```
pub fn read_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path.as_ref())
}

/// Read entire file contents as UTF-8 text.
///
/// Examples
/// ```
/// use soil::{read_text, write_file, remove_file};
/// write_file("read_text_example.txt", "Hello, world!").unwrap();
/// let text = read_text("read_text_example.txt").unwrap();
/// assert_eq!(text, "Hello, world!");
/// remove_file("read_text_example.txt").unwrap();
/// ```
pub fn read_text<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    fs::read_to_string(path.as_ref())
}

/// Write data to a file (creates or overwrites).
///
/// Accepts either text or bytes.
///
/// Examples
/// ```
/// use soil::{write_file, read_text, read_bytes, remove_file};
/// write_file("story.txt", "Hello").unwrap();
/// let text = read_text("story.txt").unwrap();
/// assert_eq!(text, "Hello");
/// write_file("data.bin", b"binary data").unwrap();
/// let bytes = read_bytes("data.bin").unwrap();
/// assert_eq!(bytes, b"binary data");
/// remove_file("story.txt").unwrap();
/// remove_file("data.bin").unwrap();
/// ```
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> Result<(), std::io::Error> {
    fs::write(path.as_ref(), contents)
}

/// Create a hard link.
///
/// Examples
/// ```
/// use soil::{write_file, create_hard_link, metadata, remove_file};
/// write_file("hardlink_original.txt", "shared content").unwrap();
/// create_hard_link("hardlink_original.txt", "hardlink_link.txt").unwrap();
/// let orig_meta = metadata("hardlink_original.txt").unwrap();
/// let link_meta = metadata("hardlink_link.txt").unwrap();
/// assert_eq!(orig_meta.len(), link_meta.len());
/// remove_file("hardlink_original.txt").unwrap();
/// remove_file("hardlink_link.txt").unwrap();
/// ```
pub fn create_hard_link<P1: AsRef<Path>, P2: AsRef<Path>>(
    original: P1,
    link: P2,
) -> Result<(), std::io::Error> {
    fs::hard_link(original.as_ref(), link.as_ref())
}

/// Read the target of a symbolic link (like `readlink`).
pub fn read_symlink<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    match fs::read_link(path.as_ref()) {
        Ok(path_buf) => Ok(path_buf.to_string_lossy().into_owned()),
        Err(error) => Err(error),
    }
}

/// Set file or directory permissions.
pub fn set_permissions<P: AsRef<Path>>(
    path: P,
    permissions: fs::Permissions,
) -> Result<(), std::io::Error> {
    fs::set_permissions(path.as_ref(), permissions)
}

/// Get metadata for a path without following symlinks (like `lstat`).
pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<fs::Metadata, std::io::Error> {
    fs::symlink_metadata(path.as_ref())
}

/// Create a symbolic link.
#[cfg(unix)]
pub fn create_symlink<P1: AsRef<Path>, P2: AsRef<Path>>(
    original: P1,
    link: P2,
) -> Result<(), std::io::Error> {
    std::os::unix::fs::symlink(original.as_ref(), link.as_ref())
}

/// Create a symbolic link (Windows: supports files and directories).
#[cfg(windows)]
pub fn create_symlink<P1: AsRef<Path>, P2: AsRef<Path>>(
    original: P1,
    link: P2,
) -> Result<(), std::io::Error> {
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
            let _ = remove_dir_all(&self.test_root);
        }
    }

    fn setup_test() -> TestGuard {
        let test_root = get_unique_test_root();
        let _ = ensure_dir(&test_root);
        TestGuard { test_root }
    }

    #[test]
    fn test_resolve_path() {
        let guard = setup_test();
        let _ = ensure_dir(format!("{}/test", guard.test_root));
        let canonical_path = resolve_path(format!("{}/test", guard.test_root));

        assert!(canonical_path.is_ok(), "Failed to canonicalize path");
    }

    #[test]
    fn test_copy_file() {
        let guard = setup_test();
        let _ = ensure_dir(format!("{}/test", guard.test_root));

        let src = &*format!("{}/test/src.txt", guard.test_root);
        let dst = &*format!("{}/test/dst.txt", guard.test_root);

        write_file(src, "test content").expect("Failed to create test file");

        assert!(copy_file(src, dst).is_ok(), "Failed to copy file");

        assert!(exists(dst), "Destination file was not created");
    }

    #[test]
    fn test_list_dir() {
        let guard = setup_test();
        let test_dir = &*format!("{}/list_test", guard.test_root);
        let _ = ensure_dir(test_dir);

        write_file(format!("{}/file1.txt", test_dir), "content1").unwrap();
        write_file(format!("{}/file2.txt", test_dir), "content2").unwrap();
        let _ = ensure_dir(format!("{}/subdir", test_dir));

        let contents = list_dir(test_dir).unwrap();
        assert!(contents.len() >= 3);
        assert!(contents.contains(&"file1.txt".to_string()));
        assert!(contents.contains(&"file2.txt".to_string()));
        assert!(contents.contains(&"subdir".to_string()));
    }

    #[test]
    fn test_create_dir() {
        let guard = setup_test();
        let parent_dir = &*format!("{}/mkdir_parent", guard.test_root);
        let child_dir = &*format!("{}/mkdir_parent/mkdir_child", guard.test_root);

        let _ = ensure_dir(parent_dir);
        assert!(create_dir(child_dir).is_ok());
        assert!(exists(child_dir));
    }

    #[test]
    fn test_remove_file() {
        let guard = setup_test();
        let test_file = &*format!("{}/temp_remove.txt", guard.test_root);
        write_file(test_file, "temporary content").unwrap();
        assert!(exists(test_file));

        assert!(remove_file(test_file).is_ok());
        assert!(!exists(test_file));
    }

    #[test]
    fn test_remove_empty_dir() {
        let guard = setup_test();
        let test_dir = &*format!("{}/empty_dir", guard.test_root);
        let _ = create_dir(test_dir);
        assert!(exists(test_dir));

        assert!(remove_empty_dir(test_dir).is_ok());
        assert!(!exists(test_dir));
    }

    #[test]
    fn test_remove_dir_all() {
        let guard = setup_test();
        let root_dir = &*format!("{}/test_dir", guard.test_root);
        let _ = ensure_dir(format!("{}/deep/nested", root_dir));
        write_file(format!("{}/file.txt", root_dir), "content").unwrap();
        write_file(format!("{}/deep/another.txt", root_dir), "more content").unwrap();

        assert!(remove_dir_all(root_dir).is_ok());
        assert!(!exists(root_dir));
    }

    #[test]
    fn test_move_path() {
        let guard = setup_test();
        let original = &*format!("{}/original.txt", guard.test_root);
        let moved = &*format!("{}/moved.txt", guard.test_root);

        write_file(original, "content").unwrap();
        assert!(exists(original));

        assert!(move_path(original, moved).is_ok());
        assert!(!exists(original));
        assert!(exists(moved));
    }

    #[test]
    fn test_metadata() {
        let guard = setup_test();
        let test_file = &*format!("{}/file.txt", guard.test_root);
        write_file(test_file, "sample content").unwrap();

        let md = metadata(test_file).unwrap();
        assert!(md.len() > 0);
        assert!(md.is_file());
    }

    #[test]
    fn test_read_bytes() {
        let guard = setup_test();
        let test_file = &*format!("{}/bytes_test.txt", guard.test_root);
        let content = b"binary content";
        write_file(test_file, content).unwrap();

        let harvested = read_bytes(test_file).unwrap();
        assert_eq!(harvested, content);
    }

    #[test]
    fn test_read_text() {
        let guard = setup_test();
        let test_file = &*format!("{}/text.txt", guard.test_root);
        let content = "Once upon a time...";
        write_file(test_file, content).unwrap();

        let txt = read_text(test_file).unwrap();
        assert_eq!(txt, content);
    }

    #[test]
    fn test_write_file_text() {
        let guard = setup_test();
        let test_file = &*format!("{}/write_text.txt", guard.test_root);
        let content = "Hello";

        assert!(write_file(test_file, content).is_ok());
        let retrieved = read_text(test_file).unwrap();
        assert_eq!(retrieved, content);
    }

    #[test]
    fn test_write_file_bytes() {
        let guard = setup_test();
        let test_file = &*format!("{}/write_bytes.bin", guard.test_root);
        let content = b"binary content";

        assert!(write_file(test_file, content).is_ok());
        let retrieved = read_bytes(test_file).unwrap();
        assert_eq!(retrieved, content);
    }

    #[test]
    fn test_create_hard_link() {
        let guard = setup_test();
        let original = &*format!("{}/hardlink_original.txt", guard.test_root);
        let linked = &*format!("{}/hardlink_linked.txt", guard.test_root);

        write_file(original, "shared content").unwrap();

        assert!(create_hard_link(original, linked).is_ok());
        assert!(exists(linked));

        let orig_content = read_bytes(original).unwrap();
        let link_content = read_bytes(linked).unwrap();
        assert_eq!(orig_content, link_content);
    }

    #[test]
    fn test_ensure_dir() {
        let guard = setup_test();
        let created_dir = ensure_dir(format!("{}/test", guard.test_root));

        assert!(created_dir.is_ok(), "Failed to create directory");
    }

    #[test]
    fn test_exists() {
        let guard = setup_test();
        let _ = ensure_dir(format!("{}/test", guard.test_root));
        let path = &*format!("{}/test", guard.test_root);

        assert!(exists(path), "Path does not exist");
    }

    #[test]
    #[cfg(unix)]
    fn test_create_symlink() {
        let guard = setup_test();
        let original = &*format!("{}/symlink_target.txt", guard.test_root);
        let link = &*format!("{}/symlink_link.txt", guard.test_root);

        write_file(original, "soft content").unwrap();
        assert!(create_symlink("symlink_target.txt", link).is_ok());
        assert!(exists(link));

        let target = read_symlink(link).unwrap();
        assert_eq!(target, "symlink_target.txt");
    }

    #[test]
    fn test_read_symlink() {
        let guard = setup_test();
        let original = &*format!("{}/read_target.txt", guard.test_root);
        let link = &*format!("{}/read_link.txt", guard.test_root);

        write_file(original, "target content").unwrap();
        #[cfg(unix)]
        {
            create_symlink("read_target.txt", link).unwrap();
            let target = read_symlink(link).unwrap();
            assert_eq!(target, "read_target.txt");
        }
    }

    #[test]
    fn test_set_permissions() {
        let guard = setup_test();
        let test_file = &*format!("{}/perms_test.txt", guard.test_root);
        write_file(test_file, "content").unwrap();

        let md = metadata(test_file).unwrap();
        let mut perms = md.permissions();
        let original_readonly = perms.readonly();
        perms.set_readonly(!original_readonly);

        assert!(set_permissions(test_file, perms).is_ok());

        let new_md = metadata(test_file).unwrap();
        assert_eq!(new_md.permissions().readonly(), !original_readonly);
    }

    #[test]
    #[cfg(unix)]
    fn test_symlink_metadata() {
        let guard = setup_test();
        let original = &*format!("{}/lstat_target.txt", guard.test_root);
        let link = &*format!("{}/lstat_link.txt", guard.test_root);

        write_file(original, "outer content").unwrap();
        create_symlink("lstat_target.txt", link).unwrap();

        let md = symlink_metadata(link).unwrap();
        assert!(md.file_type().is_symlink());

        let target_md = metadata(original).unwrap();
        assert!(target_md.is_file());
    }
}
