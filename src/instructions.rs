struct Instruction {
    symbol: Symbol,
    address: i16,
    indirect: bool,
}

enum Symbol {
    AND,
    ADD,
    LDA,
    STA,
    BUN,
    BSA,
    ISZ,
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
    INP,
    OUT,
    SKI,
    SKO,
    ION,
    IOF,
}

