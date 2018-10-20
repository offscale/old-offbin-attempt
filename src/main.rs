extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;

mod executor;
mod structs;

use crate::executor::*;
use crate::structs::*;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Config file
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
}

fn main() {
    let options = Opt::from_args();

    let mut config = File::open(options.config).expect("Unable to open the config file");
    let mut contents = String::new();
    config.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let config: Config = serde_yaml::from_str(&contents).expect("Invalid config file format");
    println!("{:#?}", config);

    interpret_config(&config);
}