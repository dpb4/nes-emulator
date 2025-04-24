#![cfg(test)]
use crate::cpu::{StatusFlags, CPU};

#[test]
fn test_setting() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::CARRY, 1);
    cpu.set_flag(StatusFlags::DECIMAL, 1);
    cpu.set_flag(StatusFlags::INTERRUPT_DISABLE, 1);
    cpu.set_flag(StatusFlags::NEGATIVE, 1);
    cpu.set_flag(StatusFlags::OVERFLOW, 1);
    cpu.set_flag(StatusFlags::ZERO, 1);

    assert_eq!(cpu.flags.bits(), 0b11001111);
}

#[test]
fn test_getting() {
    let mut cpu = CPU::new();

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    cpu.set_flag(StatusFlags::CARRY, 1);
    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 1);
    assert_eq!(cpu.flags.bits(), 0b00000001);
    cpu.set_flag(StatusFlags::CARRY, 0);

    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    cpu.set_flag(StatusFlags::ZERO, 1);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 1);
    assert_eq!(cpu.flags.bits(), 0b00000010);
    cpu.set_flag(StatusFlags::ZERO, 0);

    assert_eq!(cpu.get_flag(StatusFlags::INTERRUPT_DISABLE), 0);
    cpu.set_flag(StatusFlags::INTERRUPT_DISABLE, 1);
    assert_eq!(cpu.get_flag(StatusFlags::INTERRUPT_DISABLE), 1);
    assert_eq!(cpu.flags.bits(), 0b00000100);
    cpu.set_flag(StatusFlags::INTERRUPT_DISABLE, 0);

    assert_eq!(cpu.get_flag(StatusFlags::DECIMAL), 0);
    cpu.set_flag(StatusFlags::DECIMAL, 1);
    assert_eq!(cpu.get_flag(StatusFlags::DECIMAL), 1);
    assert_eq!(cpu.flags.bits(), 0b00001000);
    cpu.set_flag(StatusFlags::DECIMAL, 0);

    assert_eq!(cpu.get_flag(StatusFlags::OVERFLOW), 0);
    cpu.set_flag(StatusFlags::OVERFLOW, 1);
    assert_eq!(cpu.get_flag(StatusFlags::OVERFLOW), 1);
    assert_eq!(cpu.flags.bits(), 0b01000000);
    cpu.set_flag(StatusFlags::OVERFLOW, 0);

    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    cpu.set_flag(StatusFlags::NEGATIVE, 1);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 1);
    assert_eq!(cpu.flags.bits(), 0b10000000);
    cpu.set_flag(StatusFlags::NEGATIVE, 0);
}
