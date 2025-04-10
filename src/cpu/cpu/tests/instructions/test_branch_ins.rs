// BCC, BCS, BEQ, BNE, BPL, BMI, BVC, BVS

use crate::cpu::{cpu::tests::set_multiple_bytes, instructions, StatusFlags, CPU};
use instructions as IN;

#[test]
fn test_bcs() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::CARRY, 1);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BCS);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BCS);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BCS);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::CARRY, 0);
    cpu.execute(IN::BCS);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}

#[test]
fn test_bcc() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::CARRY, 0);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BCC);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BCC);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BCC);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::CARRY, 1);
    cpu.execute(IN::BCC);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}

#[test]
fn test_beq() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::ZERO, 1);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BEQ);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BEQ);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BEQ);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::ZERO, 0);
    cpu.execute(IN::BEQ);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}

#[test]
fn test_bne() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::ZERO, 0);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BNE);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BNE);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BNE);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::ZERO, 1);
    cpu.execute(IN::BNE);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}

#[test]
fn test_bmi() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::NEGATIVE, 1);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BMI);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BMI);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BMI);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::NEGATIVE, 0);
    cpu.execute(IN::BMI);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}

#[test]
fn test_bpl() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::NEGATIVE, 0);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BPL);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BPL);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BPL);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::NEGATIVE, 1);
    cpu.execute(IN::BPL);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}

#[test]
fn test_bvc() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::OVERFLOW, 0);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BVC);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BVC);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BVC);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::OVERFLOW, 1);
    cpu.execute(IN::BVC);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}

#[test]
fn test_bvs() {
    let mut cpu = CPU::new();

    cpu.set_flag(StatusFlags::OVERFLOW, 1);
    set_multiple_bytes(&mut cpu, 2111, &vec![0, 0, 0, 3, 0, 0, 0, 0, 0xff, 0x10]);

    cpu.program_counter = 2111;
    cpu.execute(IN::BVS);

    assert_eq!(cpu.cycle_count, 3);
    assert_eq!(cpu.program_counter, 2113);

    cpu.execute(IN::BVS);

    assert_eq!(cpu.cycle_count, 6);
    assert_eq!(cpu.program_counter, 2113 + 2 + 3);

    cpu.execute(IN::BVS);

    assert_eq!(cpu.cycle_count, 9);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1);

    cpu.set_flag(StatusFlags::OVERFLOW, 0);
    cpu.execute(IN::BVS);

    assert_eq!(cpu.cycle_count, 11);
    assert_eq!(cpu.program_counter, (2113 + 2 + 3) + 2 - 1 + 2);
}
