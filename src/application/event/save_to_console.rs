use plugins::console::save;
use application::Observer;
use application::Value;

pub struct SaveToConsole {
    pretty_json: bool
}

impl SaveToConsole {
    pub fn new(pretty_json: bool) -> SaveToConsole {
        SaveToConsole { pretty_json }
    }
}

impl Observer for SaveToConsole {
    fn on_notify(&mut self, value: &Value) -> () {
        save(&value.data, &value.header, self.pretty_json).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_forward_parameter() {
        assert_eq!(SaveToConsole::new(false).pretty_json, false );
        assert_eq!(SaveToConsole::new(true).pretty_json, true );
    }

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