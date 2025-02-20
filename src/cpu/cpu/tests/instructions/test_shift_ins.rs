// ASL, LSR, ROL, ROR
use crate::cpu::{
    cpu::tests::{get_example_byte, set_byte_example, set_multiple_bytes},
    instructions, Flag, CPU,
};
use instructions as IN;

#[test]
fn test_asl() {
    let mut cpu = CPU::new();

    cpu.accumulator = 0b10001010;
    cpu.execute(IN::ASL_AC);

    assert_eq!(cpu.get_flag(Flag::Carry), 1);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.accumulator, 0b00010100);

    cpu.accumulator = 0b01101101;
    cpu.execute(IN::ASL_AC);

    assert_eq!(cpu.get_flag(Flag::Carry), 0);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.accumulator, 0b11011010);
}
