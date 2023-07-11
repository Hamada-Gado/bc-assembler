#![allow(unused)]

mod errors;

mod arg_parser;

mod constants;

mod instructions;
mod memory;
mod registers;

use clap::Parser;
use std::{collections::HashMap, fs::File};

use arg_parser::Args;
use constants::MEMORY_WORDS_SIZE;
use memory::Memory;
use registers::Register;

fn main() {
    let args = Args::new();

    let mut memory: Memory = Memory::new();
    memory.load_program_to_memory(args.get_input_file());

    let registers: HashMap<Register, i16> = Register::new();
}

#[cfg(test)]
mod test {}
