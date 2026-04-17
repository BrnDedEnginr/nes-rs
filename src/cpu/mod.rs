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

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;

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
            status_register: StatusFlags::from_bits_truncate(0b100100),
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
            status_register: StatusFlags::from_bits_truncate(0b100100),
            memory: Memory::new(),
        }
    }

    fn get_memory_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implicit => {
                panic!("WTF BRO!?")
            }
            AddressingMode::Accumulator => self.accumulator as u16,
            AddressingMode::Immediate => {
                let addr = self.program_counter;
                self.program_counter += 1;
                addr
            }
            AddressingMode::ZeroPage => {
                let addr = self.memory.read(self.program_counter) as u16;
                self.program_counter += 1;
                addr
            }
            AddressingMode::Absolute => {
                let addr = self.memory.read_u16(self.program_counter);
                self.program_counter += 2;
                addr
            }
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
                let arg = self.memory.read(self.program_counter);
                let addr = (arg as u16 + self.index_x as u16) % 256;
                self.program_counter += 1;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let arg = self.memory.read(self.program_counter);
                let addr = (arg as u16 + self.index_y as u16) % 256;
                self.program_counter += 1;
                addr
            }
            AddressingMode::Absolute_X => {
                let arg = self.memory.read_u16(self.program_counter);
                let addr = arg + self.index_x as u16;
                self.program_counter += 2;
                addr
            }
            AddressingMode::Absolute_Y => {
                let arg = self.memory.read_u16(self.program_counter);
                let addr = arg + self.index_y as u16;
                self.program_counter += 2;
                addr
            }
            AddressingMode::Indirect_X => {
                let arg = self.memory.read(self.program_counter);
                let addr = self.memory.read((arg as u16 + self.index_x as u16) % 256) as u16
                    + self
                        .memory
                        .read((arg as u16 + self.index_x as u16 + 1) % 256)
                        as u16
                        * 256;
                self.program_counter += 2;
                addr
            }
            AddressingMode::Indirect_Y => {
                let arg = self.memory.read(self.program_counter);
                let addr = self.memory.read(arg as u16) as u16
                    + self.memory.read((arg as u16 + 1) % 256) as u16 * 256
                    + self.index_y as u16;
                self.program_counter += 2;
                addr
            }
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.index_x = 0;
        self.index_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status_register = StatusFlags::from_bits_truncate(0b100100);
        self.program_counter = self.memory.read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.memory.load(program);
        self.program_counter = self.memory.read_u16(0xFFFC);
        self.run();
    }

    pub fn run(&mut self) {
        let instructions = &*instruction::CPU_INSTRUCTIONS_MAP;

        loop {
            let opcode = self.memory.read(self.program_counter);
            self.program_counter += 1;

            let err_msg = &format!(
                "YOU FUCKED UP BRO, I GOT NOT FUCKING CLUE WHAT {:#04x} IS SUPPOSED TO BE",
                opcode
            );

            let instruction = instructions.get(&opcode).expect(err_msg);

            match opcode {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&instruction.addressing_mode)
                }
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&instruction.addressing_mode)
                }
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&instruction.addressing_mode),
                0x86 | 0x96 | 0x8E => self.stx(&instruction.addressing_mode),
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&instruction.addressing_mode),
                0x84 | 0x94 | 0x8C => self.sty(&instruction.addressing_mode),
                0x00 => return,
                _ => {
                    panic!("instruction {:#04x} unkown", opcode)
                }
            }
        }
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.status_register.insert(StatusFlags::ZERO);
        } else {
            self.status_register.remove(StatusFlags::ZERO);
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            self.status_register.insert(StatusFlags::NEGATIVE);
        } else {
            self.status_register.remove(StatusFlags::NEGATIVE);
        }
    }

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        let addr = self.get_memory_address(addressing_mode);
        let value = self.memory.read(addr);
        self.accumulator = value;

        self.update_zero_flag(self.accumulator);
        self.update_negative_flag(self.accumulator);
    }

    fn sta(&mut self, addressing_mode: &AddressingMode) {
        let addr = self.get_memory_address(addressing_mode);
        self.memory.write(addr, self.accumulator);
    }

    fn ldx(&mut self, addressing_mode: &AddressingMode) {
        let addr = self.get_memory_address(addressing_mode);
        let value = self.memory.read(addr);
        self.index_x = value;

        self.update_zero_flag(self.index_x);
        self.update_negative_flag(self.index_x);
    }

    fn stx(&mut self, addressing_mode: &AddressingMode) {
        let addr = self.get_memory_address(addressing_mode);
        self.memory.write(addr, self.index_x);
    }

    fn ldy(&mut self, addressing_mode: &AddressingMode) {
        let addr = self.get_memory_address(addressing_mode);
        let value = self.memory.read(addr);
        self.index_y = value;

        self.update_zero_flag(self.index_y);
        self.update_negative_flag(self.index_y);
    }

    fn sty(&mut self, addressing_mode: &AddressingMode) {
        let addr = self.get_memory_address(addressing_mode);
        self.memory.write(addr, self.index_y);
    }
}
