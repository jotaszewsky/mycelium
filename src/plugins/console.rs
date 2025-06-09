extern crate rand;

use console::style;
use application::event_source::EventSource;
use application::Value;
use serde_json::json;
use colored_json::to_colored_json_auto;
use std::io;

pub struct Console {
    pretty_json: bool
}

impl Console {

    pub fn new(pretty_json: bool) -> Console {
        Console { pretty_json }
    }

    pub fn publish(&mut self, value: &Value) -> Result<(), ()> {
        if let Some(header) = &value.header {
            println!("{}", style("Header:").cyan());
            match self.pretty_json {
                true => println!("{}", to_colored_json_auto(&json!(header)).unwrap()),
                false => println!("{}", header)
            }
        }
        println!("{}", style("Body:").cyan());
        let output: String = String::from_utf8_lossy(&value.data).to_string();
        match self.pretty_json {
            true => println!("{}", to_colored_json_auto(&json!(output)).unwrap()),
            false => println!("{}", output)
        }
        Ok(())
    }

    pub fn consume(&mut self, add_header: bool, event_source: EventSource) -> Result<(), ()> {
        let mut data: String = String::new();
        let mut header: Option<String> = None;
        println!("Please type json message...");
        io::stdin().read_line(&mut data).unwrap();
        if add_header {
            println!("Please type json header...");
            let mut line: String = String::new();
            io::stdin().read_line(&mut line).unwrap();
            header = Some(line);
        }
        event_source.notify(Value {
            data: data.as_bytes().to_vec(),
            header
        });
        Ok(())
    }
}

