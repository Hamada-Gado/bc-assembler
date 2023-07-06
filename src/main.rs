mod constants;
mod instructions;
mod registers;

use std::collections::HashMap;

use constants::*;
use registers::Register;

fn main() {
    let _memory: [i16; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let _registers: HashMap<Register, i16> = Register::new();
    
}


