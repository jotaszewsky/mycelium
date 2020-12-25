extern crate rand;

use application::event_source::EventSource;
use application::Value;
use serde_json::json;
use colored_json::to_colored_json_auto;

use std::io;

pub fn save(message: &String, header: &Option<String>, pretty_json: bool) -> Result<(), ()> {
    if let Some(header) = header {
        println!("\n## HEADERS:\n");
        match pretty_json {
            true => println!("{}", to_colored_json_auto(&json!(header)).unwrap()),
            false => println!("{}", header)
        }
    }
    println!("\n## BODY:\n");
    match pretty_json {
        true => println!("{}", to_colored_json_auto(&json!(message)).unwrap()),
        false => println!("{}", message)
    }
    Ok(())
}

pub fn load(event_source: EventSource) -> Result<(), ()> {
    let mut input = String::new();
    println!("Please type json message...");
    io::stdin().read_line(&mut input).unwrap();
    event_source.notify(Value {
        data: input,
        header: None
    });
    Ok(())
}
