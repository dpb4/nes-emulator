#![allow(non_snake_case)]
use super::instructions::{AddressingMode, Instruction};

#[derive(Debug)]
pub struct CPU {
    pub reg_x: u8,
    pub reg_y: u8,
    pub accumulator: u8,
    pub stack_pointer: u8,
    pub program_counter: u8,
    pub flags: u8,
    memory: [u8; 0xffff],
}

pub enum Flag {
    Carry,
    Zero,
    Interrupt,
    Decimal,
    Overflow,
    Negative,
}

impl Flag {
    pub fn bit(&self) -> u8 {
        match self {
            Flag::Carry => 0,
            Flag::Zero => 1,
            Flag::Interrupt => 2,
            Flag::Decimal => 3,
            Flag::Overflow => 6,
            Flag::Negative => 7,
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg_x: 0,
            reg_y: 0,
            accumulator: 0,
            stack_pointer: 0,
            program_counter: 0,
            flags: 0,
            memory: [0; 0xffff],
        }
    }

    pub fn set_flag(&mut self, flag: Flag, bit: u8) {
        if bit == 1 {
            self.flags |= 1 << flag.bit();
        } else if bit == 0 {
            self.flags &= !(1 << flag.bit());
        } else {
            panic!("attempting to set flag to something that isn't 0 or 1");
        }
    }

    pub fn get_flag(&mut self, flag: Flag) -> u8 {
        (self.flags & (1 << flag.bit())) >> flag.bit()
    }

    pub fn execute(&mut self, ins: &'static Instruction) {
        match ins {
            // ADC_A => 1.add,
            _ => (),
        }
    }

    pub fn read_mem_raw(&self, address: u16) -> u8 {
        return *self.memory.get(address as usize).expect(&format!(
            "SEGFAULT \n\n\n\n\n just kidding. address {:#x} out of bounds",
            address
        ));
    }

    pub fn get_addr_8bit(&self, address: u8, mode: AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            ZeroPage => address as u16,
            ZeroPageX => self.reg_x.wrapping_add(address) as u16,
            ZeroPageY => self.reg_y.wrapping_add(address) as u16,
            IndexedIndirect => {
                let lo_byte = self.read_mem_raw(self.get_addr_8bit(address, ZeroPageX));
                let hi_byte = self.read_mem_raw(self.get_addr_8bit(address + 1, ZeroPageX));
                ((hi_byte as u16) << 8) | (lo_byte as u16)
            }
            IndirectIndexed => {
                let lo_byte = self.read_mem_raw(self.get_addr_8bit(address, ZeroPage));
                let hi_byte = self.read_mem_raw(self.get_addr_8bit(address + 1, ZeroPage));
                (((hi_byte as u16) << 8) | (lo_byte as u16)) + self.reg_y as u16
            }
            Relative | Immediate | Accumulator | Implicit => {
                panic!("{:?} does not need memory address", mode)
            }
            _ => {
                panic!("wrong mode, use get_addr_16bit for {:?}", mode)
            }
        }
    }

    pub fn get_addr_16bit(&self, address: u16, mode: AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            Indirect => {
                let lo_byte = self.read_mem_raw(address);
                let hi_byte = self.read_mem_raw(address + 1);
                ((hi_byte as u16) << 8) | (lo_byte as u16)
            }
            Absolute => address,
            AbsoluteX => address + (self.reg_x as u16),
            AbsoluteY => address + (self.reg_y as u16),
            Relative | Immediate | Accumulator | Implicit => {
                panic!("{:?} does not need memory address", mode)
            }
            _ => {
                panic!("wrong mode, use get_addr_8bit for {:?}", mode)
            }
        }
    }

    pub fn read_8bit(&self, address: u8, mode: AddressingMode) -> u8 {
        self.read_mem_raw(self.get_addr_8bit(address, mode))
    }

    pub fn read_16bit(&self, address: u16, mode: AddressingMode) -> u8 {
        self.read_mem_raw(self.get_addr_16bit(address, mode))
    }
}
