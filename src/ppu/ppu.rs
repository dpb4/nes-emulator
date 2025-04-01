#![allow(non_snake_case)]

use crate::make16;

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],

    pub mirroring: Mirroring,
    addr: AddrRegister,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom: chr_rom,
            mirroring: mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            addr: AddrRegister::new(),
        }
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }
}

pub enum Mirroring {
    Horizontal,
    Vertical,
}

pub struct AddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            value: (0, 0), // high byte first, lo byte second
            hi_ptr: true,
        }
    }
    fn set_addr(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get_addr() > 0x3fff {
            self.set_addr(self.get_addr() & 0x3fff);
        }
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }
        if self.get_addr() > 0x3fff {
            self.set_addr(self.get_addr() & 0x3fff);
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    pub fn get_addr(&self) -> u16 {
        make16!(self.value.0, self.value.1)
    }
}
