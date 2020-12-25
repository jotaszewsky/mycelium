use std::fs::write;
use std::fs::metadata;
use std::fs::remove_file;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use application::Temporary;

#[derive(Debug)]
pub struct State {
    path: PathBuf
}

impl State {
    pub fn new(path: Option<PathBuf>) -> State {
        match path {
            Some(path) => State { path },
            None => State { path: PathBuf::from("/tmp/mycelium") }
        }
    }
}

impl Temporary for State {
    fn store(&mut self, key: String, data: Vec<u8>) -> Result<(), ()> {
        create_dir_all(&self.path).unwrap();
        write(&self.path.join(key), data).unwrap();
        Ok(())
    }

    fn read(&mut self, key: String) -> Result<Vec<u8>, std::io::Error> {
        let mut f = File::open(self.path.join(&key).to_str().unwrap())?;
        let metadata = metadata(&self.path.join(&key))?;
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer)?;

        Ok(buffer)
    }

    fn clear(&mut self, key: String) -> () {
        remove_file(&self.path.join(key)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;
    use std::fs;
    use self::rand::Rng;

    fn generate_path(post_string: Option<String>) -> PathBuf {
        match post_string {
            Some(post) => PathBuf::from(format!("/tmp/test-{}{}", rand::thread_rng().gen_range(0, 100000000), post)),
            None => PathBuf::from(format!("/tmp/test-{}", rand::thread_rng().gen_range(0, 100000000)))
        }
    }

    #[test]
    fn constructor_default_path() {
        let path_buffer_default: PathBuf = PathBuf::from("/tmp/mycelium");
        assert_eq!(State::new(None).path, path_buffer_default );
    }

    #[test]
    fn constructor_custom_path() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        assert_eq!(State::new(Some(path.to_path_buf())).path, path.to_path_buf() );
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn store_create_directory() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        state.store(String::from("qwertyuiop"), String::from("test").into_bytes()).unwrap();
        assert!(state.path.is_dir());
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn store_create_nested_directory() {
        let path = &generate_path(Some(String::from("/nested")));
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        state.store(String::from("qwertyuiop"), String::from("test").into_bytes()).unwrap();
        assert!(state.path.is_dir());

        let mut path_to_buf = path.to_path_buf();
        path_to_buf.pop();
        let _ = fs::remove_dir_all(path_to_buf);
    }

    #[test]
    fn store_using_key_as_filename() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        state.store(String::from("qwertyuiop"), String::from("test").into_bytes()).unwrap();
        assert!(state.path.join(String::from("qwertyuiop")).is_file());
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn store_using_key_with_spaces_as_filename() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        state.store(String::from("qwertyuiop asdfghjkl"), String::from("test").into_bytes()).unwrap();
        assert_eq!(state.path.join(String::from("qwertyuiop asdfghjkl")).is_file(), true);
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn store_using_key_with_dots_as_filename() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        state.store(String::from(".qwertyuiop"), String::from("test").into_bytes()).unwrap();
        assert_eq!(state.path.join(String::from(".qwertyuiop")).is_file(), true);
        let _ = fs::remove_dir_all(path);
    }

    // maybe error better ?
    #[test]
    fn store_using_key_with_special_chars_as_filename() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        state.store(String::from("!@#$%^&*"), String::from("test").into_bytes()).unwrap();
        assert_eq!(state.path.join(String::from(".qwertyuiop")).is_file(), false);
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn store_write_binary_to_file() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        state.store(String::from("qwertyuiop"), String::from("test").into_bytes()).unwrap();
        let read: String = String::from_utf8_lossy(
            &fs::read(state.path.join(String::from("qwertyuiop"))).unwrap()
        ).to_string();
        assert_eq!(read, String::from("test"));
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    #[should_panic]
    fn read_wrong_key_error() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);
        let mut state: State = State::new(Some(path.to_path_buf()));
        match state.read(String::from("qwertyuiop")) {
            Err(_err) => panic!(),
            Ok(_value) => ()
        }
    }

    #[test]
    #[should_panic]
    fn read_not_binary_value_error() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("not_binary"), "test").unwrap();

        let mut state: State = State::new(Some(path.to_path_buf()));
        match state.read(String::from("not_binary")) {
            Err(_err) => {
                let _ = fs::remove_dir_all(path);
                panic!()
            },
            Ok(value) => {
                let _ = fs::remove_dir_all(path);
                let response: String = bincode::deserialize(&value).unwrap();
                assert_eq!(response, String::from("test"));
            }
        }
    }

    #[test]
    fn read_binary_value_response() {
        let path = &generate_path(None);
        let _ = fs::remove_dir_all(path);

        let _ = fs::create_dir_all(path).unwrap();

        let mut output_vec: Vec<String>;
        output_vec = Vec::new();
        output_vec.push(String::from("test"));
        fs::write(path.join("binary"), bincode::serialize(&output_vec).unwrap()).unwrap();

        let mut state: State = State::new(Some(path.to_path_buf()));
        match state.read(String::from("binary")) {
            Err(_err) => {
                let _ = fs::remove_dir_all(path);
                panic!()
            },
            Ok(value) => {
                let _ = fs::remove_dir_all(path);
                let response: Vec<String> = bincode::deserialize(&value).unwrap();
                for output in response {
                    assert_eq!(output, String::from("test"));
                }
            }
        }
    }
}
