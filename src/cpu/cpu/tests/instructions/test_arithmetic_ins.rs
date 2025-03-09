// ADC, SBC, INC, DEC, INC, DEX, INY, DEY
use crate::cpu::{cpu::tests::set_multiple_bytes, instructions, Flag, CPU};
use instructions as IN;

#[test]
fn test_inc() {
    let mut cpu = CPU::new();
    set_multiple_bytes(
        &mut cpu,
        0,
        &vec![0, 0x34, 0x12, 0, 0x35, 0x12, 0, 0x36, 0x12, 0, 0x37, 0x12],
    );
    set_multiple_bytes(&mut cpu, 0x1234, &vec![0x11, 0, 0x88, 0xff]);

    cpu.execute(IN::INC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.memory[0x1234], 0x12);

    cpu.execute(IN::INC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.memory[0x1235], 0x1);

    cpu.execute(IN::INC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.memory[0x1236], 0x89);

    cpu.execute(IN::INC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.memory[0x1237], 0);
}

#[test]
fn test_inx() {
    let mut cpu = CPU::new();

    cpu.reg_x = 0x11;

    cpu.execute(IN::INX);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_x, 0x12);

    cpu.reg_x = 0x0;

    cpu.execute(IN::INX);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_x, 0x1);

    cpu.reg_x = 0x88;

    cpu.execute(IN::INX);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.reg_x, 0x89);

    cpu.reg_x = 0xff;

    cpu.execute(IN::INX);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_x, 0x0);
}

#[test]
fn test_iny() {
    let mut cpu = CPU::new();

    cpu.reg_y = 0x11;

    cpu.execute(IN::INY);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_y, 0x12);

    cpu.reg_y = 0x0;

    cpu.execute(IN::INY);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_y, 0x1);

    cpu.reg_y = 0x88;

    cpu.execute(IN::INY);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.reg_y, 0x89);

    cpu.reg_y = 0xff;

    cpu.execute(IN::INY);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_y, 0x0);
}

#[test]
fn test_dec() {
    let mut cpu = CPU::new();
    set_multiple_bytes(
        &mut cpu,
        0,
        &vec![0, 0x34, 0x12, 0, 0x35, 0x12, 0, 0x36, 0x12, 0, 0x37, 0x12],
    );
    set_multiple_bytes(&mut cpu, 0x1234, &vec![0x11, 0, 0x1, 0xff]);

    cpu.execute(IN::DEC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.memory[0x1234], 0x10);

    cpu.execute(IN::DEC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.memory[0x1235], 0xff);

    cpu.execute(IN::DEC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.memory[0x1236], 0x0);

    cpu.execute(IN::DEC_A);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.memory[0x1237], 0xfe);
}

#[test]
fn test_dex() {
    let mut cpu = CPU::new();

    cpu.reg_x = 0x11;

    cpu.execute(IN::DEX);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_x, 0x10);

    cpu.reg_x = 0x0;

    cpu.execute(IN::DEX);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.reg_x, 0xff);

    cpu.reg_x = 0x1;

    cpu.execute(IN::DEX);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_x, 0x0);

    cpu.reg_x = 0xff;

    cpu.execute(IN::DEX);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.reg_x, 0xfe);
}

#[test]
fn test_dey() {
    let mut cpu = CPU::new();

    cpu.reg_y = 0x11;

    cpu.execute(IN::DEY);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_y, 0x10);

    cpu.reg_y = 0x0;

    cpu.execute(IN::DEY);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.reg_y, 0xff);

    cpu.reg_y = 0x1;

    cpu.execute(IN::DEY);
    assert_eq!(cpu.get_flag(Flag::Zero), 1);
    assert_eq!(cpu.get_flag(Flag::Negative), 0);
    assert_eq!(cpu.reg_y, 0x0);

    cpu.reg_y = 0xff;

    cpu.execute(IN::DEY);
    assert_eq!(cpu.get_flag(Flag::Zero), 0);
    assert_eq!(cpu.get_flag(Flag::Negative), 1);
    assert_eq!(cpu.reg_y, 0xfe);
}
