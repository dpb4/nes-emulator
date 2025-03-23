#![allow(non_snake_case)]

use crate::memory::{
    cartridge_rom::CartridgeROM,
    memory_bus::{MemoryBus, PRG_ROM_START},
};

use super::instructions::{get_instruction, AddressingMode, Instruction, JMP_A, JMP_I, JSR_A};

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

    logged: bool,
    pub log: String,
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
            memory: MemoryBus::new(CartridgeROM::dummy()),
            logged: false,
            log: String::new(),
        }
    }

    pub fn new_program(raw_bytes: Vec<u8>, logged: bool) -> Self {
        Self {
            reg_x: 0,
            reg_y: 0,
            accumulator: 0,
            stack_pointer: 0xfd,
            flags: 0x24,
            program_counter: 0xc000,
            cycle_count: 0,
            memory: MemoryBus::new(match CartridgeROM::new(raw_bytes) {
                Ok(c) => c,
                Err(msg) => panic!("{msg}"),
            }),
            logged,
            log: String::new(),
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

    pub fn tick(&mut self) {
        let opcode = self.memory.read(self.program_counter);
        let ins = *get_instruction(opcode);
        if self.logged {
            let log = self.logged_execute(ins);
            println!("{log}");
            self.log.push_str(&log);
            self.log.push('\n');
        }
        self.execute(ins); // TODO change reference
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
                self.set_zn_flags(self.accumulator);
            }

            IN::PLP => {
                self.flags = self.pull_stack() & 0b11101111 | 0b00100000;
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
                //TODO check this
                make16!(
                    self.memory.read(self.get_addr_8bit(address + 1, ZeroPageX)),
                    self.memory.read(self.get_addr_8bit(address, ZeroPageX))
                )
            }
            IndirectIndexed => {
                // TODO add a method for this
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

    pub fn logged_execute(&mut self, ins: Instruction) -> String {
        // CODE FROM https://github.com/bugzmanov/nes_ebook/blob/master/code/ch5.1/src/trace.rs

        let begin = self.program_counter;
        let mut hex_dump = vec![];
        hex_dump.push(ins.opcode);

        let (mem_addr, stored_value) = match ins.mode {
            AddressingMode::Immediate | AddressingMode::Implicit => (0, 0),
            _ => match ins.bytes {
                2 => {
                    let addr = self.memory.read(begin + 1);
                    if ins.mode != AddressingMode::Relative {
                        (addr as u16, self.read_8bit(addr, ins.mode))
                    } else {
                        (addr as u16, 0)
                    }
                }
                3 => {
                    let addr = make16!(self.memory.read(begin + 2), self.memory.read(begin + 1));
                    (addr, self.read_16bit(addr, ins.mode))
                }
                _ => {
                    unreachable!()
                }
            },
        };

        let tmp = match ins.bytes {
            1 => match ins.opcode {
                0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
                _ => String::from(""),
            },
            2 => {
                let address: u8 = self.memory.read(begin + 1);

                hex_dump.push(address);

                match ins.mode {
                    AddressingMode::Immediate => format!("#${:02x}", address),
                    AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::ZeroPageX => format!(
                        "${:02x},X @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::ZeroPageY => format!(
                        "${:02x},Y @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::IndexedIndirect => format!(
                        "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                        address,
                        (address.wrapping_add(self.reg_x)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::IndirectIndexed => format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        address,
                        (mem_addr.wrapping_sub(self.reg_y as u16)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::Implicit | AddressingMode::Relative => {
                        // assuming local jumps: BNE, BVS, etc....
                        let address: usize =
                            (begin as usize + 2).wrapping_add((address as i8) as usize);
                        format!("${:04x}", address)
                    }

                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                        ins.mode, ins.opcode
                    ),
                }
            }
            3 => {
                let address_lo = self.memory.read(begin + 1);
                let address_hi = self.memory.read(begin + 2);
                hex_dump.push(address_lo);
                hex_dump.push(address_hi);

                let address = make16!(address_hi, address_lo);

                if ins == JMP_A || ins == JMP_I || ins == JSR_A {
                    format!("${:04x}", address)
                } else {
                    match ins.mode {
                        AddressingMode::Implicit => {
                            if ins.opcode == 0x6c {
                                //jmp indirect
                                let jmp_addr = if address & 0x00FF == 0x00FF {
                                    let lo = self.memory.read(address);
                                    let hi = self.memory.read(address & 0xFF00);
                                    (hi as u16) << 8 | (lo as u16)
                                } else {
                                    make16!(
                                        self.memory.read(address + 1),
                                        self.memory.read(address)
                                    )
                                };

                                // let jmp_addr = cpu.mem_read_u16(address);
                                format!("(${:04x}) = {:04x}", address, jmp_addr)
                            } else {
                                format!("${:04x}", address)
                            }
                        }
                        AddressingMode::Absolute => {
                            format!("${:04x} = {:02x}", mem_addr, stored_value)
                        }
                        AddressingMode::AbsoluteX => format!(
                            "${:04x},X @ {:04x} = {:02x}",
                            address, mem_addr, stored_value
                        ),
                        AddressingMode::AbsoluteY => format!(
                            "${:04x},Y @ {:04x} = {:02x}",
                            address, mem_addr, stored_value
                        ),
                        _ => panic!(
                            "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                            ins.mode, ins.opcode
                        ),
                    }
                }
            }
            _ => String::from(""),
        };

        let hex_str = hex_dump
            .iter()
            .map(|z| format!("{:02x}", z))
            .collect::<Vec<String>>()
            .join(" ");
        let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ins.name, tmp)
            .trim()
            .to_string();

        format!(
            "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
            asm_str, self.accumulator, self.reg_x, self.reg_y, self.flags, self.stack_pointer,
        )
        .to_ascii_uppercase()
    }

    /*
        pub fn trace(cpu: &CPU) -> String {
        let ref opscodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        let code = cpu.mem_read(cpu.program_counter);
        let ops = opscodes.get(&code).unwrap();

        let begin = cpu.program_counter;
        let mut hex_dump = vec![];
        hex_dump.push(code);

        let (mem_addr, stored_value) = match ops.mode {
            AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
            _ => {
                let addr = cpu.get_absolute_address(&ops.mode, begin + 1);
                (addr, cpu.mem_read(addr))
            }
        };

        let tmp = match ops.len {
            1 => match ops.code {
                0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
                _ => String::from(""),
            },
            2 => {
                let address: u8 = cpu.mem_read(begin + 1);
                // let value = cpu.mem_read(address));
                hex_dump.push(address);

                match ops.mode {
                    AddressingMode::Immediate => format!("#${:02x}", address),
                    AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::ZeroPage_X => format!(
                        "${:02x},X @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::ZeroPage_Y => format!(
                        "${:02x},Y @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::Indirect_X => format!(
                        "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                        address,
                        (address.wrapping_add(cpu.register_x)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::Indirect_Y => format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        address,
                        (mem_addr.wrapping_sub(cpu.register_y as u16)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::NoneAddressing => {
                        // assuming local jumps: BNE, BVS, etc....
                        let address: usize =
                            (begin as usize + 2).wrapping_add((address as i8) as usize);
                        format!("${:04x}", address)
                    }

                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                        ops.mode, ops.code
                    ),
                }
            }
            3 => {
                let address_lo = cpu.mem_read(begin + 1);
                let address_hi = cpu.mem_read(begin + 2);
                hex_dump.push(address_lo);
                hex_dump.push(address_hi);

                let address = cpu.mem_read_u16(begin + 1);

                match ops.mode {
                    AddressingMode::NoneAddressing => {
                        if ops.code == 0x6c {
                            //jmp indirect
                            let jmp_addr = if address & 0x00FF == 0x00FF {
                                let lo = cpu.mem_read(address);
                                let hi = cpu.mem_read(address & 0xFF00);
                                (hi as u16) << 8 | (lo as u16)
                            } else {
                                cpu.mem_read_u16(address)
                            };

                            // let jmp_addr = cpu.mem_read_u16(address);
                            format!("(${:04x}) = {:04x}", address, jmp_addr)
                        } else {
                            format!("${:04x}", address)
                        }
                    }
                    AddressingMode::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::Absolute_X => format!(
                        "${:04x},X @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::Absolute_Y => format!(
                        "${:04x},Y @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                        ops.mode, ops.code
                    ),
                }
            }
            _ => String::from(""),
        };

        let hex_str = hex_dump
            .iter()
            .map(|z| format!("{:02x}", z))
            .collect::<Vec<String>>()
            .join(" ");
        let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ops.mnemonic, tmp)
            .trim()
            .to_string();

        format!(
            "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
            asm_str, cpu.register_a, cpu.register_x, cpu.register_y, cpu.status, cpu.stack_pointer,
        )
        .to_ascii_uppercase()
    } */
}
