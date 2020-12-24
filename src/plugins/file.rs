extern crate rand;

use self::rand::{thread_rng, Rng};
use self::rand::distributions::Alphanumeric;

use application::event_source::EventSource;
use application::Value;

use std::fs::read_to_string;
use std::fs::write;
use std::fs::remove_file;
use std::path::PathBuf;

pub fn save(message: &str, output: &PathBuf, filename_pattern: &Option<FilenamePatterns>) -> Result<(), ()> {
    match filename_pattern {
        Some(FilenamePatterns::random) => write(output.join(generate_random_filename()), message.as_bytes()).unwrap(),
        Some(FilenamePatterns::index) => write(output.join(generate_index_filename(output)), message.as_bytes()).unwrap(),
        None => write(output.join(generate_random_filename()), message.as_bytes()).unwrap()
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

pub fn load(input: &PathBuf, remove_used: bool, event_source: EventSource) -> Result<(), ()> {
    if !input.is_file() && !input.is_dir() {
        panic!("Wrong input path");
    }
    if input.is_file() {
        event_source.notify(Value {
            data: read_to_string(input).unwrap(),
            header: None
        });
        if remove_used {
            remove_file(input).expect("Something went wrong deleting the file")
        }
    }
    if input.is_dir() {
        for entry in input.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                event_source.notify(Value {
                    data: read_to_string(entry.path()).unwrap(),
                    header: None
                });
                if remove_used {
                    remove_file(entry.path()).expect("Something went wrong deleting the file")
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;
    use std::fs;
    use self::rand::Rng;
    use application::Observer;
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
            assert_eq!(value.data, self.assert);
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
        let _ = save("test", &path, &None);
    }

    #[test]
    fn save_file_default() {
        let path: PathBuf = generate_path(None);
        let _ = fs::create_dir_all(&path).unwrap();
        let _ = save("test", &path, &None);
        for entry in path.read_dir().unwrap() {
            if let Ok(entry) = entry {
                assert_eq!(read_to_string(entry.path()).unwrap(), "test")
            }
        }
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn save_file_filename_index() {
        let path: PathBuf = generate_path(None);
        let _ = fs::create_dir_all(&path).unwrap();
        let _ = save("test", &path, &Some(FilenamePatterns::index));
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
        let path: &PathBuf = &generate_path(None);
        let _ = load(path, false, EventSource::new());
    }

    #[test]
    fn load_from_single_file_and_dont_remove() {
        let path: &PathBuf = &generate_path(None);

        let _ = fs::remove_dir_all(path);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("test-one.json"), "test").unwrap();

        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let _ = load(&path.join("test-one.json"), false, event_source);

        assert!(path.join("test-one.json").is_file());

        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn load_from_single_file_and_remove() {
        let path: &PathBuf = &generate_path(None);

        let _ = fs::remove_dir_all(path);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("test-one.json"), "test").unwrap();

        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let _ = load(&path.join("test-one.json"), true, event_source);

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

        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let _ = load(&path, false, event_source);

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

        let mut event_source: EventSource = EventSource::new();
        event_source.register_observer(Arc::new(Mutex::new(SaveToAssertMock { assert: String::from("test") })));
        let _ = load(&path, true, event_source);

        assert!(!path.join("test-one.json").is_file());
        assert!(!path.join("test-two.json").is_file());

        let _ = fs::remove_dir_all(path);
    }
}
