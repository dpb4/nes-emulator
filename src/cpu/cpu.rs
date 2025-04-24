#![allow(non_snake_case)]

use crate::{
    make16,
    memory::memory_bus::{InterruptType::NonMaskable, MemoryBus},
};

use super::instructions::{get_instruction, AddressingMode, Instruction, JMP_A, JMP_I, JSR_A};

use bitflags::bitflags;

// #[cfg(test)]
// mod tests;

const STACK_START: u16 = 0x100;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct StatusFlags: u8 {
        const CARRY = 0b00000001;
        const ZERO  = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL = 0b00001000;
        const BREAK = 0b00010000;
        const BREAK2_U = 0b00100000;
        const OVERFLOW = 0b01000000;
        const NEGATIVE = 0b10000000;
    }
}

#[derive(Debug)]
pub struct CPU {
    pub reg_x: u8,
    pub reg_y: u8,
    pub accumulator: u8,
    pub stack_pointer: u8,
    pub flags: StatusFlags,
    pub program_counter: u16,
    pub cycle_count: usize,
    // pub memory: MemoryBus,
    logged: bool,
    pub log: String,
}

impl CPU {
    pub fn new_program(logged: bool) -> Self {
        Self {
            reg_x: 0,
            reg_y: 0,
            accumulator: 0,
            stack_pointer: 0xfd,
            flags: StatusFlags::from_bits_truncate(0x24),
            program_counter: 0xc000,
            cycle_count: 7,
            // TODO FIX THIS !!!!!!!! cycles should start at 0, logging is 1 instr behind (fix ppu too)
            logged,
            log: String::new(),
        }
    }

    pub fn set_flag(&mut self, flag: StatusFlags, bit: u8) {
        self.flags.set(flag, bit == 1);
    }

    pub fn get_flag(&self, flag: StatusFlags) -> u8 {
        if self.flags.contains(flag) {
            1
        } else {
            0
        }
    }

    pub fn run_once(&mut self, memory: &mut MemoryBus) {
        if let Some(NonMaskable) = memory.poll_interrupt() {
            self.interrupt_nmi(memory);
        }
        let cycles = self.tick(memory);
        memory.tick_ppu(cycles);
    }

    pub fn run_count(&mut self, memory: &mut MemoryBus, count: usize) {
        for _ in 0..count {
            self.run_once(memory);
        }
    }

    pub fn tick(&mut self, memory: &mut MemoryBus) -> usize {
        let cycle_count_before = self.cycle_count;

        let opcode = memory.read(self.program_counter);
        let ins = get_instruction(opcode);

        if self.logged {
            let log = self.logged_execute(memory, ins);
            println!("{log}");
            self.log.push_str(&log);
            self.log.push('\n');
        }

        self.execute(memory, ins);

        self.cycle_count - cycle_count_before
    }

    pub fn execute(&mut self, memory: &mut MemoryBus, ins: Instruction) {
        use super::instructions::InstructionName as IN;

        self.program_counter = self.program_counter.wrapping_add(1); // TODO do this in fetch?
        self.cycle_count += ins.cycles as usize; // TODO add oops cycles

        let n = ins.name;

        match n {
            /* ACCESS INSTRUCTIONS =========================================
            ============================================================= */
            IN::LDA => {
                let val = self.fetch_value(memory, ins);

                self.set_zn_flags(val);
                self.accumulator = val;
            }

            IN::LDX => {
                let val = self.fetch_value(memory, ins);

                self.set_zn_flags(val);
                self.reg_x = val;
            }

            IN::LDY => {
                let val = self.fetch_value(memory, ins);

                self.set_zn_flags(val);
                self.reg_y = val;
            }

            IN::STA => {
                self.store_value(memory, self.accumulator, ins);
            }

            IN::STX => {
                self.store_value(memory, self.reg_x, ins);
            }

            IN::STY => {
                self.store_value(memory, self.reg_y, ins);
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
                let val = self.fetch_value(memory, ins);
                self.add_to_acc(val);
                self.set_zn_flags(self.accumulator);
            }

            IN::SBC => {
                let val = !self.fetch_value(memory, ins);
                self.add_to_acc(val);
                self.set_zn_flags(self.accumulator);
            }

            IN::INC => {
                // TODO rmw
                let (val, addr) = self.fetch_value_keep_addr(memory, ins);
                memory.write(addr, val.wrapping_add(1));
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
                let (val, addr) = self.fetch_value_keep_addr(memory, ins);
                memory.write(addr, val.wrapping_sub(1));
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
                    self.set_flag(StatusFlags::CARRY, self.accumulator >> 7);
                    self.accumulator <<= 1;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(memory, ins);
                    self.set_flag(StatusFlags::CARRY, val >> 7);
                    val <<= 1;
                    self.set_zn_flags(val);
                    memory.write(addr, val);
                }
                // TODO rmw
            }

            IN::LSR => {
                // TODO rmw
                if ins.mode == AddressingMode::Accumulator {
                    self.set_flag(StatusFlags::CARRY, self.accumulator & 1);
                    self.accumulator >>= 1;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(memory, ins);

                    self.set_flag(StatusFlags::CARRY, val & 1);
                    val >>= 1;
                    self.set_zn_flags(val);
                    memory.write(addr, val);
                }
            }

            IN::ROL => {
                if ins.mode == AddressingMode::Accumulator {
                    let old_c = self.get_flag(StatusFlags::CARRY);
                    self.set_flag(StatusFlags::CARRY, self.accumulator >> 7);
                    self.accumulator <<= 1;
                    self.accumulator |= old_c;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(memory, ins);
                    let old_c = self.get_flag(StatusFlags::CARRY);
                    self.set_flag(StatusFlags::CARRY, val >> 7);
                    val <<= 1;
                    val |= old_c;
                    self.set_zn_flags(val);
                    memory.write(addr, val);
                };
            }

            IN::ROR => {
                if ins.mode == AddressingMode::Accumulator {
                    let old_c = self.get_flag(StatusFlags::CARRY);
                    self.set_flag(StatusFlags::CARRY, self.accumulator & 1);
                    self.accumulator >>= 1;
                    self.accumulator |= old_c << 7;
                    self.set_zn_flags(self.accumulator);
                } else {
                    let (mut val, addr) = self.fetch_value_keep_addr(memory, ins);
                    let old_c = self.get_flag(StatusFlags::CARRY);
                    self.set_flag(StatusFlags::CARRY, val & 1);
                    val >>= 1;
                    val |= old_c << 7;
                    self.set_zn_flags(val);
                    memory.write(addr, val);
                };
            }

            /* BITWISE INSTRUCTIONS ========================================
            ============================================================= */
            IN::AND => {
                self.accumulator &= self.fetch_value(memory, ins);
                self.set_zn_flags(self.accumulator);
            }

            IN::BIT => {
                let val = self.fetch_value(memory, ins);
                self.set_flag(StatusFlags::ZERO, (val & self.accumulator == 0) as u8);
                self.set_flag(StatusFlags::OVERFLOW, val >> 6 & 1);
                self.set_flag(StatusFlags::NEGATIVE, val >> 7);
            }

            IN::ORA => {
                self.accumulator |= self.fetch_value(memory, ins);
                self.set_zn_flags(self.accumulator);
            }

            IN::EOR => {
                self.accumulator ^= self.fetch_value(memory, ins);
                self.set_zn_flags(self.accumulator);
            }

            /* COMPARE INSTRUCTIONS ========================================
            ============================================================= */
            IN::CMP => {
                let val = self.fetch_value(memory, ins);
                self.set_flag(
                    StatusFlags::CARRY,
                    if self.accumulator >= val { 1 } else { 0 },
                );
                self.set_flag(
                    StatusFlags::ZERO,
                    if self.accumulator == val { 1 } else { 0 },
                );
                self.set_flag(
                    StatusFlags::NEGATIVE,
                    self.accumulator.wrapping_sub(val) >> 7,
                );
            }

            IN::CPX => {
                let val = self.fetch_value(memory, ins);
                self.set_flag(StatusFlags::CARRY, if self.reg_x >= val { 1 } else { 0 });
                self.set_flag(StatusFlags::ZERO, if self.reg_x == val { 1 } else { 0 });
                self.set_flag(StatusFlags::NEGATIVE, self.reg_x.wrapping_sub(val) >> 7);
            }

            IN::CPY => {
                let val = self.fetch_value(memory, ins);
                self.set_flag(StatusFlags::CARRY, if self.reg_y >= val { 1 } else { 0 });
                self.set_flag(StatusFlags::ZERO, if self.reg_y == val { 1 } else { 0 });
                self.set_flag(StatusFlags::NEGATIVE, self.reg_y.wrapping_sub(val) >> 7);
            }

            /* BRANCH INSTRUCTIONS =========================================
            ============================================================= */
            IN::BCC => {
                if self.get_flag(StatusFlags::CARRY) == 0 {
                    self.cycle_count += 1;
                    // TODO find better way?
                    let byte = self.get_next_u8(memory);
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
                if self.get_flag(StatusFlags::CARRY) == 1 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8(memory);
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
                if self.get_flag(StatusFlags::ZERO) == 1 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8(memory);
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
                if self.get_flag(StatusFlags::ZERO) == 0 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8(memory);
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
                if self.get_flag(StatusFlags::NEGATIVE) == 0 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8(memory);
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
                if self.get_flag(StatusFlags::NEGATIVE) == 1 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8(memory);
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
                if self.get_flag(StatusFlags::OVERFLOW) == 0 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8(memory);
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
                if self.get_flag(StatusFlags::OVERFLOW) == 1 {
                    self.cycle_count += 1;
                    let byte = self.get_next_u8(memory);
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
                    self.program_counter = self.get_next_u16(memory);
                } else {
                    let addr = self.get_next_u16(memory);
                    if addr & 0xff == 0xff {
                        self.program_counter = make16!(memory.read(addr - 0xff), memory.read(addr));
                    } else {
                        self.program_counter =
                            make16!(memory.read(addr.wrapping_add(1)), memory.read(addr));
                    }
                }
            }

            IN::JSR => {
                let sr_addr = self.get_next_u16(memory);

                self.stack_push_16bit(memory, self.program_counter - 1);
                self.program_counter = sr_addr;
            }

            IN::RTI => {
                self.flags = StatusFlags::from_bits_truncate(
                    self.stack_pull(memory) & 0b11101111 | 0b00100000,
                );
                self.program_counter = self.stack_pull_16bit(memory);
            }

            IN::RTS => {
                self.program_counter = self.stack_pull_16bit(memory);
                self.program_counter += 1;
            }

            IN::BRK => {
                return;
                // todo!("havent implemented BRK yet");
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
                self.stack_push(memory, self.accumulator);
            }

            IN::PHP => {
                self.stack_push(memory, self.flags.bits() | 0b00110000);
            }

            IN::PLA => {
                self.accumulator = self.stack_pull(memory);
                self.set_zn_flags(self.accumulator);
            }

            IN::PLP => {
                self.flags = StatusFlags::from_bits_truncate(
                    self.stack_pull(memory) & 0b11101111 | 0b00100000,
                );
                // TODO the I flag needs to be delayed 1 instr
            }

            /* FLAG INSTRUCTIONS ===========================================
            ============================================================= */
            IN::CLC => {
                self.set_flag(StatusFlags::CARRY, 0);
            }

            IN::CLD => {
                self.set_flag(StatusFlags::DECIMAL, 0);
            }

            IN::CLI => {
                // TODO this needs to be delayed by 1 instruction
                self.set_flag(StatusFlags::INTERRUPT_DISABLE, 0);
            }

            IN::CLV => {
                self.set_flag(StatusFlags::OVERFLOW, 0);
            }

            IN::SEC => {
                self.set_flag(StatusFlags::CARRY, 1);
            }

            IN::SED => {
                self.set_flag(StatusFlags::DECIMAL, 1);
            }

            IN::SEI => {
                self.set_flag(StatusFlags::INTERRUPT_DISABLE, 1);
            }

            /* OTHER INSTRUCTIONS ==========================================
            ============================================================= */
            IN::NOP => {
                // do nothing
            }
        };
    }

    fn add_to_acc(&mut self, val: u8) {
        let sum =
            (self.accumulator as u16) + (val as u16) + (self.get_flag(StatusFlags::CARRY) as u16);

        let carry = sum > 0xff;

        if carry {
            self.set_flag(StatusFlags::CARRY, 1);
        } else {
            self.set_flag(StatusFlags::CARRY, 0);
        }

        let result = sum as u8;

        if (val ^ result) & (result ^ self.accumulator) & 0x80 != 0 {
            self.set_flag(StatusFlags::OVERFLOW, 1);
        } else {
            self.set_flag(StatusFlags::OVERFLOW, 0);
        }

        self.accumulator = result;
    }

    // INCREMENTS PC
    fn get_next_u8(&mut self, memory: &mut MemoryBus) -> u8 {
        self.program_counter += 1;
        memory.read(self.program_counter - 1)
    }

    fn get_next_u16(&mut self, memory: &mut MemoryBus) -> u16 {
        self.program_counter += 2;
        make16!(
            memory.read(self.program_counter - 1),
            memory.read(self.program_counter - 2)
        )
    }

    pub fn get_addr_8bit(
        &mut self,
        memory: &mut MemoryBus,
        address: u8,
        mode: AddressingMode,
    ) -> u16 {
        use AddressingMode as M;
        match mode {
            M::ZeroPage => address as u16,
            M::ZeroPageX => self.reg_x.wrapping_add(address) as u16,
            M::ZeroPageY => self.reg_y.wrapping_add(address) as u16,
            M::IndexedIndirect => {
                let hi = self.get_addr_8bit(memory, address.wrapping_add(1), M::ZeroPageX);
                let lo = self.get_addr_8bit(memory, address, M::ZeroPageX);
                make16!(memory.read(hi), memory.read(lo))
            }
            M::IndirectIndexed => {
                // TODO add a method for this
                let hi = self.get_addr_8bit(memory, address.wrapping_add(1), M::ZeroPage);
                let lo = self.get_addr_8bit(memory, address, M::ZeroPage);
                make16!(memory.read(hi), memory.read(lo)).wrapping_add(self.reg_y as u16)
            }
            M::Relative | M::Immediate | M::Accumulator | M::Implicit => {
                panic!("{:?} does not need memory address", mode)
            }
            _ => {
                panic!("wrong mode, use get_addr_16bit for {:?}", mode)
            }
        }
    }

    pub fn get_addr_16bit(
        &mut self,
        memory: &mut MemoryBus,
        address: u16,
        mode: AddressingMode,
    ) -> u16 {
        use AddressingMode as M;
        match mode {
            M::Indirect => {
                make16!(memory.read(address.wrapping_add(1)), memory.read(address))
            }
            M::Absolute => address,
            M::AbsoluteX => address.wrapping_add(self.reg_x as u16),
            M::AbsoluteY => address.wrapping_add(self.reg_y as u16),
            M::Relative | M::Immediate | M::Accumulator | M::Implicit => {
                panic!("{:?} does not need memory address", mode)
            }
            _ => {
                panic!("wrong mode, use get_addr_8bit for {:?}", mode)
            }
        }
    }

    pub fn read_8bit(&mut self, memory: &mut MemoryBus, address: u8, mode: AddressingMode) -> u8 {
        let addr = self.get_addr_8bit(memory, address, mode);
        memory.read(addr)
    }

    pub fn read_16bit(&mut self, memory: &mut MemoryBus, address: u16, mode: AddressingMode) -> u8 {
        let addr = self.get_addr_16bit(memory, address, mode);
        memory.read(addr)
    }

    fn fetch_value(&mut self, memory: &mut MemoryBus, ins: Instruction) -> u8 {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::Immediate => self.get_next_u8(memory),
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.get_next_u8(memory);
                self.read_8bit(memory, addr, mode)
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.get_next_u16(memory);
                self.read_16bit(memory, addr, mode)
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    fn fetch_value_keep_addr(&mut self, memory: &mut MemoryBus, ins: Instruction) -> (u8, u16) {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.get_next_u8(memory);
                (
                    self.read_8bit(memory, addr, mode),
                    self.get_addr_8bit(memory, addr, mode),
                )
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.get_next_u16(memory);
                (
                    self.read_16bit(memory, addr, mode),
                    self.get_addr_16bit(memory, addr, mode),
                )
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    fn store_value(&mut self, memory: &mut MemoryBus, val: u8, ins: Instruction) {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.get_next_u8(memory);
                let write_addr = self.get_addr_8bit(memory, addr, mode);
                memory.write(write_addr, val);
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.get_next_u16(memory);
                let write_addr = self.get_addr_16bit(memory, addr, mode);
                memory.write(write_addr, val);
            }
            _ => panic!("cannot store value for {:?}", mode),
        }
    }

    fn set_zn_flags(&mut self, val: u8) {
        self.set_flag(StatusFlags::ZERO, if val == 0 { 1 } else { 0 });
        self.set_flag(StatusFlags::NEGATIVE, val >> 7);
    }

    fn inc_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    fn dec_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_peek(&mut self, memory: &mut MemoryBus) -> u8 {
        memory.read(self.stack_pointer as u16 + STACK_START)
    }

    fn stack_pull(&mut self, memory: &mut MemoryBus) -> u8 {
        self.inc_sp();
        self.stack_peek(memory)
    }

    fn stack_push(&mut self, memory: &mut MemoryBus, val: u8) {
        memory.write(self.stack_pointer as u16 + STACK_START, val);
        self.dec_sp();
    }

    fn stack_push_16bit(&mut self, memory: &mut MemoryBus, val: u16) {
        self.stack_push(memory, (val >> 8) as u8);
        self.stack_push(memory, (val & 0xff) as u8);
    }

    fn stack_pull_16bit(&mut self, memory: &mut MemoryBus) -> u16 {
        let lo = self.stack_pull(memory);
        let hi = self.stack_pull(memory);
        make16!(hi, lo)
    }

    fn interrupt_nmi(&mut self, memory: &mut MemoryBus) {
        self.stack_push_16bit(memory, self.program_counter);
        let mut new_flags = self.flags;
        new_flags.remove(StatusFlags::BREAK);
        new_flags.insert(StatusFlags::BREAK2_U);

        self.stack_push(memory, new_flags.bits());
        self.flags.insert(StatusFlags::INTERRUPT_DISABLE);

        memory.tick_ppu(2);
        self.program_counter = memory.read_16bit(0xFFFA); //TODO make this a constant
    }

    pub fn logged_execute(&mut self, memory: &mut MemoryBus, ins: Instruction) -> String {
        // CODE FROM https://github.com/bugzmanov/nes_ebook/blob/master/code/ch5.1/src/trace.rs

        let begin = self.program_counter;
        let mut hex_dump = vec![];
        hex_dump.push(ins.opcode);

        let (mem_addr, stored_value) = match ins.mode {
            AddressingMode::Immediate | AddressingMode::Implicit => (0, 0),
            _ => match ins.bytes {
                1 => (0, 0),
                2 => {
                    let addr = memory.read(begin.wrapping_add(1));
                    if ins.mode != AddressingMode::Relative {
                        (addr as u16, self.read_8bit(memory, addr, ins.mode))
                    } else {
                        (addr as u16, 0)
                    }
                }
                3 => {
                    let addr = make16!(
                        memory.read(begin.wrapping_add(2)),
                        memory.read(begin.wrapping_add(1))
                    );
                    (addr, self.read_16bit(memory, addr, ins.mode))
                }
                _ => {
                    println!("{:?} causing problems", ins);
                    unreachable!()
                }
            },
        };

        let tmp = match ins.bytes {
            1 => match ins.opcode {
                0x0a | 0x4a | 0x2a | 0x6a => "A ".to_string(),
                _ => String::from(""),
            },
            2 => {
                let address: u8 = memory.read(begin.wrapping_add(1));

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
                        address.wrapping_add(self.reg_x),
                        make16!(
                            memory.read((address.wrapping_add(self.reg_x)).wrapping_add(1) as u16),
                            memory.read(address.wrapping_add(self.reg_x) as u16)
                        ),
                        stored_value
                    ),
                    AddressingMode::IndirectIndexed => format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        address,
                        memory.read(mem_addr.wrapping_sub(self.reg_y as u16)),
                        {
                            let hi = memory
                                .read(mem_addr.wrapping_sub(self.reg_y as u16).wrapping_add(1));
                            let lo = memory.read(mem_addr.wrapping_sub(self.reg_y as u16));
                            memory.read(make16!(hi, lo).wrapping_add(self.reg_y as u16))
                        },
                        stored_value
                    ),
                    AddressingMode::Implicit | AddressingMode::Relative => {
                        // assuming local jumps: BNE, BVS, etc....
                        let address: usize = ((begin as usize).wrapping_add(2))
                            .wrapping_add((address as i8) as usize);
                        format!("${:04x}", address)
                    }

                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                        ins.mode, ins.opcode
                    ),
                }
            }
            3 => {
                let address_lo = memory.read(begin.wrapping_add(1));
                let address_hi = memory.read(begin.wrapping_add(2));
                hex_dump.push(address_lo);
                hex_dump.push(address_hi);

                let address = make16!(address_hi, address_lo);

                if ins == JMP_A || ins == JSR_A {
                    format!("${:04x}", address)
                } else if ins == JMP_I {
                    let jmp_addr = if address & 0x00FF == 0x00FF {
                        let lo = memory.read(address);
                        let hi = memory.read(address & 0xFF00);
                        (hi as u16) << 8 | (lo as u16)
                    } else {
                        make16!(memory.read(address.wrapping_add(1)), memory.read(address))
                    };

                    // let jmp_addr = cpu.mem_read_u16(address);
                    format!("(${:04x}) = {:04x}", address, jmp_addr)
                } else {
                    match ins.mode {
                        AddressingMode::Implicit => {
                            format!("${:04x}", address)
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
            "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} PPU:{: >3},{: >3} CYC:{}",
            asm_str,
            self.accumulator,
            self.reg_x,
            self.reg_y,
            self.flags,
            self.stack_pointer,
            memory.get_ppu_scanline(),
            memory.get_ppu_cycles(),
            self.cycle_count
        )
        .to_ascii_uppercase()
    }
}
