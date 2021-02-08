use application::state::State;
use application::Temporary;
use plugins;
use Output;
use Input;
use application::event::{SaveToAmqp, SaveToFile, SaveToConsole, SaveToMongoDB};
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
                    Output::File { output, filename_pattern } => {
                        event_source.register_observer(Arc::new(Mutex::new(SaveToFile::new(
                            output, filename_pattern
                        ))));
                    },
                    Output::Console { pretty_json } => {
                        event_source.register_observer(Arc::new(Mutex::new(SaveToConsole::new(
                            pretty_json
                        ))));
                    },
                    Output::MongoDB { dsn, database, collection } => {
                        event_source.register_observer(Arc::new(Mutex::new(SaveToMongoDB::new(
                            dsn,
                            database,
                            collection
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
                    let mut amqp = plugins::amqp::Amqp::new(&url, queue, queue_arguments);
                    amqp.consume(acknowledgement, count, prefetch_count, event_source).unwrap();
                    amqp.close().unwrap();
                },
                Input::File {input, remove_used } => {
                    let mut file = plugins::file::File::new(input, None);
                    file.consume(remove_used, event_source).unwrap();
                },
                Input::Console { add_header } => {
                    let mut console = plugins::console::Console::new(false);
                    console.consume(add_header, event_source).unwrap();
                },
                Input::MongoDB { dsn, database, collection, count } => {
                    let mut mongodb = plugins::mongodb::MongoDB::new(dsn, database, collection);
                    mongodb.consume(count, event_source).unwrap();
                }
            }
        },
        Err(_err) => {
            println!("Read source not set");
        }
    }
    Ok(())
}
