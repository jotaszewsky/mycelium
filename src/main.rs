mod plugins;
mod application;
mod service;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate colored_json;
extern crate structopt;
extern crate console;
extern crate jq_rs;
extern crate flate2;

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
    #[structopt(about = "Defines the middleware")]
    Middleware {
        #[structopt(flatten)]
        middleware: Middleware
    },
    #[structopt(about = "Defines the multiple middlewares")]
    MultiMiddleware {
        #[structopt(flatten)]
        middleware: Middleware,
        #[structopt(short, long)]
        clear: bool
    },
    #[structopt(about = "Defines the state by yaml")]
    Apply {
        #[structopt(short, long, parse(from_os_str), help = "Path to yaml file")]
        input: std::path::PathBuf,
    },
    #[structopt(about = "Clear state")]
    Clear {
    },
    #[structopt(about = "Show mycelium connections")]
    Show {
    },
    #[structopt(about = "Opening the connection by using mycelium")]
    Connection {
    }
}

#[derive(StructOpt, Serialize, Deserialize, Debug)]
pub enum Input {
    #[structopt(about = "Read from amqp queue")]
    Amqp {
        #[structopt(short, long, help = "Dsn to amqp")]
        url: String,
        #[structopt(short, long, help = "Amqp queue name")]
        queue: String,
        #[structopt(short, long, parse(try_from_str = parse_acknowledgements), help = "Optional acknowledgement")]
        acknowledgement: Option<plugins::amqp::Acknowledgements>,
        #[structopt(short, long, default_value = "1", help = "Limit number of unacknowledged messages")]
        prefetch_count: u16,
        #[structopt(short, long, default_value = "0", required_if("acknowledgements", "nack_requeue"), help = "Message limit, when exceeded, the worker will switch off")]
        count: usize
    },
    #[structopt(about = "Read from files")]
    File {
        #[structopt(short, long, parse(from_os_str), help = "Path to json file or directory to read, can load all files from a folder or a specific file")]
        input: std::path::PathBuf,
        #[structopt(short, long, help = "Flag for remove used files")]
        remove_used: bool,
    },
    #[structopt(about = "Read from console")]
    Console {
        #[structopt(short, long, help = "Flag for give ability to set header")]
        add_header: bool,
    },
    #[structopt(about = "Read from mongodb")]
    MongoDB {
        #[structopt(short, long, help = "MongoDb dsn")]
        dsn: String,
        #[structopt(short, long, help = "Database name")]
        database: String,
        #[structopt(short, long, help = "Collection name")]
        collection: String,
        #[structopt(short, long, default_value = "0", help = "Message limit, when exceeded, the worker will switch off")]
        count: usize
    }
}

#[derive(StructOpt, Serialize, Deserialize, Debug)]
pub enum Output {
    #[structopt(about = "Publish to amqp exchange")]
    Amqp {
        #[structopt(short, long, help = "Dsn to amqp")]
        url: String,
        #[structopt(short, long, help = "Amqp exchange name")]
        exchange: String,
        #[structopt(short, long, default_value = "", help = "Routing key")]
        routing_key: String,
    },
    #[structopt(about = "Publish to files")]
    File {
        #[structopt(short, long, parse(from_os_str), help = "Path to directory to store json files")]
        output: std::path::PathBuf,
        #[structopt(short, long, parse(try_from_str = parse_filename_patterns), help = "A pattern how mycelium will name json files")]
        filename_pattern: Option<plugins::file::FilenamePatterns>,
    },
    #[structopt(about = "Publish to command line")]
    Console {
        #[structopt(short, long, help = "Flag to colored json response")]
        pretty_json: bool
    },
    #[structopt(about = "Read from mongodb")]
    MongoDB {
        #[structopt(short, long, help = "MongoDb dsn")]
        dsn: String,
        #[structopt(short, long, help = "Database name")]
        database: String,
        #[structopt(short, long, help = "Collection name")]
        collection: String,
    }
}

#[derive(StructOpt, Serialize, Deserialize, Debug)]
pub enum Middleware {
    #[structopt(about = "Json query")]
    JQ {
        #[structopt(short, long, help = "Query to json")]
        query: String
    },
    #[structopt(about = "Zlib compression")]
    Zlib {
        #[structopt(short, long, help = "Flag for decompression, false means compression")]
        decompression: bool
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

fn parse_filename_patterns(src: &str) -> Result<plugins::file::FilenamePatterns, String> {
    match src {
        "random" => Ok(plugins::file::FilenamePatterns::random),
        "index" => Ok(plugins::file::FilenamePatterns::index),
        _ => Err(format!("Invalid filename pattern: {}", src))
    }
}

fn main() -> Result<(),()> {
    match Cli::from_args() {
        Cli::Connection {} => service::connection::execute(),
        Cli::Show {} => service::show::execute(),
        Cli::Apply { input } => service::apply::execute(input),
        Cli::Read { input } => service::read::execute(input),
        Cli::Write { output } => service::write::execute(output),
        Cli::MultiWrite { output, clear } => service::multi_write::execute(output, clear),
        Cli::Middleware { middleware } => service::middleware::execute(middleware),
        Cli::MultiMiddleware { middleware, clear } => service::multi_middleware::execute(middleware, clear),
        Cli::Clear {} => service::clear::execute(),
    }
}
