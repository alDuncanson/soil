use std::fs;

pub const TEST_ROOT: &str = "./test_root";

pub fn trace_to_root(path: &str) -> Result<String, std::io::Error> {
    match fs::canonicalize(path) {
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(error) => Err(error),
    }
}

pub fn propagate_leaf(scion: &str, rootstock: &str) -> Result<(), std::io::Error> {
    match fs::copy(scion, rootstock) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

pub fn grow_branch(path: &str) -> Result<(), std::io::Error> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

pub fn exists(path: &str) -> bool {
    fs::exists(path).is_ok()
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
