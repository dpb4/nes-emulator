use crate::cpu::{StatusFlags, CPU};

impl CPU {
    pub fn set_status_bit(&mut self, bit: StatusFlags, val: u8) {
        self.flags.set(bit, val == 1);
    }

    pub fn get_status_bit(&self, bit: StatusFlags) -> u8 {
        if self.flags.contains(bit) {
            1
        } else {
            0
        }
    }

    pub fn set_zn_flags(&mut self, val: u8) {
        self.set_status_bit(StatusFlags::ZERO, if val == 0 { 1 } else { 0 });
        self.set_status_bit(StatusFlags::NEGATIVE, val >> 7);
    }
}
