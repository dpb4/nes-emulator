use super::instructions::Instruction;

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
            _ => (),
        }
    }
}
