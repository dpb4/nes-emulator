#![allow(non_snake_case)]

use crate::{
    memory::{cartridge::Cartridge, memory_bus::InterruptType},
    ppu::ppu_registers::*,
    HEIGHT, WIDTH,
};

// pub mod ppu;
pub mod ppu_registers;

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

    pub fn draw_to_buffer(&self, target: &mut [u8; WIDTH * HEIGHT]) {
        self.draw_background(target);
    }

    // pub fn get_frame_pixel_buffer(&self) -> [u8; WIDTH * HEIGHT] {
    //     let mut frame = [0; WIDTH * HEIGHT];
    //     self.draw_background(&mut frame);
    //     // println!("{:?}", frame);
    //     frame
    // }

    fn draw_background(&self, frame: &mut [u8; WIDTH * HEIGHT]) {
        let bank_offset = if self
            .regs
            .ctrl
            .contains(ControlFlags::BACKGROUND_PATTERN_TABLE_ADDR)
        {
            0x1000
        } else {
            0
        };

        for tile_index in 0..960 {
            let pattern_index = self.vram[tile_index] as usize;
            let tile_x = tile_index % 32;
            let tile_y = tile_index / 32;

            let tile_offset = bank_offset + (pattern_index * 16);
            let tile_slice = &self.cart.chr_rom[tile_offset..(tile_offset + 16)];

            let local_chunk_palette = self.get_background_chunk_palette(tile_x, tile_y);

            for local_pix_y in 0..8 {
                let color_bit_hi = tile_slice[local_pix_y];
                let color_bit_lo = tile_slice[local_pix_y + 8];
                for local_pix_x in 0..8 {
                    let bit = 7 - local_pix_x;
                    let color_select =
                        (((color_bit_hi >> bit) & 1) << 1) | ((color_bit_lo >> bit) & 1);
                    let pix_x = tile_x * 8 + local_pix_x;
                    let pix_y = tile_y * 8 + local_pix_y;
                    frame[pix_x + WIDTH * pix_y] = local_chunk_palette[color_select as usize];
                }
            }
        }
    }

    fn get_background_chunk_palette(&self, tile_x: usize, tile_y: usize) -> [u8; 4] {
        // chunk layout:
        // A A | B B
        // A A | B B
        // ---------
        // C C | D D
        // C C | D D

        // the attrib table has chunks of 4x4 tiles, so find 4x4 block of given coord
        let attrib_table_index = (tile_x / 4) + (8 * (tile_y / 4));
        let attrib_byte = self.vram[960 + attrib_table_index];

        // split into 2x2 grid within 4x4 chunk
        let pallet_index = match ((tile_x % 4) / 2, (tile_y % 4) / 2) {
            (0, 0) => (attrib_byte >> 0) & 0b11,
            (1, 0) => (attrib_byte >> 2) & 0b11,
            (0, 1) => (attrib_byte >> 4) & 0b11,
            (1, 1) => (attrib_byte >> 6) & 0b11,
            _ => unreachable!(),
        };

        // pallette table has weird layout for 13 colours:
        // [UBG, {0,1,2}, {3,4,5}, {6,7,8}, {9,10,11}]
        let pallete_start: usize = 1 + (pallet_index as usize) * 4;
        [
            self.palette_table[0],
            self.palette_table[pallete_start],
            self.palette_table[pallete_start + 1],
            self.palette_table[pallete_start + 2],
        ]
    }

    pub fn tick(&mut self, cycles: usize) -> bool {
        self.cycle_count += cycles;
        if self.cycle_count >= 341 {
            self.cycle_count -= 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.regs.stat.insert(StatusFlags::VBLANK);
                self.regs.stat.remove(StatusFlags::SPRITE_0_HIT);
                if self.regs.ctrl.contains(ControlFlags::VBLANK_NMI_ENABLE) {
                    self.interrupt = Some(InterruptType::NonMaskable);
                }
                // println!("PPU palette table: {:?}", self.palette_table);
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
        // if data & 0b10000000 != 0 {
        //     println!("STATUS IS NEGATIVE!!!\n\n\n");
        // }
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
