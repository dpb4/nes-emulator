// AND, ORA, EOR, BIT

use crate::cpu::{
    cpu::tests::{get_example_byte, set_byte_example, set_multiple_bytes},
    instructions, Flag, CPU,
};
use instructions as IN;

#[test]
fn test_and() {
    let mut cpu = CPU::new();
    set_byte_example(&mut cpu);
    set_multiple_bytes(
        &mut cpu,
        0,
        &vec![255, 0b11110000, 0b00001111, 0b00001110, 0b00001010],
    );

    cpu.accumulator = 0b11010011 as u8;

    cpu.execute(IN::AND_IM);

    assert_eq!(cpu.program_counter, 2);
    assert_eq!(cpu.accumulator, 0b11010000 as u8);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.cycle_count, 2);

    cpu.accumulator = 255;
    cpu.execute(IN::AND_Z);

    assert_eq!(cpu.program_counter, 4);
    assert_eq!(cpu.accumulator, get_example_byte(0b00001110));
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.cycle_count, 5);
}

#[test]
fn test_bit() {
    let mut cpu = CPU::new();

    cpu.accumulator = 255;
    set_multiple_bytes(&mut cpu, 1, &vec![0x01, 0x01, 0, 0x02, 0x01, 0, 0x03, 0x01]);
    set_multiple_bytes(&mut cpu, 0x0101, &vec![0b01000000, 0b10000000, 0]);

    cpu.execute(IN::BIT_A);

    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Overflow), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);

    cpu.execute(IN::BIT_A);

    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Overflow), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);

    cpu.execute(IN::BIT_A);

    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Overflow), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
}

#[test]
fn test_eor() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xf0, 0, 0x0, 0, 0xff]);

    cpu.accumulator = 255;
    cpu.execute(IN::EOR_IM);

    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.accumulator, !0xf0);

    cpu.accumulator = 255;
    cpu.execute(IN::EOR_IM);

    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.accumulator, !0x0);

    cpu.accumulator = 255;
    cpu.execute(IN::EOR_IM);

    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.accumulator, !0xff);
}
#[test]
fn test_ora() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xf0, 0, 0x00, 0, 0x00]);

    cpu.accumulator = 0xab;
    cpu.execute(IN::ORA_IM);

    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.accumulator, 0xfb);

    cpu.accumulator = 0xab;
    cpu.execute(IN::ORA_IM);

    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.accumulator, 0xab);

    cpu.accumulator = 0x0;
    cpu.execute(IN::ORA_IM);

    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.accumulator, 0x00);
}
