use strum_macros::EnumString;

#[derive(EnumString)]
pub enum Symbol {
    // Memory reference
    AND,
    ADD,
    LDA,
    STA,
    BUN,
    BSA,
    ISZ,

    // Register
    CLA,
    CLE,
    CMA,
    CME,
    CIR,
    CIL,
    INC,
    SPA,
    SNA,
    SZA,
    SZE,
    HLT,

    // I/O
    INP,
    OUT,
    SKI,
    SKO,
    ION,
    IOF,

    // Number Representation
    HEX,
    DEC,
}

impl Symbol {
    pub fn value(&self, is_direct: bool) -> &str {
        match *self {
            Symbol::AND => {
                if is_direct {
                    "8"
                } else {
                    "0"
                }
            }
            Symbol::ADD => {
                if is_direct {
                    "9"
                } else {
                    "1"
                }
            }
            Symbol::LDA => {
                if is_direct {
                    "A"
                } else {
                    "2"
                }
            }
            Symbol::STA => {
                if is_direct {
                    "B"
                } else {
                    "3"
                }
            }
            Symbol::BUN => {
                if is_direct {
                    "C"
                } else {
                    "4"
                }
            }
            Symbol::BSA => {
                if is_direct {
                    "D"
                } else {
                    "5"
                }
            }
            Symbol::ISZ => {
                if is_direct {
                    "E"
                } else {
                    "6"
                }
            }
            Symbol::CLA => "7800",
            Symbol::CLE => "7400",
            Symbol::CMA => "7200",
            Symbol::CME => "7100",
            Symbol::CIR => "7080",
            Symbol::CIL => "7040",
            Symbol::INC => "7020",
            Symbol::SPA => "7010",
            Symbol::SNA => "7008",
            Symbol::SZA => "7004",
            Symbol::SZE => "7002",
            Symbol::HLT => "7001",
            Symbol::INP => "F800",
            Symbol::OUT => "F400",
            Symbol::SKI => "F200",
            Symbol::SKO => "F100",
            Symbol::ION => "F080",
            Symbol::IOF => "F040",
            Symbol::HEX => "F020",
            Symbol::DEC => "F010",
        }
    }

    pub fn is_memory_reference(&self) -> bool {
        match self {
            Symbol::AND
            | Symbol::ADD
            | Symbol::LDA
            | Symbol::STA
            | Symbol::BUN
            | Symbol::BSA
            | Symbol::ISZ => true,
            _ => false,
        }
    }

    pub fn is_register(&self) -> bool {
        match self {
            Symbol::CLA
            | Symbol::CLE
            | Symbol::CMA
            | Symbol::CME
            | Symbol::CIR
            | Symbol::CIL
            | Symbol::INC
            | Symbol::SPA
            | Symbol::SNA
            | Symbol::SZA
            | Symbol::SZE
            | Symbol::HLT => true,
            _ => false,
        }
    }

    pub fn is_io(&self) -> bool {
        match self {
            Symbol::INP | Symbol::OUT | Symbol::SKI | Symbol::SKO | Symbol::ION | Symbol::IOF => {
                true
            }
            _ => false,
        }
    }

    pub fn is_number_representation(&self) -> bool {
        match self {
            Symbol::HEX | Symbol::DEC => true,
            _ => false,
        }
    }
}

pub struct Instruction {
    pub symbol: Symbol,
    pub address: Option<i16>,
    pub indirect: bool,
    private: (),
}

impl Instruction {
    pub fn new(symbol: Symbol, address: i16, indirect: bool) -> Instruction {
        Instruction {
            symbol,
            address: Some(address),
            indirect,
            private: (),
        }
    }

    pub fn encode(&self) -> i16 {
        todo!()
    }

    pub fn decode(word: i16) -> Self {
        todo!()
    }
}
