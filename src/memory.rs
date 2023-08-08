use std::collections::HashMap;
use std::str::FromStr;

use crate::constants::{MEMORY_REFERENCE_SYMBOLS, MEMORY_WORDS_SIZE};
use crate::errors::assembling_error;
use crate::instructions::{Instruction, Symbol};

pub struct Memory {
    pub ram: [u16; MEMORY_WORDS_SIZE],
    program_start_address: u16,
    variables: HashMap<String, String>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; MEMORY_WORDS_SIZE],
            program_start_address: 0,
            variables: HashMap::new(),
        }
    }

    /**
     * Loads a program from a file into memory
     */
    pub fn load_program_to_memory(&mut self, mut file: String) {
        let mut address: u16 = 0;

        for (line_number, line) in file.lines().enumerate() {
            if line.trim() == "" {
                continue;
            }

            let line = line.trim();

            if line.trim().starts_with("ORG") {
                let line = match line.split_once("//") {
                    Some((line, _)) => line,
                    None => line,
                };

                let address_str = line
                    .split_once("ORG ")
                    .expect(
                        assembling_error(line_number, line, "ORG must be followed by an address")
                            .as_str(),
                    )
                    .1;

                address = u16::from_str_radix(address_str.trim(), 16).expect(
                    assembling_error(
                        line_number,
                        line,
                        "ORG must be followed by a valid address in hex",
                    )
                    .as_str(),
                );
                continue;
            }

            let (variable, instruction) = split_line(line);

            if instruction == "" {
                if variable != "" {
                    panic!(
                        "{}",
                        assembling_error(
                            line_number,
                            line,
                            "Variable must be followed by an instruction or value"
                        )
                    )
                }
                continue;
            }

            if variable != "" {
                self.variables.insert(
                    variable.to_string(),
                    format!(
                        "{:0>3}",
                        base16::encode_upper(&address.to_be_bytes()).trim_matches('0')
                    ),
                );
            }

            let word = match parse_instruction(instruction, &self.variables) {
                Ok(parsed) => parsed,
                Err(error) => panic!("{}", assembling_error(line_number, line, error)),
            };
            dbg!(&word);
            self.ram[address as usize] = u16::from_str_radix(word.as_str(), 16).unwrap();

            address += 1;
            assert!(
                address < MEMORY_WORDS_SIZE as u16,
                "{}",
                assembling_error(line_number, line, "Program is too large to fit in memory")
            );
        }

        // todo!();
    }
}

fn split_line(line: &str) -> (&str, &str) {
    let line = line.trim();

    let line = match line.split_once("//") {
        Some((line, _)) => line,
        None => line,
    };

    let split = match line.split_once(",") {
        Some((variable, instruction)) => (variable.trim(), instruction.trim()),
        None => ("", line.trim()),
    };

    split
}

fn parse_instruction<'a>(
    instruction: &str,
    variables: &HashMap<String, String>,
) -> Result<String, &'a str> {
    let mut instruction = instruction.split_whitespace();

    let symbol = match instruction.next() {
        Some(symbol) => match Symbol::from_str(symbol) {
            Ok(symbol) => symbol,
            Err(_) => return Err("Instruction must have a valid symbol"),
        },
        None => return Err("Instruction must have a symbol"),
    };

    let mut base = 16;
    if let Symbol::DEC = symbol {
        base = 10;
    }

    if !symbol.is_memory_reference() && !symbol.is_number_representation() {
        if instruction.next() != None {
            return Err("Non Memory Reference Instructions must have only a symbol");
        }

        return Ok(symbol.value(false).to_string());
    }

    let address = match instruction.next() {
        Some(address) => match i16::from_str_radix(address, base) {
            Ok(_) => address,
            Err(_) => match variables.get(address) {
                Some(address) => address,
                None => return Err("Instruction must have a valid address"),
            },
        },
        None => return Err("Memory Reference Instruction must have an address"),
    };

    let indirect = match instruction.next() {
        Some(indirect) if indirect == "I" => true,
        None => false,
        _ => return Err("Instruction have an invalid indirect flag"),
    };

    if instruction.next() != None {
        return Err("Instruction must have only a symbol, address, and optional indirect flag");
    }

    Ok(format!("{}{}", symbol.value(indirect), address))
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn split_line1() {
        let line = "LOP, LDA 0 // Load the first number into the accumulator";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "LOP");
        assert_eq!(instruction, "LDA 0");
    }

    #[test]
    fn split_line2() {
        let line = "LDA 0 // Load the first number into the accumulator";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "");
        assert_eq!(instruction, "LDA 0");
    }

    #[test]
    fn split_line3() {
        let line = "LOP, LDA 0";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "LOP");
        assert_eq!(instruction, "LDA 0");
    }

    #[test]
    fn split_line4() {
        let line = "LDA 0";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "");
        assert_eq!(instruction, "LDA 0");
    }

    #[test]
    fn split_line5() {
        let line = "LDA 40 I// Load the first number indirectly into AC";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "");
        assert_eq!(instruction, "LDA 40 I");
    }

    #[test]
    fn split_line6() {
        let line = "VAR, HEX 1004";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "VAR");
        assert_eq!(instruction, "HEX 1004");
    }

    #[test]
    fn split_line7() {
        let line = "";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "");
        assert_eq!(instruction, "");
    }

    #[test]
    fn split_line8() {
        let line = "  ";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "");
        assert_eq!(instruction, "");
    }

    #[test]
    fn split_line9() {
        let line = "// this is a comment";
        let (variable, instruction) = super::split_line(line);
        assert_eq!(variable, "");
        assert_eq!(instruction, "");
    }

    fn set_up_variables() -> HashMap<String, String> {
        let mut variables: HashMap<String, String> = HashMap::new();
        variables.insert("VAR".to_string(), "00A".to_string());
        variables
    }

    #[test]
    fn parse_instruction1() {
        let instruction = "STA VAR";
        let variables = set_up_variables();
        let word = super::parse_instruction(instruction, &variables).unwrap();
        println!("{}", word);
        assert!(word == "300A")
    }

    #[test]
    fn parse_instruction2() {
        let instruction = "LDA 002 I";
        let variables = set_up_variables();
        let word = super::parse_instruction(instruction, &variables).unwrap();
        println!("{}", word);
        assert!(word == "A002")
    }

    #[test]
    fn parse_instruction3() {
        let instruction = "LDA VAR";
        let variables = set_up_variables();
        let word = super::parse_instruction(instruction, &variables).unwrap();
        println!("{}", word);
        assert!(word == "200A")
    }

    #[test]
    fn test_memory() {
        let file = "
                    ORG 064     // Start the program at memory location 64(hex) = 100(dec)
            X,      DEC 000     // Store the result here
            LOP,    LDA 000     // Load the first number into the accumulator
                    LDA 001     // Load the second number into the accumulator
                    ADD 002     // Add the second number to the first
                    STA X I     // Store the result indirectly
                    HLT         // Halt the program";

        let mut memory = Memory::new();
        memory.load_program_to_memory(file.to_string());
        for i in 100..110 {
            println!("{:?}", base16::encode_upper(&memory.ram[i].to_be_bytes()));
        }

        assert!(false);
    }
}
