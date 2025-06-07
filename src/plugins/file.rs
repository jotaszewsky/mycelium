extern crate rand;

use self::rand::{thread_rng, Rng};
use self::rand::distributions::Alphanumeric;

use application::event_source::EventSource;
use application::Value;

use std::fs::{read, read_to_string, write, remove_file};
use std::path::PathBuf;

pub struct File {
    path: PathBuf,
    filename_pattern: Option<FilenamePatterns>
}

impl File {

    pub fn new(path: PathBuf, filename_pattern: Option<FilenamePatterns>) -> File {
        File { path, filename_pattern }
    }

    pub fn publish(&mut self, message: &[u8], header: &Option<String>) -> Result<(), ()> {
        match self.filename_pattern {
            Some(FilenamePatterns::random) | None => {
                let filename = generate_random_filename();
                write(self.path.join(&filename), message).unwrap();
                if let Some(header) = header {
                    write(self.path.join(format!("{}.header", &filename)), header.as_bytes()).unwrap()
                }
            },
            Some(FilenamePatterns::index) => {
                let filename = generate_index_filename(&self.path);
                write(self.path.join(&filename), message).unwrap();
                if let Some(header) = header {
                    write(self.path.join(format!("{}.header", &filename)), header.as_bytes()).unwrap()
                }
            }
        }
        Ok(())
    }

    pub fn consume(&mut self, remove_used: bool, event_source: EventSource) -> Result<(), ()> {
        if !self.path.is_file() && !self.path.is_dir() {
            panic!("Wrong input path");
        }
        if self.path.is_file() {
            event_source.notify(Value {
                data: read(&self.path).unwrap(),
                header: header_read_to_string(&self.path)
            });
            if remove_used {
                remove_file(&self.path).expect("Something went wrong deleting the file");
                header_remove_file(&self.path).expect("Something went wrong deleting the file")
            }
        }
        if self.path.is_dir() {
            for entry in self.path.read_dir()
                .unwrap()
                .filter_map(Result::ok)
                .filter(|file| file.path().extension().unwrap() == "json")
                .into_iter() {
                event_source.notify(Value {
                    data: read(entry.path()).unwrap(),
                    header: header_read_to_string(&entry.path())
                });
                if remove_used {
                    remove_file(entry.path()).expect("Something went wrong deleting the file");
                    header_remove_file(&entry.path()).expect("Something went wrong deleting the file")
                }
            }
        }
        Ok(())
    }
}

fn header_read_to_string(path: &PathBuf) -> Option<String> {
    if path.join(String::from(".header")).is_file() {
        return Some(read_to_string(&path.join(String::from(".header"))).unwrap())
    }
    None
}

fn header_remove_file(path: &PathBuf) -> Result<(), ()> {
    if path.join(String::from(".header")).is_file() {
        remove_file(path.join(String::from(".header")));
    }
    Ok(())
}

fn generate_random_filename() -> String {
    let random: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    format!("{}.json", random)
}

fn generate_index_filename(output: &PathBuf) -> String {
    let index: String = (output.read_dir().unwrap().count()+1).to_string();
    format!("{}.json", index)
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum FilenamePatterns {
    random,
    index
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;
    use std::fs;
    use self::rand::Rng;
    use application::Observer;
    use application::Pipe;
    use std::sync::{Arc, Mutex};

    /*
    * How create test doubles in rust?
    * solution ?
    * Test double, i think
    */
    pub struct SaveToAssertMock {
        pub assert: String
    }

    impl Observer for SaveToAssertMock {
        fn on_notify(&mut self, value: &Value) -> () {
            assert!(true);
            assert_eq!(value.data, self.assert.as_bytes().to_vec());
        }
    }
    /*
    * end of test double
    */

    fn generate_path(post_string: Option<String>) -> PathBuf {
        match post_string {
            Some(post) => PathBuf::from(format!("/tmp/test-{}{}", rand::thread_rng().gen_range(0, 100000000), post)),
            None => PathBuf::from(format!("/tmp/test-{}", rand::thread_rng().gen_range(0, 100000000)))
        }
    }

    #[test]
    fn save_generate_random_name() {
        let random: String = generate_random_filename();
        assert_eq!(random.len(), 35);
        assert_ne!(random, generate_random_filename());
    }

    #[test]
    fn save_random_name_json() {
        let random: String = generate_random_filename();
        let extension: String = random.chars().skip(30).take(5).collect();
        assert_eq!(extension, String::from(".json"));
    }

    #[test]
    fn save_generate_index_name() {
        let path: PathBuf = generate_path(None);
        let _ = fs::remove_dir_all(&path);
        let _ = fs::create_dir_all(&path).unwrap();
        let name: String = generate_index_filename(&path);
        assert_eq!(name.len(), 6);
        assert_eq!(name, generate_index_filename(&path));
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn save_index_name_json() {
        let path: PathBuf = generate_path(None);
        let _ = fs::remove_dir_all(&path);
        let _ = fs::create_dir_all(&path).unwrap();
        let name: String = generate_index_filename(&path);
        let extension: String = name.chars().skip(1).take(5).collect();
        assert_eq!(extension, String::from(".json"));
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    #[should_panic]
    fn save_wrong_output_error() {
        let path: PathBuf = generate_path(None);
        let mut file: File = File::new(path, None);
        let _ = file.publish("test".as_bytes(), &None);
    }

    #[test]
    fn save_file_default() {
        let path: &PathBuf = &generate_path(None);
        let _ = fs::create_dir_all(&path).unwrap();
        let mut file: File = File::new(path.to_path_buf(), None);
        let _ = file.publish("test".as_bytes(), &None);

        for entry in path.read_dir().unwrap() {
            if let Ok(entry) = entry {
                assert_eq!(read_to_string(entry.path()).unwrap(), "test")
            }
        }
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn save_file_filename_index() {
        let path: &PathBuf = &generate_path(None);
        let _ = fs::create_dir_all(&path).unwrap();
        let mut file: File = File::new(path.to_path_buf(), Some(FilenamePatterns::index));
        let _ = file.publish("test".as_bytes(), &None);

        for entry in path.read_dir().unwrap() {
            if let Ok(entry) = entry {
                assert_eq!(read_to_string(entry.path()).unwrap(), "test")
            }
        }
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    #[should_panic]
    fn load_from_wrong_path_error() {
        let path: PathBuf = generate_path(None);
        let mut file: File = File::new(path, None);
        let _ = file.consume(
            false,
            EventSource::new(
                Pipe::new(
                    Vec::new()
                )
            )
        );
    }

    #[test]
    fn load_from_single_file_and_dont_remove() {
        let path: &PathBuf = &generate_path(None);

        let _ = fs::remove_dir_all(path);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("test-one.json"), "test").unwrap();

        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let mut file: File = File::new(path.to_path_buf().join("test-one.json"), None);
        let _ = file.consume(false, EventSource::new(
            Pipe::new(
                Vec::new()
            )
        ));

        assert!(path.join("test-one.json").is_file());

        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn load_from_single_file_and_remove() {
        let path: &PathBuf = &generate_path(None);

        let _ = fs::remove_dir_all(path);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("test-one.json"), "test").unwrap();

        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let mut file: File = File::new(path.to_path_buf().join("test-one.json"), None);
        let _ = file.consume(true, EventSource::new(
            Pipe::new(
                Vec::new()
            )
        ));

        assert!(!path.join("test-one.json").is_file());

        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn load_from_directory_and_dont_remove() {
        let path: &PathBuf = &generate_path(None);

        let _ = fs::remove_dir_all(path);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("test-one.json"), "test").unwrap();
        fs::write(path.join("test-two.json"), "test").unwrap();

        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let mut file: File = File::new(path.to_path_buf(), None);
        let _ = file.consume(false, event_source);

        assert!(path.join("test-one.json").is_file());
        assert!(path.join("test-two.json").is_file());

        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn load_from_directory_and_remove() {
        let path: &PathBuf = &generate_path(None);

        let _ = fs::remove_dir_all(path);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("test-one.json"), "test").unwrap();
        fs::write(path.join("test-two.json"), "test").unwrap();

        let mut event_source: EventSource = EventSource::new(
            Pipe::new(
                Vec::new()
            )
        );
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let mut file: File = File::new(path.to_path_buf(), None);
        let _ = file.consume(true, event_source);

        assert!(!path.join("test-one.json").is_file());
        assert!(!path.join("test-two.json").is_file());

        let _ = fs::remove_dir_all(path);
    }
}
