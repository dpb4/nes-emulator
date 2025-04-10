// CMP, CPX, CPY
use crate::cpu::{cpu::tests::set_multiple_bytes, instructions, CPU};
use instructions as IN;

#[test]
fn test_cmp() {
    let mut cpu = CPU::new();
    set_multiple_bytes(&mut cpu, 0, &vec![0, 5, 0, 20, 0, 10]);

    cpu.accumulator = 10;
    cpu.reg_x = 100;
    cpu.reg_y = 200;

    cpu.execute(IN::CMP_IM);

    assert_eq!(cpu.flags.bits(), 0b00000001);

    cpu.execute(IN::CMP_IM);

    assert_eq!(cpu.flags.bits(), 0b10000000);

    cpu.execute(IN::CMP_IM);

    assert_eq!(cpu.flags.bits(), 0b00000011);
}

#[test]
fn test_cpx() {
    let mut cpu = CPU::new();
    set_multiple_bytes(&mut cpu, 0, &vec![0, 5, 0, 20, 0, 10]);

    cpu.accumulator = 100;
    cpu.reg_x = 10;
    cpu.reg_y = 200;

    cpu.execute(IN::CPX_IM);

    assert_eq!(cpu.flags.bits(), 0b00000001);

    cpu.execute(IN::CPX_IM);

    assert_eq!(cpu.flags.bits(), 0b10000000);

    cpu.execute(IN::CPX_IM);

    assert_eq!(cpu.flags.bits(), 0b00000011);
}

#[test]
fn test_cpy() {
    let mut cpu = CPU::new();
    set_multiple_bytes(&mut cpu, 0, &vec![0, 5, 0, 20, 0, 10]);

    cpu.accumulator = 100;
    cpu.reg_x = 200;
    cpu.reg_y = 10;

    cpu.execute(IN::CPY_IM);

    assert_eq!(cpu.flags.bits(), 0b00000001);

    cpu.execute(IN::CPY_IM);

    assert_eq!(cpu.flags.bits(), 0b10000000);

    cpu.execute(IN::CPY_IM);

    assert_eq!(cpu.flags.bits(), 0b00000011);
}
