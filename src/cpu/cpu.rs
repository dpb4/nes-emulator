#![allow(non_snake_case)]
use super::instructions::{AddressingMode, Instruction};

// #[path = "./tests.rs"]
// mod super::tests;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct CPU {
    pub reg_x: u8,
    pub reg_y: u8,
    pub accumulator: u8,
    pub stack_pointer: u8,
    pub flags: u8,
    pub program_counter: u16,
    pub cycle_count: u32,
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
            flags: 0,
            program_counter: 0,
            cycle_count: 0,
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

    pub fn execute(&mut self, ins: Instruction) {
        // use super::instructions::*;

        self.program_counter += 1;
        self.cycle_count += ins.cycles as u32; // TODO add oops cycles

        let n = ins.name;

        match n {
            "AND" => {
                self.accumulator &= self.fetch_value(ins);
                self.set_zn_flags(self.accumulator);
            }
            "ASL" => {
                if ins.mode == AddressingMode::Accumulator {
                    self.set_flag(Flag::Carry, self.accumulator >> 7);
                    self.accumulator <<= 1;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(ins);
                    self.set_flag(Flag::Carry, val >> 7);
                    val <<= 1;
                    self.set_zn_flags(val);
                    self.memory[addr as usize] = val;
                }
                // TODO read-modify-write instruction?
            }

            "BIT" => {
                let val = self.fetch_value(ins);
                self.set_flag(Flag::Zero, (val & self.accumulator == 1) as u8);
                self.set_flag(Flag::Overflow, (val >> 6) & 1);
                self.set_flag(Flag::Negative, (val >> 7) & 1);
            }

            "BCS" => {
                if self.get_flag(Flag::Carry) == 1 {
                    self.cycle_count += 1;
                    // TODO check that this works
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            "BCC" => {
                if self.get_flag(Flag::Carry) == 0 {
                    self.cycle_count += 1;
                    // TODO check that this works
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            "BEQ" => {
                if self.get_flag(Flag::Zero) == 1 {
                    self.cycle_count += 1;
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            "BNE" => {
                if self.get_flag(Flag::Zero) == 0 {
                    self.cycle_count += 1;
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            "BMI" => {
                if self.get_flag(Flag::Negative) == 1 {
                    self.cycle_count += 1;
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            "BPL" => {
                if self.get_flag(Flag::Negative) == 0 {
                    self.cycle_count += 1;
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            "BVC" => {
                if self.get_flag(Flag::Overflow) == 0 {
                    self.cycle_count += 1;
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            "BVS" => {
                if self.get_flag(Flag::Overflow) == 1 {
                    self.cycle_count += 1;
                    self.program_counter =
                        ((self.program_counter as i32) + self.get_next_u8() as i32) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            // BRK TODO
            _ => {
                todo!()
            }
        };
        // match ins {
        //     Instruction {
        //         name: "ADC:",
        //         opcode,
        //         mode,
        //         bytes,
        //         cycles,
        //     } => {
        //         panic!("dwoiwjefoji");
        //     }
        //     AND_IM => {
        //         self.accumulator &= self.get_next_u8();
        //         self.set_zn_flags(self.accumulator);
        //     }

        //     LDA_IM => {
        //         self.accumulator = self.get_next_u8();
        //     }
        //     LDA_Z | LDA_ZX | LDA_IX | LDA_IY => {
        //         let b = self.get_next_u8();
        //         self.accumulator = self.read_8bit(b, ins.mode);
        //     }
        //     LDA_A | LDA_AX | LDA_AY => {
        //         let b = self.get_next_u16();
        //         self.accumulator = self.read_16bit(b, ins.mode);
        //     }

        //     LDX_IM => {
        //         self.reg_x = self.get_next_u8();
        //     }
        //     LDX_Z | LDX_ZY => {
        //         let b = self.get_next_u8();
        //         self.reg_x = self.read_8bit(b, ins.mode);
        //     }
        //     LDX_A | LDX_AY => {
        //         let b = self.get_next_u16();
        //         self.reg_x = self.read_16bit(b, ins.mode);
        //     }

        //     LDY_IM => {
        //         self.reg_y = self.get_next_u8();
        //     }
        //     LDY_Z | LDY_ZX => {
        //         let b = self.get_next_u8();
        //         self.reg_y = self.read_8bit(b, ins.mode);
        //     }
        //     LDY_A | LDY_AX => {
        //         let b = self.get_next_u16();
        //         self.reg_y = self.read_16bit(b, ins.mode);
        //     }
        //     _ => (),
        // }
    }

    // INCREMENTS PC
    fn get_next_u8(&mut self) -> u8 {
        self.program_counter += 1;
        self.memory[(self.program_counter - 1) as usize]
    }

    fn get_next_u16(&mut self) -> u16 {
        self.program_counter += 2;
        let lb = self.memory[(self.program_counter - 2) as usize];
        let hb = self.memory[(self.program_counter - 1) as usize];
        ((hb as u16) << 8) | (lb as u16)
    }

    pub fn read_mem_raw(&self, address: u16) -> u8 {
        return self.memory[address as usize];
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

    fn fetch_value(&mut self, ins: Instruction) -> u8 {
        use AddressingMode::*;

        let mode = ins.mode;

        match mode {
            Immediate => self.get_next_u8(),
            ZeroPage | ZeroPageX | ZeroPageY | IndexedIndirect | IndirectIndexed => {
                let addr = self.get_next_u8();
                self.read_8bit(addr, mode)
            }
            Absolute | AbsoluteX | AbsoluteY | Indirect => {
                let addr = self.get_next_u16();
                self.read_16bit(addr, mode)
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    fn fetch_value_keep_addr(&mut self, ins: Instruction) -> (u8, u16) {
        use AddressingMode::*;

        let mode = ins.mode;

        match mode {
            // Immediate => self.get_next_u8(),
            ZeroPage | ZeroPageX | ZeroPageY | IndexedIndirect | IndirectIndexed => {
                let addr = self.get_next_u8();
                (self.read_8bit(addr, mode), self.get_addr_8bit(addr, mode))
            }
            Absolute | AbsoluteX | AbsoluteY | Indirect => {
                let addr = self.get_next_u16();
                (self.read_16bit(addr, mode), self.get_addr_16bit(addr, mode))
                // self.read_16bit(addr, mode)
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    fn set_zn_flags(&mut self, val: u8) {
        self.set_flag(Flag::Zero, if val == 0 { 0 } else { 1 });
        self.set_flag(Flag::Negative, (val >> 7) & 1);
    }
}
