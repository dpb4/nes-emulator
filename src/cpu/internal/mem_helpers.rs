use crate::{
    cpu::{
        instructions::{AddressingMode, Instruction},
        CPU,
    },
    make_u16, LogEvent,
};
use LogEvent as LE;

impl CPU {
    // INCREMENTS PC
    pub fn mem_read_pc_u8(&mut self) -> u8 {
        self.program_counter += 1;
        self.mem_bus.read(self.program_counter - 1)
    }

    // INCREMENTS PC
    pub fn mem_read_pc_u16(&mut self) -> u16 {
        self.program_counter += 2;
        let operand = self.mem_bus.read_16bit(self.program_counter - 2);
        
        if let Some(l) = self.logger.as_mut() {
            l.log_event(LE::OperandFetch(operand));
        }

        operand
        // make16!(
        //     self.mem_bus.read(self.program_counter - 1),
        //     self.mem_bus.read(self.program_counter - 2)
        // )
    }

    pub fn get_addr_8bit(&mut self, address: u8, mode: AddressingMode) -> u16 {
        use AddressingMode as M;
        match mode {
            M::ZeroPage => address as u16,
            M::ZeroPageX => self.reg_x.wrapping_add(address) as u16,
            M::ZeroPageY => self.reg_y.wrapping_add(address) as u16,
            M::IndexedIndirect => {
                let hi = self.get_addr_8bit(address.wrapping_add(1), M::ZeroPageX);
                let lo = self.get_addr_8bit(address, M::ZeroPageX);
                make_u16!(self.mem_bus.read(hi), self.mem_bus.read(lo))
            }
            M::IndirectIndexed => {
                // TODO add a method for this
                let hi = self.get_addr_8bit(address.wrapping_add(1), M::ZeroPage);
                let lo = self.get_addr_8bit(address, M::ZeroPage);
                make_u16!(self.mem_bus.read(hi), self.mem_bus.read(lo))
                    .wrapping_add(self.reg_y as u16)
            }
            M::Relative | M::Immediate | M::Accumulator | M::Implicit => {
                panic!("{:?} does not need memory address", mode)
            }
            _ => {
                panic!("wrong mode, use get_addr_16bit for {:?}", mode)
            }
        }
    }

    pub fn get_addr_16bit(&mut self, address: u16, mode: AddressingMode) -> u16 {
        use AddressingMode as M;
        match mode {
            M::Indirect => {
                self.mem_bus.read_16bit(address)
                // make_u16!(self.mem_bus.read(address.wrapping_add(1)), self.mem_bus.read(address))
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

    pub fn dbg_get_addr_8bit(&mut self, address: u8, mode: AddressingMode) -> u16 {
        use AddressingMode as M;
        match mode {
            M::ZeroPage => address as u16,
            M::ZeroPageX => self.reg_x.wrapping_add(address) as u16,
            M::ZeroPageY => self.reg_y.wrapping_add(address) as u16,
            M::IndexedIndirect => {
                let hi = self.dbg_get_addr_8bit(address.wrapping_add(1), M::ZeroPageX);
                let lo = self.dbg_get_addr_8bit(address, M::ZeroPageX);
                make_u16!(self.mem_bus.dbg_read(hi), self.mem_bus.dbg_read(lo))
            }
            M::IndirectIndexed => {
                // TODO add a method for this
                let hi = self.dbg_get_addr_8bit(address.wrapping_add(1), M::ZeroPage);
                let lo = self.dbg_get_addr_8bit(address, M::ZeroPage);
                make_u16!(self.mem_bus.dbg_read(hi), self.mem_bus.dbg_read(lo))
                    .wrapping_add(self.reg_y as u16)
            }
            M::Relative | M::Immediate | M::Accumulator | M::Implicit => {
                panic!("{:?} does not need memory address", mode)
            }
            _ => {
                panic!("wrong mode, use get_addr_16bit for {:?}", mode)
            }
        }
    }

    pub fn dbg_get_addr_16bit(&mut self, address: u16, mode: AddressingMode) -> u16 {
        use AddressingMode as M;
        match mode {
            M::Indirect => {
                make_u16!(
                    self.mem_bus.dbg_read(address.wrapping_add(1)),
                    self.mem_bus.dbg_read(address)
                )
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

    pub fn mem_read_with_mode_u8(&mut self, address: u8, mode: AddressingMode) -> u8 {
        let addr = self.get_addr_8bit(address, mode);
        self.mem_bus.read(addr)
    }

    pub fn mem_read_with_mode_u16(&mut self, address: u16, mode: AddressingMode) -> u8 {
        let addr = self.get_addr_16bit(address, mode);
        self.mem_bus.read(addr)
    }

    pub fn dbg_read_8bit(&mut self, address: u8, mode: AddressingMode) -> u8 {
        let addr = self.dbg_get_addr_8bit(address, mode);
        self.mem_bus.dbg_read(addr)
    }

    pub fn dbg_read_16bit(&mut self, address: u16, mode: AddressingMode) -> u8 {
        let addr = self.dbg_get_addr_16bit(address, mode);
        self.mem_bus.dbg_read(addr)
    }

    pub fn fetch_ins_operand(&mut self, ins: Instruction) -> u8 {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::Immediate => self.mem_read_pc_u8(),
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.mem_read_pc_u8();
                self.mem_read_with_mode_u8(addr, mode)
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.mem_read_pc_u16();
                self.mem_read_with_mode_u16(addr, mode)
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    pub fn fetch_value_keep_addr(&mut self, ins: Instruction) -> (u8, u16) {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.mem_read_pc_u8();
                (
                    self.mem_read_with_mode_u8(addr, mode),
                    self.get_addr_8bit(addr, mode),
                )
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.mem_read_pc_u16();
                (
                    self.mem_read_with_mode_u16(addr, mode),
                    self.get_addr_16bit(addr, mode),
                )
            }
            _ => panic!("cannot fetch value for {:?}", mode),
        }
    }

    pub fn mem_write_with_mode(&mut self, val: u8, ins: Instruction) {
        use AddressingMode as M;

        let mode = ins.mode;

        match mode {
            M::ZeroPage | M::ZeroPageX | M::ZeroPageY | M::IndexedIndirect | M::IndirectIndexed => {
                let addr = self.mem_read_pc_u8();
                let write_addr = self.get_addr_8bit(addr, mode);
                self.mem_bus.write(write_addr, val);
            }
            M::Absolute | M::AbsoluteX | M::AbsoluteY | M::Indirect => {
                let addr = self.mem_read_pc_u16();
                let write_addr = self.get_addr_16bit(addr, mode);
                self.mem_bus.write(write_addr, val);
            }
            _ => panic!("cannot store value for {:?}", mode),
        }
    }
}
