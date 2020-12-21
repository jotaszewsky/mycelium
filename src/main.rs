mod plugins;
mod application;
mod service;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate colored_json;
extern crate structopt;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Mycelium! tool for creating connections between various independent sources.")]
enum Cli {
    #[structopt(about = "Defines the state of the reader source")]
    Read {
        #[structopt(flatten)]
        input: Input
    },
    #[structopt(about = "Defines the state of the storage source")]
    Write {
        #[structopt(flatten)]
        output: Output
    },
    #[structopt(about = "Defines the state of multiple storage sources")]
    MultiWrite {
        #[structopt(flatten)]
        output: Output,
        #[structopt(short, long)]
        clear: bool
    },
    #[structopt(about = "Show mycelium connections")]
    Show {
    },
    #[structopt(about = "Opening the connection by using mycelium")]
    Connection {
    }
}

// pub?
#[derive(StructOpt, Serialize, Deserialize, Debug)]
pub enum Input {
    #[structopt(about = "Read from amqp queue")]
    Amqp {
        #[structopt(short, long, help = "Dsn to amqp")]
        url: String,
        #[structopt(short, long, help = "Amqp queue name")]
        queue: String,
        #[structopt(short, long, help = "Amqp queue arguments in json format")]
        queue_arguments: Option<String>,
        #[structopt(short, long, parse(try_from_str = parse_acknowledgements), help = "Optional acknowledgement")]
        acknowledgement: Option<plugins::amqp::Acknowledgements>,
        #[structopt(short, long, default_value = "1", help = "Limit number of unacknowledged messages")]
        prefetch_count: u16,
        #[structopt(short, long, required_if("acknowledgements", "nack_requeue"), help = "Message limit, when exceeded, the worker will switch off")]
        count: Option<usize>
    },
    #[structopt(about = "Read from files")]
    File {
        #[structopt(short, long, parse(from_os_str), help = "Path to json file or directory to read, can load all files from a folder or a specific file")]
        input: std::path::PathBuf,
        #[structopt(short, long, help = "Flag for remove used files")]
        remove_used: bool,
    }
}

// pub ?
#[derive(StructOpt, Serialize, Deserialize, Debug)]
pub enum Output {
    #[structopt(about = "Publish to amqp exchange")]
    Amqp {
        #[structopt(short, long, help = "Dsn to amqp")]
        url: String,
        #[structopt(short, long, help = "Amqp queue name")]
        queue: String
    },
    #[structopt(about = "Publish to files")]
    File {
        #[structopt(short, long, parse(from_os_str), help = "Path to directory to store json files")]
        output: std::path::PathBuf
    },
    #[structopt(about = "Publish to command line")]
    Console {
        #[structopt(short, long, help = "Flag to colored json response")]
        pretty_json: bool
    }
}

fn parse_acknowledgements(src: &str) -> Result<plugins::amqp::Acknowledgements, String> {
    match src {
        "ack" => Ok(plugins::amqp::Acknowledgements::ack),
        "nack" => Ok(plugins::amqp::Acknowledgements::nack),
        "reject" => Ok(plugins::amqp::Acknowledgements::reject),
        "nack_requeue" => Ok(plugins::amqp::Acknowledgements::nack_requeue),
        _ => Err(format!("Invalid acknowledgements: {}", src))
    }
}

fn main() -> Result<(),()> {
    let args = Cli::from_args();
    match args {
        Cli::Connection {} => service::connection::execute(),
        Cli::Show {} => service::show::execute(),
        Cli::Read { input } => service::read::execute(input),
        Cli::Write { output } => service::write::execute(output),
        Cli::MultiWrite { output, clear } => service::multi_write::execute(output, clear),
    };
    Ok(())
}
