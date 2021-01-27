extern crate serde;
extern crate serde_yaml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub max_threads: usize,
}

impl Config {
    pub fn new(file: &str) -> Config {
        let path = Path::new(file);
        let mut file = File::open(&path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let config: Config = serde_yaml::from_str(&buf[..]).unwrap();
        config
    }
}
