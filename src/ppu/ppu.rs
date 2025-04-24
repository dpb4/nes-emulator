#![allow(non_snake_case)]

use crate::{
    memory::{cartridge_rom::Cartridge, memory_bus::InterruptType},
    ppu::ppu_registers::*,
};

#[derive(Debug)]
pub struct PPU {
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],

    pub cart: Cartridge,

    pub regs: Registers,

    internal_data_buf: u8,

    pub cycle_count: usize,
    pub scanline: u16,

    pub interrupt: Option<InterruptType>,
}

impl PPU {
    pub fn new(cart: Cartridge) -> Self {
        PPU {
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            cart,
            palette_table: [0; 32],
            regs: Registers::new(),
            internal_data_buf: 0,
            cycle_count: 21,
            scanline: 0,
            interrupt: None,
        }
    }

    pub fn tick(&mut self, cycles: usize) -> bool {
        self.cycle_count += cycles;
        if self.cycle_count >= 341 {
            self.cycle_count -= 341;
            self.scanline += 1;

            if self.scanline == 241 && self.regs.ctrl.contains(ControlFlags::VBLANK_NMI_ENABLE) {
                self.regs.stat.insert(StatusFlags::VBLANK);
                self.regs.stat.remove(StatusFlags::SPRITE_0_HIT);
                if self.regs.ctrl.contains(ControlFlags::VBLANK_NMI_ENABLE) {
                    self.interrupt = Some(InterruptType::NonMaskable);
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.interrupt = None;
                self.regs.stat.remove(StatusFlags::SPRITE_0_HIT);
                self.regs.stat.remove(StatusFlags::VBLANK);
                return true;
            }
        }
        return false;
    }

    pub fn write_to_ppu_addr(&mut self, val: u8) {
        self.regs.ppu_addr.write(val);
        self.regs.scrl.toggle_latch();
    }

    pub fn write_to_ctrl(&mut self, val: u8) {
        let nmi_status_before = self.regs.ctrl.contains(ControlFlags::VBLANK_NMI_ENABLE);
        self.regs.ctrl = ControlFlags::from_bits_truncate(val);
        let nmi_status_after = self.regs.ctrl.contains(ControlFlags::VBLANK_NMI_ENABLE);
        if !nmi_status_before && nmi_status_after && self.regs.stat.contains(StatusFlags::VBLANK) {
            self.interrupt = Some(InterruptType::NonMaskable);
        }
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

    pub fn write_oam_dma(&mut self, buffer: &[u8; 256]) {
        for x in buffer.iter() {
            self.oam_data[self.regs.oam_addr as usize] = *x;
            self.regs.oam_addr = self.regs.oam_addr.wrapping_add(1);
        }
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
                self.internal_data_buf = self.cart.chr_rom[addr as usize];
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
        match (&self.cart.screen_mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.regs
            .ppu_addr
            .increment(self.regs.ctrl.get_increment_val());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}
