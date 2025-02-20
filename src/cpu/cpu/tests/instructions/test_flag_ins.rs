// CLC, SEC, CLI, SEI, CLD, SED, CLV
use crate::cpu::{instructions, Flag, CPU};
use instructions as IN;

#[test]
fn test_clc() {
    let mut cpu = CPU::new();
    cpu.set_flag(Flag::Carry, 1);
    cpu.set_flag(Flag::Overflow, 1);

    cpu.execute(IN::CLC);

    assert_eq!(cpu.get_flag(Flag::Carry), 0);
    assert_eq!(cpu.get_flag(Flag::Overflow), 1);
}

#[test]
fn test_cld() {
    let mut cpu = CPU::new();
    cpu.set_flag(Flag::Decimal, 1);
    cpu.set_flag(Flag::Overflow, 1);

    cpu.execute(IN::CLD);

    assert_eq!(cpu.get_flag(Flag::Decimal), 0);
    assert_eq!(cpu.get_flag(Flag::Overflow), 1);
}

#[test]
fn test_cli() {
    // TODO this one is special
    let mut cpu = CPU::new();
    cpu.set_flag(Flag::Interrupt, 1);
    cpu.set_flag(Flag::Overflow, 1);

    cpu.execute(IN::CLI);

    assert_eq!(cpu.get_flag(Flag::Interrupt), 0);
    assert_eq!(cpu.get_flag(Flag::Overflow), 1);
}

#[test]
fn test_clv() {
    let mut cpu = CPU::new();
    cpu.set_flag(Flag::Overflow, 1);
    cpu.set_flag(Flag::Carry, 1);

    cpu.execute(IN::CLV);

    assert_eq!(cpu.get_flag(Flag::Overflow), 0);
    assert_eq!(cpu.get_flag(Flag::Carry), 1);
}
