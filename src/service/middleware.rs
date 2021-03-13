use application::state::State;
use application::Temporary;
use Middleware;

const TEMP_KEY: &str = "middleware";

pub fn execute(middleware: Middleware) -> Result<(),()> {
    let mut temp = State::new(None);
    let mut middleware_vec: Vec<Middleware> = Vec::new();
    middleware_vec.push(middleware);
    temp.store(String::from(TEMP_KEY), bincode::serialize(&middleware_vec).unwrap())
}
