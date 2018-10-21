use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

mod execute;
mod types;

#[macro_use]
extern crate serde_derive;

pub fn run(config: &types::Config) -> Result<(), io::Error> {
    Ok(execute::interpret_config(config))
}

pub fn run_from_path(config_file: PathBuf) -> Result<(), io::Error> {
    let mut contents  = String::new();
    let mut config = File::open(config_file)?;

    config.read_to_string(&mut contents)?;
    let config: types::Config = serde_yaml::from_str(&contents).expect("Error parsing");

    run(&config)
}