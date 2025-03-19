#![cfg(test)]
use crate::cpu::{cpu::tests::*, instructions::AddressingMode::*, CPU};

#[test]
fn test_set_byte() {
    let mut cpu = CPU::new();

    set_single_byte(&mut cpu, 0x1110, 35);
    set_single_byte(&mut cpu, 0x1111, 15);
    set_single_byte(&mut cpu, 0x1112, 25);
    assert_eq!(cpu.memory.read(0x1110), 35);
    assert_eq!(cpu.memory.read(0x1111), 15);
    assert_eq!(cpu.memory.read(0x1112), 25);
}

#[test]
fn test_set_bytes() {
    let mut cpu = CPU::new();

    let bytes = vec![12, 53, 67, 21, 66, 40];

    set_multiple_bytes(&mut cpu, 500, &bytes);

    for i in 0..bytes.len() {
        assert_eq!(cpu.memory.read(500 + i as u16), bytes[i]);
    }
}

#[test]
fn test_read_everything() {
    let mut cpu = CPU::new();

    for i in 0..0x1fff_u16 {
        let val = i.wrapping_mul(1793) as u8;
        set_single_byte(&mut cpu, i, val);
        assert_eq!(cpu.memory.read(i), val);
    }
}

#[test]
fn test_read_z() {
    let mut cpu = CPU::new();
    set_byte_example(&mut cpu);

    assert_eq!(cpu.read_8bit(10, ZeroPage), get_example_byte(10));
    assert_eq!(cpu.read_8bit(0, ZeroPage), get_example_byte(0));
    assert_eq!(cpu.read_8bit(200, ZeroPage), get_example_byte(200));
}

#[test]
fn test_read_zx() {
    let mut cpu = CPU::new();
    set_byte_example(&mut cpu);
    cpu.reg_x = 62;
    cpu.reg_y = 12;

    assert_eq!(cpu.read_8bit(10, ZeroPageX), get_example_byte(10 + 62));
    assert_eq!(cpu.read_8bit(0, ZeroPageX), get_example_byte(0 + 62));
    assert_eq!(
        cpu.read_8bit(200, ZeroPageX),
        get_example_byte((200 + 62) % 256)
    );
}

#[test]
fn test_read_zy() {
    let mut cpu = CPU::new();
    set_byte_example(&mut cpu);
    cpu.reg_y = 74;
    cpu.reg_x = 65;

    assert_eq!(cpu.read_8bit(10, ZeroPageY), get_example_byte(10 + 74));
    assert_eq!(cpu.read_8bit(0, ZeroPageY), get_example_byte(0 + 74));
    assert_eq!(
        cpu.read_8bit(200, ZeroPageY),
        get_example_byte((200 + 74) % 256)
    );
}

// #[test]
// fn test_read_ix() {
//     let mut cpu = CPU::new();
//     set_byte_example(&mut cpu);
//     cpu.reg_x = 9;
//     cpu.reg_y = 100;

//     let lb = get_example_byte(196 + 9);
//     let hb = get_example_byte(197 + 9);
//     let addr_le = dbg!(((hb as u16) << 8) | (lb as u16));
//     let addr_be = ((lb as u16) << 8) | (hb as u16);

//     assert_eq!(addr_le, cpu.get_addr_8bit(196, IndexedIndirect));

//     assert_eq!(
//         cpu.read_8bit(196, IndexedIndirect),
//         dbg!(get_example_byte(addr_le))
//     );
//     assert_eq!(
//         cpu.read_8bit(196, IndexedIndirect),
//         cpu.read_16bit(addr_le, Absolute)
//     );

//     assert_ne!(
//         cpu.read_8bit(196, IndexedIndirect),
//         get_example_byte(addr_be)
//     );
//     assert_ne!(
//         cpu.read_8bit(196, IndexedIndirect),
//         cpu.read_16bit(addr_be, Absolute)
//     );
// }

// #[test]
// fn test_read_iy() {
//     let mut cpu = CPU::new();
//     set_byte_example(&mut cpu);
//     cpu.reg_y = 11;
//     cpu.reg_x = 90;

//     let lb = get_example_byte(55);
//     let hb = get_example_byte(56);
//     let addr_le = (((hb as u16) << 8) | (lb as u16)) + 11;
//     let addr_be = (((lb as u16) << 8) | (hb as u16)) + 11;

//     assert_eq!(addr_le, cpu.get_addr_8bit(55, IndirectIndexed));

//     assert_eq!(
//         cpu.read_8bit(55, IndirectIndexed),
//         get_example_byte(addr_le)
//     );
//     assert_eq!(
//         cpu.read_8bit(55, IndirectIndexed),
//         cpu.read_16bit(addr_le, Absolute)
//     );

//     assert_ne!(
//         cpu.read_8bit(55, IndirectIndexed),
//         get_example_byte(addr_be)
//     );
//     assert_ne!(
//         cpu.read_8bit(55, IndirectIndexed),
//         cpu.read_16bit(addr_be, Absolute)
//     );
// }

// #[test]
// fn test_read_i() {
//     let mut cpu = CPU::new();
//     set_byte_example(&mut cpu);

//     let lb = get_example_byte(1012);
//     let hb = get_example_byte(1013);
//     let addr_le = ((hb as u16) << 8) | (lb as u16);
//     let addr_be = ((lb as u16) << 8) | (hb as u16);

//     assert_eq!(addr_le, cpu.get_addr_16bit(1012, Indirect));

//     assert_eq!(cpu.read_16bit(1012, Indirect), get_example_byte(addr_le));
//     assert_eq!(
//         cpu.read_16bit(1012, Indirect),
//         cpu.read_16bit(addr_le, Absolute)
//     );

//     assert_ne!(cpu.read_16bit(1012, Indirect), get_example_byte(addr_be));
//     assert_ne!(
//         cpu.read_16bit(1012, Indirect),
//         cpu.read_16bit(addr_be, Absolute)
//     );
// }

#[test]
fn test_read_a() {
    let mut cpu = CPU::new();
    set_byte_example(&mut cpu);

    assert_eq!(2138, cpu.get_addr_16bit(2138, Absolute));

    assert_eq!(cpu.read_16bit(2138, Absolute), get_example_byte(2138));
}

#[test]
fn test_read_ax() {
    let mut cpu = CPU::new();
    set_byte_example(&mut cpu);
    cpu.reg_x = 46;
    cpu.reg_y = 64;

    assert_eq!(2138 + 46, cpu.get_addr_16bit(2138, AbsoluteX));

    assert_eq!(cpu.read_16bit(2138, AbsoluteX), get_example_byte(2138 + 46));
}

#[test]
fn test_read_ay() {
    let mut cpu = CPU::new();
    set_byte_example(&mut cpu);
    cpu.reg_y = 113;
    cpu.reg_x = 46;

    assert_eq!(2138 + 113, cpu.get_addr_16bit(2138, AbsoluteY));

    assert_eq!(
        cpu.read_16bit(2138, AbsoluteY),
        get_example_byte(2138 + 113)
    );
}
