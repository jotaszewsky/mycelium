use application::state::State;
use application::Temporary;
use plugins;
use Output;
use Input;
use application::event::SaveToAmqp;
use application::event::SaveToFile;
use application::event::SaveToConsole;
use application::event_source::EventSource;

use std::sync::{Arc, Mutex};

pub fn execute() -> Result<(),()> {
    let mut event_source: EventSource = EventSource::new();
    let mut temp: State = State::new(None);

    match temp.read(String::from("write")) {
        Ok(write) => {
            let output_vec: Vec<Output> = bincode::deserialize(&write).unwrap();
            for output in output_vec {
                match output {
                    Output::Amqp { url, queue } => {
                        event_source.register_observer(Arc::new(Mutex::new(SaveToAmqp::new(
                            url,
                            queue
                        ))));
                    },
                    Output::File { output } => {
                        event_source.register_observer(Arc::new(Mutex::new(SaveToFile::new(
                            output
                        ))));
                    },
                    Output::Console { pretty_json } => {
                        event_source.register_observer(Arc::new(Mutex::new(SaveToConsole::new(
                            pretty_json
                        ))));
                    }
                }
            }
        },
        Err(_err) => {
            println!("Write source not set");
        }
    }

    match temp.read(String::from("read")) {
        Ok(read) => {
            let input: Input = bincode::deserialize(&read).unwrap();
            match input {
                Input::Amqp { url, queue, queue_arguments, acknowledgement, count, prefetch_count } => {
                    plugins::amqp::consume(&url, &queue, queue_arguments, acknowledgement, count, prefetch_count, event_source).unwrap()
                },
                Input::File {input, remove_used } => {
                    plugins::file::load(&input, remove_used, event_source).unwrap()
                }
            }
        },
        Err(_err) => {
            println!("Read source not set");
        }
    }
    Ok(())
}
