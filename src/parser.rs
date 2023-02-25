use std::{io::BufReader, fs::File};

pub struct Parser {
    reader: BufReader<File>
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, &str> {
        let f = File::open(filename);
        match f {
            Ok(f) => Ok(Parser {reader: BufReader::new(f)}),
            _ => Err("unknown database")
        }
    }
}