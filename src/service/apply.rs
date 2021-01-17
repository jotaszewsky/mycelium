extern crate serde_yaml;

use std::fs::read_to_string;
use std::path::PathBuf;

use application::state::State;
use application::Temporary;
use Input;
use Output;

const READ_TEMP_KEY: &str = "read";
const WRITE_TEMP_KEY: &str = "write";

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
struct File {
    pub input: Input,
    pub output: Vec<Output>
}

pub fn execute(path: PathBuf) -> Result<(),()> {
    if !path.is_file() {
        panic!("Wrong input path");
    }
    let mut temp = State::new(None);
    let file: File = serde_yaml::from_str(&read_to_string(path).unwrap()).unwrap();

    temp.store(String::from(READ_TEMP_KEY), bincode::serialize(&file.input).unwrap())?;
    temp.store(String::from(WRITE_TEMP_KEY), bincode::serialize(&file.output).unwrap())?;
    Ok(())
}
