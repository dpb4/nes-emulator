// ASL, LSR, ROL, ROR
use crate::cpu::{instructions, StatusFlags, CPU};
use instructions as IN;

#[test]
fn test_asl() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0b10001010;
    cpu.execute(IN::ASL_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 1);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b00010100);

    cpu.accumulator = 0b01101101;
    cpu.execute(IN::ASL_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 1);
    assert_eq!(cpu.accumulator, 0b11011010);
}

#[test]
fn test_lsr() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0b10001010;
    cpu.execute(IN::LSR_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b01000101);

    cpu.accumulator = 0b01101101;
    cpu.execute(IN::LSR_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 1);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b00110110);
}

#[test]
fn test_rol() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0b10001010;
    cpu.execute(IN::ROL_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 1);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b00010100);

    cpu.execute(IN::ROL_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b00101001);

    cpu.execute(IN::ROL_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b01010010);
}

#[test]
fn test_ror() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0b10001010;
    cpu.execute(IN::ROR_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b01000101);

    cpu.execute(IN::ROR_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 1);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);
    assert_eq!(cpu.accumulator, 0b00100010);

    cpu.execute(IN::ROR_AC);

    assert_eq!(cpu.get_flag(StatusFlags::CARRY), 0);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 1);
    assert_eq!(cpu.accumulator, 0b10010001);
}
