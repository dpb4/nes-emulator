// NOP
#![cfg(test)]

use crate::cpu::{
    cpu::tests::{get_example_byte, set_byte_example, set_multiple_bytes},
    instructions::{self, AddressingMode},
    CPU,
};

use instructions as IN;
#[test]
fn test_get_next_u8() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![35, 45, 67, 13, 244]);

    assert_eq!(cpu.get_next_u8(), 35);
    assert_eq!(cpu.get_next_u8(), 45);
    assert_eq!(cpu.get_next_u8(), 67);
    assert_eq!(cpu.get_next_u8(), 13);
    assert_eq!(cpu.get_next_u8(), 244);
}

#[test]
fn test_get_next_u16() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0x35, 0x45, 0x67, 0x13, 0x42]);

    assert_eq!(cpu.get_next_u16(), 0x4535);
    assert_eq!(cpu.get_next_u8(), 0x67);
    assert_eq!(cpu.get_next_u16(), 0x4213);
}

#[test]
fn test_get_addr_8bit() {
    let mut cpu = CPU::new();
    cpu.reg_x = 2;
    cpu.reg_y = 3;

    set_multiple_bytes(&mut cpu, 200, &vec![0x35, 0x45, 0x67, 0x13, 0x42]);

    assert_eq!(cpu.get_addr_8bit(200, AddressingMode::ZeroPage), 200);
    assert_eq!(cpu.get_addr_8bit(200, AddressingMode::ZeroPageX), 202);
    assert_eq!(cpu.get_addr_8bit(200, AddressingMode::ZeroPageY), 203);
    assert_eq!(
        cpu.get_addr_8bit(200, AddressingMode::IndexedIndirect),
        0x1367
    );
    assert_eq!(
        cpu.get_addr_8bit(200, AddressingMode::IndirectIndexed),
        0x4535 + 3
    );
}

#[test]
fn test_get_addr_16bit() {
    let mut cpu = CPU::new();
    cpu.reg_x = 3;
    cpu.reg_y = 2;

    set_multiple_bytes(&mut cpu, 0x1234, &vec![0x35, 0x45, 0x67, 0x13, 0x42]);

    assert_eq!(cpu.get_addr_16bit(0x1234, AddressingMode::Absolute), 0x1234);
    assert_eq!(
        cpu.get_addr_16bit(0x1234, AddressingMode::AbsoluteX),
        0x1234 + 3
    );
    assert_eq!(
        cpu.get_addr_16bit(0x1234, AddressingMode::AbsoluteY),
        0x1234 + 2
    );
    assert_eq!(cpu.get_addr_16bit(0x1234, AddressingMode::Indirect), 0x4535);
}

#[test]
fn test_fetch_value() {
    let mut cpu = CPU::new();

    set_byte_example(&mut cpu);
    set_multiple_bytes(&mut cpu, 0, &vec![0x35, 0x45, 0x67, 0x13, 0x42]);

    assert_eq!(cpu.fetch_value(IN::AND_IM), 0x35);
    assert_eq!(cpu.fetch_value(IN::AND_Z), get_example_byte(0x45));
    assert_eq!(cpu.fetch_value(IN::AND_A), get_example_byte(0x1367));
}
