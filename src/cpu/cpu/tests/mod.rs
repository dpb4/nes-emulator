#![cfg(test)]

use super::CPU;
pub mod instructions;
pub mod test_flags;
pub mod test_memory;

pub fn set_single_byte(cpu: &mut CPU, address: u16, byte: u8) {
    cpu.memory.write(address, byte);
}

pub fn set_multiple_bytes(cpu: &mut CPU, start_address: u16, bytes: &Vec<u8>) {
    for i in 0..bytes.len() {
        set_single_byte(cpu, start_address + i as u16, bytes[i]);
    }
}

pub fn set_byte_example(cpu: &mut CPU) {
    for i in 0..0xffff_u16 {
        let val = i.wrapping_add(1857).wrapping_mul(937) as u8;
        set_single_byte(cpu, i, val);
    }
}

pub fn get_example_byte(i: u16) -> u8 {
    i.wrapping_add(1857).wrapping_mul(937) as u8
}
