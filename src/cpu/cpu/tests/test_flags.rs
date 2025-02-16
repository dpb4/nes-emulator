#![cfg(test)]
use crate::cpu::{Flag, CPU};

#[test]
fn test_setting() {
    let mut cpu = CPU::new();

    cpu.set_flag(Flag::Carry, 1);
    cpu.set_flag(Flag::Decimal, 1);
    cpu.set_flag(Flag::Interrupt, 1);
    cpu.set_flag(Flag::Negative, 1);
    cpu.set_flag(Flag::Overflow, 1);
    cpu.set_flag(Flag::Zero, 1);

    assert_eq!(cpu.flags, 0b11001111);
}

#[test]
fn test_getting() {
    let mut cpu = CPU::new();

    assert_eq!(cpu.get_flag(Flag::Carry), 0);
    cpu.set_flag(Flag::Carry, 1);
    assert_eq!(cpu.get_flag(Flag::Carry), 1);
    assert_eq!(cpu.flags, 0b00000001);
    cpu.set_flag(Flag::Carry, 0);

    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    cpu.set_flag(Flag::Zero, 1);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.flags, 0b00000010);
    cpu.set_flag(Flag::Zero, 0);

    assert_eq!(cpu.get_flag(Flag::Interrupt), 0);
    cpu.set_flag(Flag::Interrupt, 1);
    assert_eq!(cpu.get_flag(Flag::Interrupt), 1);
    assert_eq!(cpu.flags, 0b00000100);
    cpu.set_flag(Flag::Interrupt, 0);

    assert_eq!(cpu.get_flag(Flag::Decimal), 0);
    cpu.set_flag(Flag::Decimal, 1);
    assert_eq!(cpu.get_flag(Flag::Decimal), 1);
    assert_eq!(cpu.flags, 0b00001000);
    cpu.set_flag(Flag::Decimal, 0);

    assert_eq!(cpu.get_flag(Flag::Overflow), 0);
    cpu.set_flag(Flag::Overflow, 1);
    assert_eq!(cpu.get_flag(Flag::Overflow), 1);
    assert_eq!(cpu.flags, 0b01000000);
    cpu.set_flag(Flag::Overflow, 0);

    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    cpu.set_flag(Flag::Negative, 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.flags, 0b10000000);
    cpu.set_flag(Flag::Negative, 0);
}
