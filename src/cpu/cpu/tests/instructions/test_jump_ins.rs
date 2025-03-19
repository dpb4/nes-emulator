// JMP, JSR, RTS, BRK, RTI

use crate::cpu::{
    cpu::{tests::set_multiple_bytes, STACK_START},
    instructions, CPU,
};
use instructions as IN;

#[test]
fn test_jmp() {
    let mut cpu = CPU::new();

    set_multiple_bytes(&mut cpu, 0, &vec![0, 0x11, 0x02]);
    set_multiple_bytes(&mut cpu, 0x0211, &vec![0x20, 0x05]);
    set_multiple_bytes(&mut cpu, 0x0521, &vec![0xff, 0x03]);
    set_multiple_bytes(&mut cpu, 0x0300, &vec![0x02, 0x14]);
    set_multiple_bytes(&mut cpu, 0x03ff, &vec![0x56, 0x48]);
    set_multiple_bytes(&mut cpu, 0x0257, &vec![0x56, 0x08]);

    cpu.execute(IN::JMP_I);
    assert_eq!(cpu.program_counter, 0x0520);

    cpu.execute(IN::JMP_I);
    assert_eq!(cpu.program_counter, 0x0256);

    cpu.execute(IN::JMP_A);
    assert_eq!(cpu.program_counter, 0x0856);
}

#[test]
fn test_jsr_and_rts() {
    let mut cpu = CPU::new();

    cpu.program_counter = 0x1233;
    cpu.stack_pointer = 255;

    set_multiple_bytes(&mut cpu, 0x1234, &vec![0x25, 0x20, 0x54, 0x32]);

    cpu.execute(IN::JSR_A);
    assert_eq!(cpu.program_counter, 0x2025);
    assert_eq!(cpu.stack_pointer, 253);
    assert_eq!(
        cpu.memory.read(STACK_START + cpu.stack_pointer as u16 + 1),
        0x35
    );
    assert_eq!(
        cpu.memory.read(STACK_START + cpu.stack_pointer as u16 + 2),
        0x12
    );

    cpu.execute(IN::RTS);
    assert_eq!(cpu.program_counter, 0x1236);
    assert_eq!(cpu.stack_pointer, 255);
}
