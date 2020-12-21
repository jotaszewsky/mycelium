use application::state::State;
use application::Temporary;
use Input;

const TEMP_KEY: &str = "read";

pub fn execute(input: Input) -> Result<(),()> {
    let mut temp = State::new(None);
    temp.store(String::from(TEMP_KEY), bincode::serialize(&input).unwrap())
}
