use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::cpu::addressing_mode::{self, AddressingMode};

pub struct Instruction {
    mnemonic: &'static str,
    opcode: u8,
    bytes: u8,
    cycles: u8,
    addressing_mode: AddressingMode,
}

impl Instruction {
    pub fn new(
        mnemonic: &'static str,
        opcode: u8,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        Self {
            mnemonic,
            opcode,
            bytes,
            cycles,
            addressing_mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_INSTRUCTIONS: Vec<Instruction> = vec![
        // ACCESS
        // LDA
        Instruction::new("LDA", 0xA9, 2, 2, AddressingMode::Immediate),
        Instruction::new("LDA", 0xA5, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("LDA", 0xB5, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("LDA", 0xAD, 3, 4, AddressingMode::Absolute),
        Instruction::new("LDA", 0xBD, 3, 4, AddressingMode::Absolute_X),
        Instruction::new("LDA", 0xB9, 3, 4, AddressingMode::Absolute_Y),
        Instruction::new("LDA", 0xA1, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("LDA", 0xB1, 2, 5, AddressingMode::Indirect_Y),
        // STA
        Instruction::new("STA", 0x85, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("STA", 0x95, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("STA", 0x8D, 3, 4, AddressingMode::Absolute),
        Instruction::new("STA", 0x95, 3, 5, AddressingMode::Absolute_X),
        Instruction::new("STA", 0x99, 3, 5, AddressingMode::Absolute_Y),
        Instruction::new("STA", 0x81, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("STA", 0x91, 2, 6, AddressingMode::Indirect_Y),
        // LDX
        Instruction::new("LDX", 0xA2, 2, 2, AddressingMode::Immediate),
        Instruction::new("LDX", 0xA6, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("LDX", 0xB6, 2, 4, AddressingMode::ZeroPage_Y),
        Instruction::new("LDX", 0xAE, 3, 4, AddressingMode::Absolute),
        Instruction::new("LDX", 0xBE, 3, 4, AddressingMode::Absolute_Y),
        // STX
        Instruction::new("STX", 0x86, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("STX", 0x96, 2, 3, AddressingMode::ZeroPage_Y),
        Instruction::new("STX", 0x8E, 2, 3, AddressingMode::Absolute),
        // LDY
        Instruction::new("LDY", 0xA0, 2, 2, AddressingMode::Immediate),
        Instruction::new("LDY", 0xA4, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("LDY", 0xB4, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("LDY", 0xAC, 3, 4, AddressingMode::Absolute),
        Instruction::new("LDY", 0xBC, 3, 4, AddressingMode::Absolute_X),
        // STY
        Instruction::new("STY", 0x84, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("STY", 0x94, 2, 3, AddressingMode::ZeroPage_X),
        Instruction::new("STY", 0x8C, 2, 3, AddressingMode::Absolute),
        // TRANSFER
        // TAX
        Instruction::new("TAX", 0xAA, 1, 2, AddressingMode::Implicit),
        // TXA
        Instruction::new("TXA", 0x8A, 1, 2, AddressingMode::Implicit),
        // TAY
        Instruction::new("TAY", 0xA8, 1, 2, AddressingMode::Implicit),
        // TYA
        Instruction::new("TYA", 0x98, 1, 2, AddressingMode::Implicit),
        // ARITHMETIC
        // ADC
        Instruction::new("ADC", 0x69, 2, 2, AddressingMode::Immediate),
        Instruction::new("ADC", 0x65, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("ADC", 0x75, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("ADC", 0x6D, 3, 4, AddressingMode::Absolute),
        Instruction::new("ADC", 0x7D, 3, 4, AddressingMode::Absolute_X),
        Instruction::new("ADC", 0x79, 3, 4, AddressingMode::Absolute_Y),
        Instruction::new("ADC", 0x61, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("ADC", 0x71, 2, 5, AddressingMode::Indirect_Y),
        // SBC
        Instruction::new("SBC", 0xE9, 2, 2, AddressingMode::Immediate),
        Instruction::new("SBC", 0xE5, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("SBC", 0xF5, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("SBC", 0xED, 3, 4, AddressingMode::Absolute),
        Instruction::new("SBC", 0xFD, 3, 4, AddressingMode::Absolute_X),
        Instruction::new("SBC", 0xF9, 3, 4, AddressingMode::Absolute_Y),
        Instruction::new("SBC", 0xE1, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("SBC", 0xF1, 2, 5, AddressingMode::Indirect_Y),
        // INC
        Instruction::new("INC", 0xE6, 2, 5, AddressingMode::ZeroPage),
        Instruction::new("INC", 0xF6, 2, 6, AddressingMode::ZeroPage_X),
        Instruction::new("INC", 0xEE, 3, 6, AddressingMode::Absolute),
        Instruction::new("INC", 0xFE, 3, 7, AddressingMode::Absolute_X),
        // DEC
        Instruction::new("DEC", 0xC6, 2, 5, AddressingMode::ZeroPage),
        Instruction::new("DEC", 0xD6, 2, 6, AddressingMode::ZeroPage_X),
        Instruction::new("DEC", 0xCE, 3, 6, AddressingMode::Absolute),
        Instruction::new("DEC", 0xDE, 3, 7, AddressingMode::Absolute_X),
        // INX
        Instruction::new("INX", 0xE8, 1, 2, AddressingMode::Implicit),
        // DEX
        Instruction::new("DEX", 0xCA, 1, 2, AddressingMode::Implicit),
        // INY
        Instruction::new("INY", 0xC8, 1, 2, AddressingMode::Implicit),
        // DEY
        Instruction::new("DEY", 0x88, 1, 2, AddressingMode::Implicit),
        // SHIFT
        // ASL
        Instruction::new("ASL", 0x0A, 1, 2, AddressingMode::Accumulator),
        Instruction::new("ASL", 0x06, 2, 5, AddressingMode::ZeroPage),
        Instruction::new("ASL", 0x16, 2, 6, AddressingMode::ZeroPage_X),
        Instruction::new("ASL", 0x0E, 3, 6, AddressingMode::Absolute),
        Instruction::new("ASL", 0x1E, 3, 7, AddressingMode::Absolute_X),
        // LSR
        Instruction::new("LSR", 0x4A, 1, 2, AddressingMode::Accumulator),
        Instruction::new("LSR", 0x46, 2, 5, AddressingMode::ZeroPage),
        Instruction::new("LSR", 0x56, 2, 6, AddressingMode::ZeroPage_X),
        Instruction::new("LSR", 0x4E, 3, 6, AddressingMode::Absolute),
        Instruction::new("LSR", 0x5E, 3, 7, AddressingMode::Absolute_X),
        // ROL
        Instruction::new("ROL", 0x2A, 1, 2, AddressingMode::Accumulator),
        Instruction::new("ROL", 0x26, 2, 5, AddressingMode::ZeroPage),
        Instruction::new("ROL", 0x36, 2, 6, AddressingMode::ZeroPage_X),
        Instruction::new("ROL", 0x2E, 3, 6, AddressingMode::Absolute),
        Instruction::new("ROL", 0x3E, 3, 7, AddressingMode::Absolute_X),
        // ROR
        Instruction::new("ROR", 0x6A, 1, 2, AddressingMode::Accumulator),
        Instruction::new("ROR", 0x66, 2, 5, AddressingMode::ZeroPage),
        Instruction::new("ROR", 0x76, 2, 6, AddressingMode::ZeroPage_X),
        Instruction::new("ROR", 0x6E, 3, 6, AddressingMode::Absolute),
        Instruction::new("ROR", 0x7E, 3, 7, AddressingMode::Absolute_X),
        // BITWISE
        // AND
        Instruction::new("AND", 0x29, 2, 2, AddressingMode::Immediate),
        Instruction::new("AND", 0x25, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("AND", 0x35, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("AND", 0x2D, 3, 4, AddressingMode::Absolute),
        Instruction::new("AND", 0x3D, 3, 4, AddressingMode::Absolute_X),
        Instruction::new("AND", 0x39, 3, 4, AddressingMode::Absolute_Y),
        Instruction::new("AND", 0x21, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("AND", 0x31, 2, 5, AddressingMode::Indirect_Y),
        // ORA
        Instruction::new("ORA", 0x09, 2, 2, AddressingMode::Immediate),
        Instruction::new("ORA", 0x05, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("ORA", 0x15, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("ORA", 0x0D, 3, 4, AddressingMode::Absolute),
        Instruction::new("ORA", 0x1D, 3, 4, AddressingMode::Absolute_X),
        Instruction::new("ORA", 0x19, 3, 4, AddressingMode::Absolute_Y),
        Instruction::new("ORA", 0x01, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("ORA", 0x11, 2, 5, AddressingMode::Indirect_Y),
        // EOR
        Instruction::new("EOR", 0x49, 2, 2, AddressingMode::Immediate),
        Instruction::new("EOR", 0x45, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("EOR", 0x55, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("EOR", 0x4D, 3, 4, AddressingMode::Absolute),
        Instruction::new("EOR", 0x5D, 3, 4, AddressingMode::Absolute_X),
        Instruction::new("EOR", 0x59, 3, 4, AddressingMode::Absolute_Y),
        Instruction::new("EOR", 0x41, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("EOR", 0x51, 2, 5, AddressingMode::Indirect_Y),
        // BIT
        Instruction::new("BIT", 0x24, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("BIT", 0x2C, 3, 4, AddressingMode::Absolute),
        // COMPARE
        // CMP
        Instruction::new("CMP", 0xC9, 2, 2, AddressingMode::Immediate),
        Instruction::new("CMP", 0xC5, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("CMP", 0xD5, 2, 4, AddressingMode::ZeroPage_X),
        Instruction::new("CMP", 0xCD, 3, 4, AddressingMode::Absolute),
        Instruction::new("CMP", 0xDD, 3, 4, AddressingMode::Absolute_X),
        Instruction::new("CMP", 0xD9, 3, 4, AddressingMode::Absolute_Y),
        Instruction::new("CMP", 0xC1, 2, 6, AddressingMode::Indirect_X),
        Instruction::new("CMP", 0xD1, 2, 5, AddressingMode::Indirect_Y),
        // CPX
        Instruction::new("CPX", 0xE0, 2, 2, AddressingMode::Immediate),
        Instruction::new("CPX", 0xE4, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("CPX", 0xEC, 3, 4, AddressingMode::Absolute),
        // CPY
        Instruction::new("CPY", 0xC0, 2, 2, AddressingMode::Immediate),
        Instruction::new("CPY", 0xC4, 2, 3, AddressingMode::ZeroPage),
        Instruction::new("CPY", 0xCC, 3, 4, AddressingMode::Absolute),
        // BRANCH
        // JUMP
        // STACK
        // FLAGS
        // OTHER
    ];
    pub static ref CPU_INSTRUCTIONS_MAP: HashMap<u8, &'static Instruction> = {
        let mut map = HashMap::new();
        for cpu_instruction in &*CPU_INSTRUCTIONS {
            map.insert(cpu_instruction.opcode, cpu_instruction);
        }
        map
    };
}
