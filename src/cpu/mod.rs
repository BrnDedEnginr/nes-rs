use bitflags::bitflags;

use crate::{cpu::addressing_mode::AddressingMode, memory::Memory};

mod addressing_mode;
mod instruction;
mod test;

bitflags! {
    struct StatusFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL           = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

pub struct CPU {
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_register: StatusFlags,
    memory: Memory,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            accumulator: Default::default(),
            index_x: Default::default(),
            index_y: Default::default(),
            program_counter: Default::default(),
            stack_pointer: Default::default(),
            status_register: StatusFlags::from_bits_truncate(0b0),
            memory: Memory::new(),
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            program_counter: 0,
            stack_pointer: 0,
            status_register: StatusFlags::from_bits_truncate(0b0),
            memory: Memory::new(),
        }
    }

    fn get_addressed_memory(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implicit => 0,
            AddressingMode::Accumulator => self.accumulator as u16,
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.memory.read(self.program_counter) as u16,
            AddressingMode::Absolute => self.memory.read_u16(self.program_counter),
            AddressingMode::Relative => {
                let offset = self.memory.read(self.program_counter) as i8;
                self.program_counter
                    .wrapping_add(1)
                    .wrapping_add(offset as u16)
            }
            AddressingMode::Indirect => {
                let addr = self.memory.read_u16(self.program_counter);

                if addr & 0x00FF == 0x00FF {
                    let lo = self.memory.read(addr);
                    let hi = self.memory.read(addr & 0xFF00);
                    (hi as u16) << 8 | (lo as u16)
                } else {
                    self.memory.read_u16(addr)
                }
            }
            AddressingMode::ZeroPage_X => {
                let pos = self.memory.read(self.program_counter);
                pos.wrapping_add(self.index_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.memory.read(self.program_counter);
                pos.wrapping_add(self.index_y) as u16
            }
            AddressingMode::Absolute_X => {
                let base = self.memory.read_u16(self.program_counter);
                base.wrapping_add(self.index_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.memory.read_u16(self.program_counter);
                base.wrapping_add(self.index_y as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.memory.read(self.program_counter);
                let ptr = base.wrapping_add(self.index_x);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_Y => {
                let base = self.memory.read(self.program_counter);
                let lo = self.memory.read(base as u16);
                let hi = self.memory.read(base.wrapping_add(1) as u16);
                let x = u16::from_le_bytes([lo, hi]);
                x.wrapping_add(self.index_y as u16)
            }
        }
    }
}
