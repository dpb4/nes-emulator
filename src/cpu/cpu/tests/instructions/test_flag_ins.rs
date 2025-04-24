// CLC, SEC, CLI, SEI, CLD, SED, CLV
use crate::cpu::{instructions, StatusFlags, CPU};
use bitflags::Flags;
use instructions as IN;

#[test]
fn test_clc() {
    let mut cpu = CPU::new();
    cpu.set_flag(StatusFlags::CARRY, 1);
    cpu.set_flag(StatusFlags::OVERFLOW, 1);

    cpu.execute(IN::CLC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    assert_eq!(cpu.get_flag(StatusFlags::OVERFLOW), 1);
}

#[test]
fn test_cld() {
    let mut cpu = CPU::new();
    cpu.set_flag(StatusFlags::DECIMAL, 1);
    cpu.set_flag(StatusFlags::OVERFLOW, 1);

    cpu.execute(IN::CLD);

    assert_eq!(cpu.get_flag(StatusFlags::DECIMAL), 0);
    assert_eq!(cpu.get_flag(StatusFlags::OVERFLOW), 1);
}

#[test]
fn test_cli() {
    // TODO this one is special
    let mut cpu = CPU::new();
    cpu.set_flag(StatusFlags::INTERRUPT_DISABLE, 1);
    cpu.set_flag(StatusFlags::OVERFLOW, 1);

    cpu.execute(IN::CLI);

    assert_eq!(cpu.get_flag(StatusFlags::INTERRUPT_DISABLE), 0);
    assert_eq!(cpu.get_flag(StatusFlags::OVERFLOW), 1);
}

#[test]
fn test_clv() {
    let mut cpu = CPU::new();
    cpu.set_flag(StatusFlags::OVERFLOW, 1);
    cpu.set_flag(StatusFlags::CARRY, 1);

    cpu.execute(IN::CLV);

    assert_eq!(cpu.get_flag(StatusFlags::OVERFLOW), 0);
    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 1);
}

#[test]
fn test_sec() {
    let mut cpu = CPU::new();
    cpu.flags.clear();

    cpu.execute(IN::SEC);

    assert_eq!(cpu.flags.bits(), 0b00000001);
}

#[test]
fn test_sei() {
    let mut cpu = CPU::new();
    cpu.flags.clear();

    cpu.execute(IN::SEI);

    assert_eq!(cpu.flags.bits(), 0b00000100);
}

#[test]
fn test_sed() {
    let mut cpu = CPU::new();
    cpu.flags.clear();

    cpu.execute(IN::SED);

    assert_eq!(cpu.flags.bits(), 0b00001000);
}
