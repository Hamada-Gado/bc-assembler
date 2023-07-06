#![allow(unused)]

mod constants;
mod arg_parser;
mod instructions;
mod registers;

use std::{collections::HashMap, fs::File};
use clap::Parser;

use constants::MEMORY_WORDS_SIZE;
use registers::Register;
use arg_parser::Args;

fn main() {
    let args = Args::new();

    let memory: [i16; MEMORY_WORDS_SIZE] = load_program_to_memory(args.get_input_file());
    let registers: HashMap<Register, i16> = Register::new();
    
}

fn load_program_to_memory(mut file: String) -> [i16; MEMORY_WORDS_SIZE] {
    let mut memory: [i16; MEMORY_WORDS_SIZE] = [0; MEMORY_WORDS_SIZE];

    for (i, line) in file.lines().enumerate() {
    }


    // memory
    todo!()
}
