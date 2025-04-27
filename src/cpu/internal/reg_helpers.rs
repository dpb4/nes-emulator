use crate::cpu::STACK_START;
use crate::{
    cpu::{StatusFlags, CPU},
    make_u16,
};

impl CPU {
    pub fn add_to_acc(&mut self, val: u8) {
        let sum = (self.accumulator as u16)
            + (val as u16)
            + (self.get_status_bit(StatusFlags::CARRY) as u16);

        let carry = sum > 0xff;

        if carry {
            self.set_status_bit(StatusFlags::CARRY, 1);
        } else {
            self.set_status_bit(StatusFlags::CARRY, 0);
        }

        let result = sum as u8;

        if (val ^ result) & (result ^ self.accumulator) & 0x80 != 0 {
            self.set_status_bit(StatusFlags::OVERFLOW, 1);
        } else {
            self.set_status_bit(StatusFlags::OVERFLOW, 0);
        }

        self.accumulator = result;
    }

    pub fn inc_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    pub fn dec_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn stack_peek(&mut self) -> u8 {
        self.mem_bus.read(self.stack_pointer as u16 + STACK_START)
    }

    pub fn stack_pull_u8(&mut self) -> u8 {
        self.inc_sp();
        self.stack_peek()
    }

    pub fn stack_push_u8(&mut self, val: u8) {
        self.mem_bus
            .write(self.stack_pointer as u16 + STACK_START, val);
        self.dec_sp();
    }

    pub fn stack_push_u16(&mut self, val: u16) {
        self.stack_push_u8((val >> 8) as u8);
        self.stack_push_u8((val & 0xff) as u8);
    }

    pub fn stack_pull_u16(&mut self) -> u16 {
        let lo = self.stack_pull_u8();
        let hi = self.stack_pull_u8();
        make_u16!(hi, lo)
    }
}
