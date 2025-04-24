pub mod cpu;
pub mod memory;
pub mod ppu;
pub mod ui;

use std::{fs::File, io::Write};

use cpu::CPU;
use memory::{cartridge_rom::Cartridge, memory_bus::MemoryBus};
use ppu::PPU;

#[macro_export]
macro_rules! make16 {
    ($hi:expr, $lo:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

struct NESSystem {
    cpu: CPU,
    mem_bus: MemoryBus,
}

impl NESSystem {
    pub fn new(raw_bytes: Vec<u8>) -> Result<Self, &'static str> {
        Ok(NESSystem {
            cpu: CPU::new_program(true),
            mem_bus: MemoryBus::new(PPU::new(Cartridge::new(raw_bytes)?)),
        })
    }

    fn run_once() {
        todo!()
    }
}

pub fn start(raw_bytes: Vec<u8>) {
    // let mut c = cpu::CPU::new_program(true);

    let mut n = NESSystem::new(raw_bytes).unwrap();

    // let mut cart = Cartridge::dummy();

    n.cpu.run_count(&mut n.mem_bus, 5000);
    // for _ in 0..5000 {
    //     let cycles = c.tick();
    //     c.memory.tick_ppu(cycles);
    // }

    let mut log_file = File::create("logs/cpu_complete.log").unwrap();

    let _ = log_file.write_all(n.cpu.log.as_bytes());

    // println!("{}", c.log);
}
