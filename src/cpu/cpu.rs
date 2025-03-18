#![allow(non_snake_case)]

use crate::memory::memory_bus::MemoryBus;

use super::instructions::{AddressingMode, Instruction};

#[cfg(test)]
mod tests;

macro_rules! make16 {
    ($hi:expr, $lo:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

const STACK_START: u16 = 0x100;

#[derive(Debug)]
pub struct CPU {
    pub reg_x: u8,
    pub reg_y: u8,
    pub accumulator: u8,
    pub stack_pointer: u8,
    pub flags: u8,
    pub program_counter: u16,
    pub cycle_count: u32,
    memory: MemoryBus,
    // memory: [u8; 0xffff],
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
            stack_pointer: 255,
            flags: 0,
            program_counter: 0,
            cycle_count: 0,
            memory: MemoryBus::new(),
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

    pub fn get_flag(&self, flag: Flag) -> u8 {
        (self.flags & (1 << flag.bit())) >> flag.bit()
    }

    pub fn execute(&mut self, ins: Instruction) {
        use super::instructions::InstructionName as IN;

        self.program_counter += 1; // TODO do this in fetch?
        self.cycle_count += ins.cycles as u32; // TODO add oops cycles

        let n = ins.name;

        match n {
            /* ACCESS INSTRUCTIONS =========================================
            ============================================================= */
            IN::LDA => {
                let val = self.fetch_value(ins);

                self.set_zn_flags(val);
                self.accumulator = val;
            }

            IN::LDX => {
                let val = self.fetch_value(ins);

                self.set_zn_flags(val);
                self.reg_x = val;
            }

            IN::LDY => {
                let val = self.fetch_value(ins);

                self.set_zn_flags(val);
                self.reg_y = val;
            }

            IN::STA => {
                self.store_value(self.accumulator, ins);
            }

            IN::STX => {
                self.store_value(self.reg_x, ins);
            }

            IN::STY => {
                self.store_value(self.reg_y, ins);
            }

            /* TRANSFER INSTRUCTIONS =======================================
            ============================================================= */
            IN::TAX => {
                self.reg_x = self.accumulator;
                self.set_zn_flags(self.reg_x);
            }

            IN::TAY => {
                self.reg_y = self.accumulator;
                self.set_zn_flags(self.reg_y);
            }

            IN::TXA => {
                self.accumulator = self.reg_x;
                self.set_zn_flags(self.accumulator);
            }

            IN::TYA => {
                self.accumulator = self.reg_y;
                self.set_zn_flags(self.accumulator);
            }

            /* ARITHMETIC INSTRUCTIONS =====================================
            ============================================================= */
            IN::ADC => {
                let val = self.fetch_value(ins);
                self.add_to_acc(val);
                self.set_zn_flags(self.accumulator);
            }

            IN::SBC => {
                let val = (!self.fetch_value(ins)).wrapping_add(1);
                self.add_to_acc(val);
                self.set_zn_flags(self.accumulator);
            }

            IN::INC => {
                // TODO rmw
                let (val, addr) = self.fetch_value_keep_addr(ins);
                self.memory.write(addr, val.wrapping_add(1));
                self.set_zn_flags(val.wrapping_add(1));
            }

            IN::INX => {
                self.reg_x = self.reg_x.wrapping_add(1);
                self.set_zn_flags(self.reg_x);
            }

            IN::INY => {
                self.reg_y = self.reg_y.wrapping_add(1);
                self.set_zn_flags(self.reg_y);
            }

            IN::DEC => {
                // TODO rmw
                let (val, addr) = self.fetch_value_keep_addr(ins);
                self.memory.write(addr, val.wrapping_sub(1));
                self.set_zn_flags(val.wrapping_sub(1));
            }

            IN::DEX => {
                self.reg_x = self.reg_x.wrapping_sub(1);
                self.set_zn_flags(self.reg_x);
            }

            IN::DEY => {
                self.reg_y = self.reg_y.wrapping_sub(1);
                self.set_zn_flags(self.reg_y);
            }

            /* SHIFT INSTRUCTIONS ==========================================
            ============================================================= */
            IN::ASL => {
                if ins.mode == AddressingMode::Accumulator {
                    self.set_flag(Flag::Carry, self.accumulator >> 7);
                    self.accumulator <<= 1;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(ins);
                    self.set_flag(Flag::Carry, val >> 7);
                    val <<= 1;
                    self.set_zn_flags(val);
                    self.memory.write(addr, val);
                }
                // TODO rmw
            }

            IN::LSR => {
                // TODO rmw
                if ins.mode == AddressingMode::Accumulator {
                    self.set_flag(Flag::Carry, self.accumulator & 1);
                    self.accumulator >>= 1;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(ins);

                    self.set_flag(Flag::Carry, val & 1);
                    val >>= 1;
                    self.set_zn_flags(val);
                    self.memory.write(addr, val);
                }
            }

            IN::ROL => {
                if ins.mode == AddressingMode::Accumulator {
                    let old_c = self.get_flag(Flag::Carry);
                    self.set_flag(Flag::Carry, self.accumulator >> 7);
                    self.accumulator <<= 1;
                    self.accumulator |= old_c;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(ins);
                    let old_c = self.get_flag(Flag::Carry);
                    self.set_flag(Flag::Carry, val >> 7);
                    val <<= 1;
                    val |= old_c;
                    self.set_zn_flags(val);
                    self.memory.write(addr, val);
                };
            }

            IN::ROR => {
                if ins.mode == AddressingMode::Accumulator {
                    let old_c = self.get_flag(Flag::Carry);
                    self.set_flag(Flag::Carry, self.accumulator & 1);
                    self.accumulator >>= 1;
                    self.accumulator |= old_c << 7;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(ins);
                    let old_c = self.get_flag(Flag::Carry);
                    self.set_flag(Flag::Carry, val & 1);
                    val >>= 1;
                    val |= old_c << 7;
                    self.set_zn_flags(val);
                    self.memory.write(addr, val);
                };
            }

            /* BITWISE INSTRUCTIONS ========================================
            ============================================================= */
            IN::AND => {
                self.accumulator &= self.fetch_value(ins);
                self.set_zn_flags(self.accumulator);
            }

            IN::BIT => {
                let val = self.fetch_value(ins);
                self.set_flag(Flag::Zero, (val & self.accumulator == 0) as u8);
                self.set_flag(Flag::Overflow, val >> 6 & 1);
                self.set_flag(Flag::Negative, val >> 7);
            }

            IN::ORA => {
                self.accumulator |= self.fetch_value(ins);
                self.set_zn_flags(self.accumulator);
            }

            IN::EOR => {
                self.accumulator ^= self.fetch_value(ins);
                self.set_zn_flags(self.accumulator);
            }

            /* COMPARE INSTRUCTIONS ========================================
            ============================================================= */
            IN::CMP => {
                let val = self.fetch_value(ins);
                self.set_flag(Flag::Carry, if self.accumulator >= val { 1 } else { 0 });
                self.set_flag(Flag::Zero, if self.accumulator == val { 1 } else { 0 });
                self.set_flag(Flag::Negative, self.accumulator.wrapping_sub(val) >> 7);
            }

            IN::CPX => {
                let val = self.fetch_value(ins);
                self.set_flag(Flag::Carry, if self.reg_x >= val { 1 } else { 0 });
                self.set_flag(Flag::Zero, if self.reg_x == val { 1 } else { 0 });
                self.set_flag(Flag::Negative, self.reg_x.wrapping_sub(val) >> 7);
            }

            IN::CPY => {
                let val = self.fetch_value(ins);
                self.set_flag(Flag::Carry, if self.reg_y >= val { 1 } else { 0 });
                self.set_flag(Flag::Zero, if self.reg_y == val { 1 } else { 0 });
                self.set_flag(Flag::Negative, self.reg_y.wrapping_sub(val) >> 7);
            }

            /* BRANCH INSTRUCTIONS =========================================
            ============================================================= */
            IN::BCC => {
                if self.get_flag(Flag::Carry) == 0 {
                    self.cycle_count += 1;
                    // TODO find better way?
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            IN::BCS => {
                if self.get_flag(Flag::Carry) == 1 {
                    self.cycle_count += 1;
                    // TODO check that this works
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            IN::BEQ => {
                if self.get_flag(Flag::Zero) == 1 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            IN::BNE => {
                if self.get_flag(Flag::Zero) == 0 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            IN::BPL => {
                if self.get_flag(Flag::Negative) == 0 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            IN::BMI => {
                if self.get_flag(Flag::Negative) == 1 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            IN::BVC => {
                if self.get_flag(Flag::Overflow) == 0 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            IN::BVS => {
                if self.get_flag(Flag::Overflow) == 1 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8();
                    let branch = if byte & 0b10000000 != 0 {
                        -(((!byte).wrapping_add(1)) as i32)
                    } else {
                        byte as i32
                    };
                    self.program_counter = ((self.program_counter as i32) + branch) as u16;
                } else {
                    self.program_counter += 1;
                }
            }

            /* JUMP INSTRUCTIONS ===========================================
            ============================================================= */
            IN::JMP => {
                if ins.mode == AddressingMode::Absolute {
                    self.program_counter = self.get_next_u16();
                } else {
                    let addr = self.get_next_u16();
                    if addr & 0xff == 0xff {
                        self.program_counter =
                            make16!(self.memory.read(addr - 0xff), self.memory.read(addr));
                    } else {
                        self.program_counter =
                            make16!(self.memory.read(addr + 1), self.memory.read(addr));
                    }
                }
            }

            IN::JSR => {
                let sr_addr = self.get_next_u16();

                let stack_save = self.program_counter - 1;
                self.push_stack((stack_save >> 8) as u8);
                self.push_stack((stack_save & 0xff) as u8);
                self.program_counter = sr_addr;
            }

            IN::RTI => {
                self.flags = self.pull_stack();
                let lb = self.pull_stack() as u16;
                let hb = self.pull_stack() as u16;
                self.program_counter = make16!(hb, lb);
            }

            IN::RTS => {
                let lb = self.pull_stack() as u16;
                let hb = self.pull_stack() as u16;
                self.program_counter = make16!(hb, lb);
                self.program_counter += 1;
            }

            IN::BRK => {
                todo!("havent implemented BRK yet");
            }

            /* STACK INSTRUCTIONS ==========================================
            ============================================================= */
            IN::TXS => {
                self.stack_pointer = self.reg_x;
            }

            IN::TSX => {
                self.reg_x = self.stack_pointer;
                self.set_zn_flags(self.reg_x);
            }

            IN::PHA => {
                self.push_stack(self.accumulator);
            }

            IN::PHP => {
                self.push_stack(self.flags | 0b00110000);
            }

            IN::PLA => {
                self.accumulator = self.pull_stack();
            }

            IN::PLP => {
                self.flags = self.pull_stack();
                // TODO the I flag needs to be delayed 1 instr
            }

            /* FLAG INSTRUCTIONS ===========================================
            ============================================================= */
            IN::CLC => {
                self.set_flag(Flag::Carry, 0);
            }

            IN::CLD => {
                self.set_flag(Flag::Decimal, 0);
            }

            IN::CLI => {
                // TODO this needs to be delayed by 1 instruction
                self.set_flag(Flag::Interrupt, 0);
            }

            IN::CLV => {
                self.set_flag(Flag::Overflow, 0);
            }

            IN::SEC => {
                self.set_flag(Flag::Carry, 1);
            }

            IN::SED => {
                self.set_flag(Flag::Decimal, 1);
            }

            IN::SEI => {
                self.set_flag(Flag::Interrupt, 1);
            }

            /* OTHER INSTRUCTIONS ==========================================
            ============================================================= */
            IN::NOP => {
                // do nothing
            }
        };
    }

    fn add_to_acc(&mut self, val: u8) {
        let sum = (self.accumulator as u16) + (val as u16) + (self.get_flag(Flag::Carry) as u16);

        let carry = sum > 0xff;

        if carry {
            self.set_flag(Flag::Carry, 1);
        } else {
            self.set_flag(Flag::Carry, 0);
        }

        let result = sum as u8;

        if (val ^ result) & (result ^ self.accumulator) & 0x80 != 0 {
            self.set_flag(Flag::Overflow, 1);
        } else {
            self.set_flag(Flag::Overflow, 0);
        }

        self.accumulator = result;
    }

    // INCREMENTS PC
    fn get_next_u8(&mut self) -> u8 {
        self.program_counter += 1;
        self.memory.read(self.program_counter - 1)
    }

    fn get_next_u16(&mut self) -> u16 {
        self.program_counter += 2;
        make16!(
            self.memory.read(self.program_counter - 1),
            self.memory.read(self.program_counter - 2)
        )
    }

    // pub fn read_mem_raw(&self, address: u16) -> u8 {
    //     return self.memory[address as usize];
    // }

    pub fn get_addr_8bit(&self, address: u8, mode: AddressingMode) -> u16 {
        use AddressingMode::*; // TODO
        match mode {
            ZeroPage => address as u16,
            ZeroPageX => self.reg_x.wrapping_add(address) as u16,
            ZeroPageY => self.reg_y.wrapping_add(address) as u16,
            IndexedIndirect => {
                make16!(
                    self.memory.read(self.get_addr_8bit(address + 1, ZeroPageX)),
                    self.memory.read(self.get_addr_8bit(address, ZeroPageX))
                )
            }
            IndirectIndexed => {
                make16!(
                    self.memory.read(self.get_addr_8bit(address + 1, ZeroPage)),
                    self.memory.read(self.get_addr_8bit(address, ZeroPage))
                ) + (self.reg_y as u16)
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
        use AddressingMode as M;
        match mode {
            M::Indirect => {
                make16!(self.memory.read(address + 1), self.memory.read(address))
            }
            M::Absolute => address,
            M::AbsoluteX => address + (self.reg_x as u16),
            M::AbsoluteY => address + (self.reg_y as u16),
            M::Relative | M::Immediate | M::Accumulator | M::Implicit => {
                panic!("{:?} does not need memory address", mode)
            }
            _ => {
                panic!("wrong mode, use get_addr_8bit for {:?}", mode)
            }
        }
    }

    pub fn read_8bit(&self, address: u8, mode: AddressingMode) -> u8 {
        self.memory.read(self.get_addr_8bit(address, mode))
    }

    pub fn read_16bit(&self, address: u16, mode: AddressingMode) -> u8 {
        self.memory.read(self.get_addr_16bit(address, mode))
    }

    fn fetch_value(&mut self, ins: Instruction) -> u8 {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::Immediate => self.get_next_u8(),
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.get_next_u8();
                self.read_8bit(addr, mode)
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.get_next_u16();
                self.read_16bit(addr, mode)
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    fn fetch_value_keep_addr(&mut self, ins: Instruction) -> (u8, u16) {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.get_next_u8();
                (self.read_8bit(addr, mode), self.get_addr_8bit(addr, mode))
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.get_next_u16();
                (self.read_16bit(addr, mode), self.get_addr_16bit(addr, mode))
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    fn store_value(&mut self, val: u8, ins: Instruction) {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.get_next_u8();
                self.memory.write(self.get_addr_8bit(addr, mode), val);
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.get_next_u16();
                self.memory.write(self.get_addr_16bit(addr, mode), val);
            }
            _ => panic!("cannot store value for {:?}", mode),
        }
    }

    fn set_zn_flags(&mut self, val: u8) {
        self.set_flag(Flag::Zero, if val == 0 { 1 } else { 0 });
        self.set_flag(Flag::Negative, val >> 7);
    }

    fn inc_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    fn dec_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn peek_stack(&self) -> u8 {
        self.memory.read(self.stack_pointer as u16 + STACK_START)
    }

    fn pull_stack(&mut self) -> u8 {
        self.inc_sp();
        self.peek_stack()
    }

    fn push_stack(&mut self, val: u8) {
        self.memory
            .write(self.stack_pointer as u16 + STACK_START, val);
        self.dec_sp();
    }
}
