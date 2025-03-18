// TAX, TXA, TAY, TYA
use crate::cpu::{instructions, Flag, CPU};
use instructions as IN;

#[test]
fn test_tax() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0x12;
    cpu.reg_x = 0x34;
    cpu.reg_y = 0x56;

    cpu.execute(IN::TAX);

    assert_eq!(cpu.accumulator, 0x12);
    assert_eq!(cpu.reg_x, 0x12);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
}

#[test]
fn test_txa() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0x12;
    cpu.reg_x = 0x84;
    cpu.reg_y = 0x56;

    cpu.execute(IN::TXA);

    assert_eq!(cpu.accumulator, 0x84);
    assert_eq!(cpu.reg_x, 0x84);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
}

#[test]
fn test_tay() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0x12;
    cpu.reg_y = 0x34;
    cpu.reg_x = 0x56;

    cpu.execute(IN::TAY);

    assert_eq!(cpu.accumulator, 0x12);
    assert_eq!(cpu.reg_y, 0x12);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
}

#[test]
fn test_tya() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0x12;
    cpu.reg_y = 0x84;
    cpu.reg_x = 0x56;

    cpu.execute(IN::TYA);

    assert_eq!(cpu.accumulator, 0x84);
    assert_eq!(cpu.reg_y, 0x84);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
}
