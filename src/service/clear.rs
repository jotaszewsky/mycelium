use application::state::State;
use application::Temporary;

const READ_TEMP_KEY: &str = "read";
const WRITE_TEMP_KEY: &str = "write";

pub fn execute() -> Result<(),()> {
    let mut temp = State::new(None);
    temp.clear(String::from(READ_TEMP_KEY));
    temp.clear(String::from(WRITE_TEMP_KEY));
    Ok(())
}
