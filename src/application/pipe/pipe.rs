use application::Value;
use Middleware;

#[derive(Debug)]
pub struct Pipe {
    middlewares: Vec<Middleware>
}

impl Pipe {
    pub fn new(middlewares: Vec<Middleware>) -> Pipe {
        Pipe { middlewares }
    }

    pub fn dispatch(&self, mut value: Value) -> Value {
        if self.middlewares.len() > 0 {
            for middleware in &self.middlewares {
                match middleware {
                    Middleware::JQ { query } => {
                        value.data = jq_rs::run(&query, &value.data).unwrap();
                    }
                }
            }
        }
        return value;
    }
}

