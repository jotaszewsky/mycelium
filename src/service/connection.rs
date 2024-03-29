use application::state::State;
use application::Temporary;
use application::Pipe;
use plugins;
use Output;
use Input;
use Middleware;
use application::event::{SaveToAmqp, SaveToFile, SaveToConsole, SaveToMongoDB, UpdateProgressBar};
use application::event_source::EventSource;

use std::sync::{Arc, Mutex};

pub fn execute() -> Result<(),()> {
    let mut temp: State = State::new(None);

    let middlewares_vec: Vec<Middleware> = match bincode::deserialize(
        &temp.read(String::from("middleware")).unwrap()
    ) {
        Ok(result) => result,
        Err(_err) => {
            Vec::new()
        }
    };
    let mut event_source: EventSource = EventSource::new(
        Pipe::new(
            middlewares_vec
        )
    );

    match temp.read(String::from("write")) {
        Ok(write) => {
            let output_vec: Vec<Output> = bincode::deserialize(&write).unwrap();
            for output in output_vec {
                match output {
                    Output::Amqp { url, exchange, routing_key } => {
                        event_source.register_observer(Arc::new(Mutex::new(SaveToAmqp::new(
                            url,
                            exchange,
                            routing_key
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
                Input::Amqp { url, queue, acknowledgement, count, prefetch_count } => {
                    event_source.register_observer(Arc::new(Mutex::new(UpdateProgressBar::new(
                        count
                    ))));
                    let mut amqp = plugins::amqp::Amqp::new(&url);
                    amqp.consume(queue, acknowledgement, count, prefetch_count, event_source).unwrap();
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
                    event_source.register_observer(Arc::new(Mutex::new(UpdateProgressBar::new(
                        count
                    ))));
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
