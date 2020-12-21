use application::state::State;
use application::Temporary;
use Output;

const TEMP_KEY: &str = "write";

pub fn execute(output: Output, clear: bool) -> Result<(),()> {
    let mut temp = State::new(None);
    if clear {
        temp.clear(String::from(TEMP_KEY));
    }
    let mut output_vec: Vec<Output>;
    match temp.read(String::from(TEMP_KEY)) {
        Ok(write) => {
            output_vec = bincode::deserialize(&write).unwrap();
        },
        Err(_err) => {
            output_vec = Vec::new();
        }
    }
    output_vec.push(output);
    temp.store(String::from(TEMP_KEY), bincode::serialize(&output_vec).unwrap())
}
