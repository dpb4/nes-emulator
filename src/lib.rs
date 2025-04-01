pub mod cpu;
pub mod memory;
pub mod ppu;

use std::{fs::File, io::Write};

#[macro_export]
macro_rules! make16 {
    ($hi:expr, $lo:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

pub fn start(raw_bytes: Vec<u8>) {
    let mut c = if raw_bytes.len() == 0 {
        cpu::CPU::new()
    } else {
        cpu::CPU::new_program(raw_bytes, true)
    };

    for _ in 0..5000 {
        c.tick();
    }

    let mut file = File::create("logs/cpu.log").unwrap();

    let _ = file.write_all(c.log.as_bytes());

    // println!("{}", c.log);
}
