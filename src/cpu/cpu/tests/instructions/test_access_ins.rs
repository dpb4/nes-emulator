// LDA, STA, LDX, STX, LDY, STY
use crate::cpu::{cpu::tests::set_multiple_bytes, instructions, Flag, CPU};
use instructions as IN;

#[test]
pub fn test_lda() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xab, 0, 0x00, 0, 0x05]);

    cpu.execute(IN::LDA_IM);
    assert_eq!(cpu.accumulator, 0xab);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);

    cpu.execute(IN::LDA_IM);
    assert_eq!(cpu.accumulator, 0x00);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);

    cpu.execute(IN::LDA_IM);
    assert_eq!(cpu.accumulator, 0x05);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
}

#[test]
pub fn test_ldx() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xab, 0, 0x00, 0, 0x05]);

    cpu.execute(IN::LDX_IM);
    assert_eq!(cpu.reg_x, 0xab);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);

    cpu.execute(IN::LDX_IM);
    assert_eq!(cpu.reg_x, 0x00);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);

    cpu.execute(IN::LDX_IM);
    assert_eq!(cpu.reg_x, 0x05);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
}

#[test]
pub fn test_ldy() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xab, 0, 0x00, 0, 0x05]);

    cpu.execute(IN::LDY_IM);
    assert_eq!(cpu.reg_y, 0xab);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);

    cpu.execute(IN::LDY_IM);
    assert_eq!(cpu.reg_y, 0x00);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);

    cpu.execute(IN::LDY_IM);
    assert_eq!(cpu.reg_y, 0x05);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
}

#[test]
pub fn test_sta() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xab, 0xcd, 0, 0x33]);

    cpu.accumulator = 0xbc;
    cpu.execute(IN::STA_A);
    assert_eq!(cpu.memory[0xcdab as usize], 0xbc);

    cpu.accumulator = 0xde;
    cpu.execute(IN::STA_Z);
    assert_eq!(cpu.memory[0x33 as usize], 0xde);
}

#[test]
pub fn test_stx() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xab, 0xcd, 0, 0x33]);

    cpu.reg_x = 0xbc;
    cpu.execute(IN::STX_A);
    assert_eq!(cpu.memory[0xcdab as usize], 0xbc);

    cpu.reg_x = 0xde;
    cpu.execute(IN::STX_Z);
    assert_eq!(cpu.memory[0x33 as usize], 0xde);
}

#[test]
pub fn test_sty() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0xab, 0xcd, 0, 0x33]);

    cpu.reg_y = 0xbc;
    cpu.execute(IN::STY_A);
    assert_eq!(cpu.memory[0xcdab as usize], 0xbc);

    cpu.reg_y = 0xde;
    cpu.execute(IN::STY_Z);
    assert_eq!(cpu.memory[0x33 as usize], 0xde);
}
