// PHA, PLS, PHP, PLP, TXS, TSX
use crate::cpu::{instructions, StatusFlags, CPU};
use instructions as IN;

#[test]
fn test_pha() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0x15;

    cpu.execute(IN::PHA);

    cpu.accumulator = 0x88;

    cpu.execute(IN::PHA);

    assert_eq!(cpu.stack_pointer, 253);

    assert_eq!(cpu.pull_stack(), 0x88);
    assert_eq!(cpu.stack_pointer, 254);

    assert_eq!(cpu.pull_stack(), 0x15);
    assert_eq!(cpu.stack_pointer, 255);

    assert_eq!(cpu.pull_stack(), 0);
    assert_eq!(cpu.stack_pointer, 0);
}

#[test]
fn test_php() {
    let mut cpu = CPU::new();

    cpu.flags = StatusFlags::from_bits(0b10101100).unwrap();

    cpu.execute(IN::PHP);

    cpu.flags = StatusFlags::from_bits(0b10010111).unwrap();

    cpu.execute(IN::PHP);

    assert_eq!(cpu.stack_pointer, 253);

    assert_eq!(cpu.pull_stack(), 0b10110111);
    assert_eq!(cpu.stack_pointer, 254);

    assert_eq!(cpu.pull_stack(), 0b10111100);
    assert_eq!(cpu.stack_pointer, 255);

    assert_eq!(cpu.pull_stack(), 0);
    assert_eq!(cpu.stack_pointer, 0);
}

#[test]
fn test_pla() {
    let mut cpu = CPU::new();

    cpu.push_stack(0xf3);
    cpu.push_stack(0x61);

    cpu.execute(IN::PLA);

    assert_eq!(cpu.accumulator, 0x61);
    assert_eq!(cpu.stack_pointer, 254);

    cpu.execute(IN::PLA);

    assert_eq!(cpu.accumulator, 0xf3);
    assert_eq!(cpu.stack_pointer, 255);

    cpu.execute(IN::PLA);

    assert_eq!(cpu.accumulator, 0);
    assert_eq!(cpu.stack_pointer, 0);
}

#[test]
fn test_plp() {
    let mut cpu = CPU::new();

    cpu.push_stack(0b10111100);
    cpu.push_stack(0b00110000);

    cpu.execute(IN::PLP);

    assert_eq!(cpu.flags.bits(), 0b00110000);
    assert_eq!(cpu.stack_pointer, 254);

    cpu.execute(IN::PLP);

    assert_eq!(cpu.flags.bits(), 0b10111100);
    assert_eq!(cpu.stack_pointer, 255);

    cpu.execute(IN::PLP);

    assert_eq!(cpu.flags.bits(), 0);
    assert_eq!(cpu.stack_pointer, 0);
}

#[test]
fn test_txs() {
    let mut cpu = CPU::new();

    cpu.reg_x = 0x81;

    cpu.execute(IN::TXS);

    assert_eq!(cpu.stack_pointer, 0x81);
}

#[test]
fn test_tsx() {
    let mut cpu = CPU::new();

    cpu.stack_pointer = 0;

    cpu.execute(IN::TSX);

    assert_eq!(cpu.reg_x, 0);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 1);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);

    cpu.stack_pointer = 0x3f;

    cpu.execute(IN::TSX);

    assert_eq!(cpu.reg_x, 0x3f);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 0);

    cpu.stack_pointer = 0xde;

    cpu.execute(IN::TSX);

    assert_eq!(cpu.reg_x, 0xde);
    assert_eq!(cpu.get_flag(StatusFlags::ZERO), 0);
    assert_eq!(cpu.get_flag(StatusFlags::NEGATIVE), 1);
}
