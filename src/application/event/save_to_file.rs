use plugins::file::{save, FilenamePatterns};
use application::{Observer, Value};

pub struct SaveToFile {
    output: std::path::PathBuf,
    filename_patter: Option<FilenamePatterns>
}

impl SaveToFile {
    pub fn new(output: std::path::PathBuf, filename_patter: Option<FilenamePatterns>) -> SaveToFile {
        SaveToFile { output, filename_patter }
    }
}

impl Observer for SaveToFile {
    fn on_notify(&mut self, value: &Value) -> () {
        save(&value.data, &self.output, &self.filename_patter).unwrap();
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
    fn constructor_forward_parameter() {
        let path = &generate_path(None);
        assert_eq!(SaveToFile::new(path.to_path_buf(), None).output.to_str(), path.to_str() );
    }

    #[test]
    #[should_panic]
    fn notify_save_no_directory_err() {
        let path = &generate_path(None);
        let mut save_to_file: SaveToFile = SaveToFile::new(path.to_path_buf(), None);
        save_to_file.on_notify(&Value { data: String::from("test"), header: None });
    }

    #[test]
    fn notify_save_directory_exists() {
        let path = &generate_path(None);
        fs::create_dir_all(path).unwrap();

        let mut save_to_file: SaveToFile = SaveToFile::new(path.to_path_buf(), None);
        save_to_file.on_notify(&Value { data: String::from("test"), header: None });

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