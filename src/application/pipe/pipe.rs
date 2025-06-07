use application::Value;
use Middleware;
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};
use std::io::prelude::*;

#[derive(Debug)]
pub struct Pipe {
    middlewares: Vec<Middleware>
}

impl Pipe {
    pub fn new(middlewares: Vec<Middleware>) -> Pipe {
        Pipe { middlewares }
    }

    pub fn dispatch(&self, mut value: Value) -> Value {
        if self.middlewares.len() > 0 {
            for middleware in &self.middlewares {
                match middleware {
                    Middleware::JQ { query } => {
                        let output: String = String::from_utf8_lossy(&value.data).to_string();
                        value.data = jq_rs::run(&query, &output).unwrap().as_bytes().to_vec();
                    },
                    Middleware::Zlib { decompression } => {
                        let buffer = Vec::new();
                        if *decompression {
                            let mut decoder = ZlibDecoder::new(buffer);
                            decoder.write_all(&value.data).unwrap();
                            value.data = decoder.finish().unwrap();
                            continue;
                        }
                        let mut encoder = ZlibEncoder::new(buffer, Compression::default());
                        encoder.write_all(&value.data).unwrap();
                        value.data = encoder.finish().unwrap();
                    }
                }
            }
        }
        return value;
    }
}

