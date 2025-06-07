use plugins::file::{FilenamePatterns, File};
use application::{Observer, Value};

pub struct SaveToFile {
    file: File
}

impl SaveToFile {
    pub fn new(output: std::path::PathBuf, filename_pattern: Option<FilenamePatterns>) -> SaveToFile {
        SaveToFile { file: File::new(output, filename_pattern) }
    }
}

impl Observer for SaveToFile {
    fn on_notify(&mut self, value: &Value) -> () {
        self.file.publish(&value.data, &value.header).unwrap();
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;
    use std::fs;
    use self::rand::Rng;
    use std::path::PathBuf;

    fn generate_path(post_string: Option<String>) -> PathBuf {
        match post_string {
            Some(post) => PathBuf::from(format!("/tmp/test-{}{}", rand::thread_rng().gen_range(0, 100000000), post)),
            None => PathBuf::from(format!("/tmp/test-{}", rand::thread_rng().gen_range(0, 100000000)))
        }
    }

    #[test]
    fn constructor_no_errors() {
        let path = generate_path(None);
        SaveToFile::new(path, None);
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn notify_save_no_directory_err() {
        let path = &generate_path(None);
        let mut save_to_file: SaveToFile = SaveToFile::new(path.to_path_buf(), None);
        save_to_file.on_notify(&Value { data: String::from("test").into(), header: None });
    }

    #[test]
    fn notify_save_directory_exists() {
        let path = &generate_path(None);
        fs::create_dir_all(path).unwrap();

        let mut save_to_file: SaveToFile = SaveToFile::new(path.to_path_buf(), None);
        save_to_file.on_notify(&Value { data: String::from("test").into(), header: None });

        if path.is_dir() {
            for entry in path.read_dir().expect("read_dir call failed") {
                if let Ok(entry) = entry {
                    assert_eq!(fs::read_to_string(entry.path()).unwrap(), String::from("test"));
                }
            }
        }
        let _ = fs::remove_dir_all(path);
    }
}