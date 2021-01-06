extern crate rand;

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

    pub fn publish(&mut self, message: &String, header: &Option<String>) -> Result<(), ()> {
        if let Some(header) = header {
            println!("\n## HEADERS:\n");
            match self.pretty_json {
                true => println!("{}", to_colored_json_auto(&json!(header)).unwrap()),
                false => println!("{}", header)
            }
        }
        println!("\n## BODY:\n");
        match self.pretty_json {
            true => println!("{}", to_colored_json_auto(&json!(message)).unwrap()),
            false => println!("{}", message)
        }
        Ok(())
    }

    pub fn consume(&mut self, event_source: EventSource) -> Result<(), ()> {
        let mut input = String::new();
        println!("Please type json message...");
        io::stdin().read_line(&mut input).unwrap();
        event_source.notify(Value {
            data: input,
            header: None
        });
        Ok(())
    }
}

