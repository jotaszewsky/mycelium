extern crate serde_yaml;

use std::fs::read_to_string;
use std::path::PathBuf;

use application::state::State;
use application::Temporary;
use Input;
use Output;
use Middleware;

const READ_TEMP_KEY: &str = "read";
const WRITE_TEMP_KEY: &str = "write";
const MIDDLEWARE_TEMP_KEY: &str = "middleware";

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
struct File {
    pub input: Input,
    pub output: Vec<Output>,
    pub middleware: Option<Vec<Middleware>>
}

pub fn execute(path: PathBuf) -> Result<(),()> {
    if !path.is_file() || path.extension().unwrap() != "yaml" {
        panic!("Wrong input path or extension!");
    }
    let mut temp = State::new(None);
    let file: File = serde_yaml::from_str(&read_to_string(path).unwrap()).expect(
        "Wrong yaml format!"
    );
    let middleware: Vec<Middleware> = match file.middleware {
        Some(middleware) => middleware,
        None => Vec::new()
    };
    temp.store(String::from(READ_TEMP_KEY), bincode::serialize(&file.input).unwrap())?;
    temp.store(String::from(WRITE_TEMP_KEY), bincode::serialize(&file.output).unwrap())?;
    temp.store(String::from(MIDDLEWARE_TEMP_KEY), bincode::serialize(&middleware).unwrap())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_not_existing_file() {
        let path = PathBuf::from("/tmp/test_not_existing_file.yml");
        let _ = execute(path);
    }

    #[test]
    #[should_panic]
    fn test_not_yaml_extension() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/assets/apply/default.json");
        let _ = execute(path);
    }

    #[test]
    #[should_panic]
    fn test_not_yaml_format() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/assets/apply/wrong.yaml");
        let _ = execute(path);
    }

    /*
    * For the rest of tests we need test doubles.
    */
}