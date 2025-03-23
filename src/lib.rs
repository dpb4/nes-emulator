pub mod cpu;
pub mod memory;

use std::{fs::File, io::Write};

pub fn start(raw_bytes: Vec<u8>) {
    let mut c = if raw_bytes.len() == 0 {
        cpu::CPU::new()
    } else {
        cpu::CPU::new_program(raw_bytes, true)
    };

    for _ in 0..500 {
        c.tick();
    }

    let mut file = File::create("logs/cpu.log").unwrap();

    let _ = file.write_all(c.log.as_bytes());

    // println!("{}", c.log);
}
