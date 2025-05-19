pub mod cpu;
pub mod memory;
pub mod ppu;
pub mod ui;

use chrono::prelude::Utc;
use cpu::{instructions::Instruction, CPU};
use memory::{cartridge::Cartridge, memory_bus::MemoryBus};
use ppu::PPU;
use std::{fs::File, io::Write};

#[macro_export]
macro_rules! make_u16 {
    ($hi:expr, $lo:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;

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
}

impl NESSystem {
    pub fn new(raw_bytes: Vec<u8>) -> Result<Self, &'static str> {
        let mem_bus = MemoryBus::new(PPU::new(Cartridge::new(raw_bytes)?));
        let mut cpu = CPU::new_program(false, mem_bus, None);
        cpu.reset();
        Ok(NESSystem { cpu })
    }

    // pub fn get_frame_pixel_buffer(&self) -> [u8; WIDTH * HEIGHT] {
    //     self.cpu.mem_bus.get_frame_pixel_buffer()
    // }

    pub fn render(&self, target: &mut [u8; WIDTH * HEIGHT]) {
        self.cpu.mem_bus.render(target);
    }

    pub fn tick_once(&mut self) {
        self.cpu.run_once();
    }

    pub fn tick_n(&mut self, count: usize) {
        self.cpu.run_count(count);
    }

    pub fn tick_one_frame(&mut self) {
        let starting_cycles = self.cpu.cycle_count;
        while self.cpu.cycle_count - starting_cycles < 29781 {
            self.cpu.run_once();
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

pub enum LogEvent {
    InstructionFetch(Instruction),
    BadOpcode(u8),
    OperandFetch(u16),
    MemoryReadIntermediate(u16, u8),
    MemoryRead(u16, u8),
    MemoryWrite(u8, u16),
    StackPush(u8),
    StackPull(u8),
    NMIInterupt,
    StateUpdate(cpu::CPUStateLog),
}

pub trait Logger: std::fmt::Debug + Send {
    fn log_event(&mut self, le: LogEvent);
    fn log_state(&mut self);
}
