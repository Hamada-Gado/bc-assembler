use std::fs::{File, self};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {

    #[arg(short, long, value_name= "FILE", help = "Input file, written in assembly, to be executed")]
    input_file: std::path::PathBuf,
}

impl Args {
    pub fn new() -> Self {
        let args = Args::parse();
        if !args.input_file.is_file() {
            panic!("Input file does not exist");
        }
        args
    }

    pub fn get_input_file(&self) -> String {
        fs::read_to_string(&self.input_file).unwrap()
    }
}