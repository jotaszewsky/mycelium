extern crate rand;

use self::rand::{thread_rng, Rng};
use self::rand::distributions::Alphanumeric;

use application::event_source::EventSource;
use application::Value;

use std::fs::read_to_string;
use std::fs::write;
use std::fs::remove_file;
use std::path::PathBuf;

pub fn save(message: &str, output: &PathBuf) -> Result<(), ()> {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    write(output.join(format!("{}.json", rand_string)), message.as_bytes()).unwrap();
    Ok(())
}

pub fn load(input: &PathBuf, remove_used: bool, event_source: EventSource) -> Result<(), ()> {
    if input.is_file() {
        event_source.notify(Value {
            data: read_to_string(input).unwrap()
        });
        if remove_used {
            remove_file(input).expect("Something went wrong deleting the file")
        }
    }
    if input.is_dir() {
        for entry in input.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                event_source.notify(Value {
                    // to chyba jest bug?
                    data: read_to_string(input).unwrap()
                });
                if remove_used {
                    remove_file(entry.path()).expect("Something went wrong deleting the file")
                }
            }
        }
    }
    Ok(())
}
