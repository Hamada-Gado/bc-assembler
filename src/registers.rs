use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum Register {
    AR,
    PC,
    DR,
    AC,
    IR,
    TR,
    OUTR,
    INPR,
    E,
}

impl Register {
    pub fn new() -> HashMap<Register, i16> {
        let mut registers: HashMap<Register, i16> = HashMap::new();
        registers.insert(Register::AR, 0);
        registers.insert(Register::PC, 0);
        registers.insert(Register::DR, 0);
        registers.insert(Register::AC, 0);
        registers.insert(Register::IR, 0);
        registers.insert(Register::TR, 0);
        registers.insert(Register::OUTR, 0);
        registers.insert(Register::INPR, 0);
        registers.insert(Register::E, 0);

        registers
    }
}