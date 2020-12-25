use application::Observer;
use application::Value;
use serde_json::json;
use colored_json::to_colored_json_auto;

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
        if let Some(header) = &value.header {
            println!("\n## HEADERS:\n");
            match self.pretty_json {
                true => println!("{}", to_colored_json_auto(&json!(header)).unwrap()),
                false => println!("{}", header)
            }
        }
        println!("\n## BODY:\n");
        match self.pretty_json {
            true => println!("{}", to_colored_json_auto(&json!(value.data)).unwrap()),
            false => println!("{}", value.data)
        }
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