pub mod cpu;
pub mod memory;
pub mod ppu;
pub mod ui;

use chrono::prelude::Utc;
use cpu::CPU;
use memory::{cartridge::Cartridge, memory_bus::MemoryBus};
use ppu::PPU;
use std::{fs::File, io::Write, time};

#[macro_export]
macro_rules! make16 {
    ($hi:expr, $lo:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

pub enum InputType {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

pub struct NESSystem {
    cpu: CPU,
    mem_bus: MemoryBus,
}

impl NESSystem {
    pub fn new(raw_bytes: Vec<u8>) -> Result<Self, &'static str> {
        let mut cpu = CPU::new_program(false);
        let mut mem_bus = MemoryBus::new(PPU::new(Cartridge::new(raw_bytes)?));
        cpu.reset(&mut mem_bus);
        Ok(NESSystem { cpu, mem_bus })
    }

    pub fn get_frame_pixel_buffer(&self) -> [u8; 256 * 240] {
        self.mem_bus.get_frame_pixel_buffer()
    }

    pub fn tick_once(&mut self) {
        self.cpu.run_once(&mut self.mem_bus);
    }

    pub fn tick_n(&mut self, count: usize) {
        self.cpu.run_count(&mut self.mem_bus, count);
    }

    pub fn tick_one_frame(&mut self) {
        let starting_cycles = self.cpu.cycle_count;
        while self.cpu.cycle_count - starting_cycles < 29781 {
            self.cpu.run_once(&mut self.mem_bus);
        }
    }

    pub fn save_log(&self) {
        let mut log_file = File::create(format!(
            "logs/ran_{}.log",
            Utc::now().format("%Y-%m-%d_%H-%M-%S")
        ))
        .unwrap();

        let _ = log_file.write_all(self.cpu.log.as_bytes());
    }
}

pub fn start(raw_bytes: Vec<u8>) {
    // let mut c = cpu::CPU::new_program(true);

    let mut n = NESSystem::new(raw_bytes).unwrap();

    // let mut cart = Cartridge::dummy();

    // let now = time::Instant::now();
    n.cpu.run_count(&mut n.mem_bus, 5000);
    // for _ in 0..10 {
    //     n.cpu.program_counter = 0xc000;
    // }
    // let dur = now.elapsed();
    // let dm = dur.as_millis();
    // let du = dur.as_micros();
    // let frames = (n.cpu.cycle_count as f64) / 29780.5;

    // println!("Time elapsed: {dm} millis = {du} micros\n = ~{} millis = ~{} micros per frame (goal: < 16.6 millis)", (dm as f64) / frames, (du as f64) / frames);
    // for _ in 0..5000 {
    //     let cycles = c.tick();
    //     c.memory.tick_ppu(cycles);
    // }

    // let mut log_file = File::create(format!(
    //     "logs/ran_{}.log",
    //     Utc::now().format("%Y-%m-%d_%H:%M:%S")
    // ))
    // .unwrap();

    // let _ = log_file.write_all(n.cpu.log.as_bytes());

    // println!("{}", c.log);
}
