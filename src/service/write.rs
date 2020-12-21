use application::state::State;
use application::Temporary;
use Output;

const TEMP_KEY: &str = "write";

pub fn execute(output: Output) -> Result<(),()> {
    let mut temp = State::new(None);
    let mut output_vec: Vec<Output> = Vec::new();
    output_vec.push(output);
    temp.store(String::from(TEMP_KEY), bincode::serialize(&output_vec).unwrap())
}
