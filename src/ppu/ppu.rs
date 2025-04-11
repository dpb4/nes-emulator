#![allow(non_snake_case)]

use bitflags::bitflags;

use crate::make16;

bitflags! {
    #[derive(Debug)]
    pub struct ControlFlags: u8 {
        const BASE_NAMETABLE_ADDR_LO = 0b00000001;
        const BASE_NAMETABLE_ADDR_HI = 0b00000010;
        const VRAM_ADDR_INCREMENT = 0b00000100;
        const SPRITE_PATTERN_TABLE_ADDR = 0b00001000;
        const BACKGROUND_PATTERN_TABLE_ADDR = 0b00010000;
        const SPRITE_SIZE = 0b00100000;
        const MASTER_SLAVE_SELECT = 0b01000000;
        const VBLANK_NMI_ENABLE = 0b10000000;
    }
}

impl ControlFlags {
    pub fn get_increment_val(&self) -> u8 {
        if self.contains(ControlFlags::VRAM_ADDR_INCREMENT) {
            32
        } else {
            1
        }
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct MaskFlags: u8 {
        const GREYSCALE_ENABLE = 0b00000001;
        const LEFT_EDGE_BACKGROUND = 0b00000010;
        const LEFT_EDGE_SPRITE= 0b00000100;
        const BACKGROUND_ENABLE = 0b00001000;
        const SPRITE_ENABLE = 0b00010000;
        const EMPHASIZE_RED = 0b00100000;
        const EMPHASIZE_GREEN = 0b01000000;
        const EMPHASIZE_BLUE = 0b10000000;
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct StatusFlags: u8 {
        const SPRITE_OVERFLOW = 0b00100000;
        const SPRITE_0_HIT = 0b01000000;
        const VBLANK = 0b10000000;
    }
}

#[derive(Debug)]
pub struct Registers {
    pub ctrl: ControlFlags,
    pub mask: MaskFlags,
    pub stat: StatusFlags,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub scrl: u8,
    pub ppu_addr: PPUAddressRegister,
    pub ppu_data: u8,
    pub oam_dma: u8,
}

impl Registers {
    fn new() -> Self {
        Self {
            ctrl: ControlFlags::empty(),
            mask: MaskFlags::empty(),
            stat: StatusFlags::empty(),
            oam_addr: 0,
            oam_data: 0,
            scrl: 0,
            ppu_addr: PPUAddressRegister::new(),
            ppu_data: 0,
            oam_dma: 0,
        }
    }
}

#[derive(Debug)]
pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],

    pub mirroring: Mirroring,

    pub regs: Registers,

    internal_data_buf: u8,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom: chr_rom,
            mirroring: mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            regs: Registers::new(),
            internal_data_buf: 0,
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.regs.ppu_addr.update(value);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        self.regs.ctrl = ControlFlags::from_bits_truncate(value);
    }

    fn increment_vram_addr(&mut self) {
        self.regs
            .ppu_addr
            .increment(self.regs.ctrl.get_increment_val());
    }

    fn read_data(&mut self) -> u8 {
        let addr = self.regs.ppu_addr.get_addr();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                addr
            ),

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }

            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        // code from: https://github.com/bugzmanov/nes_ebook/blob/master/code/ch6.1/src/ppu/mod.rs

        // Horizontal:      Vertical:
        //   [ A ] [ a ]      [ A ] [ B ]
        //   [ B ] [ b ]      [ a ] [ b ]
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
}

#[derive(Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct PPUAddressRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl PPUAddressRegister {
    pub fn new() -> Self {
        PPUAddressRegister {
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
