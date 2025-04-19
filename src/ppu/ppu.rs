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
    pub scrl: PPUScrollRegister,
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
            scrl: PPUScrollRegister::new(),
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
            chr_rom,
            mirroring: mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            regs: Registers::new(),
            internal_data_buf: 0,
        }
    }

    pub fn write_to_ppu_addr(&mut self, val: u8) {
        self.regs.ppu_addr.write(val);
        self.regs.scrl.toggle_latch();
    }

    pub fn write_to_ctrl(&mut self, val: u8) {
        self.regs.ctrl = ControlFlags::from_bits_truncate(val);
    }

    pub fn write_to_mask(&mut self, val: u8) {
        self.regs.mask = MaskFlags::from_bits_truncate(val);
    }

    pub fn write_to_oam_addr(&mut self, val: u8) {
        self.regs.oam_addr = val;
    }

    pub fn write_to_oam_data(&mut self, val: u8) {
        self.oam_data[self.regs.oam_addr as usize] = val;
        self.regs.oam_addr = self.regs.oam_addr.wrapping_add(1);
        dbg!("warning: writing to PPU oam_data; this shouldn't really happen");
    }

    pub fn write_to_scrl(&mut self, val: u8) {
        self.regs.scrl.write(val);
        self.regs.ppu_addr.toggle_latch();
    }

    pub fn write_to_data(&mut self, val: u8) {
        let addr = self.regs.ppu_addr.get_addr();
        self.increment_vram_addr();

        match addr {
            // TODO make these into constants?
            0..=0x1fff => {
                panic!("attempting to write to chr rom at 0x{:x}", addr);
            }

            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = val;
            }

            0x3000..=0x3eff => {
                panic!(
                    "attempting to write to unused ppu memory (0x3000..0x3eff) 0x{:x}",
                    addr
                )
            }

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let addr_mirror = addr - 0x10;
                self.palette_table[(addr_mirror - 0x3f00) as usize] = val;
            }

            0x3f00..=0x3fff => {
                self.palette_table[(addr - 0x3f00) as usize] = val;
            }
            _ => panic!("bad write to ppu data at 0x{:x}", addr),
        }
    }

    fn increment_vram_addr(&mut self) {
        self.regs
            .ppu_addr
            .increment(self.regs.ctrl.get_increment_val());
    }

    pub fn read_oam_data(&self) -> u8 {
        self.regs.oam_data
    }

    pub fn read_status(&mut self) -> u8 {
        let data = self.regs.stat.bits();
        self.regs.stat.remove(StatusFlags::VBLANK);
        self.regs.ppu_addr.reset_latch();
        self.regs.scrl.reset_latch();
        data
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.regs.ppu_addr.get_addr();
        self.increment_vram_addr();

        match addr {
            // TODO make these into constants?
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
                let addr_mirror = addr - 0x10;
                self.palette_table[(addr_mirror - 0x3f00) as usize]
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
    FourScreen,
}

#[derive(Debug)]
pub struct PPUAddressRegister {
    value: (u8, u8),
    write_to_hi_ptr: bool,
}

impl PPUAddressRegister {
    pub fn new() -> Self {
        PPUAddressRegister {
            value: (0, 0), // high byte first, lo byte second
            write_to_hi_ptr: true,
        }
    }
    fn set_addr(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn write(&mut self, data: u8) {
        if self.write_to_hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get_addr() > 0x3fff {
            self.set_addr(self.get_addr() & 0x3fff);
        }
        self.toggle_latch();
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
        self.write_to_hi_ptr = true;
    }

    pub fn toggle_latch(&mut self) {
        self.write_to_hi_ptr = !self.write_to_hi_ptr;
    }

    pub fn get_addr(&self) -> u16 {
        make16!(self.value.0, self.value.1)
    }
}

#[derive(Debug)]
pub struct PPUScrollRegister {
    x_scroll: u8,
    y_scroll: u8,
    write_to_x: bool,
}

impl PPUScrollRegister {
    pub fn new() -> Self {
        Self {
            x_scroll: 0,
            y_scroll: 0,
            write_to_x: true,
        }
    }

    pub fn write(&mut self, val: u8) {
        if self.write_to_x {
            self.x_scroll = val;
        } else {
            self.y_scroll = val;
        }
        self.toggle_latch();
    }

    pub fn reset_latch(&mut self) {
        self.write_to_x = true;
    }

    pub fn toggle_latch(&mut self) {
        self.write_to_x = !self.write_to_x;
    }
}
