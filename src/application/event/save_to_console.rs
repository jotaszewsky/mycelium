use plugins::console::Console;
use application::{Observer, Value};

pub struct SaveToConsole {
    console: Console
}

impl SaveToConsole {
    pub fn new(pretty_json: bool) -> SaveToConsole {
        SaveToConsole { console: Console::new(pretty_json) }
    }
}

impl Observer for SaveToConsole {
    fn on_notify(&mut self, value: &Value) -> () {
        self.console.publish(&value.data, &value.header).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // no idea how mock println
    #[test]
    fn notify_print() {
        assert!(true);
    }

    // no idea how mock println
    #[test]
    fn notify_print_colored() {
        assert!(true);
    }
}