use application::state::State;
use application::Temporary;
use Middleware;

const TEMP_KEY: &str = "middleware";

pub fn execute(middleware: Middleware, clear: bool) -> Result<(),()> {
    let mut temp = State::new(None);
    if clear {
        temp.clear(String::from(TEMP_KEY));
    }
    let mut middleware_vec: Vec<Middleware>;
    match temp.read(String::from(TEMP_KEY)) {
        Ok(middleware) => {
            middleware_vec = bincode::deserialize(&middleware).unwrap();
        },
        Err(_err) => {
            middleware_vec = Vec::new();
        }
    }
    middleware_vec.push(middleware);
    temp.store(String::from(TEMP_KEY), bincode::serialize(&middleware_vec).unwrap())
}
