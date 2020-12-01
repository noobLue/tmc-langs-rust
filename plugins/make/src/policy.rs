//! Contains the language policy for the plugin.

use std::path::{Path, PathBuf};
use tmc_langs_framework::policy::StudentFilePolicy;

pub struct MakeStudentFilePolicy {
    config_file_parent_path: PathBuf,
}

impl MakeStudentFilePolicy {
    pub fn new(config_file_parent_path: PathBuf) -> Self {
        Self {
            config_file_parent_path,
        }
    }
}

impl StudentFilePolicy for MakeStudentFilePolicy {
    fn get_config_file_parent_path(&self) -> &Path {
        &self.config_file_parent_path
    }

    fn is_student_source_file(path: &Path) -> bool {
        path.starts_with("src")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_student_source_file() {
        assert!(MakeStudentFilePolicy::is_student_source_file(Path::new(
            "src"
        )));
        assert!(MakeStudentFilePolicy::is_student_source_file(Path::new(
            "src/file"
        )));
        assert!(MakeStudentFilePolicy::is_student_source_file(Path::new(
            "src/dir/file"
        )));
    }

    #[test]
    fn is_not_student_source_file() {
        assert!(!MakeStudentFilePolicy::is_student_source_file(Path::new(
            "srcc"
        )));
        assert!(!MakeStudentFilePolicy::is_student_source_file(Path::new(
            "dir/src/file"
        )));
    }
}
